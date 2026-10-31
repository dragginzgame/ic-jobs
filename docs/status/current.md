# Current handoff

## Accepted scope and delivery

The maintainer authorizes adopting the new Platform, Wire, Tooling, Memory,
Storage and Shared Tooling, pushing necessary source changes, and releasing both
Scheduling groups to GitHub and crates.io. Delivery remains subject to the full
configured gates; no guard, receipt or installed recovery check is waived.
Source is prepared on `main`; no real commit/tag/push/upload has run in this batch.
Both groups still have package version 0.33.1. Proposed delivery order is Timers
0.34.0, then Jobs 0.33.2, with each upload at its own pushed annotated tag.

Issue [#82 — Version Jobs and Timers independently](https://github.com/dragginzgame/ic-scheduling/issues/82)
approves two groups in the existing nine-member workspace. Policy and complete
member map are `ci/cargo-release-groups.json` and root Cargo.toml. Every member
has an explicit version matching its group. There is no root workspace version.
Canonical ledgers are `releases/jobs/CHANGELOG.md` and
`releases/timers/CHANGELOG.md`; the root changelog is an index. Coordinated
history through 0.33.1 is preserved once at `releases/history.md`, and prior group
history is retained. The historical coordinated release is `v0.33.1`, `03b166f`.

Release/resume/publication requires explicit `RELEASE_GROUP=jobs|timers`.
Missing/empty/unknown selections and cross-group PACKAGE overrides refuse before
effects. Tags are `jobs-vX.Y.Z` and `timers-vX.Y.Z`. Direct delivery uses only
`origin`/`main`; publication is a separate command. Shared owns exact file plans,
transactions, retained journals and source/prepared receipts. Old attempts must
use their recorded source/policy/layout; adoption never reinterprets journals.

## Current suppliers and evidence

The root catalog/lock now selects Platform 0.15.2 and canonical Types 0.15.1,
Wire 0.13.0, Endpoint 0.2.0, Testkit/Native 0.75.0, Build/Host 0.73.1,
Memory/Storage 0.59.0 and Logging/Metrics 0.23.2. Testkit's typed calls now share
Wire 0.13 Limits/Error with the consumer. Endpoint imports replace the removed
Wire endpoint feature. Memory retains its own Wire 0.12 codec identity; no
consumer error/type bridge or duplicate transport was added. Metrics defaults
remain disabled; its optional native schemas still belong to Wire 0.12.

The runtime graph has one registry Platform facade, macro generator and exact
runtime guard at 0.15.2, shared by Timers, Memory and the qualification consumer.
The Timers guard and its source-override rejection fixtures select the qualified
0.15 line, preserving executor/IC0-only library features, private FFI confinement
and host/protocol exclusions. Jobs remains Serde-only and independent of Timers.

Shared is canonically exported from clean committed revision
`39a0a7612dab88d97d8e8f612fbfcf0b04fe83e6` (131 payloads). The supplier commit
subject is 0.27.20 while its VERSION still records 0.27.19; snapshot metadata
faithfully records that VERSION. Its delegated-delivery and explicit-cleanup
rules are adopted without hand edits to owned payloads. No cleanup was run.

Current evidence is `/tmp/ic-scheduling-adopt-all.AQGpWV/`, including the complete
incoming dirty source archive, supplier index observations, exporter impact and
refresh logs, metadata, Clippy and consumer/policy checks. The full native/Wasm
Clippy gate passes. Rust 1.91 consumer library, API-wire, composition and stored
Wire tests pass. Both group plans, pins, direct boundaries, all graph contexts,
timer provider/source identity checks and their rejection fixtures pass. Tools
are prepared through the canonical host/IC/Build/Testkit setup, with old tool
selections retained. Full release validation and installed recovery remain next.

Earlier snapshot/group evidence is `/tmp/ic-scheduling-shared018/` and
`/tmp/ic-scheduling-82/`: both real adapter fixtures cover preparation rollback,
exact changed commit files, unrelated group/version/notes preservation, tampered
receipts, group tags, wrong-group recovery, same-group resume and publication.
Full product gates and registry effects were substituted inside those fixtures;
they never established a real release. Earlier graph/runtime evidence remains
at `/tmp/ic-scheduling-stack-latest/` and the prior cohort/persistence/tooling
adoption directories. None qualifies live recovery on this new graph.
Failed supplier retries remain at `/tmp/ic-scheduling-stack-retry.ixU7gK/`,
`/tmp/ic-scheduling-release-retry.dLPpLh/`, `/tmp/ic-scheduling-retry-20261031/`
and `/tmp/ic-scheduling-deploy-20261031/`. Do not relabel failures as acceptance.

## Retained recovery and provenance constraints

The complete release gate remains combined CI, explicit Testkit/server setup
and installed Timers watchdog/cohort qualification. Core/MSRV/Wasm and composed
consumer checks remain distinct. Native tests do not prove IC recovery; no new
macOS execution is claimed by this batch.

Consumers own metadata, storage, lifecycle reconstruction and handler selection.
Every restore uses `Job::restore`. Running/Uncertain effects remain blocked until
exact receipt reconciliation or explicit owner disposition. Queue bytes remain
bounded Wire `JQW\x01`, with no JSON fallback, reset or converter. Preserve old
binaries/backups, original attempts and receipts before retiring installations.
Memory's allocation ledger is separately owned: never clear only queue or ledger.
Logging remains heap-only, bounded to 32 KiB and recorded after committed work;
Metrics reset neither clears logs nor reuses IDs. Keep explicit Platform endpoint
attributes, local size guards and their refusal/rollback behavior.

Jobs provenance is `50fac19a55624c2110778e21ba5bb15ef8bf469a` (0.17.51).
Timers provenance is the dirty imported sibling at
`651efce370bd6aec6290b74118eb709488062d0a` (0.32.4), never a build input.
Migration custody remains `/tmp/ic-scheduling-merge.7PRSi3Nq/`.
The private Canic contribution remains `/tmp/canic-jobs-adapter.dPFVZ7FD/source`,
branch `codex/jobs-application-adapter`, commit
`fc14bbe1baa3c0fdc09b0550e25a1d3fdcd9d2d1`. It needs separate integration and
qualification; Jobs tests establish neither Canic readiness nor a shipped mount API.
