# Current handoff

## Source and pending work

Delivered source: Jobs **0.10.4**, `a4a58d18895a2536754216cb665e5efa811b4d01`.
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

The final graph selects Memory 0.39.0, Testkit 0.44.0, Timers 0.22.6 and Build
0.3.3. A concurrent edit selected Testkit 0.44 and Host 0.18.4 during this task;
it is preserved and excluded from task-attributed changes. Earlier `before/`
and un-suffixed logs retain the Testkit 0.43 graph. Final `*-044.log` evidence
records the updated graph. The reviewed Shared Tooling snapshot remains 0.12.7.

## Current proof and blockers

Evidence: `target/qualification/011-cut/`, including the before tree, selected
source hashes and logs. Focused checks cover 45 core tests, 19 native consumer
scenarios, five capture tests, Rust 1.88 core/consumer native and Wasm floors,
release Wasm, strict Clippy and rustdoc. The substituted recovery wrapper passes
on Bash 5 and genuine Bash 3.2, including optimizer arguments, frozen originals,
failed final capture and retained logs. These checks do not execute PocketIC.
The prepared Binaryen executable also produced all three optimized variants from
the real consumer Wasm; final artifact/tool hashes are under `binaryen-044/`.
The original `binaryen/` evidence remains bound to its earlier graph.

Two incoming dependency-adoption failures remain, with fresh logs in that evidence:
Memory 0.39 rejects the frozen 0.7.5 image's `slot0` ledger field (22 consumer tests
pass, one fails), and Testkit 0.44 does not export the harness's `PocketIcTimeExt`
(recovery harness compilation fails). Frozen bytes and selected dependencies are
preserved. Earlier successful checks do not qualify this graph for either gate.

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
