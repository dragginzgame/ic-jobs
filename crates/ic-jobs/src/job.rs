use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{MissedRunPolicy, RetryPolicy, Schedule};

/// Application-assigned identity. Never reuse it within a retained job history.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct JobId(pub u128);

/// Stable logical effect identity, shared by retries of the same occurrence.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExecutionId {
    /// Owning job.
    pub job: JobId,
    /// Zero-based successful-run sequence.
    pub occurrence: u64,
}

/// Identity of one dispatch. Completion must match the current attempt exactly.
///
/// This is a correlation value, not an authorization credential.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Attempt {
    /// Logical effect identity for destination-side idempotency.
    pub execution: ExecutionId,
    /// Strictly increasing dispatch sequence within this job.
    pub sequence: u64,
}

/// Current durable state of a job.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum JobState {
    /// Eligible at or after the supplied time.
    Pending {
        /// Next dispatch deadline, including retry backoff.
        due_ns: u64,
    },
    /// Dispatch intent exists; its result has not been recorded.
    Running {
        /// Time at which this attempt started.
        started_ns: u64,
    },
    /// The effect may have occurred. Automatic dispatch is blocked.
    Uncertain {
        /// Time at which the unresolved attempt started.
        started_ns: u64,
    },
    /// A one-shot job succeeded.
    Completed,
    /// A permanent failure or exhausted retry budget stopped the job.
    Failed,
    /// Pending work was cancelled.
    Cancelled,
}

/// Application classification of an attempt's result.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Outcome {
    /// The effect succeeded.
    Success,
    /// Failure is known and repeating the effect is safe.
    RetryableFailure,
    /// The job should terminate.
    PermanentFailure,
    /// The effect's result is unknown; reconciliation is required.
    Uncertain,
}

/// Canonical, fixed-size metadata to persist beside the application's payload.
///
/// Decode within your storage boundary and pass through [Job::restore].
/// The application owns atomic writes, access control and retained-history identity.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct JobRecord {
    /// Application-assigned job identity.
    pub id: JobId,
    /// Original scheduling policy.
    pub schedule: Schedule,
    /// Retry policy for each occurrence.
    pub retry: RetryPolicy,
    /// Number of dispatches allocated so far.
    pub sequence: u64,
    /// Number of successful recurring occurrences.
    pub occurrence: u64,
    /// Dispatch count for the current occurrence.
    pub attempts: u32,
    /// Original deadline of the current occurrence, before retry backoff.
    pub scheduled_ns: u64,
    /// Current execution state.
    pub state: JobState,
    /// Time of the last transition; clocks must not move behind it.
    pub updated_ns: u64,
    /// Latest recorded outcome; this is not a full execution history.
    pub last_outcome: Option<Outcome>,
}

/// Checked transitions of one application-owned durable job.
///
/// Persist each successful transition. No method performs an external effect.
/// Errors leave the record unchanged.
#[derive(Debug)]
pub struct Job(JobRecord);

/// A rejected configuration, stored record or state transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JobError {
    /// A recurring interval was zero.
    ZeroInterval,
    /// Retry budget or delays were invalid.
    InvalidRetryPolicy,
    /// A timestamp calculation exceeded the nanosecond range.
    DeadlineOverflow,
    /// A dispatch or occurrence counter cannot advance.
    CounterExhausted,
    /// Stored fields contradict the job contract.
    InvalidRecord,
    /// Dispatch was requested before its deadline.
    NotDue,
    /// This operation requires pending work.
    NotPending,
    /// The supplied completion does not identify the active dispatch.
    StaleAttempt,
    /// The job has an unresolved effect and cannot be cancelled.
    InFlight,
    /// A supplied timestamp precedes the previous transition.
    TimeWentBackwards,
    /// Explicit reconciliation requires an uncertain attempt.
    NotUncertain,
}

impl fmt::Display for JobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::ZeroInterval => "recurring interval must be positive",
            Self::InvalidRetryPolicy => "invalid retry budget or delay",
            Self::DeadlineOverflow => "job deadline overflow",
            Self::CounterExhausted => "job counter exhausted",
            Self::InvalidRecord => "inconsistent stored job record",
            Self::NotDue => "job is not due",
            Self::NotPending => "job is not pending",
            Self::StaleAttempt => "completion does not match the active attempt",
            Self::InFlight => "job has an unresolved effect",
            Self::TimeWentBackwards => "time precedes the previous transition",
            Self::NotUncertain => "job is not awaiting reconciliation",
        };
        f.write_str(message)
    }
}

impl std::error::Error for JobError {}

impl Job {
    /// Create a pending job with an application-assigned identity.
    pub fn new(id: JobId, schedule: Schedule, retry: RetryPolicy) -> Result<Self, JobError> {
        schedule.validate()?;
        retry.validate()?;
        Ok(Self(JobRecord {
            id,
            schedule,
            retry,
            sequence: 0,
            occurrence: 0,
            attempts: 0,
            scheduled_ns: schedule.first(),
            state: JobState::Pending {
                due_ns: schedule.first(),
            },
            updated_ns: 0,
            last_outcome: None,
        }))
    }

    /// Validate a decoded record without changing its execution state.
    ///
    /// Running and uncertain records remain blocked after reconstruction.
    /// Restoring does not grant permission to repeat an external effect.
    pub fn restore(record: JobRecord) -> Result<Self, JobError> {
        record.schedule.validate()?;
        record.retry.validate()?;
        let invalid = || Err(JobError::InvalidRecord);
        if record.attempts > record.retry.max_attempts
            || record.scheduled_ns < record.schedule.first()
            || (record.occurrence == 0 && record.scheduled_ns != record.schedule.first())
            || record.sequence < u64::from(record.attempts)
            || record
                .sequence
                .checked_sub(u64::from(record.attempts))
                .is_none_or(|n| n < record.occurrence)
        {
            return invalid();
        }
        match record.schedule {
            Schedule::Once { at_ns } if record.occurrence != 0 || record.scheduled_ns != at_ns => {
                return invalid();
            }
            Schedule::FixedRate {
                first_at_ns,
                every_ns,
                missed,
            } => {
                let delta = record.scheduled_ns - first_at_ns;
                if !delta.is_multiple_of(every_ns)
                    || delta / every_ns < record.occurrence
                    || (missed == MissedRunPolicy::CatchUp && delta / every_ns != record.occurrence)
                {
                    return invalid();
                }
            }
            Schedule::AfterCompletion {
                first_at_ns,
                every_ns,
            } if record
                .occurrence
                .checked_mul(every_ns)
                .and_then(|n| first_at_ns.checked_add(n))
                .is_none_or(|n| n > record.scheduled_ns) =>
            {
                return invalid();
            }
            _ => {}
        }
        if record.sequence == 0 {
            if record.occurrence != 0
                || record.attempts != 0
                || record.last_outcome.is_some()
                || record.scheduled_ns != record.schedule.first()
                || !matches!(record.state, JobState::Pending { .. } | JobState::Cancelled)
            {
                return invalid();
            }
        } else if record.attempts == 0
            && (record.occurrence == 0 || record.last_outcome != Some(Outcome::Success))
        {
            return invalid();
        }
        let completion_floor_ns = if record.sequence > 0 && record.attempts == 0 {
            let every_ns = match record.schedule {
                Schedule::FixedRate { every_ns, .. }
                | Schedule::AfterCompletion { every_ns, .. } => every_ns,
                Schedule::Once { .. } => return invalid(),
            };
            // The next deadline minus its interval is the prior completion
            // (AfterCompletion) or its earliest possible time (FixedRate).
            let floor = record
                .scheduled_ns
                .checked_sub(every_ns)
                .ok_or(JobError::InvalidRecord)?;
            if record.updated_ns < floor {
                return invalid();
            }
            Some(floor)
        } else {
            None
        };
        match record.state {
            JobState::Pending { due_ns } => {
                if due_ns < record.scheduled_ns
                    || (record.attempts == 0 && due_ns != record.scheduled_ns)
                    || (record.attempts > 0
                        && (record.attempts >= record.retry.max_attempts
                            || record.updated_ns < record.scheduled_ns
                            || record.last_outcome != Some(Outcome::RetryableFailure)
                            || record
                                .updated_ns
                                .checked_add(record.retry.delay(record.attempts))
                                != Some(due_ns)))
                {
                    return invalid();
                }
                if let Some(completion_floor_ns) = completion_floor_ns {
                    // Only successful recurrence produces another zero-attempt
                    // pending record. Reuse the transition's deadline calculation.
                    if record
                        .schedule
                        .successor(completion_floor_ns, record.updated_ns)
                        .ok()
                        != Some(Some(record.scheduled_ns))
                    {
                        return invalid();
                    }
                }
            }
            JobState::Running { started_ns } | JobState::Uncertain { started_ns } => {
                let previous_outcome = if record.attempts > 1 {
                    Some(Outcome::RetryableFailure)
                } else if record.occurrence > 0 {
                    Some(Outcome::Success)
                } else {
                    None
                };
                if record.attempts == 0
                    || started_ns < record.scheduled_ns
                    || started_ns > record.updated_ns
                    || (matches!(record.state, JobState::Running { .. })
                        && started_ns != record.updated_ns)
                    || (matches!(record.state, JobState::Running { .. })
                        && record.last_outcome != previous_outcome)
                    || (matches!(record.state, JobState::Uncertain { .. })
                        && record.last_outcome != Some(Outcome::Uncertain))
                {
                    return invalid();
                }
            }
            JobState::Completed => {
                if !matches!(record.schedule, Schedule::Once { .. })
                    || record.attempts == 0
                    || record.updated_ns < record.scheduled_ns
                    || record.last_outcome != Some(Outcome::Success)
                {
                    return invalid();
                }
            }
            JobState::Failed => {
                if record.attempts == 0
                    || record.updated_ns < record.scheduled_ns
                    || !(record.last_outcome == Some(Outcome::PermanentFailure)
                        || (record.last_outcome == Some(Outcome::RetryableFailure)
                            && record.attempts == record.retry.max_attempts))
                {
                    return invalid();
                }
            }
            JobState::Cancelled => {
                if record.attempts > 0
                    && (record.attempts >= record.retry.max_attempts
                        || record.last_outcome != Some(Outcome::RetryableFailure)
                        || record.updated_ns < record.scheduled_ns)
                {
                    return invalid();
                }
            }
        }
        Ok(Self(record))
    }

    /// Return the metadata the application must persist.
    #[must_use]
    pub const fn record(&self) -> JobRecord {
        self.0
    }

    /// Return the next dispatch deadline, excluding unresolved and terminal jobs.
    #[must_use]
    pub const fn next_due_ns(&self) -> Option<u64> {
        match self.0.state {
            JobState::Pending { due_ns } => Some(due_ns),
            _ => None,
        }
    }

    /// Return the correlation value of an unresolved attempt.
    #[must_use]
    pub const fn active_attempt(&self) -> Option<Attempt> {
        match self.0.state {
            JobState::Running { .. } | JobState::Uncertain { .. } => Some(self.attempt()),
            _ => None,
        }
    }

    const fn attempt(&self) -> Attempt {
        Attempt {
            execution: ExecutionId {
                job: self.0.id,
                occurrence: self.0.occurrence,
            },
            sequence: self.0.sequence,
        }
    }

    fn check_time(&self, now_ns: u64) -> Result<(), JobError> {
        if now_ns < self.0.updated_ns {
            Err(JobError::TimeWentBackwards)
        } else {
            Ok(())
        }
    }

    /// Allocate dispatch intent. Commit the resulting record before external work.
    ///
    /// The application must not reuse this occurrence's effect identity with a
    /// changed payload, including after an upgrade.
    pub fn start(&mut self, now_ns: u64) -> Result<Attempt, JobError> {
        self.check_time(now_ns)?;
        let JobState::Pending { due_ns } = self.0.state else {
            return Err(JobError::NotPending);
        };
        if now_ns < due_ns {
            return Err(JobError::NotDue);
        }
        let sequence = self
            .0
            .sequence
            .checked_add(1)
            .ok_or(JobError::CounterExhausted)?;
        let attempts = self
            .0
            .attempts
            .checked_add(1)
            .ok_or(JobError::CounterExhausted)?;
        if attempts > self.0.retry.max_attempts {
            return Err(JobError::InvalidRecord);
        }
        self.0.sequence = sequence;
        self.0.attempts = attempts;
        self.0.state = JobState::Running { started_ns: now_ns };
        self.0.updated_ns = now_ns;
        Ok(self.attempt())
    }

    /// Record the exact running attempt's result.
    ///
    /// Classify lost replies as uncertain. A retryable failure asserts repeating
    /// the effect is safe. Counter/deadline errors leave dispatch intent intact.
    pub fn finish(
        &mut self,
        attempt: Attempt,
        now_ns: u64,
        outcome: Outcome,
    ) -> Result<(), JobError> {
        if !matches!(self.0.state, JobState::Running { .. }) || attempt != self.attempt() {
            return Err(JobError::StaleAttempt);
        }
        self.transition(now_ns, outcome)
    }

    /// Explicitly resolve an uncertain effect after application reconciliation.
    ///
    /// Success records a confirmed effect. RetryableFailure asserts retry safety;
    /// PermanentFailure records a terminal disposition. Uncertain keeps it blocked.
    pub fn resolve(
        &mut self,
        attempt: Attempt,
        now_ns: u64,
        outcome: Outcome,
    ) -> Result<(), JobError> {
        if !matches!(self.0.state, JobState::Uncertain { .. }) {
            return Err(JobError::NotUncertain);
        }
        if attempt != self.attempt() {
            return Err(JobError::StaleAttempt);
        }
        self.transition(now_ns, outcome)
    }

    fn transition(&mut self, now_ns: u64, outcome: Outcome) -> Result<(), JobError> {
        self.check_time(now_ns)?;
        let mut next = self.0;
        next.updated_ns = now_ns;
        next.last_outcome = Some(outcome);
        match outcome {
            Outcome::Success => {
                if let Some(deadline) = next.schedule.successor(next.scheduled_ns, now_ns)? {
                    next.occurrence = next
                        .occurrence
                        .checked_add(1)
                        .ok_or(JobError::CounterExhausted)?;
                    next.attempts = 0;
                    next.scheduled_ns = deadline;
                    next.state = JobState::Pending { due_ns: deadline };
                } else {
                    next.state = JobState::Completed;
                }
            }
            Outcome::RetryableFailure if next.attempts < next.retry.max_attempts => {
                let due_ns = now_ns
                    .checked_add(next.retry.delay(next.attempts))
                    .ok_or(JobError::DeadlineOverflow)?;
                next.state = JobState::Pending { due_ns };
            }
            Outcome::RetryableFailure | Outcome::PermanentFailure => next.state = JobState::Failed,
            Outcome::Uncertain => {
                let started_ns = match next.state {
                    JobState::Running { started_ns } | JobState::Uncertain { started_ns } => {
                        started_ns
                    }
                    _ => return Err(JobError::StaleAttempt),
                };
                next.state = JobState::Uncertain { started_ns };
            }
        }
        self.0 = next;
        Ok(())
    }

    /// Cancel pending work. An unresolved effect must be reconciled first.
    pub fn cancel(&mut self, now_ns: u64) -> Result<(), JobError> {
        self.check_time(now_ns)?;
        match self.0.state {
            JobState::Running { .. } | JobState::Uncertain { .. } => {
                return Err(JobError::InFlight);
            }
            JobState::Pending { .. } => {}
            _ => return Err(JobError::NotPending),
        }
        self.0.state = JobState::Cancelled;
        self.0.updated_ns = now_ns;
        Ok(())
    }
}
