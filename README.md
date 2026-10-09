# IC Jobs

IC Jobs provides durable job metadata and scheduling policies for Internet
Computer applications. It sits above IC Timers: applications persist job records
and execute handlers; IC Timers owns volatile wakeups.

The crate supports:

- one-shot deadlines, fixed-rate intervals and recurrence after completion;
- skipping missed intervals or catching up one occurrence at a time;
- per-occurrence retry budgets with capped exponential backoff;
- cancellation of pending work and exact attempt correlation;
- validated record reconstruction and explicit uncertain-effect reconciliation;
- bounded batch scheduling with earliest-deadline selection and dispatch intent;
- an optional adapter to the existing IC Timers watchdog runtime.

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

All times are absolute IC nanoseconds. The caller supplies its trusted clock.
A due timestamp is an eligibility threshold, not a punctual-execution guarantee.

Store the canonical `JobRecord` beside the application payload and restore it with
`Job::restore`. IcyDB is a possible storage owner; the crate deliberately does
not impose a database or a storage trait. Maintain a bounded due-time index in
that owner instead of scanning an unbounded job collection.

A restored Running job remains Running. If a reply cannot be reconciled directly,
record Uncertain with its active attempt and use `resolve` only after establishing
the actual result or retry safety. A lost reply never automatically authorizes
another external effect.

`ExecutionId` identifies the same logical effect across retries. Use it as a
destination-side idempotency key. `Attempt` identifies a particular dispatch,
allowing stale replies to be rejected. Never reuse a job ID or change an
occurrence's payload under the same effect identity.

## Scheduling a batch

`Scheduler` borrows a bounded batch of validated, application-owned jobs. It
selects the earliest pending deadline, including retry backoff, and starts at
most one due attempt per call. Equal deadlines follow the supplied batch order.

```rust
use ic_jobs::{Job, JobId, RetryPolicy, Schedule, Scheduler};

let mut jobs = [
    Job::new(JobId(1), Schedule::Once { at_ns: 200 }, RetryPolicy::NONE)?,
    Job::new(JobId(2), Schedule::Once { at_ns: 100 }, RetryPolicy::NONE)?,
];
let next_wakeup = Scheduler::new(&mut jobs).next_due_ns();
assert_eq!(next_wakeup, Some(100));
if let Some((attempt, running_record)) = Scheduler::new(&mut jobs).start_next(100)? {
    assert_eq!(attempt.execution.job, JobId(2));
    // Commit running_record with the payload/index before dispatching the effect.
    // Record its outcome with the owning Job's finish method and persist again.
    # assert_eq!(running_record, jobs[1].record());
}
# Ok::<(), ic_jobs::JobError>(())
```

Running, uncertain and terminal jobs have no dispatch deadline. Reconstruct all
stored records through `Job::restore`; uncertain effects become eligible only
after explicit disposition. A selected job's transition error leaves the batch
unchanged and does not fall through to another job.

The scheduler reports deadlines only for its supplied batch. Applications bound
candidate selection through their storage owner's due-time index and keep that
index current after each transition. Use the batch deadline for IC Timers when
it covers the earliest pending work, or obtain the global deadline from the index.

## Timer integration

Enable `features = ["timers"]` to use `ic_jobs::timers`. The application owns one
volatile watchdog registration, calls IC Timers initialization in its lifecycle,
and passes its earliest pending job deadline to `timers::reconcile`.
A bounded callback returns `timers::complete_batch` with the actual completion
classification and the newly derived deadline.

Reconcile after inserting or cancelling jobs. After upgrades, restore records
and reconstruct the watchdog synchronously before downstream hooks.
Unresolved effects have no dispatch deadline; applications own a separate
reconciliation workflow for those effects.

Watchdog callbacks cannot await. External delivery needs a persisted intent and
a consumer-owned continuation/outbox; do not put async work inside the synchronous
watchdog callback. All owners in a final canister must resolve one IC Timers
package identity. No direct CDK timers are used here.

## Scope and validation

The library provides metadata and transitions; it does not itself supply stable
storage, a global queue, handler registry, full history, calendar cron parsing, timezones,
Canic lifecycle exports, email delivery or exactly-once effects. Records retain
the latest outcome; applications own any longer audit history.

See [SAFETY.md](SAFETY.md) for persistence and recovery obligations.
Run `cargo run -p ic-jobs --locked --example persisted_job` for a runnable
reconstruction example. Focused checks are listed by `make help`.

The package requires Rust 1.88 (edition 2024), including the optional IC Timers
path. Development formatting uses
Rust 1.99.0. Linux and macOS Intel/Apple Silicon host workflows are maintained;
native macOS and live IC qualification are pending.

Common tooling is adopted from reviewed Shared Tooling revision
`4e274a2219c0b0cc3af68ec65658b373253518fb` (0.1.30).
Run `make install-tools` explicitly to prepare the checkout-local host, IC and
Rust tools; `make tools-check` verifies them offline. For an interactive shell,
prepend `$PWD/.tools/host/bin:$PWD/.tools/ic/bin:$PWD/.tools/rust/bin` to PATH.
System prerequisites are Git, Bash, Make, Perl, curl, tar and rustup;
see [local setup](docs/local-setup.md). Install `wasm32-unknown-unknown`
explicitly before Wasm checks. `make install-hooks` enables the reviewed
formatting hook after formatter preparation.

The public repository is [dragginzgame/ic-jobs](https://github.com/dragginzgame/ic-jobs).
Run `make version` to inspect the local version. `make package` verifies the
crate tarball offline and `make publish-check` performs a publication dry run
with registry access. Both permit working edits and upload nothing.

The explicitly authorized `make release-patch`, `make release-minor` and
`make release-major` commands run the complete gate, prepare metadata, commit,
tag and atomically push to `origin`/`main`. Package publication is a separate
`make publish` command requiring clean source and the matching pushed annotated
tag. See [the IC Jobs delivery procedure](docs/releasing.md) for prerequisites
and interruption recovery. Configuring these commands creates no release or
package publication.
