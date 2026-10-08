use serde::{Deserialize, Serialize};

use crate::JobError;

/// What a fixed-rate schedule does when several occurrences are overdue.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MissedRunPolicy {
    /// Advance to the first aligned deadline strictly after completion.
    Skip,
    /// Run one overdue occurrence at a time; consumers bound each work batch.
    CatchUp,
}

/// An absolute schedule. All timestamps and durations are nanoseconds.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Schedule {
    /// Run one occurrence.
    Once {
        /// Earliest dispatch time.
        at_ns: u64,
    },
    /// Run at fixed intervals relative to the original deadline.
    FixedRate {
        /// First occurrence's deadline.
        first_at_ns: u64,
        /// Positive interval between scheduled occurrences.
        every_ns: u64,
        /// Policy for overdue occurrences.
        missed: MissedRunPolicy,
    },
    /// Wait a fixed interval after each successful completion.
    AfterCompletion {
        /// First occurrence's deadline.
        first_at_ns: u64,
        /// Positive delay after successful completion.
        every_ns: u64,
    },
}

impl Schedule {
    pub(crate) fn validate(self) -> Result<(), JobError> {
        match self {
            Self::FixedRate { every_ns: 0, .. } | Self::AfterCompletion { every_ns: 0, .. } => {
                Err(JobError::ZeroInterval)
            }
            _ => Ok(()),
        }
    }

    pub(crate) const fn first(self) -> u64 {
        match self {
            Self::Once { at_ns } => at_ns,
            Self::FixedRate { first_at_ns, .. } | Self::AfterCompletion { first_at_ns, .. } => {
                first_at_ns
            }
        }
    }

    pub(crate) fn successor(
        self,
        scheduled_ns: u64,
        finished_ns: u64,
    ) -> Result<Option<u64>, JobError> {
        let next = match self {
            Self::Once { .. } => return Ok(None),
            Self::AfterCompletion { every_ns, .. } => finished_ns.checked_add(every_ns),
            Self::FixedRate {
                every_ns,
                missed: MissedRunPolicy::CatchUp,
                ..
            } => scheduled_ns.checked_add(every_ns),
            Self::FixedRate {
                every_ns,
                missed: MissedRunPolicy::Skip,
                ..
            } => {
                let steps = (finished_ns - scheduled_ns) / every_ns;
                steps
                    .checked_add(1)
                    .and_then(|steps| steps.checked_mul(every_ns))
                    .and_then(|delta| scheduled_ns.checked_add(delta))
            }
        };
        next.map(Some).ok_or(JobError::DeadlineOverflow)
    }
}

/// A bounded retry budget for each occurrence, including its first attempt.
///
/// Exponential delays double after each failure and stop growing at the cap.
/// The same execution identity is retained across retries.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RetryPolicy {
    /// Total allowed attempts, including the initial dispatch. Must be positive.
    pub max_attempts: u32,
    /// Positive first retry delay.
    pub initial_delay_ns: u64,
    /// Maximum retry delay, at least the initial delay.
    pub max_delay_ns: u64,
}

impl RetryPolicy {
    /// One attempt with no automatic retries.
    pub const NONE: Self = Self {
        max_attempts: 1,
        initial_delay_ns: 1,
        max_delay_ns: 1,
    };

    pub(crate) fn validate(self) -> Result<(), JobError> {
        if self.max_attempts == 0
            || self.initial_delay_ns == 0
            || self.max_delay_ns < self.initial_delay_ns
        {
            Err(JobError::InvalidRetryPolicy)
        } else {
            Ok(())
        }
    }

    pub(crate) fn delay(self, failed_attempts: u32) -> u64 {
        // Overflow means the mathematical delay exceeds every representable cap.
        let factor = 1_u64.checked_shl(failed_attempts - 1).unwrap_or(u64::MAX);
        self.initial_delay_ns
            .saturating_mul(factor)
            .min(self.max_delay_ns)
    }
}
