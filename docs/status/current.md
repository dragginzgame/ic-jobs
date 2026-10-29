# Current handoff

## Accepted scope and inputs

Released baseline: `f8fc7325af316fa5ca84b26a701e5e57059006f0` (Jobs 0.17.41).
The accepted uncommitted batch implements consumer-owned job-outcome metrics and
repairs the macOS demo tooling fixture. The compatible draft is 0.17.42: published
Jobs APIs, runtime dependencies, stored records and existing Wire replies remain
unchanged. Package versions remain 0.17.41. Staging, commits, pushes, release and
publication remain maintainer-owned.

Concurrent lock selections were preserved, not chosen by this task: Build/Host/
Testkit 0.69.11 and Platform/macros 0.7.5. The qualified graph also contains
Memory/Storage 0.54.0, Timers 0.32.0, Metrics 0.17.0 and Wire/derive 0.7.21.
The only task-owned lock change adds Metrics to the unpublished consumer's edges.
Shared Tooling remains reviewed 0.26.25 at
`6b6bbd9c9e5798f0ecf6105637392a7a55cd450e` (121 verified payloads).
Sibling repositories stayed read-only; snapshot payloads are unchanged.

## Delivered behavior

The consumer composes fixed shared MeasurementCounters and a MeasurementWindow
for committed dispatch, retry dispatch, success, retryable failure, permanent
failure, uncertainty and explicit reconciliation. Observations publish only after
Cell commits succeed; stale/refused operations, idle turns and failed writes add
nothing. Retry exhaustion does not reclassify an admitted outcome. Repeated
still-uncertain resolution counts reconciliation/uncertainty, never success.

Manager-only `job_metrics` and `reset_job_metrics` use distinct unit-request Wire
identities and existing budgets. The demo exposes `metrics` and `reset-metrics`.
Reports preserve u128 totals, sticky actual-overflow flags, trusted IC canister
version and the shared nonwrapping reset sequence. Reset touches neither stable
bytes nor timers. Reconstruction starts an empty window; observations do not
backfill durable records. Compare only within the same canister/version and
available reset sequence. Settings changes can advance version without resetting
counts. No dynamic labels or histogram are needed by this reporting requirement.

Metrics is selected directly only for consumer Wasm/tests. Native DTOs remain
independent primitive projections; published Jobs remains Serde-only. The
[consumer guide](../../apps/job-consumer/README.md#committed-job-outcome-diagnostics)
owns the report semantics and limitations. Owning scope:
[#76 — Add bounded job-outcome Metrics at committed consumer transitions](https://github.com/dragginzgame/ic-jobs/issues/76).

Both macOS jobs in [CI run 40845742688](https://github.com/dragginzgame/ic-jobs/actions/runs/40845742688)
failed in the demo-command fixture: direct recipe Cargo escaped its stub PATH
and ran outside a Cargo workspace. The fixture now binds stub PATH before
launching Make, covering direct and recursive recipes, and prints its retained
failure log while preserving exit status. Linux legacy-tool probes alone did not
reproduce the native executable-lookup behavior; native macOS rerun remains due.

## Maintained contracts

Consumer storage, metadata, lifecycle, effect and timer authority remain local.
Every restore crosses Job::restore; uncertain effects remain blocked until explicit
exact-receipt disposition. Error ordering, retries, task/scheduling policy and
stable/Wire formats remain unchanged. Metrics reset cannot unblock work.
Backing identity, exclusive allocation custody and dropping all handles before
reopening remain required. Native VectorMemory tests do not establish IC recovery.

## Qualification and remaining work

Evidence: `/tmp/ic-jobs-metrics-macos.dHKxquDK/` retains the base, incoming handoff,
qualified lock/metadata, macOS failure artifact, command logs, tools/inputs,
consumer Wasm and final patch. Failed legacy-tool bootstrap attempts are retained
alongside the successful clean Bash 3.2.57 build and fixture run.

Fresh checks pass: snapshot/pins/boundaries and all five dependency graphs;
strict native consumer/harness and consumer Wasm Clippy; 57 Rust 1.91 consumer
unit/Wire/composition tests; focused native capture/parser/reply-budget tests;
Rust 1.91 release consumer Wasm; ShellCheck with sourced helpers; formatting,
document links and diff checks. The seven-phase consumer tooling fixture passes
with current Linux tools and Linux-built GNU Make 3.81/Bash 3.2.57. Substitute
compiler/server/optimizer effects prove caller wiring, not installed recovery.

The new ignored live metrics authorization/reset/upgrade case is compiled but
not executed. The harness now has ten explicitly gated live cases. Native macOS
Intel/Apple Silicon CI and current-graph PocketIC acceptance remain outstanding;
full make ci and live qualification were not selected. No new instruction/cycle
cost or actual IC metrics trap-rollback claim is made. Platform call ownership
and current installed qualification remain separate in
[#75 — Keep job-consumer ICC execution under Platform ownership](https://github.com/dragginzgame/ic-jobs/issues/75).

Core Rust 1.91 native/Wasm evidence remains applicable to unchanged runtime
source/features/Serde at `/tmp/ic-jobs-upstream-adoption.0jRpTmuM/`. Prior adapter
reference evidence remains at `/tmp/ic-jobs-adapter-reference.rbwXpr7F/` and
Memory adoption evidence at `/tmp/ic-jobs-memory054.sqjjQkAo/`; their older graphs
do not establish current installed acceptance. The last recorded three-host live
qualification used Jobs 0.17.14/Tooling 0.63.2:
[108 successful executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
