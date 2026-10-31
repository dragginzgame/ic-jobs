# Current handoff

## Accepted scope and source

IC Scheduling combines `ic-jobs` and `ic-timers` in one nine-member workspace.
Both published libraries use shared version 0.33.1, recorded by local tag
`v0.33.1` and release commit `03b166f`. Pending notes select 0.33.2 for the
compatible IC Tooling update; package versions remain unchanged.
The user approved and GitHub completed the repository rename to
`dragginzgame/ic-scheduling`; origin now points to that repository. The physical
checkout is now `ic-scheduling`; open `ic-scheduling.code-workspace` for the combined
editor workspace. This adoption performs no commit, push, tag or publication.
Private Jobs packages and directories now use `ic-jobs-canister-tests`,
`ic-jobs-canister-test-support` and `ic-jobs-test-consumer`. All seven support
packages remain `publish = false`; their roles are documented in the root README.
Rename checks pass: 45 consumer library tests, 4 API-wire tests, Rust 1.91 release
Wasm build, capture build, consumer-tooling fixtures, five dependency graph
contexts, formatting, documentation links and snapshot integrity. Evidence is
retained at `/tmp/ic-scheduling-names-evidence/`; no live recovery ran.

Jobs baseline: `50fac19a55624c2110778e21ba5bb15ef8bf469a` (0.17.51).
Timers was imported from the authorized dirty sibling working tree on
`651efce370bd6aec6290b74118eb709488062d0a` (0.32.4), including its pending
Platform/Metrics migration. The sibling remains untouched and is not a build
input. Migration evidence and incoming patches are retained at
`/tmp/ic-scheduling-merge.7PRSi3Nq/`. Imported historical Timers evidence retains
its original source identities and does not qualify this workspace.

Shared Tooling is reviewed 0.27.11 at
`62edfbd43087a718bf0283e9040676a49211976d`, adopted through its exporter.
Earlier dirty shared adoption and the private `ic-jobs-canister-test-support`
transport extraction remain included. Snapshot-owned files retain that adoption.

## Dependency state

Active direct dependencies: Platform 0.11.1, Wire 0.10.0, Metrics/Logging 0.21.0,
Tooling/Testkit 0.72.1, Memory/Storage 0.58.0 and local Timers 0.33.1.
The coordinated Tooling release resolves the earlier Wire limits mismatch in
both native harnesses. Platform owns one version-matched runtime guard; Memory
and Storage now share Platform 0.11.1 and Wire 0.10 with the other consumers.
The lockfile no longer contains the older Platform 0.10 or Wire 0.8 copies.
The deployed graphs still exclude registry executor and IC0 providers.

Timers exposes Metrics types, so consumers exchanging them must select Metrics
0.21 and native Wire metadata 0.10; runtime users coordinate on Platform 0.11.
The 0.33.1 release notes retain the earlier coordinated migration.
The notes retain the public dependency identity changes and consumer actions. Queue
and ledger bytes, validated restore, report fields and arithmetic are unchanged.
Logging remains heap-only with the existing retained-text budget.

Memory/Storage adoption evidence is in `/tmp/ic-scheduling-persistence-latest/`;
the preceding coordinated adoption is in `/tmp/ic-scheduling-latest-adoption/`.
Previous failed candidates and focused evidence remain in
`/tmp/ic-scheduling-dependency-adoption/`,
`/tmp/ic-scheduling-observability-019/` and
`/tmp/ic-scheduling-adoption-071/`; they do not qualify the new graph.

## Validation and maintained gates

Tooling 0.72.1 adoption passes locked metadata, direct boundaries, fourteen
Jobs/Timers graph contexts, Rust 1.91 strict all-target harness Clippy, six
native harness tests, Timers harness compilation, both CLI admission checks,
release metadata fixtures and release-depth-2 consumer-tooling fixtures.
Evidence is in `/tmp/ic-scheduling-tooling-latest/`. The pre-existing lockfile
update, including `log` 0.4.35, was preserved. No live PocketIC or full CI ran.

The earlier nested consumer-tooling failure was fixed by reading its retained
failure log instead of omitted console details. Historical focused evidence is
in `/tmp/ic-scheduling-nested-fixture-fix/`; the original failure remains in
`/tmp/jobs-consumer-tooling.QllqL7/` and release-state logs.

Memory/Storage 0.58 adoption passes locked metadata, direct boundaries and all
five Jobs graph contexts, Rust 1.91 strict native/Wasm Clippy, 71 consumer tests
(library, API-wire, composition and stored-wire), and the release consumer Wasm
build. The deployed graph now requires exactly one Platform identity. Evidence
is in `/tmp/ic-scheduling-persistence-latest/`; no live recovery was run.

The preceding coordinated Linux adoption passed full locked metadata, direct dependency boundaries,
five Jobs and nine Timers graph contexts, Rust 1.91 strict native/Wasm Clippy,
253 native tests, Timers harness compilation, and release Wasm builds for the
Jobs consumer and Timers runtime probe. Selected Testkit 0.72 server and Build
release-editor CLIs pass offline admission. Timer dependency-policy fixtures,
consumer-tooling fixtures, dependency pins, documentation links and diff checks
pass. Evidence is in `/tmp/ic-scheduling-latest-adoption/`; failed fixture
attempts are retained separately from retries.

Earlier consolidation evidence includes Jobs core tests, Timers doctests,
provider confinement, package archives and release/publication fixtures under
`/tmp/ic-scheduling-*.log` and the migration evidence directory. It remains
historical evidence, not a fresh package or release qualification of this graph.

The combined CI includes both products' native checks and probe gates. Explicit
release verification additionally retains Timers watchdog and cohort live gates;
ordinary CI does not launch PocketIC. The manually selected recovery workflow
covers Jobs and Timers on Linux and macOS Intel/Apple Silicon. No combined-source
live recovery, macOS matrix, full CI or release gate has yet passed. Earlier live
receipts cannot establish recovery on this new graph.

Earlier evidence remains separate at
`/tmp/ic-jobs-platform-observability.azJk5Hk6/` (11 native, 2 live tests) and
`/tmp/ic-jobs-upstream-adoption.haqjWO7J/` (74 native, 11 live tests).
The unpublished dependency candidate at
`/tmp/ic-jobs-platform091-wire080.41ddtgBu/` is superseded by this consolidation.

## Retained authority and recovery constraints

Jobs remains Serde-only and platform-neutral. Timers owns its existing private
volatile timer registry and IC dispatch; consumers own durable metadata, storage,
lifecycle reconstruction and handler selection. Every restore uses `Job::restore`.
Running/Uncertain effects remain blocked until explicit consumer disposition.

Queue bytes remain `JQW\x01` bounded Wire; no JSON fallback, reset or converter.
Keep prior binaries/backups, attempts and receipts before retiring installations.
Memory's ledger is separately owned: never clear only a queue or ledger. Follow
the [consumer guide](../../apps/ic-jobs-test-consumer/README.md) and
[fixture provenance](../../crates/ic-jobs/tests/fixtures/README.md).
Logging remains heap-only, bounded to 32 KiB, and records after committed work;
Metrics reset does not clear logs or reuse IDs. Keep explicit Platform attributes
and local size guards to preserve rejection behavior.

The private Canic contribution remains at
`/tmp/canic-jobs-adapter.dPFVZ7FD/source`, branch `codex/jobs-application-adapter`,
commit `fc14bbe1baa3c0fdc09b0550e25a1d3fdcd9d2d1`. Its source must be integrated
and qualified separately before claiming Canic readiness; Jobs tests do not
qualify Canic's storage-placement lease owner or imply a shipped mount API.
