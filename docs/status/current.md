# Current handoff

## Accepted scope

Released baseline: `36d6a78d026e249beeca4458c1e7e07dee45b2cd` (Jobs 0.17.36).
Adopt published upstream dependency patches and their selected contributor CLIs.
The compatible changelog draft remains 0.17.37; workspace package versions remain
0.17.36. Changes are uncommitted, including the preceding adoption. Staging,
commits, releases and publication remain maintainer-owned. No blocker remains.

Root minimums now select Build/Host/Testkit 0.69.3, Memory/Storage 0.51.13 and
Platform 0.7.1. These versions, including Platform macros 0.7.1, were already
selected in the incoming dirty Cargo.lock; its bytes remain unchanged. Timers
0.31.11, Wire/derive 0.7.18 and Metrics 0.16.3 remain selected. Incoming unrelated
selections, including unicode-width 0.2.3, were preserved.

Published producer changes were reviewed at Tooling `e3a71a92`, persistence
`3acffe1` and Platform `b301438`. Tooling preserves public APIs, cache identities
and process/recovery custody while simplifying internal orchestration.
Persistence's non-initializing cell load is additive; Jobs retains its existing
bounded fixture-cell restoration. Platform's Principal and runtime changes
require no Jobs caller migration. Siblings stayed read-only.

Shared Tooling remains the reviewed 0.26.25 snapshot at
`6b6bbd9c9e5798f0ecf6105637392a7a55cd450e`, with all 121 payloads intact.
This batch updates registry dependencies, not the Shared Tooling snapshot.

## Maintained contracts

No application Rust source, features, CI workflows or public APIs changed.
Scheduling, retries, pause/resume, rescheduling and reconciliation are preserved.
The core remains Serde-only. Storage, metadata, lifecycle and timer authority stay
consumer-owned. Every restore still uses Job::restore; uncertain effects remain
blocked until explicit disposition, with error ordering and exact receipts intact.
No retained-data migration or client regeneration is required. Rust floors remain
1.91 for the core and consumer integration, qualified separately.

The Wasm consumer retains one Platform identity, shared with Timers and registry
executor 2.0.0 / ic0 1.2.0. All five dependency graph selections pass.
Contributor setup remains `make install-testkit-tools install-release-tools`,
followed by the existing offline check targets. Tooling 0.69.3's Testkit CLI and
release metadata editor are prepared and admitted. Earlier installations and
PocketIC 16.1.0 server bytes remain intact.

## Qualification and evidence

`/tmp/ic-jobs-upstream-20261028.RndANsBT/` retains the released base, incoming
patch/manifests/lockfile/handoff, producer review, locked metadata, commands,
focused logs, CLI receipts, source/tool identities, Wasm artifacts and final patch.

Fresh checks pass: snapshot/pins/dependency boundaries and all five graphs;
affected native/Wasm Clippy; formatting/documentation; 50 consumer unit, codec
and composition tests; and capture, recovery-boundary and queue-parser tests.
The consumer release Wasm builds on Rust 1.91. Explicit live tests stay ignored.
Core Rust 1.91 native/Wasm evidence from
`/tmp/ic-jobs-upstream-adoption.0jRpTmuM/` is reused because its source, features
and Serde dependency closure are unchanged; that path also retains the preceding
0.69.1/0.51.11 adoption evidence.

The release metadata fixture passes with the selected real editor and local Git,
using substitute validation/registry/upload effects. Consumer tooling fixtures
pass with the real capture binary and substitute compiler/server/optimizer effects,
covering admission, workflow/demo routing, rejected inputs, frozen artifact custody
and changed-input refusal. These establish native preparation and caller wiring,
not installed IC recovery. Full `make ci`, live PocketIC and native macOS
qualification were not selected for this dependency batch.

Prior Shared formatter/hook evidence remains at
`/tmp/ic-jobs-shared-02625.5rE0846k/`. Linux live evidence at
`/tmp/ic-jobs-tooling-wire-shared.1SRGqz/` uses older inputs. The last recorded
three-host live run qualified Jobs 0.17.14 with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
