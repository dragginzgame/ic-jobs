# Current handoff

## Current scope

Adopt Build/Host 0.73.2 and Wire/Derive 0.13.1, then push the scoped source changes
under the existing instruction. Endpoint remains 0.2.0. No package-version bump,
new release, tag or registry upload is selected for this follow-up.
Jobs remains 0.33.2; Timers remains 0.34.0. Pending notes select Jobs 0.33.3 for
consumer/tooling changes and Timers 0.35.0 for the earlier exposed Metrics change.

Admission found an already dirty Cargo.lock containing the requested updates and
additional Platform, Memory/Storage, Metrics and Logging patches. Preserve those
other edits unstaged. The original bytes and a scoped lock candidate are retained
at `/tmp/ic-scheduling-build-host-wire.DDr9aE/`. Qualification and the source
commit use only the Build/Host/Wire changes. Qualification restored the original
working lock byte-for-byte; commit preparation must preserve that restoration,
leaving unrelated patch selections outside this commit.

## Workspace and suppliers

All nine members remain in one workspace with explicit group versions. The root
Cargo catalog shows each member once in Jobs/Timers blocks with full versions,
paths and ledger labels. `ci/cargo-release-groups.json` owns complete alias
rosters and primary version sources. There is no root workspace version.
Catalog entries add no dependency edges unless inherited by a member.

The delivered graph selects Platform 0.15.2 / Types 0.15.1, Wire 0.13.1,
Endpoint 0.2.0, Testkit/Native 0.75.0, Build/Host 0.73.2, Memory/Storage 0.60.0,
Logging 0.23.2 and direct Metrics 0.24.0. Testkit separately retains arithmetic-only
Metrics 0.23; no cross-version measurement conversion is introduced. Metrics
defaults remain disabled. Memory codec errors and optional Metrics schemas align
with Wire 0.13. Jobs remains Serde-only and independent of Timers.

Timers, Memory and the qualification consumer share one registry Platform facade,
macro generator and exact runtime guard. Preserve the Timers 0.15 source identity
check, executor/IC0-only library features, private FFI and host/protocol exclusions.
The Build metadata editor is selected from the root lock through managed setup;
refresh it with `make install-release-tools` before offline checks.

Shared 0.27.22 is canonically exported from clean committed revision
`f4f431627290e18f0ccc1e5f8525d16b197cf7c3` (131 payloads). No snapshot-owned file
is edited locally. Grouped formatting preserves the root catalogs. No cleanup or
new Testkit/server installation is selected for this dependency refresh.

## Evidence and limits

Current evidence is `/tmp/ic-scheduling-build-host-wire.DDr9aE/`: original/scoped
locks, registry observations, selected metadata, tool setup and focused checks.
The scoped delivery graph passes Rust 1.91 strict native/Wasm Clippy, 71 consumer,
five capture/CLI, five Timers build-coordinator and three protocol tests. Consumer
tooling fixtures and both real group metadata/rollback adapters pass, alongside
all fourteen graph contexts, pins, group admission, formatting and local links.
Fixture product gates/setup/uploads were substitutes. The managed 0.73.2 editor
is prepared. Published implementation sources match the prior patches except a
Host Process test timing adjustment; no consumer code change was needed.
No full CI or new installed recovery ran; local checks do not qualify macOS.
These results do not qualify the unrelated working-lock patch selections.

The preceding Memory/Storage 0.60 and Metrics 0.24 adoption passed Rust 1.91 native/
Wasm Clippy, 172 Timers and 71 consumer tests plus all fourteen graph contexts;
its evidence remains `/tmp/ic-scheduling-memory-metrics.oyBtpP/`.
Shared catalog/adapter evidence remains `/tmp/ic-scheduling-shared022.sL8syL/`:
real Cargo/editor/Git fixture checks cover exact files, both groups' metadata,
rollback, receipt refusal, unrelated identities, resume and publication routing;
full product gates/setup/uploads were substitutes. Earlier Shared evidence is
`/tmp/ic-scheduling-shared020.P1MNg7/`. Native Linux and both macOS CI jobs passed
for `be3dc8c`; those passes do not qualify newer source or lock selections.

Earlier real release qualification remains `/tmp/ic-scheduling-adopt-all.AQGpWV/`:
full configured gates, 43 installed watchdog tests and live cohort qualification
for each release, plus 11 original-Wasm consumer recovery tests at the Jobs tag.
Original consumer evidence remains `/tmp/jobs-recovery.dfKRF6/`. Registry/download/
upload checksums and embedded source identities matched the published tags.
See the [prior handoff](https://github.com/dragginzgame/ic-scheduling/blob/be3dc8c20f1e7834ec05ad55da6aa263ba10dc05/docs/status/current.md)
for retained earlier supplier retries and historical evidence locations.

## Release and recovery custody

Canonical ledgers are `releases/jobs/CHANGELOG.md` and
`releases/timers/CHANGELOG.md`. The root changelog is an index; coordinated history
through 0.33.1 is retained once at `releases/history.md`. The released identities
remain Timers `timers-v0.34.0` / `ed255a393ef7245ffcc8cf25d413a0f883f4b6c2` and
Jobs `jobs-v0.33.2` / `4a76520d2265ab598113f00c4fbde0111268ffe2`.
Both real journals are complete; do not replay effects or rewrite registry artifacts.

Require explicit `RELEASE_GROUP=jobs|timers` for release/resume/publication.
Missing/empty/unknown groups and cross-group PACKAGE selections refuse before
effects. Direct delivery uses `origin`/`main`; publication is separate. Shared owns
exact plans, transactions and source/prepared receipts. Reconcile old attempts
against their recorded source, policy and layout; adoption never reinterprets them.
Both releases retain complete CI, Testkit/server setup and installed watchdog/cohort
qualification; grouping never narrows the combined gate or isolates resolution.

Consumers own metadata, storage, lifecycle reconstruction and handler selection.
Every restore uses `Job::restore`; Running/Uncertain effects remain blocked until
exact receipts or explicit owner disposition. Retain bounded Wire `JQW\x01` queue
bytes, validation, retry identities and rollback. No JSON fallback, reset or converter.
Preserve old binaries/backups, attempts and receipts before retiring installations.
Memory's allocation ledger is separately owned: never clear only queue or ledger.
Logging stays heap-only, bounded to 32 KiB and recorded after committed work;
Metrics reset neither clears logs nor reuses IDs. Keep endpoint attributes and
local size guards. Native tests alone do not establish IC recovery.

Jobs provenance is `50fac19a55624c2110778e21ba5bb15ef8bf469a` (0.17.51); Timers
is the dirty imported sibling at `651efce370bd6aec6290b74118eb709488062d0a`
(0.32.4), never a build input. Migration custody: `/tmp/ic-scheduling-merge.7PRSi3Nq/`.
The private Canic contribution remains `/tmp/canic-jobs-adapter.dPFVZ7FD/source`,
branch `codex/jobs-application-adapter`, commit
`fc14bbe1baa3c0fdc09b0550e25a1d3fdcd9d2d1`; it requires separate integration
and qualification. Jobs tests do not establish Canic readiness or a shipped mount API.
