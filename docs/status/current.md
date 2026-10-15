# Current handoff

## Source and pending work

Delivered source: Jobs **0.10.4**, `a4a58d18895a2536754216cb665e5efa811b4d01`.
The current 0.11 preparation source is `5e73064`, with the focused fixes below.
The pending changelog is **0.11.0** because the public batch API and timer feature
are removed in a hard cut. Own package versions, staged work, Git refs, release
and publication remain maintainer-owned; no release or upload ran.

Use `next_due_ns(&jobs)` and `start_next(&mut jobs, now_ns)` instead of constructing
`Scheduler`. Consumers depend on IC Timers directly for watchdog reconciliation
and completion; Jobs has no runtime Timers dependency. All in-tree callers,
examples, package docs and floor checks use this contract. Job records, ordering,
transition errors and uncertain-effect disposition are unchanged.

The Rust capture tool owns the Jobs optimizer recipe, logs and producer retirement.
It reuses IC Build's published `retain_to` finalization, avoiding a second generic
capture owner. Jobs still selects original/O3/Os/Oz and PocketIC qualification.
Failed optimizer/capture attempts retain producer evidence; successful capture
retires only its producer. Hashing uses the reviewed shared checksum helper.

The current graph selects Memory 0.39.0, Testkit 0.44.0, Timers 0.23.0 and Build
0.3.3. The admission repair preserves all dependency selections. Prior API-cut
`before/` and un-suffixed logs retain the Testkit 0.43 graph; its `*-044.log`
evidence records Testkit 0.44 with Timers 0.22.6. Those receipts do not qualify
the current Timers 0.23 graph. The reviewed Shared Tooling snapshot remains 0.12.7.

## Current proof and blockers

Earlier API-cut evidence: `target/qualification/011-cut/`, including the before
tree, selected source hashes and logs. Its focused checks cover 45 core tests, 19 native consumer
scenarios, five capture tests, Rust 1.88 core/consumer native and Wasm floors,
release Wasm, strict Clippy and rustdoc. The substituted recovery wrapper passes
on Bash 5 and genuine Bash 3.2, including optimizer arguments, frozen originals,
failed final capture and retained logs. These checks do not execute PocketIC.
The prepared Binaryen executable also produced all three optimized variants from
the real consumer Wasm; final artifact/tool hashes are under `binaryen-044/`.
The original `binaryen/` evidence remains bound to its earlier graph.

The earlier Memory/Testkit failures are repaired in
`target/qualification/memory039-admission/`. All 23 consumer tests, five capture
tests, release Wasm, recovery harness compilation and native/Wasm Clippy pass via
`make check-consumer`. The frozen two-slot image is now qualified for refusal on
Memory 0.39, with zero stable writes and unchanged pages during both recovery and
installation requests. The separate frozen Jobs JSON still reopens on current
memory and retains blocked effects. No image was rewritten and no reset was run.
Retained pre-cut installations still need explicit owner disposition.

The recovery harness reads native PocketIC nanoseconds directly and uses Testkit's
public rejection field. These are compilation checks; no live test was selected.
The initial failed attempt and prior failing logs remain with their source scope.
Current-graph native consumer tests and Rust 1.88 native/Wasm floor checks also
pass; the two frozen inputs, Cargo manifests and lockfile retain their exact hashes.

Prior focused evidence remains in `target/qualification/consumer-layout-cleanup/`,
`target/qualification/dispatch-lookup-cleanup/`,
`target/qualification/active-delivery-cleanup/`,
`target/qualification/deadline-reply-cleanup/` and
`target/qualification/receipt-extraction-cleanup/`, with its original source scope.
Older proof is recorded in [history through 0.10.3](history-through-0.10.3.md) and
[history through 0.9.1](history-through-0.9.1.md).

## Acceptance boundaries

Full `make ci` and live PocketIC recovery were not selected. Local Linux execution
and artifact transformation do not establish macOS Intel/Apple Silicon acceptance,
real IC rollback, installed upgrades or timer delivery for this source/graph.
The last recorded live evidence is Jobs 0.8.1, source
`f9c0c0eb4e8363565be63f9434a0cf8b9940294b`,
[run 38289182820](https://github.com/dragginzgame/ic-jobs/actions/runs/38289182820):
84 passes across seven scenarios, four Wasm variants and three hosts.

Native acceptance links remain [#41](https://github.com/dragginzgame/ic-jobs/issues/41),
[#42](https://github.com/dragginzgame/ic-jobs/issues/42),
[#43](https://github.com/dragginzgame/ic-jobs/issues/43),
[#44](https://github.com/dragginzgame/ic-jobs/issues/44) and
[#46](https://github.com/dragginzgame/ic-jobs/issues/46).
Shared fixture/completion adoption is tracked in
[#47](https://github.com/dragginzgame/ic-jobs/issues/47) and
[#48](https://github.com/dragginzgame/ic-jobs/issues/48).
The assertion-probe proposal is
[Shared Tooling PR #198](https://github.com/dragginzgame/shared-tooling/pull/198);
Jobs keeps its reviewed snapshot and local probe pending reviewed adoption.
GitHub owns current issue/PR status; no new hosted result was inspected here.

See the [package migration guide](../../crates/ic-jobs/README.md#011-api-cut),
[safety contract](../../SAFETY.md), [consumer qualification](../../apps/job-consumer/README.md)
and [delivery procedure](../releasing.md).
