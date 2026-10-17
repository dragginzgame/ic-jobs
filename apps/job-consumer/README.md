# Jobs test consumer

This package also owns the native notification/maintenance composition harness
in `tests/composition.rs`. Run `make test-consumers` for its 19 scenarios;
`check-msrv` qualifies core and composition separately on Rust 1.91. Memory and
Timers integration dependencies belong here, outside the core package tests.

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

Memory's fixture-only `FixtureCell` bootstraps the `test.jobs.` namespace and
opens the permanent `test.jobs.consumer.management.v1` key. Installation selects
bounded fresh creation; recovery selects bounded retained opening, with Cell
header and size admission before payload loading. One stable cell retains the JSON snapshot:
manager/receiver configuration, ID authority, JobRecords, immutable payloads,
counter and receipts. The queue holds at most 32 jobs and 32 receipts, each
delivery payload is at most 256 UTF-8 bytes, and serialized state is at most
24 KiB. The complete bounded queue supplies the global earliest deadline, so no
separate unbounded due index is needed. Every decoded record crosses
`Job::restore`, including future work. Successful decoding retains validated jobs
for the complete synchronous operation. Each in-memory entry keeps its job and
payload together; the scheduler borrows only the jobs through an iterator. Commit
revalidates the encoded snapshot before replacing stable bytes. Separate
job/payload alignment state is unnecessary. The stored JSON shape is unchanged.
Invalid retained state refuses recovery; installation cannot overwrite existing
allocations. Oversized-cell refusal preserves the exact cell header and payload;
bootstrap may update allocation bookkeeping before that refusal. The frozen
incompatible-ledger cases separately retain their zero-write proof. This is a new test installation
format, not a replacement reader for any existing deployment. The maintainer
approved retirement/reset of the disposable 0.5 installations for this hard cut;
create fresh 0.6 fixtures. Upgrading old retained state is unsupported and must
not reuse its discriminator or allocation as though its format were unchanged.

IC Memory 0.39 also changes the physical ledger envelope to one current snapshot.
The frozen 0.7.5 two-slot image is deliberately refused during both recovery and
installation, with zero stable writes. The separate frozen Jobs JSON fixture
still reopens on fresh current-format memory and retains blocked effects. This
does not establish an upgrade path for old physical images: their owner must
explicitly dispose of or preserve retained state before adopting the new ledger.
There is no compatibility reader, automatic reset or rewrite of historical bytes.

IC Memory 0.41 changes the logical ledger discriminator and checked slot encoding.
The frozen 0.40 registry-produced image is also refused for recovery and installation,
with zero writes, unchanged pages and identical backing bytes. Existing 0.40
installations require explicit owner disposition before deployment; adopting the
crate does not authorize clearing their ledger or preserving untracked cells.
The unchanged Jobs JSON remains independently readable on fresh current memory.

Memory 0.45 removes allocator schema metadata and changes the outer ledger format.
Requests now contain only their authority and key. Earlier physical installations
require their owner's explicit retention/disposition decision before deployment;
this source update does not clear, convert or reset their backing memory. Jobs'
JSON entries and restore boundary remain unchanged on fresh current memory.

`Increment` performs a local counter effect and completes its job in one stable
replacement. A counter overflow has no effect and permanently fails that job,
including recurrence, without undoing unrelated attempts in the bounded batch.
Storage or transition failures still refuse the complete batch. `Deliver`
persists Running intent before calling an independent
receiver. The synchronous watchdog only prepares a bounded batch; an application
continuation performs each external call. An exact matching receipt records
Success. Other responses remain Uncertain. No automatic retries are configured.
Running and Uncertain records never redispatch after an upgrade.

Delivery replies use `ic-testkit-candid`'s native/Wasm decoder with an 8 KiB wire
ceiling, 4 KiB header, 64 type entries and finite decoding/skipping work limits.
These consumer-owned limits are separate from the host harness's management-page
allowance. Transport, decode and remote application errors remain unreadable
evidence; they do not become an authoritative absent receipt or a safe retry.
This decoder has no PocketIC, process or server dependency.

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
and `-Oz` outputs using the input's declared Wasm features. The Rust capture tool
owns each optimizer invocation, its log, exact capture and producer retirement;
IC Build owns capture finalization and retained-path errors. It
retains every input/output, hash, optimizer log and per-variant test/server log.
Failures stop before later variants and retain earlier evidence. This target
uses the unpublished host harness's `jobs-capture-artifact` executable, backed by
IC Build's exact capture and explicit retention. Jobs owns evidence filenames,
optimizer flags and final custody. Rename failures retain captured bytes and
print their location; success removes only the emptied staging child. The
original and every optimized output are captured before qualification, and the
capture executable's digest is recorded alongside source/tool evidence. Tooling
fixtures compile this helper and exercise it without launching PocketIC.
This target does not change the normal `build-consumer` artifact or select a production
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
