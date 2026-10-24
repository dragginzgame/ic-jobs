# Current handoff

## Selected source and dependencies

The released baseline is `c39359d` (Jobs 0.17.24). The incoming lockfile already
selected Wire/derive 0.7.1. The catalog now requires that patch and Build/Host/
Testkit 0.68; the lock selects all six tooling packages at published 0.68.0.
Only those six tooling packages change from the task's incoming resolved set.
Platform/macros remain 0.6.14, Memory/Storage 0.50.16, Timers 0.30.11 and
transitive Metrics 0.15.8. Their incoming selections are preserved.

Reviewed Wire release `2978dee` and Tooling release `8f281177`. Tooling's later
0.68.1 work is an uncommitted supplier draft and is not adopted. Wire 0.7.1 adds
opt-in host schema advice and packed signed bytes; Jobs selects neither. Testkit
0.68 adopts Wire's existing 0.7 contract. Jobs already uses the one-pass decoder
and does not enable Testkit's optional Wire helpers. No Rust caller changes,
field conversion, regeneration or stored-data migration are needed.

Shared Tooling remains 0.26.13 at `2f4a95f88743982716f12def87f0a30c197dd07b`, exported
from its clean committed source through the canonical preview/refresh workflow.
The snapshot verifies 117 payloads. Common executable pins, Rust 1.99.1 development
and Rust 1.91 core/consumer floors are unchanged. Jobs still selects only
`wasm-opt`; the release editor and Testkit CLI are prepared under current policy.
Jobs enables the common external IC dependency inventory, retaining local
bans on direct Platform macros and the retired Testkit Candid adapter.

The compatible batch is recorded in the 0.17.25 draft. Package versions, staging,
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

Inputs, prior handoff and focused logs are under
`target/qualification/wire-071-tooling-068/`. They bind source `c39359d`, its
incoming Wire 0.7.1 lockfile update and the current patch/dependency selection.

Native consumer/Wire, capture, parser, composition and memory-observer tests
pass; the ignored live recovery/demo/process tests compile. Separate core and
consumer Rust 1.91/Wasm checks, native/Wasm strict Clippy, all five graph contexts,
boundaries, pins, snapshot integrity, formatting and documentation links pass.
Release/publication and consumer-tooling fixtures pass with the prepared 0.68
editor. The 0.68 release editor and Testkit CLI pass offline admission; setup
safely waited for and reused another checkout's completed Testkit installation.

Full CI, live PocketIC execution and fresh macOS Intel/Apple Silicon qualification
were not run. Keep explicit `make test-demo`, `make test-demo-cli` and
`make test-canister[-optimized]` selection separate from native compilation.

Prior Wire/Shared adoption evidence remains at
`target/qualification/upstream-wire-070/`, including the native 330-value frame
comparison and recovery-fixture alignment. Tooling 0.67 evidence remains at
`target/qualification/tooling-067/`.
The last recorded three-host live recovery run qualified Jobs 0.17.14 at
`7d1f808de0bc14d823bd1d3f04a5a62f06ad7a52` with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
Its source-bound artifacts remain in `target/qualification/recovery-01714/`;
they do not qualify the current graph. Preserve earlier qualification directories,
failed attempts, release plans and unresolved effects.
