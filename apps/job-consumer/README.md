# Jobs test consumer

This unpublished canister is an application-owned consumer of Jobs, Memory and
Timers. It is a qualification fixture with deliberate fault endpoints, and must
not be deployed as a production service. The Jobs library acquires no storage,
authentication, payload, lifecycle or direct CDK timer ownership.

The manager principal is supplied at installation and persisted. Management
calls use the actual IC caller before reading or mutating the queue. `create`
allocates an identity, `inspect` reads one job, `list` returns up to eight jobs
after an exclusive ID cursor, `cancel` cancels Pending/Paused work, and `dispatch_due`
starts at most two eligible attempts. All task history is retained: IDs cannot
be reused. `pause` suspends pending work; `resume` restores its original deadline
and policy. `reschedule` replaces timing from an unstarted occurrence forward,
preserving identity/payload and keeping paused work paused. Already-attempted,
blocked and terminal work refuses rescheduling. There is no force-run command.

One Memory host bootstraps the `test.jobs.` namespace and opens the permanent
`test.jobs.consumer.management.v1` key. One stable cell retains the bounded JSON snapshot:
manager/receiver configuration, ID authority, JobRecords, immutable payloads,
counter and receipts. The queue holds at most 32 jobs and 32 receipts, each
delivery payload is at most 256 UTF-8 bytes, and serialized state is at most
24 KiB. The complete bounded queue supplies the global earliest deadline, so no
separate unbounded due index is needed. Every decoded record crosses
`Job::restore`, including future work. Invalid retained state refuses recovery;
installation cannot overwrite existing state. This is a new test installation
format, not a replacement reader for any existing deployment. The maintainer
approved retirement/reset of the disposable 0.5 installations for this hard cut;
create fresh 0.6 fixtures. Upgrading old retained state is unsupported and must
not reuse its discriminator or allocation as though its format were unchanged.

`Increment` performs a local counter effect and completes its job in one stable
replacement. A counter overflow has no effect and permanently fails that job,
including recurrence, without undoing unrelated attempts in the bounded batch.
Storage or transition failures still refuse the complete batch. `Deliver`
persists Running intent before calling an independent
receiver. The synchronous watchdog only prepares a bounded batch; an application
continuation performs each external call. An exact matching receipt records
Success. Other responses remain Uncertain. No automatic retries are configured.
Running and Uncertain records never redispatch after an upgrade.

The receiver uses the same fixture Wasm with `delivery_sender` configured to the
sender canister's principal. It authenticates this caller and retains each
logical job/occurrence receipt with its exact attempt and payload. Repetition of
the same envelope returns its existing receipt; conflicting payload/attempt
evidence is refused. `reconcile_delivery` reads a retained receipt and finishes
the exact blocked attempt. Missing evidence leaves the job blocked. Management
authentication and receiver authentication are independent of correlation IDs.

`init` creates state. `post_upgrade` bootstraps/open-validates retained memory,
restores the complete bounded queue, initializes Timers and reconstructs the
consumer's single watchdog. No provider handles or closures are persisted.
Successful create/management/dispatch/completion rederives its deadline; no Pending
work leaves it inactive. Reconciliation failure traps the same synchronous
segment as the stable write, preventing a falsely successful persisted schedule.
External-call continuations have separate message boundaries.

The shared batch budget is two attempts, including failed local attempts and
prepared deliveries. Watchdog work counts those handled attempts rather than
confirmed external effects, and an empty batch reports NoWork. CatchUp can still
require many later callbacks and delay newer deadlines; the batch limit is not
a backlog or cycle budget. Prefer Skip when only the current maintenance state
matters. The manager-only `fixture_trap_watchdog_batch` fault control traps after
the stable batch write and before any delivery escapes. Clearing it does not
rearm the timer, so the live recovery case must use the previously armed successor.

Run focused native/Wasm/host compilation with `make check-consumer`. Native
VectorMemory checks qualify cold reopening, bounded CatchUp/Skip batches,
refused writes, identity retention, exact receipt disposition and corrupt-record
refusal. They do not establish IC upgrades or message-wide trap rollback.

The host harness lives in [jobs-canister-tests](../../crates/jobs-canister-tests).
All its live tests are ignored by ordinary test execution. Explicit preparation
and live qualification are separate commands:

```bash
cargo fetch --locked
make install-testkit-tools
make install-consumer-server
make test-canister
```

To qualify the selected Binaryen optimizer against the same seven live recovery
scenarios, explicitly prepare it with `make install-ic-tools`, then run
`make test-canister-optimized`. This tests the original Wasm and its `-O3`, `-Os`
and `-Oz` outputs using the input's declared Wasm features. It
retains every input/output, hash, optimizer log and per-variant test/server log.
Failures stop before later variants and retain earlier evidence. This target
does not change the normal `build-consumer` artifact or select a production
optimization policy; ordinary CI still compiles without launching PocketIC.

The canonical Shared Cargo installer selects the published Testkit CLI directly
from the single `ic-testkit` package in [Cargo.lock](../../Cargo.lock), using
`--lockfile Cargo.lock` for both explicit setup and offline admission. There is
no separate CLI version pin. Rerun `make install-testkit-tools` when changing the
locked library selection; malformed, ambiguous or unsupported package selections
fail before installation. Testkit owns PocketIC setup,
offline admission and server lifecycle; this repo has no second server installer
or pin catalog. Normal checks install nothing. `test-canister` checks the CLI and
server before executing tests and uses Cargo metadata to locate the built Wasm,
including a caller-selected target directory. Linux x86-64 and both macOS
architectures use Testkit's supported server selections. Host Testkit/Host crates
remain outside the canister and public Jobs normal Wasm dependency graphs.

After [the updated recovery workflow](../../.github/workflows/recovery.yml) is delivered
to `main`, select **Actions → Canister recovery → Run workflow** and choose the
tag or branch to qualify. This manual trigger runs all seven tests on the original and
three optimized variants on Linux, Apple Silicon and Intel macOS.
Ordinary push/PR CI still compiles the harness without starting
PocketIC. See [GitHub's manual workflow instructions](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/manually-run-a-workflow).

Each host uploads a `jobs-recovery-<host>-<run>-<attempt>` artifact for 14 days on
success or failure. It records the selected commit, lockfile, snapshot, Rust
toolchains, Testkit receipt, test outcome/output and the actual tested Wasm with
its SHA-256. Testkit captures server stdout/stderr in the same artifact; failed
setup additionally retains canonical tool evidence. `test-canister` admits the
prepared server and compiles Wasm and the host harness before starting the
15-minute server lifetime. Local runs retain the Wasm/hash and server logs in
the reported `jobs-recovery.*` directory under `RUNNER_TEMP`, `TMPDIR` or `/tmp`.

The live harness covers management authorization, wakeup changes, real local and
external timer delivery, upgrade retention, overdue policies, synchronous
write/timer rollback, lost sender-result traps with retained destination receipts,
absent-receipt blocking and invalid-record upgrade refusal with explicit
controller-snapshot recovery. Management coverage adds paused/rescheduled
upgrade retention, authorization, wakeup stop/rearm, overdue resume policies and
real trap rollback after management and timer reconciliation. Deliberate `fixture_*` endpoints implement these
faults in the unpublished Wasm rather than faking IC behavior with `cfg(test)`.
Their API must never be copied into a production canister. Live execution and
native macOS acceptance are recorded separately in the
[handoff](../../docs/status/current.md). Completed
[#11](https://github.com/dragginzgame/ic-jobs/issues/11) acceptance covers the older
five-scenario v0.5.6 harness; completed [#20](https://github.com/dragginzgame/ic-jobs/issues/20)
acceptance covers the six-case v0.5.9 harness. Current management/storage inputs
require their own live evidence in [#23](https://github.com/dragginzgame/ic-jobs/issues/23). Harness
compilation alone does not establish that acceptance.
