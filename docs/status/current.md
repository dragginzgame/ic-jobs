# Current handoff

## Source and pending work

Delivered Jobs **0.13.1**: `eac5d650a4316d7c7d315fa459c1e046486277b2`.
Pending compatible **0.13.2** reuses the consumer durable-result wrapper for
both dispatch and management, reads delivery payloads without mutable access and
uses Shared's dependency-order fixture instead of local AWK parsing.
Shared Tooling remains **0.15.2** at
`ca98798920188da9a21041ff9fabb76ec18f41ac` (110 payloads). The canonical exporter adds the dependency-order and workflow
extraction helpers already present in that revision; no baseline refresh or
executable pin update is needed.
Release preparation now uses Shared's metadata renderer directly. Jobs no longer
requires cargo-edit, a separate release-tool catalog or local setup/check targets.
Common tool admission and metadata refusal/rollback boundaries remain in place.

The graph selects Memory 0.43.0, Testkit 0.48.0, Timers 0.25.1, Build 0.9.0 and
direct Host FS/Process 0.21.0. Rust floor is 1.91.0; development is 1.99.0.
Incoming manifest/lock edits are preserved. Own versions, index, refs, release and
publication remain maintainer-owned.

`start_next` returns `(index, attempt)`. Commit `jobs[index].record()` with its
immutable payload/index before external dispatch. All restore paths retain
`Job::restore`; ordering, transition errors and uncertain-effect disposition are
unchanged. Consumers own storage and depend on Timers directly. Jobs has no
runtime Timers dependency or lifecycle exports. The capture tool owns Jobs'
optimizer recipe and uses IC Build's retained path; Jobs selects the live gate.

## Qualification

View/snapshot/probe cleanup evidence is in
`target/qualification/view-snapshot-cleanup-probes/`. Inspection/list replies now
derive their checked identity from the validated job at one view boundary; the
composition snapshot no longer derives unused cloning. All 23 consumer and
19 composition tests, separate Rust 1.91 core/consumer Wasm checks, native/Wasm
consumer Clippy and formatting pass on the preserved incoming graph.

[Shared PR #237](https://github.com/dragginzgame/shared-tooling/pull/237) adds
cleanup refusal to the standard exit-probe roster. Producer callers pass Bash 5
and genuine Bash 3.2. A canonical prospective 111-payload export at
`cd2a673353275d09be8223420b88e5a165a84534` passes integrity and the actual Jobs
release/publication fixtures on both shells with external effects substituted.
The apply-checked Jobs deletion is retained under `adoption/`; active Shared
0.15.2 still needs its explicit cleanup case. Reviewed export and delivered/native
acceptance remain with [Jobs #60](https://github.com/dragginzgame/ic-jobs/issues/60).
No active snapshot, root index/ref, own version, live recovery, release or
publication changed. The Shared contribution used its isolated PR workflow;
no sibling working checkout was edited. Linux Bash 3.2 is not macOS proof.

Capture/admission/roster cleanup evidence is in
`target/qualification/capture-admission-roster-cleanup/`. Recovery-only Candid,
Testkit, consumer and Serde edges are development dependencies; the capture
binary's normal graph excludes Testkit/PocketIC and the consumer package. Its
Rust 1.91 build and five tests pass, as does recovery-harness compilation on the
incoming Build 0.9 / Testkit 0.48 graph. Cargo discovery executes all 45 current
core integration tests without a manual target roster. The actual hosted Testkit
selection step uses the Make adapter once, keeping its checked path separate
from retained diagnostics; success/refusal and the remaining consumer-tooling
fixtures pass Bash 5 and genuine Bash 3.2 with tool/server effects substituted.
ShellCheck, manifest formatting and documentation links pass. Incoming root
manifest/lock selections and the shared snapshot are preserved. No live recovery
or full CI ran; native macOS acceptance remains separate.

[Delivered 0.13.0 CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38753013217)
passes Linux, macOS Apple Silicon and macOS Intel at
`123b8f0c9a5abcabd9498503d947d45908aa49df`. This supplies delivered/native acceptance for shared fixture allocation
and completion admission (#47/#48); it does not qualify later edits.

Current-graph focused Rust receipts are in
`target/qualification/dispatch-write-toolchain-cleanup/`: 19 composition tests,
23 consumer tests, five capture tests, harness compilation, strict Clippy and
separate Rust 1.91 core/consumer Wasm checks pass. Actual CI/recovery toolchain
inputs and release/evidence/consumer fixtures pass Bash 5 and genuine Bash 3.2.
Source snapshots bind receipts to their graph; earlier graphs remain historical.

Current tooling cleanup receipts are in
`target/qualification/release-frontend-handoff-cleanup/`. Canonical export preview
confirmed that `scripts/dev/format-frontend.sh` remains required by the adopted
`test-git-hooks.sh` frontend cases. It is retained with its explicit snapshot root;
removal requires splitting the canonical hook fixture first, not deleting a
snapshot payload. No runtime frontend formatting flow is enabled in Jobs.
Release/publication and consumer-tooling fixtures pass Bash 5 and genuine Bash
3.2 with external effects substituted. Hook qualification, snapshot integrity,
pins, release adapters, source-following ShellCheck and documentation links pass.
The first hook run caught a deleted-catalog fixture input; its failure is retained
and the corrected run passes. No Rust source changed in this tooling batch.

Dispatch/delivery/formatting cleanup evidence is in
`target/qualification/dispatch-delivery-format-cleanup/`, based on delivered
0.13.1. All 23 consumer tests pass, including idle zero-write, unchanged bytes,
exact receipts and refusal ordering. Separate Rust 1.91 core/consumer Wasm checks,
strict native/Wasm consumer Clippy and recovery-harness compilation pass. Actual
formatting/hook checks pass with prepared formatters; the Jobs caller also passes
on genuine Bash 3.2. Snapshot integrity, pins, release adapters, ShellCheck, Perl
syntax and documentation links pass. Existing test expectations follow the same
deadline/payload results; no new test flow, public API or storage layout is added.
The old graph-bound receipts above remain scoped to their recorded inputs.

Release/memory/watchdog evidence is in
`target/qualification/release-memory-watchdog-cleanup/`. Release preflight no
longer probes cargo-sort help after common-tool admission; actual release and
publication substitutes pass Bash 5 and genuine Bash 3.2, with ShellCheck passing.

The prospective memory owner is Testkit PR #97, not a second Memory API; Memory
#101 records that accepted decision. Exact Testkit head
`741e2e8e89cd1442e29dd3a8e1ff235260806bf0` passes all four memory IO contracts and
Rust 1.88 Wasm compilation. The Timers completed-batch proposal is refreshed
against delivered 0.25.0 `11e73bff33676e6d829baa502fb943ffa312b988`; its independent
work/deadline regression, Rust 1.91 Wasm and strict Clippy pass. Timers' explicit
maintainer-owned contribution-commit rule keeps that change as a tested patch
posted on its existing issue, without a contribution commit or PR.

Combined isolated Jobs adoption passes 19 composition tests, 23 consumer tests,
Rust 1.91 Wasm test compilation and strict consumer/composition Clippy. Counts
retain shared reset/backing and reject overflow; frozen image checks stay local.
The production consumer Wasm tree excludes Testkit/PocketIC. Qualification-only
manifests/lock and apply-checked source patches are under `adoption/`; the only
lock changes are the local Timers edge and optional native Testkit package.
These receipts qualify unpublished source, not active registry adoption. The
existing local counter/projection remains until reviewed delivery/publication;
[Jobs #56](https://github.com/dragginzgame/ic-jobs/issues/56) and
[Jobs #57](https://github.com/dragginzgame/ic-jobs/issues/57) own that acceptance.
No active path override, own version, primary index/ref, sibling working-tree, live
recovery, release or publication mutation ran. Incoming Host 0.21 selections
were preserved. The direct crates.io availability probe returned HTTP 403;
it establishes no package availability or absence.

Composition/restore/release cleanup evidence is in
`target/qualification/composition-restore-release-cleanup/`. The unchanged
19-scenario native composition harness and ordinary support modules now belong
to `apps/job-consumer/tests/`; core no longer has Memory/Timers dev-dependencies.
`test-consumers` and the separate composition MSRV check select that package;
`check-consumer` selects its library tests to avoid replaying composition.
The lock changes only the core local dependency row; package identities and all
external selections match this batch's incoming lock. Memory 0.43 / Build 0.8
catalog updates were incoming and are preserved.

Restore reuses its occurrence offset only after admitting origin bounds; checked
multiplication/addition and InvalidRecord refusals remain. All 45 core tests,
five doctests, 19 composition and 23 native consumer tests pass. Separate core/
composition Rust 1.91 checks, core/consumer Wasm and strict all-target Clippy pass.
Jobs retains eleven release/format/publication mode/control cases with unchanged
source/index/fetch/gate/state assertions. The exact pinned Shared producer's
exhaustive release and formatting matrices pass separately. Both actual Jobs
release fixtures pass Bash 5 and genuine Bash 3.2; ShellCheck, formatting,
snapshot integrity, pins, release adapters and documentation checks pass.
Earlier receipts and prospective caller patches retain their original graph and
paths; the watchdog caller patch is refreshed for the composition relocation.

Fixture/workflow/derive cleanup evidence is in
`target/qualification/fixture-workflow-derive-cleanup/`. The actual CI gate has a
stable `validate` ID; its run body and both collector bodies use the canonical
extractor. Jobs retains workflow ordering, temp-root, archive and failure-status
assertions. Active evidence fixtures pass Bash 5 and genuine Bash 3.2. All 23
consumer tests, Rust 1.91 consumer Wasm, recovery-harness compilation and strict
consumer/harness Clippy pass after removing five unused Clone derives. Candid
fields and stored codecs are unchanged. Release-retention fixtures, source-following
ShellCheck, formatting, snapshot, pins, release adapters and doc links pass.

Make-fixture bootstrap remains prospective: Shared PR #227 at
`aa401e37e1d87abd4329fd4c4986b5d88e7c6a4e` is still open. Both affected Jobs
fixtures pass both shells in an isolated copy using that helper and the current
common Make source; producer preparation/refusal tests also pass both shells.
The apply-checked caller patch and exact helper are retained under `adoption/`.
This does not qualify or adopt the complete proposed Shared revision. Active
copy lists await canonical export from reviewed source and actual requalification,
tracked in [Jobs #58](https://github.com/dragginzgame/ic-jobs/issues/58).

Release-owner convergence evidence is in
`target/qualification/release-owner-convergence/`. One installer substitute now
covers common leaf routing; Jobs retains late parallel admission and setup/check
refusals before gate effects. Adapter mismatch and missing-version preparation
checks reuse the existing patch-release repository. Shared PR #231's exhaustive
leaf fixture and PR #232's selected-source metadata fixture pass both shells;
the relevant production helpers match the active pin. Generic cache tests remain
active while the manual preparation block remains.

[Shared PR #233](https://github.com/dragginzgame/shared-tooling/pull/233),
`c00f3602bcdc193230acd2c54bd5a39680e23d08`, fixes shared preflight's lost
RELEASE_MAKE selection. Both producer shells pass selected paths with spaces and
failure propagation. Its prospective Jobs adapter and narrower cache fixture
are qualified through a canonical 112-payload export in isolation; the active
110-payload snapshot and preparation adapter remain unchanged pending reviewed
export, tracked in [Jobs #59](https://github.com/dragginzgame/ic-jobs/issues/59).
Earlier export/setup refusals are retained separately from corrected
qualification. No root index/ref, package version, sibling checkout, release or
publication mutation was performed; the scoped Shared contribution used its
isolated PR workflow.

Full local CI and live PocketIC were not selected for this batch. Linux Bash 3.2
execution does not establish native macOS acceptance. The last recorded live
recovery evidence is Jobs 0.8.1, source
`f9c0c0eb4e8363565be63f9434a0cf8b9940294b`,
[run 38289182820](https://github.com/dragginzgame/ic-jobs/actions/runs/38289182820):
84 passes across seven scenarios, four Wasm variants and three hosts. Current IC
recovery, upgrades and timer delivery still require explicitly selected live proof.
Retained pre-cut Memory installations still require explicit owner disposition;
no reset, migration or deployment ran.

## Upstream follow-up

GitHub owns live tracker status. Shared PR #213's dependency-order helper is
already in the reviewed 0.15.2 revision. This batch exports and uses it; Jobs #53
retains delivered/native acceptance. Shared PR #220's optional-maintenance change
postdates that pin; Jobs #54 retains its separate adoption requirement.
[Shared PR #223](https://github.com/dragginzgame/shared-tooling/pull/223) corrects
its Rust action guide; actual helper export already works in the adopted source.

[Memory PR #116](https://github.com/dragginzgame/ic-memory/pull/116) and
[Host PR #133](https://github.com/dragginzgame/ic-host-tooling/pull/133) remain open;
Jobs #50/#52 await reviewed publication and requalification on the current graph.
[Timers #64](https://github.com/dragginzgame/ic-timers/issues/64) retains the
completed-batch watchdog proposal; current callers await upstream delivery.
[Jobs draft PR #51](https://github.com/dragginzgame/ic-jobs/pull/51) now prepares
Testkit **0.47** rejection assertions. The selected registry graph now includes
0.47.0; the working tree matches its `CanisterReject` variant and requires exactly
`CanisterCalledTrap`, retaining the full error in diagnostics. Rust 1.91 recovery
harness compilation, focused strict Clippy and formatting pass; receipts are in
`target/qualification/testkit047-rejection/`. No live recovery ran. The draft is
superseded by the assertion repair delivered in 0.13.1; GitHub owns its disposition.

Earlier source-bound receipts and original tracker descriptions are preserved in
[history through 0.13.0](history-through-0.13.0.md). See also the
[package contract](../../crates/ic-jobs/README.md), [safety contract](../../SAFETY.md),
[consumer qualification](../../apps/job-consumer/README.md) and
[delivery procedure](../releasing.md).
