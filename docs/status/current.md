# Current handoff

## Source and pending work

Delivered source: Jobs **0.11.0**, `7e6c643c50dfe3f67252e51537f44391d92db2c7`.
The pending release is **0.12.0**: the Rust floor is raised to **1.91** across
the workspace, including native qualification with IC Build 0.4.0. CI, recovery
wrappers and focused checks select Rust 1.91.0; development remains on 1.99.0.
Slice deadline queries replace a private forwarder, and capture tests use Host's
fixture-directory allocator and executable-script writer. Delivery envelopes
derive their job identity from the admitted attempt; retry-deadline arithmetic
is shared by restore checks and result transitions without merging their checks.
Own package versions, staged work, Git refs, release
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

The delivered graph selects Memory 0.39.0, Testkit 0.44.0, Timers 0.23.0 and Build
0.3.3. The working tree selects Build 0.4.0, Host FS/Process/Artifacts 0.18.5 and
Cargo Platform 0.3.3; the new Rust 1.91 evidence below qualifies that graph. Prior API-cut
`before/` and un-suffixed logs retain the Testkit 0.43 graph; its `*-044.log`
evidence records Testkit 0.44 with Timers 0.22.6. Those receipts do not qualify
the current Timers 0.23 graph. The reviewed Shared Tooling snapshot remains 0.12.7.

## Current proof and blockers

Identity/retry/fixture cleanup evidence is in
`target/qualification/identity-retry-fixture/`. Rust 1.91 passes 45 core tests,
19 native composition scenarios, 23 stable-consumer tests, five capture tests,
native/Wasm floor checks, release Wasm and recovery-harness compilation.
Native/Wasm Clippy, formatting, pins and documentation links pass. This source
selects Timers 0.23.1; before files and final hashes bind the receipt to its graph.
Only Host's development-dependency edge was added; locked versions are unchanged
within this cleanup. No full CI or live PocketIC qualification ran.

Rust 1.91 evidence is in `target/qualification/msrv191/`, with before files,
source hashes and compiler identity. Core native/Wasm floor checks, 45 core
tests, 19 native composition scenarios, 23 stable-consumer tests, five capture
tests, release Wasm, recovery-harness compilation and native/Wasm Clippy pass.
The recovery-wrapper fixture passes with substituted optimizer/server effects;
no PocketIC qualification was selected. Formatting, pins, documentation links
and the unchanged shared snapshot also pass. Earlier Rust 1.88 receipts retain
their original source and graph scope.

Cleanup evidence is in `target/qualification/fixture-owner-cleanup/`.
`make check-consumer` passed all 23 native consumer tests, five capture tests,
release Wasm, recovery-harness compilation and native/Wasm Clippy on the earlier
Build 0.3.3 graph, which selected Host FS 0.18.4.

Fallible Memory fixture admission is proposed in
[Memory PR #111](https://github.com/dragginzgame/ic-memory/pull/111), stacked on
[PR #105](https://github.com/dragginzgame/ic-memory/pull/105).
Jobs adoption is tracked in [#50](https://github.com/dragginzgame/ic-jobs/issues/50).
The retained `memory-adoption/jobs-adoption.patch` and isolated checks cover
candidate `96b5a1b` on the earlier graph, including Rust 1.88 native/Wasm checks
and unchanged frozen-image refusal. Actual Jobs keeps published Memory setup
until a reviewed release supplies the helper; no path override is adopted.

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
At that earlier source, native consumer tests and Rust 1.88 native/Wasm floor
checks also passed; the two frozen inputs, Cargo manifests and lockfile retain
their exact hashes.

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

Delivered 0.11.0 native acceptance is verified at
[CI run 38702118931](https://github.com/dragginzgame/ic-jobs/actions/runs/38702118931),
source `7e6c643c50dfe3f67252e51537f44391d92db2c7`: Linux, macOS Apple Silicon
and macOS Intel all pass. The maintained implementations and regression coverage
support closing the older native/adoption acceptance issues. This run does not
qualify the pending 0.12 tree or establish live recovery.
Remaining compatible shared adoption is tracked in
[#47 — Fixture allocation](https://github.com/dragginzgame/ic-jobs/issues/47) and
[#48 — Completion admission](https://github.com/dragginzgame/ic-jobs/issues/48).
Both producers are delivered in Shared Tooling 0.14.0; Jobs still selects 0.12.7.
Memory adoption remains at [#50](https://github.com/dragginzgame/ic-jobs/issues/50).
The delivered timestamp change supersedes PR #49; draft
[PR #51](https://github.com/dragginzgame/ic-jobs/pull/51) awaits published Testkit
0.45 before its constructor change can be integrated.
The assertion-probe proposal is
[Shared Tooling PR #198](https://github.com/dragginzgame/shared-tooling/pull/198);
Jobs keeps its reviewed snapshot and local probe pending reviewed adoption.
GitHub owns current issue/PR status. No workflow was dispatched during this review.

See the [package migration guide](../../crates/ic-jobs/README.md#011-api-cut),
[safety contract](../../SAFETY.md), [consumer qualification](../../apps/job-consumer/README.md)
and [delivery procedure](../releasing.md).
