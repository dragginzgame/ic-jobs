# IC Jobs

IC Jobs provides durable job metadata and scheduling policies for Internet
Computer applications. It sits above IC Timers: applications persist job records
and execute handlers; IC Timers owns volatile wakeups.

The crate supports:

- one-shot deadlines, fixed-rate intervals and recurrence after completion;
- skipping missed intervals or catching up one occurrence at a time;
- per-occurrence retry budgets with capped exponential backoff;
- durable pause/resume, unstarted-occurrence rescheduling, pending/paused
  cancellation and exact attempt correlation;
- validated record reconstruction and explicit uncertain-effect reconciliation;
- bounded batch scheduling with earliest-deadline selection and dispatch intent;
- an optional adapter to the existing IC Timers watchdog runtime.

## Use

The unpublished [canister consumer](apps/job-consumer/README.md) demonstrates
application-owned storage, management and lifecycle integration. Its separate
PocketIC harness is compiled by CI; live qualification is explicitly selected.

```rust
use ic_jobs::{Job, JobId, Outcome, RetryPolicy, Schedule};

let mut job = Job::new(
    JobId(1),
    Schedule::Once { at_ns: 100 },
    RetryPolicy::NONE,
);
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

## Checked configuration (0.8)

Recurring `Schedule` intervals use `std::num::NonZeroU64`; absolute deadlines
still use `u64` and may be zero. Build retry budgets with
`RetryPolicy::new(max_attempts, initial_delay_ns, max_delay_ns)?`, or use
`RetryPolicy::NONE`. Read the immutable values with `max_attempts()`,
`initial_delay_ns()` and `max_delay_ns()`.

```rust
use std::num::NonZeroU64;
use ic_jobs::{Job, JobId, RetryPolicy, Schedule};

let interval = NonZeroU64::new(10).unwrap();
let retry = RetryPolicy::new(3, 5, 20)?;
let job = Job::new(JobId(1), Schedule::AfterCompletion {
    first_at_ns: 0,
    every_ns: interval,
}, retry);
assert_eq!(job.next_due_ns(), Some(0));
# Ok::<(), ic_jobs::JobError>(())
```

This is a source API cut: replace numeric interval literals and retry struct
literals, remove `?`/`unwrap()` from `Job::new`, and update exhaustive error matches
for the removed `ZeroInterval` variant. Raw inputs must reject zero before
constructing a schedule. Invalid retry configuration returns
`JobError::InvalidRetryPolicy`; invalid serialized configuration fails decoding.
Every decoded `JobRecord` still passes through `Job::restore` to check its history.
Valid retained records keep their numeric fields, variant names and JSON bytes;
this cut requires no stable-memory reset. See
[#28](https://github.com/dragginzgame/ic-jobs/issues/28).

## Scheduling a batch

`Scheduler` borrows a bounded batch of validated, application-owned jobs. It
selects the earliest pending deadline, including retry backoff, and starts at
most one due attempt per call. Equal deadlines follow the supplied batch order.

```rust
use ic_jobs::{Job, JobId, RetryPolicy, Schedule, Scheduler};

let mut jobs = [
    Job::new(JobId(1), Schedule::Once { at_ns: 200 }, RetryPolicy::NONE),
    Job::new(JobId(2), Schedule::Once { at_ns: 100 }, RetryPolicy::NONE),
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

A batch limit bounds one callback, not the entire catch-up backlog. For example,
86,400 pending one-second occurrences need at least 43,200 batches with a
two-attempt limit. Earliest-deadline selection can delay newer jobs until that
historical work drains. Prefer `Skip` for maintenance that does not need every
occurrence; use `CatchUp` only with an application-owned backlog/cycle budget.
This is a workload calculation, not a measured throughput or delivery guarantee.

## Timer integration

`Job::pause` retains the pending deadline and retry evidence but removes dispatch
eligibility. `Job::resume` restores that deadline, applying the existing policy
to overdue work. `Job::reschedule` replaces the schedule from the current
unstarted occurrence forward, preserving job/effect identity, sequence and
payload. Paused work stays paused. An occurrence that allocated an attempt cannot
be rescheduled; Running and Uncertain work must use explicit outcome disposition.
After each successful management operation, commit the record with the due-time
index before reconciling the consumer's timer.

The 0.6 JobRecord contract adds `schedule_origin` and `last_outcome_ns` and the
Paused state. Update exhaustive matches and codecs. The maintainer approved
retirement/reset of the disposable 0.5 test installations; the canister fixture
uses a distinct current memory key and fresh state, with no old-record reader.
See [#23](https://github.com/dragginzgame/ic-jobs/issues/23).

Enable `features = ["timers"]` to use `ic_jobs::timers`. The application owns one
volatile watchdog registration, calls IC Timers initialization in its lifecycle,
and passes its earliest pending job deadline to `timers::reconcile`.
A bounded callback returns `timers::complete_batch` with the actual completion
classification and the newly derived deadline.

Reconcile after inserting or managing jobs. After upgrades, restore records
and reconstruct the watchdog synchronously before downstream hooks.
Unresolved effects have no dispatch deadline; applications own a separate
reconciliation workflow for those effects.

Watchdog callbacks cannot await. External delivery needs a persisted intent and
a consumer-owned continuation/outbox; do not put async work inside the synchronous
watchdog callback. All owners in a final canister must resolve one IC Timers
package identity. No direct CDK timers are used here.

Timer completion classifies the dispatch callback, separately from each job's
effect outcome. A bounded batch can succeed while recording a known job failure
or preparing an external attempt whose result is still unknown. Report no work
for an empty batch. A watchdog recovery successor restores a callback opportunity;
it cannot make a permanently failing handler progress. Dispose of known no-effect
handler failures through the job's exact attempt, retaining fail-closed handling
for corrupt storage and unresolved external effects.

The optional adapter selects IC Timers 0.21. Applications using its public timer
types must align their direct timer dependency to 0.21. For exchanged measurement
summaries, use `ic_timers::MeasurementSummary` to share Timers' selected Metrics
package identity.

Timers 0.21 publishes a new watchdog registration only after its requested
initial state succeeds. A failed first reconciliation leaves the consumer slot
empty and releases the new callback; retry can install the callback supplied
by that retry. Later failures retain existing authority. The instruction-summary
type selects Metrics 0.8; use the Timers re-export when exchanging summaries.

## Scope and validation

The library provides metadata and transitions; it does not itself supply stable
storage, a global queue, handler registry, full history, calendar cron parsing, timezones,
Canic lifecycle exports, email delivery or exactly-once effects. Records retain
the latest outcome; applications own any longer audit history.

See [SAFETY.md](SAFETY.md) for persistence and recovery obligations.
Run `cargo run -p ic-jobs --locked --example persisted_job` for a runnable
reconstruction example. Focused checks are listed by `make help`.
Run `make test-consumers` for application-owned notification and cache-maintenance
fixtures covering commit failures, reconstruction, receipt reconciliation and
bounded recurring work. The notification fixture also exercises authorized
management, bounded inspection/listing and committed wakeup decisions.
Both fixtures contribute checked, key-only storage requests using the
development dependency IC Memory 0.35. A native host fixture bootstraps one pool
and reopens stable cells, retaining payloads/deadlines and blocked effects.
The unpublished [canister consumer](apps/job-consumer/README.md) exercises real
management, upgrade, rollback and timer reconstruction through PocketIC. See the
current handoff for the source-bound results and outstanding host qualification.
See [test consumers](docs/test-consumers.md) for their
storage and timer boundaries.

The package requires Rust 1.88 (edition 2024), including the optional IC Timers
path. Development formatting uses
Rust 1.99.0. Linux and macOS Intel/Apple Silicon host workflows are maintained;
see the
[current handoff](docs/status/current.md) for source-bound native and live
PocketIC recovery qualification.

Common tooling is adopted through the reviewed
[Shared Tooling snapshot](.shared-tooling.snapshot).
Run `make install-tools` explicitly to prepare and select the host, IC and
Rust tools in order, then selected cargo-edit; `make tools-check` verifies the
same complete set offline. Host tools include jq, yq, ripgrep with PCRE2 and cloc;
Rust tools include cargo-sort, cargo-sort-derives and candid-extractor. For an interactive shell,
prepend `$PWD/.tools/host/bin:$PWD/.tools/ic/bin:$PWD/.tools/rust/common/bin` to PATH.
The reviewed [common Make entrypoint](docs/make-interface.md) owns contributor
setup, formatting, hooks, snapshot verification and release routing; Jobs retains
its product checks and metadata adapters. Snapshot entrypoints are recorded in
`.shared-tooling.snapshot`; refresh derives their required companions.
Shared Tooling stores verified host, IC and Cargo-built bundles once per user
while each checkout retains its own pinned selection through these paths. Cargo
reuse binds the exact compiler, native environment, published source and lock,
target and profile. Tool builds use the shared standard environment; application
compiler flags and wrappers remain local. After adoption, run
`make install-tools` and `make tools-check`; previous installations remain
retained. Use the same
`SHARED_TOOLING_STORE` setting for setup, checks and evidence collection; see
[storage configuration](docs/local-setup.md#shared-executable-storage).
Common Rust tools retain `.tools/rust/common/bin` as their command path.
Jobs' pinned cargo-edit also uses the shared owner: `make install-release-tools`
prepares its selected binary and `make release-tools-check` admits it. Release
metadata uses the returned checked path. The older cargo-edit root is retained;
rerun explicit setup before release commands on this snapshot.
The shared IC bundle contains Quill, ICP CLI, didc, ic-wasm and wasm-opt.
After updating an older six-tool bundle, run `make install-ic-tools` explicitly,
then `make ic-tools-check`; previous bundles and receipts are retained.
The separately lock-selected Testkit CLI also needs explicit
`make install-testkit-tools` and `make testkit-tools-check` after this adoption.
PocketIC provisioning belongs to IC Testkit. The unpublished
[consumer harness](apps/job-consumer/README.md) uses its separately selected
Testkit setup/check contract; common setup and ordinary CI do not launch it.
System prerequisites are Git, Bash, Make, Perl, curl, tar and rustup;
see [local setup](docs/local-setup.md). Install `wasm32-unknown-unknown`
explicitly on both Rust 1.88.0 and 1.99.0 before the consumer's build and Clippy
checks. `make install-hooks` enables the reviewed
formatting hook after formatter preparation.
`make formatting-hook-check` exercises the hook and Jobs' actual formatter in
disposable Git copies. Complete CI runs these checks on every supported host.

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
