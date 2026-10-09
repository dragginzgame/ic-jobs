# IC Jobs

Durable job metadata and scheduling policies for Internet Computer applications.
Applications own persistence and effects; IC Jobs validates metadata transitions
and offers an optional adapter to IC Timers.

## Use

```rust
use ic_jobs::{Job, JobId, Outcome, RetryPolicy, Schedule};

let mut job = Job::new(
    JobId(1),
    Schedule::Once { at_ns: 100 },
    RetryPolicy::NONE,
)?;
let attempt = job.start(100)?;
// Persist job.record() and the immutable payload before dispatching the effect.
job.finish(attempt, 101, Outcome::Success)?;
// Persist job.record() again after recording the result.
# Ok::<(), ic_jobs::JobError>(())
```

Times are absolute IC nanoseconds supplied by the application. A due timestamp
is an eligibility threshold, not a punctual-execution guarantee.

The crate supports one-shot deadlines, fixed-rate intervals, recurrence after
completion, missed-run policies, bounded retries, pending cancellation and exact
attempt correlation. Persist the canonical `JobRecord` beside the application
payload and reconstruct it through `Job::restore`.

## Scheduling a batch

`Scheduler` borrows a bounded batch of application-owned jobs, derives its next
pending deadline, and starts one earliest due attempt at a time. It leaves storage,
payload lookup, effect execution and completion persistence with the application.

```rust
use ic_jobs::{Job, JobId, RetryPolicy, Schedule, Scheduler};

let mut jobs = [
    Job::new(JobId(1), Schedule::Once { at_ns: 200 }, RetryPolicy::NONE)?,
    Job::new(JobId(2), Schedule::Once { at_ns: 100 }, RetryPolicy::NONE)?,
];
assert_eq!(Scheduler::new(&mut jobs).next_due_ns(), Some(100));
if let Some((attempt, running_record)) = Scheduler::new(&mut jobs).start_next(100)? {
    assert_eq!(attempt.execution.job, JobId(2));
    // Commit running_record with the payload/index before dispatching the effect.
    // Record its outcome with the owning Job's finish method and persist again.
    # assert_eq!(running_record, jobs[1].record());
}
# Ok::<(), ic_jobs::JobError>(())
```

Equal deadlines follow the supplied batch order. A selected job's transition error
leaves every record unchanged; it does not select another job. Use a bounded
candidate batch from the application's due-time index. The reported deadline
covers only that batch; the storage owner supplies any wider queue's earliest
deadline to IC Timers.

A restored Running job remains Running. Uncertain effects block dispatch until
the application establishes the result or explicitly determines that repetition
is safe. `ExecutionId` identifies the logical effect across retries; `Attempt`
identifies one dispatch. Never reuse a job ID or change an occurrence's payload
under the same effect identity.

Enable the `timers` feature for the optional IC Timers adapter. Applications own
the volatile watchdog, lifecycle initialization, bounded callbacks and any
consumer-owned asynchronous delivery continuation. This crate supplies no stable
storage, global queue, handler registry or exactly-once effects.

The optional adapter uses IC Timers 0.16. Align direct timer dependencies and all
timer owners in the canister to that package identity. For exchanged measurement
summaries, select IC Metrics 0.3 or use `ic_timers::MeasurementSummary`.

The package requires Rust 1.88, with or without the optional timers feature.
See the [repository guide](https://github.com/dragginzgame/ic-jobs) and
[safety contract](https://github.com/dragginzgame/ic-jobs/blob/main/SAFETY.md)
for the complete persistence and recovery obligations.
