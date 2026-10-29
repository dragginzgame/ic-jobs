# Current handoff

## Accepted scope and current state

Released baseline: `cdb48cc9d1c3281e2b898bcfe4a19e2a673b4560` (Jobs 0.17.47).
The user requested the new IC Tooling, Memory and Storage packages. The local
batch adopts Build/Host/Testkit 0.70.2 and Memory/Storage 0.56.0; matching
Testkit and release-metadata CLIs are explicitly prepared and admitted offline.
The incoming lock selections of Tooling 0.70.2 and Platform/macros 0.8.2 are
preserved. Timers 0.32.1, Metrics 0.17.4, Logging 0.17.2, Wire/derive 0.7.23
and PocketIC 16.1.0 remain selected.

Draft notes target 0.17.48: the published Serde-only Jobs library's API, behavior,
features and Rust floor are unchanged. The unpublished qualification canister's
Memory format cut and required operator action are explicit below and in its
[consumer guide](../../apps/job-consumer/README.md). Workspace versions remain
0.17.47. No staging, commit, push, release, publication or deployment was selected.
Sibling checkouts stayed read-only. Shared Tooling remains reviewed 0.26.25 at
`6b6bbd9c9e5798f0ecf6105637392a7a55cd450e` (121 verified payloads).

## Memory installation boundary

Memory 0.56 replaces CBOR allocation-ledger/Cell values with bounded Wire,
using `ICML`/`ICMWIRE1`; application queue JSON, RPC bytes, collection/substrate
headers and Jobs records are unchanged. Both retained opening and fresh creation
refuse the earlier physical ledger before writes. This is not an in-place upgrade
path for retained installations. Use a fresh qualification canister or resolve
and qualify an explicit consumer-owned transition before installing over data.
Preserve old binaries/backups and exact outstanding attempts/receipts until their
owner resolves disposition. Reinstallation discards application data and needs
owner authorization; never clear only the ledger while leaving collections.
This source update resets no data and adds no converter, fallback or auto-reset.

The frozen Memory 0.55.2 image comes from the released baseline on Rust 1.91.
Its old producer reconstructs exact backing bytes, cold-opens the same Running
delivery and refuses redispatch after releasing all handles. Under 0.56, both
open modes refuse with zero writes, unchanged pages and byte-identical backing;
its independently decoded current queue JSON still crosses Job::restore as
Running/blocked. See [fixture provenance](../../crates/ic-jobs/tests/fixtures/README.md).
Current-format reopening, blocked uncertainty and all prior refusal tests pass.

## Maintained runtime contract

Every decoded job crosses Job::restore. Metadata, allocation, storage, clock,
lifecycle, timers and effects stay consumer-owned. Error ordering, scheduling and
explicit uncertain-effect disposition are unchanged. No database, lifecycle
exports, global handler registry or direct IC timer provider enters pure Jobs.
All handles drop before reopening a backing. Neither stable diagnostics adapter
is selected; their separate format retirements do not apply here.

Each owned Wire envelope goes once through Platform's Call::unbounded_wait.
Running intent commits first; exact job/occurrence/attempt/payload/receipt
identities remain authoritative. Rejects, invalid replies, timeout, abandoned
futures, reopen and upgrade do not prove no effect or permit redispatch.
No generic call layer, physical retry/fallback or mandatory ic-call adapter is
needed. Metrics observes committed transitions; bounded heap-only logs and metrics
never prove completion or authorize retries. Reset changes no job/timer state.

## Local qualification and evidence

`/tmp/ic-jobs-tooling0702-memory056.8W4xazfe/` retains incoming inputs, producer
diffs/review, frozen old-producer source and logs, qualified lock/metadata, current
patch, compiler/CLI identities and Wasm. Rust 1.91 passed 62 consumer tests,
six native harness tests and the release Wasm build. Strict native/Wasm Clippy,
snapshot/pin/dependency-boundary admission and all five graph checks pass.
All seven consumer tooling phases and the release-metadata fixture group pass;
the latter uses real local Git with substitute gate/registry/upload effects.
Core Rust-floor evidence remains applicable to unchanged source/features/Serde at
`/tmp/ic-jobs-upstream-adoption.0jRpTmuM/`.

No new live PocketIC, full `make ci` or hosted matrix was selected for this batch.
The new Memory 0.56/Platform 0.8.2 graph has local Linux native/Wasm evidence;
new-format installed recovery and macOS native/live qualification remain separate.
Do not apply older recovery results to the changed ledger format.

## Completed issue acceptance and historical live evidence

[#75 — Keep job-consumer ICC execution under Platform ownership](https://github.com/dragginzgame/ic-jobs/issues/75)
is closed. [Recovery run 40909177479](https://github.com/dragginzgame/ic-jobs/actions/runs/40909177479)
passed Linux, Apple Silicon macOS and Intel macOS: eleven cases on original/O3/Os/Oz
per host, 132 live executions. It tests v0.17.46/878e93c752ff5777ad5b11ba2e44611b3c0c319e,
Platform 0.8.1, Timers 0.32.0, Metrics 0.17.3 and Wire 0.7.22. Per-host exact
source/lock, tools, Wasm and server logs remain in workflow artifacts; closure
observations are also at `/tmp/ic-jobs-close75.TxcKAWyv/`.

[#76 — Add bounded job-outcome Metrics at committed consumer transitions](https://github.com/dragginzgame/ic-jobs/issues/76)
is closed. Delivered [0.17.47 CI](https://github.com/dragginzgame/ic-jobs/actions/runs/40917200431)
passes all three hosts. Its Timers 0.32.1/Metrics 0.17.4/Wire 0.7.23 graph has
separate Linux installed evidence: eleven cases across four Wasm variants,
44 executions including trapped-message metrics rollback, retained under
`/tmp/ic-jobs-issue-qualification.qMGHSSmz/live/jobs-recovery.Tdw2kS/`.
That run uses Memory 0.55.2 and Platform 0.8.1; it is historical for this batch.
No instruction/cycle cost claim or new histogram requirement follows from it.
