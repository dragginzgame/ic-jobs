# Current handoff

## Source and pending work

Delivered Jobs **0.13.0**: `123b8f0c9a5abcabd9498503d947d45908aa49df`.
Pending compatible **0.13.1** contains restore/deadline simplification, narrower
request derives, shared test restoration, moved dispatch payloads, a one-shot
fixture write control and Shared Tooling **0.15.2** adoption at
`ca98798920188da9a21041ff9fabb76ec18f41ac` (108 payloads).
Release preparation now uses Shared's metadata renderer directly. Jobs no longer
requires cargo-edit, a separate release-tool catalog or local setup/check targets.
Common tool admission and metadata refusal/rollback boundaries remain in place.

The graph selects Memory 0.42.0, Testkit 0.46.0, Timers 0.24.0, Build 0.6.0 and
direct Host FS/Process 0.20.1. Rust floor is 1.91.0; development is 1.99.0.
Incoming manifest/lock edits are preserved. Own versions, index, refs, release and
publication remain maintainer-owned.

`start_next` returns `(index, attempt)`. Commit `jobs[index].record()` with its
immutable payload/index before external dispatch. All restore paths retain
`Job::restore`; ordering, transition errors and uncertain-effect disposition are
unchanged. Consumers own storage and depend on Timers directly. Jobs has no
runtime Timers dependency or lifecycle exports. The capture tool owns Jobs'
optimizer recipe and uses IC Build's retained path; Jobs selects the live gate.

## Qualification

[Delivered 0.13.0 CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38753013217)
passes Linux, macOS Apple Silicon and macOS Intel at the exact delivered source
above. This supplies delivered/native acceptance for shared fixture allocation
and completion admission (#47/#48); it does not qualify pending 0.13.1 edits.

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

GitHub owns live tracker status. Shared PRs #213 (dependency-order fixture) and
#220 (optional maintenance payloads) are merged; Jobs #53/#54 await adoption of a
reviewed snapshot containing those commits. The current 0.15.2 pin predates both.
[Shared PR #223](https://github.com/dragginzgame/shared-tooling/pull/223) corrects
its Rust action guide; actual helper export already works in the adopted source.

[Memory PR #116](https://github.com/dragginzgame/ic-memory/pull/116) and
[Host PR #133](https://github.com/dragginzgame/ic-host-tooling/pull/133) remain open;
Jobs #50/#52 await reviewed publication and requalification on the current graph.
[Timers #64](https://github.com/dragginzgame/ic-timers/issues/64) retains the
completed-batch watchdog proposal; current callers await upstream delivery.
[Jobs draft PR #51](https://github.com/dragginzgame/ic-jobs/pull/51) now prepares
Testkit **0.47** rejection assertions. Its earlier retry/Candid changes are already
delivered; this remaining draft must wait for the new published contract.

Earlier source-bound receipts and original tracker descriptions are preserved in
[history through 0.13.0](history-through-0.13.0.md). See also the
[package contract](../../crates/ic-jobs/README.md), [safety contract](../../SAFETY.md),
[consumer qualification](../../apps/job-consumer/README.md) and
[delivery procedure](../releasing.md).
