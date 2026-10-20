# Current handoff

## Accepted scope and source

Delivered Jobs **0.17.7** is `87a2a07`. Pending **0.17.8** is unstaged and includes
the maintainer's dependency updates plus the accepted Wire/Principal and tooling
adoptions. The workspace release version remains 0.17.7; commits, releases and
crates.io publication remain user-owned.

The published library retains its Serde-only API and Rust **1.91** floor. Core
owns checked records, transitions and finite-batch selection. Consumers own
storage, payloads, endpoints, lifecycle, authentication and effect disposition.
Every decoded record crosses `Job::restore`; Running/Uncertain work stays blocked
until explicit reconciliation. Error precedence and commit-before-dispatch remain.
The [root ownership map](../../README.md#scope-and-validation) is authoritative.

## Current adoption

Platform/macros **0.4.0** supplies the owned Principal and Wasm runtime. Jobs
already uses raw calls with Wire frames, so retired Candid convenience methods
and combined runtime/codec errors need no caller replacement. PocketIC retains
the external Principal; the harness converts explicitly at application DTOs.
Wire frames and stored JSON retain their existing bytes and method identities.
The Principal compatibility test covers all byte lengths, including leading 0x02.

The graph caps Platform at two identities: application 0.4 and released Timers
**0.29.4** / Testkit **0.57.0** on 0.2. Wasm still has one Platform macro, Timers,
registry executor **2.0.0** and ic0 **1.2.0**, with Candid/full CDK forbidden.
Native DTO builds exclude runtime/storage integrations. Core excludes all IC and
qualification tooling; direct dependency and resolved graph policies stay local.

Direct Host FS/Process **0.31.1** disables defaults; test-support stays dev-only.
Their Rust sources match 0.30.1. Build **0.22.1** and Testkit retain Host **0.30.1**;
keep those released transitive selections without a local bridge or override.

Shared Tooling **0.25.4** is a reviewed committed-source adoption, revision
`db555ab6cf4a93af9a183f18d921d3b0ee97c725`, not a claim of a tagged Shared release.
The canonical exporter refreshed all 114 owned payloads; tool pin catalogs are
unchanged. The new build policy requires explicit tool preparation once:
`make install-tools install-release-tools install-testkit-tools`, followed by
`make tools-check release-tools-check testkit-tools-check`. These now pass.
Consumer fixtures check selected versions and absence of effects without freezing
the previous admission-error sentence. No Host CLI feature is needed here.

## Validation and evidence

Linux Rust 1.91 passes **54 focused native tests**, separate core/consumer MSRV
and Wasm checks, release Wasm compilation, strict native/Wasm Clippy, all four
graph contexts, direct boundaries and pins on the current dependency selection.
Release/publication, consumer-tooling, formatting-evidence and hook fixtures pass,
with substituted external gate/registry effects and no release or upload here.
All nine live recovery scenarios pass on original, `-O3`, `-Os` and `-Oz` Wasm:
**36 successful Linux runs** on the current source and dependency selection.

Current evidence is in `target/qualification/shared-0254-platform-040-host-0311/`:
source/registry comparisons, snapshot preview/export, tool setup/admission and
focused logs. `live/jobs-recovery.3908tO/` owns the four tested artifacts/hashes,
selected Testkit receipt, lock/snapshot, optimizer identity and server/test logs.
`before-handoff.md` preserves the preceding uncommitted handoff;
its exact source and incoming patch are retained beside it. Older build outputs,
failed fixtures and tool generations remain intact.

The lock advanced to Build 0.22.1 and rustls-webpki 0.103.16 during adoption.
Affected native/tooling checks and all 36 live runs were repeated; the
`lock-refresh-*` logs qualify that selection and earlier receipts stay retained.

The preceding Platform 0.3 receipt in `target/qualification/platform-030/` passed
54 native tests and 36 live runs; `live/jobs-recovery.MebqcD/` owns its artifacts.
It does not qualify Platform 0.4 automatically. The released base separately
passes the three-host matrix in [CI run 39303608525](https://github.com/dragginzgame/ic-jobs/actions/runs/39303608525).
Full CI and native macOS execution remain separate from current Linux checks.

## Blocker and next actions

[#56 — Adopt canonical Testkit memory IO observer after publication](https://github.com/dragginzgame/ic-jobs/issues/56)
requires a published observer from [Testkit PR #97](https://github.com/dragginzgame/ic-testkit/pull/97).
Keep test-only `CountedMemory` and its no-write/backing assertions until that gate
is met; do not add an unpublished source override or another measurement owner.

Retain the explicit live gate and qualify actual artifacts after runtime changes.
The fixture/harness must be rebuilt together; old Candid application requests have
no compatibility reader. Preserve stored data and unresolved effects.
[Release preparation](../releasing.md) uses direct `origin`/`main` delivery;
publication is a separate authorization. Earlier committed narrative is available
at [the released handoff](https://github.com/dragginzgame/ic-jobs/blob/87a2a077bee436fdc6aca271c85072793cdf2ade/docs/status/current.md)
and [the older archive](history-through-0.17.1.md).
