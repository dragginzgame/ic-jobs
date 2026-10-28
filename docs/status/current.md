# Current handoff

## Accepted scope

Released baseline: `ed4c97831e99380c09349fbac04b3e4bbb742384` (Jobs 0.17.38).
Adopt Memory/Storage 0.53.0 while preserving the preceding uncommitted Platform
0.7.3 adoption and incoming Tooling 0.69.6 selection. The compatible changelog
draft remains 0.17.39; package versions remain 0.17.38. All changes are uncommitted.
Staging, commits, releases and publication remain maintainer-owned. No blocker remains.

Memory/Storage minimums are 0.53.0; Platform remains 0.7.3. The bounded lock update
changes only Memory/Storage from incoming 0.52.1 to 0.53.0. Other incoming lock
selections are preserved. Build/Host/Testkit minimums remain 0.69.5 with incoming
0.69.6 selected. Timers 0.31.15, Wire/derive 0.7.19 and Metrics 0.16.3 remain selected.
Platform/macros retain one matching 0.7.3 identity. No features changed.

Reviewed producer revisions: persistence `798b435` (0.53.0) and Tooling `cba072ee`
(0.69.6). Siblings stayed read-only. Shared Tooling remains the reviewed 0.26.25
snapshot at `6b6bbd9c9e5798f0ecf6105637392a7a55cd450e`; all 121 payloads verify.
No Shared snapshot adoption or new ic-call dependency was selected.

## Memory contract and maintained behavior

Memory 0.53 trusts exclusive physical backing custody between full manager checks.
Construction, initial bootstrap, successful lease opening and growth validate
manager metadata; growth also checks its projected changes afterward. Ordinary
size/read/write operations enforce bounds and poisoning without metadata scans.
Raw backing changes during a live lease are prohibited and need not be detected
before the next ordinary IO. This narrows 0.52's per-access detection guarantee.
Detected divergence still poisons existing handles; duplicate owners still refuse
before backing IO, with custody retained through every detached handle.

Jobs already uses truthful MemoryBacking identity and releases prior owners and
all detached handles before reopening. Native metering delegates identity to the
underlying VectorMemory; the canister uses the standard backing. No Jobs Rust
source change is needed. Existing alias/refusal, corruption and exact retained
header/payload assertions pass. No unmanaged manager/store path was introduced.
The consumer README now describes checkpoints instead of per-IO metadata reads.
Producer measurements are not Jobs IC performance or installed-recovery evidence.

The published Jobs API and Serde-only core remain unchanged. Consumer storage,
metadata, lifecycle and timer authority remain consumer-owned. Every restore
crosses Job::restore; uncertain effects stay blocked until explicit disposition,
with error ordering and exact receipts intact. Scheduling, retries, pause/resume,
rescheduling and reconciliation remain supported. MGR, ledger, cell, Jobs and
Wire formats are unchanged; no reset, migration or client regeneration is needed.
Rust support remains 1.91 for the core and consumer integration separately.

The incoming Tooling 0.69.6 graph is qualified with the selected Testkit CLI and
release metadata editor prepared through `make install-testkit-tools install-release-tools`.
Both CLIs and PocketIC 16.1.0 pass offline admission. Earlier installations and
server bytes remain intact. No Tooling manifest minimum was changed in this batch.

## Qualification and evidence

`/tmp/ic-jobs-memory053.yxQRme11/` retains incoming patch/manifests/lock/handoff,
released base, producer review, runtime and bounded lock diffs, metadata, commands,
logs, CLI receipts, source/tool identities, Wasm artifacts and final patch.

Fresh checks pass: snapshot/pins/dependency boundaries and all five graphs;
affected native/Wasm Clippy; formatting/documentation; 51 consumer unit, codec
and composition tests; and capture, recovery-boundary and queue-parser tests.
The consumer release Wasm builds on Rust 1.91. Live tests remain explicitly ignored.

Release metadata fixtures pass with the actual selected editor and local Git,
using substitute validation/registry/upload effects. Consumer tooling fixtures
pass with the real capture binary and substitute compiler/server/optimizer effects,
covering admission, workflow/demo routing, invalid inputs, frozen artifact custody
and changed-input refusal. They establish native preparation and caller wiring,
not installed IC recovery. Full `make ci`, live PocketIC and native macOS
qualification were not selected for this batch.

Core Rust 1.91 native/Wasm evidence at
`/tmp/ic-jobs-upstream-adoption.0jRpTmuM/` remains applicable because core source,
features and Serde dependency closure are unchanged. Earlier graph evidence at
`/tmp/ic-jobs-memory0521.cOCNAs3h/` and `/tmp/ic-jobs-upstream-memory052.G3JslrmS/`
is retained with its original dependency/validation contracts.
Prior Shared formatter/hook evidence is at `/tmp/ic-jobs-shared-02625.5rE0846k/`.
Linux live evidence at `/tmp/ic-jobs-tooling-wire-shared.1SRGqz/` uses older inputs.
The last recorded three-host live run qualified Jobs 0.17.14 with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
