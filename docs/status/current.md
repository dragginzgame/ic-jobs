# Current handoff

## Accepted scope and source

Delivered Jobs **0.17.8** is `f002f8b`; the workspace version remains 0.17.8.
Pending **0.17.9** covers the aligned qualification packages, Platform 0.5,
Shared Tooling 0.25.8 and early Clippy ordering. Its changelog now describes the
complete pending selection and contributor setup. The maintainer's manifest/lock
updates and earlier source/tooling work remain intact. Commits, releases and
crates.io publication remain user-owned.

The published library retains its Serde-only API and Rust **1.91** floor. Core
owns checked records, transitions and finite-batch selection. Consumers own
storage, payloads, endpoints, lifecycle, authentication and effect disposition.
Every decoded record crosses `Job::restore`; Running/Uncertain work stays blocked
until explicit reconciliation. Error precedence and commit-before-dispatch remain.
The [root ownership map](../../README.md#scope-and-validation) is authoritative.

## Adopted packages and compatibility review

Build, Host Artifacts/FS/Process and Testkit select published **0.62.0** from
IC Tooling, matching registry archive source `f36f9dc9`. Their crate names remain
unchanged and their common Rust 1.91 minimum matches Jobs' existing floor.
Jobs' capture/retention and process-status interfaces retain their implementations;
Build's changed Cargo observation API is outside those callers. Testkit's selected
native implementation matches 0.59; its Platform/Host dependency identities are
now aligned with the direct selections. Optional FS backends stay disabled.

Memory and Storage both select **0.50.0** from IC Persistence, archive source
`f452b5cf`. Their library sources match released Memory 0.49.1 / Storage 0.5.8.
The matching Memory identity preserves native adapter interoperability; stored
formats, allocation IDs, lease custody and reopen behavior require no conversion
or backing reset. No umbrella dependency or source override is used.

Platform/macros **0.5.0** supplies explicit-codec entrypoints and raw calls.
The accepted source removes two retired no-op `hidden` options. PocketIC retains
its external Principal; conversions remain explicit at application DTOs.
Timers **0.29.6** retains Platform **0.4.1**. The two-version Wasm graph keeps one
macro, Timers, executor **2.0.0** and ic0 **1.2.0**, without Candid/full CDK.
Native DTO builds exclude runtime/storage integrations.
Wire **0.5.3** retains Jobs' native codec behavior and frames. Its finite-encoder
benchmark improvements are not Jobs measurements; the changelog claims no speedup.

Shared Tooling **0.25.8**, revision
`0a2d2ee3b5e0ae77755c2915ac28836d4d5ba359`, retains 114 canonical payloads.
CI runs the existing three-command Clippy gate after tool admission and before
builds/tests/packaging, including under parallel Make. Publication/dry-run use the
same gate; actual upload rechecks source cleanliness. Release preparation retains
failed Cargo metadata evidence and dates numbered `- Unreleased` headings.

## Validation and evidence

The selected Build editor and Testkit server CLI **0.62.0** were prepared through
`make install-release-tools install-testkit-tools`; both offline admissions pass.
Linux Rust 1.91 passes **54 native tests**: 28 queue/codec, two API Wire, 19
composition, four capture and one native recovery-harness test. Nine live tests
remain ignored. Separate core/consumer MSRV and Wasm checks, release Wasm build,
all three strict native/Wasm Clippy commands, all four dependency graph contexts,
direct boundaries and pins pass. Current adapters need no further Rust changes.

Release/publication and consumer-tooling fixtures pass against the selected tools.
They exercise real disposable Git/caller/parser paths while substituting full
release gates, registry/upload and PocketIC effects. They do not qualify live IC
recovery. Full CI and native macOS were not run for this review.

Evidence: `target/qualification/packages-062-persistence-050/` owns package-source
comparisons, setup logs, retained prior handoff, selected inputs and two successful
checkpointed runs: `native/validation.a376c4/` and `tooling/validation.kza8qZ/`.
Both runs bind the same manifests, lock, snapshot, Makefile and compiler selection.
Earlier Platform adoption evidence at `target/qualification/shared-0258-platform-050/`
remains source-bound; it does not qualify the new package identities automatically.

## Blocker and next actions

[#56 — Adopt canonical Testkit memory IO observer after publication](https://github.com/dragginzgame/ic-jobs/issues/56)
requires a published memory observer; the coordinated native Testkit package
currently supplies allocation measurements. Retain `CountedMemory` and its
no-write/backing assertions until the observer publication/adoption gate is met.

For contributor preparation, follow the pending changelog's locked fetch and
selected-tool setup commands. Retain the explicitly selected live gate for IC
recovery qualification and rebuild fixture/harness together. Preserve stored data
and unresolved effects; old Candid requests remain negative fixtures only.
[Release preparation](../releasing.md) uses direct `origin`/`main` delivery;
publication is a separate authorization. Older narrative remains in the
[released handoff](https://github.com/dragginzgame/ic-jobs/blob/f002f8b/docs/status/current.md)
and [the older archive](history-through-0.17.1.md).
