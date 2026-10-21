# Current handoff

## Accepted scope and source

Delivered Jobs **0.17.10** is `7e74882`; the workspace version remains 0.17.10.
Pending **0.17.11** adopts Platform 0.6 in the unpublished qualification consumer
and recovery harness. No Rust source, public Jobs API, endpoint, Wire frame or
stored JSON format changes are required. Commits, releases and publication remain
user-owned. The completed release-safety repairs remain part of delivered history.

The published library retains its Serde-only API and Rust **1.91** floor. Core
owns checked records, transitions and finite-batch selection. Consumers own
storage, payloads, endpoints, lifecycle, authentication and effect disposition.
Every decoded record crosses `Job::restore`; Running/Uncertain work stays blocked
until explicit reconciliation. Error precedence and commit-before-dispatch remain.
The [root ownership map](../../README.md#scope-and-validation) is authoritative.

## Platform adoption and compatibility

The catalog selects Platform **0.6** with defaults disabled. Both the native DTO
owner and recovery harness explicitly select its **principal** feature; Wasm
keeps its existing **cdk** feature, which includes Principal and the runtime.
This avoids relying on another package's feature unification for native types.
The registry Platform/macros **0.6.0** archives identify source `73dda4a5`.
Principal and Account source files and the entrypoint generator match 0.5.1;
changes gate type modules and remove unused signature reference code from normal
builds. Jobs selects no signature feature. No data reset or conversion is needed.

Released Timers **0.30.0** and native Testkit **0.62.1** still require Platform
**0.5**, selecting **0.5.1** transitively. Timers uses only its executor and raw
system interfaces; no owned Principal crosses that boundary. The Wasm graph now
admits at most two Platform wrappers while still requiring exactly one executor
**2.0.0**, ic0 **1.2.0**, Timers and Platform macro identity. Native PocketIC calls
retain the external Principal and explicit conversion to Jobs' Platform 0.6 DTOs.
Retire the older Wasm wrapper after a published Timers release adopts 0.6 and is
qualified here. This adoption neither patches nor edits sibling repositories.

Core, qualification Wasm, ordinary native DTO and capture normal/build graphs
remain Candid-free. Candid stays in native recovery tests through Testkit and
PocketIC's required external management and transport protocols. Dev-dependency
placement is unchanged. No fallback codec or direct upstream dependency is added.

Tooling **0.62.1**, Memory/Storage **0.50.1**, Timers **0.30.0** and Wire **0.5.4**
remain selected. The selected Build editor and Testkit CLI pass offline admission;
this graph change needs no tool installation. Storage formats, allocation IDs,
lease custody and reopen behavior are unchanged; optional FS backends stay disabled.
Shared Tooling **0.25.9**, revision
`85aded191d85ac4a8c0e5deab207f6be18895862`, retains its 114 canonical payloads.

## Validation and evidence

Linux Rust 1.91 passes separate core/consumer MSRV and Wasm checks, the release
consumer Wasm build and **54 native tests**: 28 queue/codec, two API Wire, 19
composition, four capture and one native recovery-harness test. Nine live tests
remain explicitly ignored. All three strict Clippy commands pass. Native DTO
admission checks the consumer alone; byte-compatibility tests exercise Principal
Wire frames and stored JSON across supported Principal lengths.

All five dependency graph contexts, direct boundaries, pins, grouped manifest
ordering, snapshot integrity and documentation links pass. Release/publication
fixtures pass with the actual adapters and selected metadata editor, substituting
complete release gates and registry/upload effects. No full CI, native macOS or
live PocketIC recovery run was selected. Native results do not establish IC
callback, trap, upgrade or uncertain-effect recovery behavior.

Evidence: `target/qualification/platform-060/` retains the preceding handoff,
registry release observations, source comparisons, exact inputs, focused logs
and the original graph refusal before admitting the two published wrappers.
The native run is `native/validation.xggeqt/`; release/publication fixtures are
recorded in `tooling/validation.Cmojct/`.
Earlier qualification remains at `target/qualification/upstream-patches-complexity/`
and `target/qualification/release-safety-0259/`, bound to those original inputs.

## Blocker and next actions

[#56 — Adopt canonical Testkit memory IO observer after publication](https://github.com/dragginzgame/ic-jobs/issues/56)
retains the published-observer gate. Keep `CountedMemory` and its no-write/backing
assertions until the supplying observer is published and adopted.

Follow [release preparation](../releasing.md) for direct `origin`/`main` delivery.
Rebuild fixture and harness together before explicitly selecting live IC
qualification. Preserve stored data and unresolved effects. Publication remains
separately authorized. Earlier narrative remains in the
[released handoff](https://github.com/dragginzgame/ic-jobs/blob/7e74882/docs/status/current.md)
and [the older archive](history-through-0.17.1.md).
