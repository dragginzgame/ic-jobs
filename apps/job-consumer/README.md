# Jobs test consumer

This unpublished canister is an application-owned consumer of Jobs, Memory and
Timers. It is a qualification fixture with deliberate fault endpoints, and must
not be deployed as a production service. The Jobs library acquires no storage,
authentication, payload, lifecycle or direct CDK timer ownership.

The manager principal is supplied at installation and persisted. Management
calls use the actual IC caller before reading or mutating the queue. `create`
allocates an identity, `inspect` reads one job, `list` returns up to eight jobs
after an exclusive ID cursor, `cancel` cancels Pending work, and `dispatch_due`
starts at most two eligible attempts. All task history is retained: IDs cannot
be reused. There is no force-run or arbitrary rescheduling command.

One Memory host bootstraps the `test.jobs.` namespace and opens the permanent
`test.jobs.consumer.v1` key. One stable cell retains the bounded JSON snapshot:
manager/receiver configuration, ID authority, JobRecords, immutable payloads,
counter and receipts. The queue holds at most 32 jobs and 32 receipts, each
delivery payload is at most 256 UTF-8 bytes, and serialized state is at most
24 KiB. The complete bounded queue supplies the global earliest deadline, so no
separate unbounded due index is needed. Every decoded record crosses
`Job::restore`, including future work. Invalid retained state refuses recovery;
installation cannot overwrite existing state. This is a new test installation
format, not a replacement reader for any existing deployment.

`Increment` performs a local counter effect and completes its job in one stable
replacement. `Deliver` persists Running intent before calling an independent
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
Successful create/cancel/dispatch/completion rederives its deadline; no Pending
work leaves it inactive. Reconciliation failure traps the same synchronous
segment as the stable write, preventing a falsely successful persisted schedule.
External-call continuations have separate message boundaries.

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

The canonical Shared Cargo installer owns the pinned Testkit CLI selection in
[ci/testkit-tools.env](../../ci/testkit-tools.env). Testkit owns PocketIC setup,
offline admission and server lifecycle; this repo has no second server installer
or pin catalog. Normal checks install nothing. `test-canister` checks the CLI and
server before executing tests and uses Cargo metadata to locate the built Wasm,
including a caller-selected target directory. Linux x86-64 and both macOS
architectures use Testkit's supported server selections. Host Testkit/Host crates
remain outside the canister and public Jobs normal Wasm dependency graphs.

After [the recovery workflow](../../.github/workflows/recovery.yml) is delivered
to `main`, select **Actions → Canister recovery → Run workflow** and choose the
branch to qualify. This manual trigger runs all five tests on Linux, Apple Silicon
and Intel macOS. Ordinary push/PR CI still compiles the harness without starting
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
controller-snapshot recovery. Deliberate `fixture_*` endpoints implement these
faults in the unpublished Wasm rather than faking IC behavior with `cfg(test)`.
Their API must never be copied into a production canister. Live execution and
native macOS acceptance are recorded separately in the
[handoff](../../docs/status/current.md); harness compilation alone does not close
[#11](https://github.com/dragginzgame/ic-jobs/issues/11).
