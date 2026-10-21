# Current handoff

## Accepted scope and source

Delivered Jobs **0.17.9** is `dc4c8af`; the workspace version remains 0.17.9.
Pending **0.17.10** combines the accepted release-safety repairs with current
upstream qualification dependencies and local codec simplification. Incoming
source/tooling work and the already updated lockfile are preserved. Commits,
releases and crates.io publication remain user-owned.

The published library retains its Serde-only API and Rust **1.91** floor. Core
owns checked records, transitions and finite-batch selection. Consumers own
storage, payloads, endpoints, lifecycle, authentication and effect disposition.
Every decoded record crosses `Job::restore`; Running/Uncertain work stays blocked
until explicit reconciliation. Error precedence and commit-before-dispatch remain.
The [root ownership map](../../README.md#scope-and-validation) is authoritative.

## Adopted packages and simplifications

The catalog now requires Tooling **0.62.1**, Memory/Storage **0.50.1**, Platform
**0.5.1**, Timers **0.29.7** and Wire **0.5.4** as patch minimums. The incoming
lock already selected these published releases; no further resolution was needed.
Registry archives identify Tooling source `66e70087`, Persistence `d61e5c01` and
Platform `fe628b35`. Build, FS, Testkit, Memory and Storage library source bytes
match their preceding reviewed releases. Process changes its crash-barrier test
helper; Platform improves raw argument copying and removes unused private code.
Stored formats, allocation IDs, lease custody and reopen behavior need no reset.
Optional filesystem backends stay disabled. The Build editor and Testkit server
CLI **0.62.1** are prepared through their existing installers and pass admission.

The selected Wasm graph has one Platform/macros **0.5.1**, Timers, executor
**2.0.0** and ic0 **1.2.0**. Its former two-Platform allowance is retired. Core,
qualification Wasm, ordinary native DTOs and capture normal/build graphs exclude
Candid. A fifth graph check protects the capture boundary. Native recovery tests
still need Testkit's Candid/PocketIC features and PocketIC's external management
and transport DTOs; defaults cannot remove those required upstream dependencies.
Jobs-owned protocols remain Wire, with explicit Principal conversions at PocketIC.

Receive and receipt entrypoints share delivery codec hooks, retaining separate
method IDs and their existing limits, response types and authentication. Encoding
failure still traps beside the receipt write. There is no endpoint, stored-byte,
public Jobs API or sibling-consumer migration.

Shared Tooling **0.25.9**, revision
`85aded191d85ac4a8c0e5deab207f6be18895862`, retains 114 canonical payloads.
The accepted recovery fix requires confirmed delivery before incrementing past
committed preparation. Publication keeps HEAD/version/tag unchanged through
admission and Clippy and requires clean source. These pending repairs remain intact.

## Validation and evidence

Linux Rust 1.91 passes separate core/consumer MSRV and Wasm checks, the release
consumer Wasm build and **54 native tests**: 28 queue/codec, two API Wire, 19
composition, four capture and one native recovery-harness test. Nine live tests
remain explicitly ignored. All three strict Clippy commands pass, with the Wasm
command repeated after formatting the changed codec hooks.

All five dependency graph contexts, direct boundaries, pins, grouped manifest
ordering, Rust formatting, snapshot integrity and local documentation links pass.
Release/publication and consumer-tooling fixtures pass with the selected 0.62.1
tools; they substitute complete release gates, registry/upload and PocketIC effects.
No full CI, native macOS or live IC recovery run was selected for this batch.

Evidence: `target/qualification/upstream-patches-complexity/` retains the prior
handoff, incoming diff, upstream source comparisons, input identities and focused
logs. The native run is `native/validation.XwL8Mm/`; the owning tooling run is
`tooling/validation.7HUQWS/`. Its nested failure logs are deliberate refusal
fixtures, not independent failed release attempts. Earlier release-safety evidence
remains at `target/qualification/release-safety-0259/`, bound to its original source.

## Blocker and next actions

[#56 — Adopt canonical Testkit memory IO observer after publication](https://github.com/dragginzgame/ic-jobs/issues/56)
retains the published-observer adoption gate. Current releases still supply no
replacement for Jobs' write-counting Memory adapter. Keep `CountedMemory` and its
no-write/backing assertions until that gate is met.

Follow [release preparation](../releasing.md) for direct `origin`/`main` delivery.
Rebuild fixture and harness together before explicitly selecting live IC
qualification. Preserve stored data and unresolved effects. Publication remains
separately authorized. Earlier narrative remains in the
[released handoff](https://github.com/dragginzgame/ic-jobs/blob/dc4c8af/docs/status/current.md)
and [the older archive](history-through-0.17.1.md).
