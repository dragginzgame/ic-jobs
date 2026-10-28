# Current handoff

## Accepted scope

Released baseline: `b620e9304175e268551da32b538a2bd2bdc708dd` (Jobs 0.17.37).
Adopt published upstream dependencies, including Memory/Storage 0.52.0 and the
selected contributor CLIs. The compatible changelog draft is 0.17.38; workspace
package versions remain 0.17.37. Changes are uncommitted. Staging, commits,
releases and publication remain maintainer-owned. No implementation blocker remains.

Root minimums now select Memory/Storage 0.52.0, Build/Host/Testkit 0.69.5,
Platform 0.7.2, Timers 0.31.15 and Wire 0.7.19. The incoming dirty lock already
selected all except Memory/Storage; the bounded update changes only those two
entries from 0.51.13 to 0.52.0. Incoming selections are preserved. Platform/macros
and Wire/derive each have one matching identity; Metrics remains 0.16.3.

Producer changes were reviewed at persistence `3acc1a7`, Tooling `c5b6fae8`,
Platform `95b7caf`, Timers `4a77ac1` and Wire `793134a`. Siblings stayed read-only.
Shared Tooling remains the reviewed 0.26.25 snapshot at
`6b6bbd9c9e5798f0ecf6105637392a7a55cd450e`, with all 121 payloads intact.
This batch updates registry dependencies, not the Shared Tooling snapshot.

## Memory adoption and maintained contracts

Queue opening now requires MemoryBacking. The canister's standard backing is
supported upstream. The native metering newtype delegates physical identity to
VectorMemory while reusing Tooling's IO counters. Distinct wrappers over the same
backing refuse overlapping runtime construction. A new regression verifies refusal
before backing IO while a queue or detached handle remains alive, followed by
successful restoration after its final lease drops.

Retained-cell fixtures release earlier owners/handles before reopening; their
existing precise corruption errors and header/payload preservation assertions
remain intact. Memory 0.52 validates 34,848 manager metadata bytes on valid stable
IO and rejects stale manager authority without repair. This adds IO; consumer IC
instruction costs remain unmeasured. MGR, ledger, cell and Jobs formats are unchanged.

The published Jobs API and Serde-only core are unchanged. Consumer storage,
metadata, lifecycle and timer authority remain consumer-owned. Every restore
still crosses Job::restore; uncertain effects stay blocked until explicit
disposition, with error ordering and exact receipts intact. Scheduling, retries,
pause/resume, rescheduling and reconciliation remain supported. No reset,
retained-data migration or client regeneration is needed. Optional new Platform,
Wire and Timers APIs were not enabled without a consumer requirement.

Contributor setup remains `make install-testkit-tools install-release-tools`,
followed by existing offline check targets. Tooling 0.69.5's Testkit CLI and release
metadata editor are prepared and admitted. Earlier installations and PocketIC
16.1.0 server bytes remain intact. Rust support remains 1.91.

## Qualification and evidence

`/tmp/ic-jobs-upstream-memory052.G3JslrmS/` retains the released base, incoming
patch/manifests/lock/handoff, producer review, metadata, bounded lock delta,
commands/logs, CLI receipts, source/tool identities, Wasm artifacts and final patch.

Fresh checks pass: snapshot/pins/dependency boundaries and all five graphs;
affected native/Wasm Clippy; formatting/documentation; 51 consumer unit, codec
and composition tests; and capture, recovery-boundary and queue-parser tests.
The consumer release Wasm builds on Rust 1.91. Live tests stay explicitly ignored.
The first Clippy run rejected an unnecessary mut in the new regression; its log
is retained, the binding is corrected and affected checks pass.

Core Rust 1.91 native/Wasm evidence from
`/tmp/ic-jobs-upstream-adoption.0jRpTmuM/` remains applicable: core source, features
and Serde dependency closure are unchanged. Prior dependency qualification is at
`/tmp/ic-jobs-upstream-20261028.RndANsBT/` and uses an older consumer graph.

Release metadata fixtures pass with the actual selected editor and local Git,
using substitute validation/registry/upload effects. Consumer tooling fixtures
pass with the real capture binary and substitute compiler/server/optimizer effects,
covering admission, workflow/demo routing, invalid inputs, frozen artifact custody
and changed-input refusal. They establish native preparation and caller wiring,
not installed IC recovery. Full `make ci`, live PocketIC and native macOS
qualification were not selected for this batch.

Prior Shared formatter/hook evidence remains at
`/tmp/ic-jobs-shared-02625.5rE0846k/`. Linux live evidence at
`/tmp/ic-jobs-tooling-wire-shared.1SRGqz/` uses older inputs. The last recorded
three-host live run qualified Jobs 0.17.14 with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
