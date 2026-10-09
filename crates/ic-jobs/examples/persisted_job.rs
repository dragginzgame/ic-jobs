//! Runnable storage-boundary example; JSON bytes stand in for application storage.
use ic_jobs::{Job, JobError, JobId, JobRecord, Outcome, RetryPolicy, Schedule, Scheduler};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut jobs = [Job::new(
        JobId(1),
        Schedule::Once { at_ns: 100 },
        RetryPolicy::NONE,
    )?];
    let (attempt, intent) = Scheduler::new(&mut jobs)
        .start_next(100)?
        .ok_or(JobError::NotDue)?;
    // On IC, persist this record and the immutable payload in the same message
    // segment before issuing an external call. A trap before commit rolls back it.
    let committed_intent = serde_json::to_vec(&intent)?;

    // An upgrade or lost continuation restores unresolved intent, never a retry.
    let record: JobRecord = serde_json::from_slice(&committed_intent)?;
    let mut restored = Job::restore(record)?;
    assert!(
        Scheduler::new(std::slice::from_mut(&mut restored))
            .start_next(200)?
            .is_none()
    );
    restored.finish(attempt, 101, Outcome::Uncertain)?;
    // Application reconciliation confirms the original effect actually succeeded.
    restored.resolve(attempt, 102, Outcome::Success)?;
    let committed_result = serde_json::to_vec(&restored.record())?;
    println!("{}", String::from_utf8(committed_result)?);
    Ok(())
}
