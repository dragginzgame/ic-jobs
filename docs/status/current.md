# Current handoff

## Current scope

Adopt Memory/Storage 0.60.2, Testkit/Native 0.75.1 and Wire Endpoint 0.2.1, then
push the scoped source changes under the existing instruction. Wire/Derive stays
current at 0.13.1. Memory requires Platform 0.15.3, so the facade, macros and
matching exact guard are part of this update. No package bump, release or upload.
Jobs remains 0.33.2; Timers remains 0.34.0. Pending notes remain Jobs 0.33.3 and
Timers 0.35.0 for the earlier exposed Metrics type-identity change.

The incoming dirty Cargo.lock already contains these selections plus separate
Logging 0.23.3 and Metrics 0.24.1 patches. Preserve those two patches unstaged.
Original bytes and a scoped candidate are retained at
`/tmp/ic-scheduling-memory-testkit-wire.guCIFe/`. Qualification and commit use the
scoped graph with existing Logging 0.23.2 / Metrics 0.24.0, then restore the
original working lock byte-for-byte. Testkit now uses Metrics 0.24, removing the
old Metrics 0.23 lock entry without introducing cross-version conversions.
Qualification restored the original lock; commit preparation must preserve it too.

## Workspace and suppliers

All nine members stay in one workspace with explicit group versions. The root
catalog lists every member once in Jobs/Timers blocks with full versions, paths
and ledger labels. `ci/cargo-release-groups.json` owns complete alias rosters and
primary version sources. There is no root workspace version; catalog entries
add dependency edges only when inherited by a member.

The scoped graph selects Platform 0.15.3 / Types 0.15.1, Wire/Derive 0.13.1,
Endpoint 0.2.1, Testkit/Native 0.75.1, Build/Host 0.73.2, Memory/Storage 0.60.2,
Logging 0.23.2 and one Metrics 0.24.0. Metrics defaults stay disabled; Memory
codec errors, optional Metrics schemas and Testkit typed calls align on Wire 0.13.
Jobs remains Serde-only and independent of Timers.

Timers, Memory and the consumer share one registry Platform facade, macro
generator and exact runtime guard. Preserve the Timers 0.15 source check,
executor/IC0-only library features, private FFI and host/protocol exclusions.
The Build editor and Testkit CLI are lock-selected through managed setup;
prepare a changed CLI with `make install-testkit-tools`. PocketIC remains 16.1.0.

Shared 0.27.22 is canonically exported at
`f4f431627290e18f0ccc1e5f8525d16b197cf7c3` (131 payloads). No owned snapshot
payload changes here. Grouped formatting preserves the root catalogs. No cleanup.

## Qualification and limits

Current evidence is `/tmp/ic-scheduling-memory-testkit-wire.guCIFe/`: incoming and
scoped locks, registry records, source comparison, candidate source inputs, managed
CLI setup and focused checks. Rust 1.91 native/Wasm Clippy, 172 Timers, 71 consumer
and three protocol tests pass. Consumer tooling fixtures, all fourteen graph
contexts, pins/group admission, formatting and local links pass. The managed
Testkit 0.75.1 CLI is prepared; existing PocketIC 16.1.0 admission passes.
All 11 explicit live consumer recovery tests pass on the scoped graph, including
endpoint refusals, traps, upgrades, authorization and uncertain-effect handling.
Evidence is `/tmp/jobs-recovery.19LcGa/`, retaining the frozen Wasm, selected lock,
tool identity and candidate Cargo manifest/source patch against `b569b7b`.
Published implementation sources match the prior versions; only dependency and
packaging selections changed. No full CI, new macOS run or Timers cohort matrix
ran. These results do not qualify unrelated working-lock patch selections.

Earlier Build/Host/Wire evidence is `/tmp/ic-scheduling-build-host-wire.DDr9aE/`:
Rust 1.91 native/Wasm Clippy, consumer/capture/build/protocol tests, consumer
tooling and both metadata/rollback adapters passed; product gates/setup/uploads
were substitutes inside fixtures. Earlier Memory/Metrics evidence is
`/tmp/ic-scheduling-memory-metrics.oyBtpP/` (172 Timers and 71 consumer tests).
Shared catalog/adapter evidence is `/tmp/ic-scheduling-shared022.sL8syL/`;
earlier Shared evidence is `/tmp/ic-scheduling-shared020.P1MNg7/`.
Native Linux and both macOS CI jobs passed at `be3dc8c`; they do not qualify new
source or lock selections. Future hosted CI reports must bind the exact source.

Earlier actual releases retain evidence at `/tmp/ic-scheduling-adopt-all.AQGpWV/`:
complete gates, 43 installed watchdog tests and live cohort qualification per
release, plus 11 original-Wasm consumer recovery tests at the Jobs tag. Original
consumer evidence is `/tmp/jobs-recovery.dfKRF6/`. Registry/download/upload hashes
and embedded source identities matched the tags. See the
[prior handoff](https://github.com/dragginzgame/ic-scheduling/blob/be3dc8c20f1e7834ec05ad55da6aa263ba10dc05/docs/status/current.md)
for retained older retries and qualification locations; never relabel failures.

## Release and recovery custody

Canonical ledgers are `releases/jobs/CHANGELOG.md` and
`releases/timers/CHANGELOG.md`. Root CHANGELOG is an index; coordinated history
through 0.33.1 remains once at `releases/history.md`. Released identities remain
Timers `timers-v0.34.0` / `ed255a393ef7245ffcc8cf25d413a0f883f4b6c2` and
Jobs `jobs-v0.33.2` / `4a76520d2265ab598113f00c4fbde0111268ffe2`.
Both real journals are complete; do not replay effects or rewrite registry artifacts.

Require explicit `RELEASE_GROUP=jobs|timers` for release/resume/publication.
Missing/empty/unknown groups and cross-group PACKAGE selections refuse before
effects. Direct delivery is `origin`/`main`; publication is separate. Shared owns
exact plans, transactions and source/prepared receipts. Reconcile old attempts
against recorded source/policy/layout; never reinterpret them after adoption.
Release gates remain complete CI, Testkit/server preparation and installed
watchdog/cohort qualification; grouping neither narrows gates nor isolates resolution.

Consumers own metadata, storage, lifecycle reconstruction and handler selection.
Restore only through `Job::restore`; Running/Uncertain effects stay blocked until
exact receipts or explicit owner disposition. Preserve bounded Wire `JQW\x01`
queue bytes, retry identities, admission and rollback. No JSON fallback/reset/
converter. Preserve binaries/backups, attempts and receipts before retirement.
Memory's allocation ledger is separate: never clear only queue or ledger. Logging
stays heap-only, bounded to 32 KiB and recorded after committed work; Metrics reset
neither clears logs nor reuses IDs. Keep endpoint attributes and size guards.
Native tests alone do not prove IC recovery; live receipts qualify only their inputs.

Jobs provenance: `50fac19a55624c2110778e21ba5bb15ef8bf469a` (0.17.51). Timers:
dirty import `651efce370bd6aec6290b74118eb709488062d0a` (0.32.4), never a build
input. Migration custody: `/tmp/ic-scheduling-merge.7PRSi3Nq/`. The private Canic
contribution remains `/tmp/canic-jobs-adapter.dPFVZ7FD/source`, branch
`codex/jobs-application-adapter`, commit `fc14bbe1baa3c0fdc09b0550e25a1d3fdcd9d2d1`;
it needs separate integration/qualification. No shipped Canic mount API is claimed.
