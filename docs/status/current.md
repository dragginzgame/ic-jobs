# Current handoff

## Accepted scope and inputs

Released baseline: `f38aef08584a7ad632961eb4c3dcc4e01f4509a3` (Jobs 0.17.42).
It contains consumer-owned outcome metrics and the macOS demo fixture repair.
The current isolated contribution adds bounded IC Logging to the consumer only.
The compatible draft is 0.17.43: published Jobs APIs, runtime dependencies, stored
records and existing Wire replies remain unchanged. Package versions remain
0.17.42. The shared isolated contribution workflow owns its branch/PR; merge,
version changes, release and publication remain maintainer-owned.

Concurrent lock selections were preserved, not chosen by this task: Build/Host/
Testkit 0.69.11 and Platform/macros 0.7.5. The qualified graph also contains
Memory/Storage 0.54.0, Timers 0.32.0, Metrics 0.17.0 and Wire/derive 0.7.21.
The only current lock change adds registry IC Logging 0.17.1 to the unpublished
consumer's Wasm/test edges; no existing dependency selection changes.
Shared Tooling remains reviewed 0.26.25 at
`6b6bbd9c9e5798f0ecf6105637392a7a55cd450e` (121 verified payloads).
The normal sibling checkout and snapshot payloads stay unchanged. Work is in
`/tmp/ic-jobs-logging-20261029`, branch `codex/jobs-bounded-logging`.

## Released metrics behaviour

The consumer records committed dispatches, outcomes and reconciliation using fixed
MeasurementCounters and a MeasurementWindow. Manager-only `job_metrics` and
`reset_job_metrics` use the existing Wire budgets; the demo exposes `metrics` and
`reset-metrics`. Reports retain u128 totals, actual-overflow flags and nonwrapping
reset sequence. Failed/refused work and idle turns add nothing; uncertain results
never become success through projection. Reset touches no stable bytes or timers;
reconstruction starts empty. Compare only within the same canister/version and
available reset sequence; settings changes can advance version without reset.

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

## Consumer logging

IC Logging owns heap retention, append admission and bounded cursor scans. The
consumer owns fixed event selection, IC time, manager authority and Wire DTOs.
Dispatch/outcome/reconciliation events publish only after successful stable
commits. Payloads and principals are excluded; uncertain work stays blocked.
The history retains 128 events with 192-byte messages and 32-byte labels. Each
`job_logs` query returns at most eight matches after scanning at most 32 rows;
filtered empty pages may carry a continuation cursor. Append refusal increments
an overflow-aware omitted counter without refusing committed work. No age timer,
stable log store or log reset is added. Reconstruction starts empty; metrics reset
preserves logs. Cursors are local to a heap instance, never durable identity.
The demo exposes `logs [--before SEQUENCE]`. The
[consumer guide](../../apps/job-consumer/README.md#committed-transition-logs)
owns these semantics. Published Jobs and native DTO consumers do not link Logging.

## Maintained contracts

Consumer storage, metadata, lifecycle, effect and timer authority remain local.
Every restore crosses Job::restore; uncertain effects remain blocked until explicit
exact-receipt disposition. Error ordering, retries, task/scheduling policy and
stable/Wire formats remain unchanged. Metrics reset cannot unblock work.
Backing identity, exclusive allocation custody and dropping all handles before
reopening remain required. Native VectorMemory tests do not establish IC recovery.

## Qualification and remaining work

Current logs and commands are `/tmp/ic-jobs-logging-*.log`. Strict native
consumer/harness and consumer Wasm Clippy pass. Rust 1.91 passes 38 consumer unit
tests, four Wire tests, the native reply-budget and demo parser tests, and the
release consumer Wasm build. Formatting, all five dependency graphs, dependency
boundaries/inheritance, snapshot integrity, local links and diff checks pass.
No full suite, installed PocketIC case, new IC cost measurement or native macOS
qualification is selected by this contribution. The code adds one heap log owner
and a protected reader; generic retention/paging remains solely in IC Logging.

Retained 0.17.42 evidence: `/tmp/ic-jobs-metrics-macos.dHKxquDK/` retains the base, incoming handoff,
qualified lock/metadata, macOS failure artifact, command logs, tools/inputs,
consumer Wasm and final patch. Failed legacy-tool bootstrap attempts are retained
alongside the successful clean Bash 3.2.57 build and fixture run.

Those earlier checks passed: snapshot/pins/boundaries and all five dependency graphs;
strict native consumer/harness and consumer Wasm Clippy; 57 Rust 1.91 consumer
unit/Wire/composition tests; focused native capture/parser/reply-budget tests;
Rust 1.91 release consumer Wasm; ShellCheck with sourced helpers; formatting,
document links and diff checks. The seven-phase consumer tooling fixture passes
with current Linux tools and Linux-built GNU Make 3.81/Bash 3.2.57. Substitute
compiler/server/optimizer effects prove caller wiring, not installed recovery.

The new ignored live metrics authorization/reset/upgrade case is compiled but
not executed. Logging adds another explicitly gated authorization/restart case
and a rollback assertion to the watchdog case. These are compiled, not executed.
Native macOS
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
