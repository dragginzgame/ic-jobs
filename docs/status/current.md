# Current handoff

## Accepted scope and merge state

Released local baseline: `f06b09fbf6f38c9a0644a95a47732ccff46e43b5` (Jobs 0.17.43).
The current local merge brings in `9552abb0adb05b5e0f9add6955fde7bb6efcaa50`,
the remote merge of [PR #77 — Add bounded transition logs to the Jobs consumer](https://github.com/dragginzgame/ic-jobs/pull/77).
The user requested conflict resolution. The two conflicts are CHANGELOG.md and
this handoff; retain the automatically merged implementation and dependency graph.
The merge is ready for the maintainer's commit after resolution is staged.
No release, publication or push is part of this conflict repair.

The released 0.17.43 dependency notes remain intact. Pending compatible Logging
notes move to 0.17.44; package versions remain 0.17.43. Published Jobs APIs,
Serde-only runtime, stored formats and Rust 1.91 floor are unchanged. No migration
or retained-data reset is needed. Existing staging outside the two conflicts is
preserved. Sibling repositories remain read-only.

## Combined selections and producer review

The merged graph retains Metrics 0.17.1, Platform/macros 0.7.6 and adds registry
Logging 0.17.1 only for the unpublished consumer's Wasm/test edges. Native DTOs do
not depend on Logging or Metrics. Build/Host/Testkit 0.69.11, Memory/Storage
0.54.0, Timers 0.32.0 and Wire/derive 0.7.21 remain selected; no other dependency
update is required by conflict resolution. Shared Tooling remains reviewed 0.26.25
at `6b6bbd9c9e5798f0ecf6105637392a7a55cd450e` (121 payloads).

Metrics producer `53eb4ee9599c8585cf83f5e7f43031f7671b8543` moved the repository
to IC Observability without changing arithmetic, counter/window semantics or
feature boundaries. Platform producer `8871f672dfbff7cbafb3e542508fdb97ae7bd376`
shares endpoint planning and bounded decoder internals while preserving explicit
codecs, caller-selected arity and nesting admission. Jobs retains its raw
unbounded call path and caller-owned outcome handling, without physical retry.

## Diagnostics and runtime contracts

Metrics observes successful committed transitions only, distinguishing dispatch,
retry, admitted outcomes and explicit uncertain-effect reconciliation. Refused
operations, stale callbacks, idle turns and failed writes add nothing. Reports
preserve u128 totals, actual-overflow flags, trusted IC version and nonwrapping
reset sequence. Manager-only reset changes neither durable bytes nor timers.
Reopening starts empty; compare within one canister/version and available reset
sequence. Settings changes may advance version without clearing counters.

Logging owns bounded heap retention and cursor scans. The consumer records
redacted dispatch/outcome/reconciliation context after successful commits:
128 events, 192-byte messages and 32-byte labels. Manager-only job_logs returns
at most eight matches after scanning at most 32 rows; empty filtered pages may
carry a continuation cursor. Append refusal increments an overflow-aware omitted
counter without changing committed work. Reconstruction starts empty; metrics
reset preserves logs. Cursors belong to one heap instance. There is no age timer,
stable log store or log reset. The demo exposes logs [--before SEQUENCE].
The [consumer guide](../../apps/job-consumer/README.md#committed-transition-logs)
owns logging semantics; its metrics section retains the outcome report contract.

Consumer storage, metadata, lifecycle, effect and timer authority remain local.
Every restore crosses Job::restore. Uncertain effects remain blocked until explicit
exact-receipt disposition; diagnostics never authorize retry or imply success.
Error ordering, task/scheduling policy and stable/Wire formats are unchanged.
Backing identity, exclusive custody and dropping all handles before reopening
remain required. Core Jobs stays independent of storage, Logging and Metrics.

## Qualification and evidence

Conflict inputs, original index/staged patch and final resolution evidence are
retained at `/tmp/ic-jobs-conflicts.GFrtQa01/`. Conflict repair changes only these
two documents. Focused checks cover local links, formatting, diff consistency,
locked metadata, snapshot integrity and dependency boundaries/graphs. Existing
implementation files and lock selection are preserved; no runtime suite is
repeated for the documentation resolution.

PR head `8c6f1f5ced01cd59141651c2fc26bfba90cdf241` passed ordinary CI on Linux,
Apple Silicon macOS and Intel macOS in
[run 40861537940](https://github.com/dragginzgame/ic-jobs/actions/runs/40861537940).
That graph used Metrics 0.17.0/Platform 0.7.5; it establishes native macOS caller
qualification for that PR, including the earlier Make fixture repair. Review and
merge evidence is `/tmp/ic-jobs-pr77.wuvimOgp/`; original Logging checks are
`/tmp/ic-jobs-logging-*.log` and its isolated checkout `/tmp/ic-jobs-logging-20261029`.

Dependency-adoption evidence at `/tmp/ic-jobs-metrics-platform.kiF7mYHj/` passed
57 Rust 1.91 consumer tests, six native harness tests, strict native/Wasm Clippy,
release consumer Wasm and dependency/document checks before Logging was combined.
Those separate graph results do not establish current combined installed recovery.
The eleven gated live recovery cases remain separately selected; no combined
live PocketIC, native macOS matrix or IC cost claim is made by conflict repair.
Platform call ownership and current installed qualification remain tracked in
[#75 — Keep job-consumer ICC execution under Platform ownership](https://github.com/dragginzgame/ic-jobs/issues/75).

Earlier evidence remains at `/tmp/ic-jobs-metrics-macos.dHKxquDK/`,
`/tmp/ic-jobs-upstream-adoption.0jRpTmuM/`, `/tmp/ic-jobs-memory054.sqjjQkAo/` and
`/tmp/ic-jobs-adapter-reference.rbwXpr7F/`. The last recorded three-host live
qualification used Jobs 0.17.14/Tooling 0.63.2:
[108 successful executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
