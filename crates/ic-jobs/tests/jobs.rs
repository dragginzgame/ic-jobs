//! Observable scheduling, persistence and external-effect recovery contracts.
use ic_jobs::{
    Attempt, Job, JobError, JobId, JobRecord, JobState, MissedRunPolicy, Outcome, RetryPolicy,
    Schedule,
};

fn retry() -> RetryPolicy {
    RetryPolicy {
        max_attempts: 4,
        initial_delay_ns: 10,
        max_delay_ns: 15,
    }
}

fn once() -> Job {
    Job::new(JobId(42), Schedule::Once { at_ns: 100 }, retry()).unwrap()
}

fn roundtrip(job: &Job) -> Job {
    let bytes = serde_json::to_vec(&job.record()).unwrap();
    let record: JobRecord = serde_json::from_slice(&bytes).unwrap();
    Job::restore(record).unwrap()
}

#[test]
fn persistence_preserves_pending_running_and_terminal_states() {
    let mut job = roundtrip(&once());
    assert_eq!(job.start(99), Err(JobError::NotDue));
    let attempt = job.start(100).unwrap();
    let mut job = roundtrip(&job);
    assert_eq!(job.start(101), Err(JobError::NotPending));
    assert_eq!(job.active_attempt(), Some(attempt));
    job.finish(attempt, 101, Outcome::Success).unwrap();
    let mut job = roundtrip(&job);
    assert_eq!(job.record().state, JobState::Completed);
    assert_eq!(
        job.finish(attempt, 102, Outcome::Success),
        Err(JobError::StaleAttempt)
    );
    assert_eq!(job.next_due_ns(), None);
}

#[test]
fn retries_keep_effect_identity_cap_backoff_and_stop_at_budget() {
    let mut job = once();
    let first = job.start(100).unwrap();
    job.finish(first, 100, Outcome::RetryableFailure).unwrap();
    assert_eq!(job.next_due_ns(), Some(110));
    let second = job.start(110).unwrap();
    assert_eq!(first.execution, second.execution);
    assert_ne!(first.sequence, second.sequence);
    assert_eq!(
        job.finish(first, 110, Outcome::Success),
        Err(JobError::StaleAttempt)
    );
    job.finish(second, 110, Outcome::RetryableFailure).unwrap();
    assert_eq!(job.next_due_ns(), Some(125));
    let third = job.start(125).unwrap();
    job.finish(third, 125, Outcome::RetryableFailure).unwrap();
    assert_eq!(job.next_due_ns(), Some(140));
    let fourth = job.start(140).unwrap();
    job.finish(fourth, 140, Outcome::RetryableFailure).unwrap();
    assert_eq!(roundtrip(&job).record().state, JobState::Failed);
    assert_eq!(job.start(1_000), Err(JobError::NotPending));
}

#[test]
fn uncertain_effect_requires_explicit_reconciliation_after_restore() {
    let mut job = once();
    let attempt = job.start(100).unwrap();
    job.finish(attempt, 101, Outcome::Uncertain).unwrap();
    let mut job = roundtrip(&job);
    assert_eq!(job.next_due_ns(), None);
    assert_eq!(job.start(200), Err(JobError::NotPending));
    assert_eq!(job.cancel(200), Err(JobError::InFlight));
    assert_eq!(
        job.finish(attempt, 200, Outcome::Success),
        Err(JobError::StaleAttempt)
    );
    job.resolve(attempt, 200, Outcome::RetryableFailure)
        .unwrap();
    assert_eq!(job.next_due_ns(), Some(210));
    assert_eq!(roundtrip(&job).record(), job.record());
    let repeated = job.start(210).unwrap();
    assert_eq!(attempt.execution, repeated.execution);
    assert_ne!(attempt.sequence, repeated.sequence);
}

#[test]
fn interrupted_running_intent_can_be_marked_uncertain_then_confirmed() {
    let mut job = once();
    let attempt = job.start(100).unwrap();
    let mut restored = roundtrip(&job);
    restored.finish(attempt, 120, Outcome::Uncertain).unwrap();
    restored.resolve(attempt, 130, Outcome::Success).unwrap();
    assert_eq!(roundtrip(&restored).record().state, JobState::Completed);
}

#[test]
fn missed_run_policy_distinguishes_skip_catchup_and_after_completion() {
    let schedules = [
        (
            Schedule::FixedRate {
                first_at_ns: 100,
                every_ns: 10,
                missed: MissedRunPolicy::Skip,
            },
            140,
        ),
        (
            Schedule::FixedRate {
                first_at_ns: 100,
                every_ns: 10,
                missed: MissedRunPolicy::CatchUp,
            },
            110,
        ),
        (
            Schedule::AfterCompletion {
                first_at_ns: 100,
                every_ns: 10,
            },
            145,
        ),
    ];
    for (schedule, next_due) in schedules {
        let mut job = Job::new(JobId(1), schedule, RetryPolicy::NONE).unwrap();
        let attempt = job.start(130).unwrap();
        job.finish(attempt, 135, Outcome::Success).unwrap();
        assert_eq!(job.next_due_ns(), Some(next_due));
        assert_eq!(job.record().occurrence, 1);
        assert_eq!(job.record().attempts, 0);
        assert_eq!(roundtrip(&job).record(), job.record());
    }
}

#[test]
fn recurring_occurrence_has_fresh_effect_identity_and_retry_budget() {
    let mut job = Job::new(
        JobId(1),
        Schedule::FixedRate {
            first_at_ns: 100,
            every_ns: 100,
            missed: MissedRunPolicy::Skip,
        },
        retry(),
    )
    .unwrap();
    let first = job.start(100).unwrap();
    job.finish(first, 100, Outcome::RetryableFailure).unwrap();
    let retry = job.start(110).unwrap();
    job.finish(retry, 110, Outcome::Success).unwrap();
    let next = job.start(200).unwrap();
    assert_ne!(first.execution, next.execution);
    assert_eq!(job.record().attempts, 1);
    assert_eq!(roundtrip(&job).record(), job.record());
}

#[test]
fn pending_cancellation_survives_restore_and_cannot_abort_inflight_work() {
    let mut job = once();
    job.cancel(50).unwrap();
    let mut restored = roundtrip(&job);
    assert_eq!(restored.record().state, JobState::Cancelled);
    assert_eq!(restored.start(100), Err(JobError::NotPending));
    let mut running = once();
    running.start(100).unwrap();
    assert_eq!(running.cancel(101), Err(JobError::InFlight));
}

#[test]
fn timestamp_errors_leave_dispatch_intent_unchanged() {
    let mut job = Job::new(JobId(1), Schedule::Once { at_ns: u64::MAX }, retry()).unwrap();
    let attempt = job.start(u64::MAX).unwrap();
    let before = job.record();
    assert_eq!(
        job.finish(attempt, u64::MAX - 1, Outcome::Success),
        Err(JobError::TimeWentBackwards)
    );
    assert_eq!(job.record(), before);
    assert_eq!(
        job.finish(attempt, u64::MAX, Outcome::RetryableFailure),
        Err(JobError::DeadlineOverflow)
    );
    assert_eq!(job.record(), before);
    job.finish(attempt, u64::MAX, Outcome::PermanentFailure)
        .unwrap();
    assert_eq!(roundtrip(&job).record().state, JobState::Failed);
}

#[test]
fn recurrence_overflow_preserves_active_attempt() {
    let mut job = Job::new(
        JobId(1),
        Schedule::FixedRate {
            first_at_ns: u64::MAX - 1,
            every_ns: 2,
            missed: MissedRunPolicy::Skip,
        },
        RetryPolicy::NONE,
    )
    .unwrap();
    let attempt = job.start(u64::MAX - 1).unwrap();
    let before = job.record();
    assert_eq!(
        job.finish(attempt, u64::MAX, Outcome::Success),
        Err(JobError::DeadlineOverflow)
    );
    assert_eq!(job.record(), before);
}

#[test]
fn sequence_exhaustion_cannot_reuse_attempt_identity() {
    let mut job = once();
    let attempt = job.start(100).unwrap();
    job.finish(attempt, 100, Outcome::RetryableFailure).unwrap();
    let mut record = job.record();
    record.sequence = u64::MAX;
    let mut job = Job::restore(record).unwrap();
    let before = job.record();
    assert_eq!(job.start(110), Err(JobError::CounterExhausted));
    assert_eq!(job.record(), before);
}

#[test]
fn corrupt_decoded_configuration_and_state_are_rejected() {
    let record = once().record();
    let mut zero_budget = record;
    zero_budget.retry.max_attempts = 0;
    assert_eq!(
        Job::restore(zero_budget).unwrap_err(),
        JobError::InvalidRetryPolicy
    );
    let mut exhausted = record;
    exhausted.attempts = 5;
    assert_eq!(
        Job::restore(exhausted).unwrap_err(),
        JobError::InvalidRecord
    );
    let mut fake_active = record;
    fake_active.state = JobState::Running { started_ns: 100 };
    assert_eq!(
        Job::restore(fake_active).unwrap_err(),
        JobError::InvalidRecord
    );
    let mut invalid_deadline = record;
    invalid_deadline.state = JobState::Pending { due_ns: 99 };
    assert_eq!(
        Job::restore(invalid_deadline).unwrap_err(),
        JobError::InvalidRecord
    );
    let mut fake_completion = record;
    fake_completion.state = JobState::Completed;
    assert_eq!(
        Job::restore(fake_completion).unwrap_err(),
        JobError::InvalidRecord
    );
    let mut invalid_interval = record;
    invalid_interval.schedule = Schedule::AfterCompletion {
        first_at_ns: 100,
        every_ns: 0,
    };
    assert_eq!(
        Job::restore(invalid_interval).unwrap_err(),
        JobError::ZeroInterval
    );
}

#[test]
fn completion_from_another_job_cannot_change_state() {
    let mut job = once();
    let mut other = Job::new(JobId(99), Schedule::Once { at_ns: 100 }, retry()).unwrap();
    job.start(100).unwrap();
    let other_attempt: Attempt = other.start(100).unwrap();
    let before = job.record();
    assert_eq!(
        job.finish(other_attempt, 100, Outcome::Success),
        Err(JobError::StaleAttempt)
    );
    assert_eq!(job.record(), before);
}

#[test]
fn transition_sequences_remain_restorable() {
    for missed in [MissedRunPolicy::Skip, MissedRunPolicy::CatchUp] {
        let mut job = Job::new(
            JobId(7),
            Schedule::FixedRate {
                first_at_ns: 100,
                every_ns: 20,
                missed,
            },
            retry(),
        )
        .unwrap();
        let mut now = 200;
        for _ in 0..50 {
            now = now.max(job.next_due_ns().unwrap());
            let attempt = job.start(now).unwrap();
            job = roundtrip(&job);
            job.finish(attempt, now, Outcome::Uncertain).unwrap();
            job = roundtrip(&job);
            job.resolve(attempt, now + 1, Outcome::RetryableFailure)
                .unwrap();
            job = roundtrip(&job);
            now = job.next_due_ns().unwrap();
            let attempt = job.start(now).unwrap();
            job.finish(attempt, now + 1, Outcome::Success).unwrap();
            job = roundtrip(&job);
            now += 3;
        }
        job.cancel(now).unwrap();
        assert_eq!(roundtrip(&job).record().state, JobState::Cancelled);
    }
}

#[cfg(feature = "timers")]
#[test]
fn batch_adapter_projects_deadlines_into_timer_decisions() {
    use ic_jobs::timers::complete_batch;
    use ic_timers::{TimerCompletion, WatchdogDecision};
    assert_eq!(
        complete_batch(TimerCompletion::no_work(), None).decision(),
        WatchdogDecision::Stop
    );
    assert_eq!(
        complete_batch(TimerCompletion::success(2), Some(123)).decision(),
        WatchdogDecision::ScheduleAt(123)
    );
    assert_eq!(
        complete_batch(TimerCompletion::retryable_failure(0), Some(123)).decision(),
        WatchdogDecision::ScheduleAt(123)
    );
    assert_eq!(
        complete_batch(TimerCompletion::invariant_failure(0), Some(123)).decision(),
        WatchdogDecision::Stop
    );
}
