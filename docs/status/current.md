# Current handoff

## Accepted scope

Released baseline: `b51be73` (Jobs 0.17.40). The accepted uncommitted batch adopts
Tooling/Testkit 0.69.9, Timers 0.32.0 with Metrics 0.17.0, and now Memory/Storage
0.54.0. The compatible draft remains 0.17.41; package versions stay 0.17.40.
Staging, commits, releases and publication remain maintainer-owned. No blocker remains.

The latest bounded update changes only Memory/Storage 0.53.1 to 0.54.0 in the
incoming lockfile. Other pending selections are preserved: Build/Host/Testkit
0.69.9, Platform/macros 0.7.4, Timers 0.32.0, Metrics 0.17.0 and Wire/derive
0.7.21. No explicit feature list or Jobs Rust source changed.

## Memory/Storage adoption

Reviewed released persistence revision `9ca2e8e`. The breaking 0.54 boundary
moves Metrics Stable's exposed arithmetic types to Metrics 0.17. Jobs does not
use ic-metrics-stable or exchange direct Metrics values with that adapter;
Timers already selects Metrics 0.17. No caller migration or direct Metrics
selection is required. Memory and Storage library source trees are unchanged
from 0.53.1. Allocation custody, stable formats and Rust requirements are retained.
The consumer guide now describes MemoryBacking without an obsolete package line.

Truthful backing identity, duplicate-owner refusal and custody through detached
handles remain required. Earlier owners and all handles drop before reopening.
Full manager validation occurs at construction, bootstrap, lease opening and
growth; ordinary IO trusts exclusive custody while checking bounds/poisoning.
Prohibited raw changes need not be detected before ordinary IO. Jobs IC costs
remain unmeasured. No reset or retained-data migration is needed.

## Maintained runtime and integration contracts

Consumer storage, metadata, lifecycle, timer registration and effect authority
remain consumer-owned. Every restore crosses Job::restore; uncertain effects stay
blocked until explicit disposition, preserving exact receipts and error ordering.
Scheduling, retry, pause/resume, rescheduling, reconciliation and stored/Wire
formats remain unchanged. Core and consumer integration retain Rust 1.91 support
with separate qualification. The published Jobs library remains Serde-only.
The packaged adapter example, integration contract and acceptance checklist remain
intact; native substitutes do not establish actual Canic or installed IC behavior.

Prior Timers adoption reviewed `8f2fdfd` and Metrics `c45bb5a`. Timers' runtime
source is unchanged; its exposed measurement types use Metrics 0.17. Required
Serde transport and native Wire metadata do not add Wire/Candid to the Wasm timer
subtree. Recording stays disabled; no measurement windows or Metrics endpoint
are added. Application-owned Wire codecs retain their current contracts.

Prior Tooling adoption reviewed `ce660e30`. Complete cached snapshots restore
matching timer/low-memory-hook state. Jobs creates fresh instances and uses no
cached baseline recipe requiring lifecycle migration. Tooling 0.69.9 CLIs and
PocketIC 16.1.0 remain prepared/admitted; no installation changes are needed here.
Siblings stayed read-only. Shared Tooling remains reviewed 0.26.25 at
`6b6bbd9c9e5798f0ecf6105637392a7a55cd450e` (121 verified payloads).

## Qualification and evidence

`/tmp/ic-jobs-memory054.sqjjQkAo/` retains incoming patch/manifests/lock/handoff,
released base, producer review, empty library-source diff, bounded lock delta,
metadata, focused command/log evidence, Wasm artifacts, identities and final patch.

Fresh checks pass: snapshot/pins/dependency boundaries and all five graph
selections; consumer native/Wasm Clippy; formatting/documentation; and 51 consumer
unit, codec and composition tests on Rust 1.91, including allocation reopening,
blocked attempts, invalid-state refusal and retained-byte preservation. The release
consumer Wasm builds on Rust 1.91. No new tests or runtime APIs were introduced.

Prior harness/MSRV and Wire/Candid-free Timers subtree evidence remains at
`/tmp/ic-jobs-timers032.nwmIAoxY/`; that harness's native DTO/capture dependencies
and the Timers subtree are unchanged by the Memory/Storage update. Existing
Tooling fixture evidence at `/tmp/ic-jobs-tooling0699.ejTpALZM/` remains applicable
to unchanged scripts, capture dependencies and selected CLI bytes. Substitute
server/optimizer/registry/upload effects establish native preparation/caller
wiring, not installed recovery. Full `make ci`, live PocketIC and native macOS
qualification were not selected for this slice.

Core Rust 1.91 native/Wasm evidence at `/tmp/ic-jobs-upstream-adoption.0jRpTmuM/`
remains applicable to unchanged runtime source/features/Serde dependencies.
Packaged adapter evidence is at `/tmp/ic-jobs-adapter-reference.rbwXpr7F/`.
Prior Shared hook/formatter evidence is at `/tmp/ic-jobs-shared-02625.5rE0846k/`.
Linux live evidence at `/tmp/ic-jobs-tooling-wire-shared.1SRGqz/` uses older inputs.
The last recorded three-host live run qualified Jobs 0.17.14 with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
