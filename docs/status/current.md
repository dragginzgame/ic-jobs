# Current handoff

## Accepted scope and source

Delivered Jobs **0.17.11** is `6c8491e`; the workspace version remains 0.17.11.
Pending **0.17.12** adopts IC Tooling 0.63.0 and its explicit CLI features.
The incoming Memory/Storage 0.50.3 and Wire 0.5.6 lockfile updates are preserved
and included in the qualified graph. Source and notes are prepared; commits,
release execution and crates.io publication remain user-owned.

The published library retains its Serde-only API and Rust **1.91** floor. Core
owns checked records, transitions and finite-batch selection. Consumers own
storage, payloads, endpoints, lifecycle, authentication and effect disposition.
Every decoded record crosses `Job::restore`; Running/Uncertain work stays blocked
until explicit reconciliation. Error precedence and commit-before-dispatch remain.
The [root ownership map](../../README.md#scope-and-validation) is authoritative.

## Tooling adoption and compatibility

Build, Host Artifacts/FS/Process and Testkit select published **0.63.0**, archive
source `c342bb28`. Make's release editor/Testkit selectors and the standalone
live qualification wrapper explicitly request **cli**. The release and consumer
fixtures require that feature on both setup and admission paths. Library
manifests keep CLI features disabled. No Rust source migration was needed.

The selected editor and Testkit executable are prepared through the existing
installers; receipts select `cli`, offline checks pass and actual CLI help runs.
Default library normal/build closures exclude Clap; Testkit's also excludes
Metrics. Jobs' capture binary still owns its required direct Clap dependency.
Host Artifacts, FS and Process source bytes match 0.62.1. Build simplifies digest
hex formatting; capture/retention interfaces remain unchanged. Testkit removes
Candid APIs and benchmark error coupling that Jobs never used. Installation and
diagnostic interfaces used here remain available; live IC behavior is unqualified.

Native Testkit now shares Platform **0.6.0** with Jobs and no longer enables its
Candid adapter. PocketIC still requires Candid for external management/transport
DTOs. Jobs-owned RPCs remain Wire, with explicit external Principal conversions.
Core, qualification Wasm, ordinary native DTO and capture normal/build graphs
remain Candid-free. Dev-dependency placement and optional FS backend selection
are unchanged. Memory/Storage **0.50.3** and Wire **0.5.6** retain their incoming
lock rows; this task did not upgrade their selections.

Timers **0.30.0** still retains Platform **0.5.1** for executor/raw-system APIs.
The qualification Wasm admits those two wrappers while requiring one Timers,
executor **2.0.0**, ic0 **1.2.0** and Platform macro **0.6.0** identity. Retire
its older wrapper after a published Timers release adopts 0.6 and is qualified.
Shared Tooling **0.25.9**, revision
`85aded191d85ac4a8c0e5deab207f6be18895862`, retains its 114 canonical payloads.
Jobs package versions, protocols and stored formats remain unchanged; sibling
checkouts are untouched.

## Validation and evidence

Linux Rust 1.91 passes separate core/consumer MSRV and Wasm checks, the release
consumer Wasm build and **54 native tests**: 28 queue/codec, two API Wire, 19
composition, four capture and one native recovery-harness test. Nine live tests
remain explicitly ignored. All three strict Clippy commands pass. The complete
native capture and recovery harness compile against Tooling 0.63.0.

All five product dependency graph contexts, direct boundaries, pins, grouped
manifest ordering, ShellCheck, Bash syntax, snapshot and documentation links pass.
Release/publication fixtures retain real Git and the selected editor while
substituting full release gates and registry/upload effects. Consumer-tooling
fixtures exercise the actual Make/workflow/wrapper routes and frozen artifact
capture with substitute installer/server/optimizer effects. No full CI, native
macOS or live PocketIC recovery run was selected. Native evidence establishes
neither IC callback/trap/upgrade behavior nor live uncertain-effect recovery.

Evidence: `target/qualification/tooling-063/` retains the prior handoff, incoming
diff, source comparisons, dependency observations, exact inputs and focused logs.
Runs: `native/validation.tx0oIY/`, `release/validation.1Od4Iq/` and
`consumer/validation.5fYBtU/`. A standalone dev-only Testkit Cargo-tree root gave
an empty observation; default library closures were verified instead through
resolved native workspace metadata. Refused fixture logs are deliberate tests.
Earlier evidence at `target/qualification/platform-060/` remains source-bound.

## Blocker and next actions

[#56 — Adopt canonical Testkit memory IO observer after publication](https://github.com/dragginzgame/ic-jobs/issues/56)
retains the published-observer gate. Keep `CountedMemory` and its no-write/backing
assertions until the supplying observer is published and adopted.

Follow [release preparation](../releasing.md) for direct `origin`/`main` delivery.
Rebuild fixture and harness together before explicitly selecting live IC
qualification. Preserve stored data and unresolved effects. Publication remains
separately authorized. Earlier narrative remains in the
[released handoff](https://github.com/dragginzgame/ic-jobs/blob/6c8491e/docs/status/current.md)
and [the older archive](history-through-0.17.1.md).
