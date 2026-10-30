use super::*;
use crate::schedule::ScheduleError;
use crate::snapshot::TimerDirectiveSnapshot;

#[test]
fn policy_results_preserve_completion_and_force_invariant_failures_to_stop() {
    let completions = [
        TimerCompletion::success(4),
        TimerCompletion::no_work(),
        TimerCompletion::retryable_failure(2),
        TimerCompletion::invariant_failure(3),
    ];
    for completion in completions {
        for decision in [
            OnceDecision::Stop,
            OnceDecision::ContinueImmediately,
            OnceDecision::RetryAfter(Duration::from_nanos(7)),
            OnceDecision::ScheduleAt(50),
        ] {
            let result = OnceRunResult::new(completion, decision);
            assert_eq!(result.completion(), completion);
            let expected = if completion.outcome() == TimerCompletionOutcome::InvariantFailure {
                OnceDecision::Stop
            } else {
                decision
            };
            assert_eq!(result.decision(), expected);
        }
        for decision in [
            AfterCompletionDecision::Stop,
            AfterCompletionDecision::ContinueImmediately,
            AfterCompletionDecision::RetryAfter(Duration::from_nanos(7)),
            AfterCompletionDecision::ScheduleAt(50),
            AfterCompletionDecision::RecurAfterCompletion,
        ] {
            let result = AfterCompletionRunResult::new(completion, decision);
            assert_eq!(result.completion(), completion);
            let expected = if completion.outcome() == TimerCompletionOutcome::InvariantFailure {
                AfterCompletionDecision::Stop
            } else {
                decision
            };
            assert_eq!(result.decision(), expected);
        }
    }
}

#[test]
fn public_decisions_project_inert_snapshots_with_checked_delays() {
    for (once, recurring, expected) in [
        (
            OnceDecision::Stop,
            AfterCompletionDecision::Stop,
            TimerDirectiveSnapshot::Stop,
        ),
        (
            OnceDecision::ContinueImmediately,
            AfterCompletionDecision::ContinueImmediately,
            TimerDirectiveSnapshot::ContinueImmediately,
        ),
        (
            OnceDecision::RetryAfter(Duration::from_nanos(7)),
            AfterCompletionDecision::RetryAfter(Duration::from_nanos(7)),
            TimerDirectiveSnapshot::RetryAfter { delay_ns: 7 },
        ),
        (
            OnceDecision::RetryAfter(Duration::from_millis(25)),
            AfterCompletionDecision::RetryAfter(Duration::from_millis(25)),
            TimerDirectiveSnapshot::RetryAfter {
                delay_ns: 25_000_000,
            },
        ),
        (
            OnceDecision::ScheduleAt(50),
            AfterCompletionDecision::ScheduleAt(50),
            TimerDirectiveSnapshot::ScheduleAt { deadline_ns: 50 },
        ),
    ] {
        assert_eq!(TimerDirectiveSnapshot::try_from(once), Ok(expected));
        assert_eq!(TimerDirectiveSnapshot::try_from(recurring), Ok(expected));
    }
    assert_eq!(
        TimerDirectiveSnapshot::try_from(AfterCompletionDecision::RecurAfterCompletion),
        Ok(TimerDirectiveSnapshot::RecurAfterCompletion),
    );
    assert_eq!(
        TimerDirectiveSnapshot::try_from(OnceDecision::RetryAfter(Duration::MAX)),
        Err(ScheduleError::DelayOutOfRange),
    );
    assert_eq!(
        TimerDirectiveSnapshot::try_from(AfterCompletionDecision::RetryAfter(Duration::MAX)),
        Err(ScheduleError::DelayOutOfRange),
    );
}
