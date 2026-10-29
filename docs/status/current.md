# Current handoff

## Accepted scope and inputs

Released baseline: `f38aef08584a7ad632961eb4c3dcc4e01f4509a3` (Jobs 0.17.42).
The accepted batch adopts Metrics 0.17.1 and Platform/macros 0.7.6. These versions
were already selected in the incoming dirty lockfile; root requirements now match.
Targeted registry resolution selected no further packages. Other selections remain
Build/Host/Testkit 0.69.11, Memory/Storage 0.54.0, Timers 0.32.0 and Wire/derive
0.7.21. Existing feature lists are unchanged. Siblings remain read-only.

The compatible draft is 0.17.43: the published Jobs contract, Serde-only runtime,
stored formats and supported Rust floor are unchanged. No consumer migration or
retained-data reset is required. Package versions remain 0.17.42. Staging,
commits, pushes, release and publication remain maintainer-owned.
Shared Tooling remains reviewed 0.26.25 at
`6b6bbd9c9e5798f0ecf6105637392a7a55cd450e` (121 verified payloads).

## Producer review and adoption

Metrics release `53eb4ee9599c8585cf83f5e7f43031f7671b8543` moves the repository
to IC Observability and gives the existing published crate its package README.
Arithmetic, counters, windows, overflow quality and dependency/feature boundaries
are unchanged; the sole Metrics source difference is a transport doc comment.
The new unpublished Logging crate and dirty producer edits are not adopted.

Platform release `8871f672dfbff7cbafb3e542508fdb97ae7bd376` shares endpoint
planning with typed external adapters and centralizes bounded reply decoding.
Explicit guard/decode/encode paths, caller-selected arity and nesting admission
remain intact. Jobs keeps its existing Wire endpoints, raw unbounded call path,
Principal conversions and caller-owned outcome handling. No generic wrapper,
physical retry, transport ownership change or new policy is needed.

## Maintained runtime and diagnostics

Consumer storage, metadata, lifecycle, effect and timer authority remain local.
Every restore crosses Job::restore; uncertain effects remain blocked until explicit
exact-receipt disposition. Error ordering, retries, task/scheduling policy and
stable/Wire formats remain unchanged. Backing identity and exclusive allocation
custody remain required; all handles drop before reopening.

Released 0.17.42 metrics observe only successful consumer commits. Fixed shared
counters distinguish dispatch/retry, admitted outcomes and explicit resolution;
stale callbacks, refused operations, idle turns and failed writes add nothing.
Manager-only job_metrics/reset_job_metrics and CLI metrics/reset-metrics preserve
u128 totals, actual-overflow flags, trusted IC version and nonwrapping reset
sequence. Reset changes neither durable bytes, retry authority nor timers.
Reopening starts empty observations; compare only within the same canister/version
and available reset sequence. IC settings may advance version without clearing
counts. Native DTOs stay independent; keyed recording remains disabled.
The [consumer guide](../../apps/job-consumer/README.md#committed-job-outcome-diagnostics)
owns these report semantics and limitations.

## Qualification and remaining work

Current evidence: `/tmp/ic-jobs-metrics-platform.kiF7mYHj/` retains the incoming
lock/patch/handoff and released base, reviewed producer deltas and VCS identities,
registry resolution, qualified metadata/lock, command logs, Wasm and final patch.

Fresh checks pass: snapshot/pins/boundaries and all five dependency graphs,
strict native consumer/harness and consumer Wasm Clippy, 57 Rust 1.91 consumer
unit/Wire/composition tests, six native capture/parser/reply-budget tests and the
Rust 1.91 release consumer Wasm. Formatting, documentation links and diff
consistency pass. Results are bound to the source, graph and compiler identities.
Core Rust 1.91 native/Wasm evidence remains applicable to unchanged runtime
source/features/Serde at `/tmp/ic-jobs-upstream-adoption.0jRpTmuM/`.

Prior consumer metrics/macOS fixture evidence is retained at
`/tmp/ic-jobs-metrics-macos.dHKxquDK/`: 57 native consumer tests and current/legacy
Linux tooling fixtures passed. The fixture binds stub PATH before Make starts and
prints retained failures. Its scripts and Tooling selections are unchanged by
this adoption, so their current-tool and Make 3.81/Bash 3.2.57 caller evidence is
reused; Linux does not establish native macOS executable lookup or recovery.

The ten explicitly gated live recovery cases compile but were not executed for
this graph. Native macOS Intel/Apple Silicon and current-graph PocketIC acceptance
remain outstanding; full make ci and live qualification were not selected. No new
IC instruction/cycle cost or metrics trap-rollback claim is made. Direct Platform
call ownership and installed recovery remain separate in
[#75 — Keep job-consumer ICC execution under Platform ownership](https://github.com/dragginzgame/ic-jobs/issues/75).

Prior adapter evidence remains at `/tmp/ic-jobs-adapter-reference.rbwXpr7F/` and
Memory adoption evidence at `/tmp/ic-jobs-memory054.sqjjQkAo/`. Their older graphs
do not establish current installed acceptance. The last recorded three-host live
qualification used Jobs 0.17.14/Tooling 0.63.2:
[108 successful executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
