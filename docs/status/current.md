# Current handoff

## Selected source and dependencies

The released baseline is `81e0c06` (Jobs 0.17.25). The incoming lockfile already
selected Platform/macros 0.6.15, Timers 0.30.12 and Wire/derive 0.7.2. This task
raises the Platform and Timers catalog minimums to those reviewed releases and
preserves the incoming lockfile byte-for-byte. Memory/Storage remain 0.50.16,
transitive Metrics remains 0.15.8, and Build/Host/Testkit remain 0.68.0.

Reviewed Timers release `798f559` and Platform release `6dcf54a`. Published `src`
trees for Timers, Platform and Platform macros are identical to their previous
selections (0.30.11 and 0.6.14). Timers updates its own unpublished tooling/probes;
Platform aligns optional native Wire metadata with Wire 0.7.2. Jobs uses neither
upstream probe code nor Platform's `wire-schema` metadata. No Rust caller,
endpoint, codec-budget, stored-data or scheduling-policy migration is required.
The incoming Wire 0.7.2 patch changes upstream tooling only, preserving its
runtime dependencies, frame bytes and codec behavior.

Shared Tooling remains 0.26.13 at `2f4a95f88743982716f12def87f0a30c197dd07b`, exported
from its clean committed source through the canonical preview/refresh workflow.
The snapshot verifies 117 payloads. Common executable pins, Rust 1.99.1 development
and Rust 1.91 core/consumer floors are unchanged. Jobs still selects only
`wasm-opt`; the release editor and Testkit CLI are prepared under current policy.
Jobs enables the common external IC dependency inventory, retaining local
bans on direct Platform macros and the retired Testkit Candid adapter.

The compatible batch is recorded in the 0.17.26 draft. Package versions, staging,
commits, release delivery and publication remain user-owned.

## Wire and ownership contracts

Wire 0.7 removes decode-time reserialization and `Error::Noncanonical`.
Decode-only helpers require `DeserializeOwned`; encode-only helpers require
`Serialize`. Nonminimal integers still refuse as `Malformed`. Existing directional
frame/traversal budgets, emitted layouts and method identities are unchanged.
The Deserialize-only reply test and existing malformed/limit/identity tests
cover the affected Jobs boundary.

The fixture's receipts compare complete typed delivery envelopes, including
payload and attempt identity. They do not reconstruct raw frames for hashes or
signatures. Future exact-byte retries/signatures must retain original immutable
frames: successful decoding no longer proves identical re-encoding. The
[consumer guide](../../apps/job-consumer/README.md) records this contract. Rebuild
fixture and native harness together; stored JSON needs no conversion or reset.

The published core remains Serde-only. Consumers own storage, payloads,
authentication, lifecycle, scheduling integration and effect disposition. Every
decoded Job crosses `Job::restore`; Running/Uncertain effects remain blocked until
explicit reconciliation. No handler registry, database or direct timer provider
was added. The consumer Wasm graph retains one Platform identity and excludes
Candid/Host; native PocketIC dependencies do not establish IC recovery.

## Qualification and retained evidence

Inputs, prior handoff, published-source comparisons and focused logs are under
`target/qualification/timers-03012-platform-0615/`. They bind source `81e0c06`,
the incoming dependency selections and the current catalog/documentation patch.

Native consumer/Wire, capture, parser, composition and memory-observer tests
pass; ignored live recovery/demo/process tests compile. Separate core/consumer
Rust 1.91/Wasm checks, native/Wasm strict Clippy, all five graph contexts,
boundaries, pins, snapshot, formatting and documentation checks pass. Consumer
tooling fixtures and the existing 0.68 CLI offline admission pass. Wasm feature
inspection confirms the new Platform/Timers selection without host schema or
PocketIC features. The incoming lockfile remains byte-identical.

Full CI, live PocketIC execution and fresh macOS Intel/Apple Silicon qualification
were not run. Keep explicit `make test-demo`, `make test-demo-cli` and
`make test-canister[-optimized]` selection separate from native compilation.

Prior Wire 0.7.1/Tooling 0.68 evidence remains at
`target/qualification/wire-071-tooling-068/`. Wire/Shared adoption evidence remains at
`target/qualification/upstream-wire-070/`, including the native 330-value frame
comparison and recovery-fixture alignment. Tooling 0.67 evidence remains at
`target/qualification/tooling-067/`.
The last recorded three-host live recovery run qualified Jobs 0.17.14 at
`7d1f808de0bc14d823bd1d3f04a5a62f06ad7a52` with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
Its source-bound artifacts remain in `target/qualification/recovery-01714/`;
they do not qualify the current graph. Preserve earlier qualification directories,
failed attempts, release plans and unresolved effects.
