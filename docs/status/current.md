# Current handoff

## Selected source and dependencies

The released baseline is `df80990` (Jobs 0.17.23). The incoming lockfile already
selected Memory/Storage 0.50.16, Platform/macros 0.6.14, Timers 0.30.11 and Wire
0.6.13. This task preserves those updates and adopts published Wire/derive 0.7.0.
The catalog now requires the reviewed Memory/Storage, Platform and Timers patch
levels. Metrics stays transitive at 0.15.8; Build/Host/Testkit stay at 0.67.0.
Only Wire/derive change from the task's incoming resolved package set.

Shared Tooling is 0.26.13 at `2f4a95f88743982716f12def87f0a30c197dd07b`, exported
from its clean committed source through the canonical preview/refresh workflow.
The snapshot verifies 117 payloads. Common executable pins, Rust 1.99.1 development
and Rust 1.91 core/consumer floors are unchanged. Jobs still selects only
`wasm-opt`; the release editor and Testkit CLI are prepared under current policy.
Jobs now enables the common external IC dependency inventory, retaining local
bans on direct Platform macros and the retired Testkit Candid adapter.

The compatible batch is recorded in the 0.17.24 draft. Package versions, staging,
commits, release delivery and publication remain user-owned.

## Wire and ownership contracts

Wire 0.7 removes decode-time reserialization and `Error::Noncanonical`.
Decode-only helpers require `DeserializeOwned`; encode-only helpers require
`Serialize`. Nonminimal integers still refuse as `Malformed`. Existing directional
frame/traversal budgets, emitted layouts and method identities are unchanged.
The new Deserialize-only reply test and existing malformed/limit/identity tests
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

Inputs, prior handoff, snapshot impact/refresh records and focused logs are under
`target/qualification/upstream-wire-070/`. They bind the starting source above,
the incoming lockfile and the current patch/dependency selection.

Native consumer/Wire, artifact capture, parser, composition and memory-observer
tests pass; ignored live recovery/demo/process tests compile. Separate core and
consumer Rust 1.91/Wasm checks, native/Wasm strict Clippy, all five graph contexts,
boundaries, pins, snapshot integrity, formatting/hooks and release routing pass.
A real Cargo metadata probe also confirms common-inventory rejection of a renamed,
optional, target-specific IC dependency without relaxing local bans.
A separate Rust 1.91 probe compares 330 actual Init/delivery/reply values across
Wire 0.6.13 and 0.7.0, including all Principal lengths and 256-byte ASCII/UTF-8
payloads. Frames match and cross-decode in both directions. This is native codec
evidence, not installed-canister recovery or a general canonicality guarantee.
Release/publication, consumer-tooling and formatting-evidence fixtures pass.
The release fixture now verifies 0.26.13's manually committed candidate recovery:
failed fresh validation makes no delivery, and successful retry retains the old
unconfirmed plan. The initial stale-expectation failure and logs are preserved.

Full CI, live PocketIC execution and fresh macOS Intel/Apple Silicon qualification
were not run. Keep explicit `make test-demo`, `make test-demo-cli` and
`make test-canister[-optimized]` selection separate from native compilation.

Prior Tooling 0.67 evidence remains at `target/qualification/tooling-067/`.
The last recorded three-host live recovery run qualified Jobs 0.17.14 at
`7d1f808de0bc14d823bd1d3f04a5a62f06ad7a52` with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
Its source-bound artifacts remain in `target/qualification/recovery-01714/`;
they do not qualify the current graph. Preserve earlier qualification directories,
failed attempts, release plans and unresolved effects.
