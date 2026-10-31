# Current handoff

The current request adopts Memory/Storage 0.60.0 and Metrics 0.24.0 under the
existing instruction to push necessary source changes. Package versions remain
Jobs 0.33.2 / Timers 0.34.0; registry artifacts and tags below remain unchanged.
Pending notes select Jobs 0.33.3 for private consumer/tooling work and Timers
0.35.0 for the exposed Metrics type-identity change. No new release/upload runs.

## Accepted scope and delivery

The maintainer authorizes adopting the new Platform, Wire, Tooling, Memory,
Storage and Shared Tooling, pushing necessary source changes, and releasing both
Scheduling groups to GitHub and crates.io. Delivery remains subject to the full
configured gates; no guard, receipt or installed recovery check is waived.
Source adoption is committed at `1b8aa4a`. Both standard releases and crates.io
uploads completed, supplier-first, with atomic `main`/annotated-tag pushes:

| Group | Published package | Tag | Release commit |
| --- | --- | --- | --- |
| timers | [ic-timers 0.34.0](https://crates.io/crates/ic-timers/0.34.0) | `timers-v0.34.0` | `ed255a393ef7245ffcc8cf25d413a0f883f4b6c2` |
| jobs | [ic-jobs 0.33.2](https://crates.io/crates/ic-jobs/0.33.2) | `jobs-v0.33.2` | `4a76520d2265ab598113f00c4fbde0111268ffe2` |

Every group member retains its group's last released numeric version. Both
retained journals are complete; do not replay effects. Current catalog changes
are pending release and do not rewrite the published artifacts.

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
Memory/Storage 0.60.0, Logging 0.23.2 and direct Metrics 0.24.0. Testkit's typed calls share
Wire 0.13 Limits/Error with the consumer. Endpoint imports replace the removed
Wire endpoint feature. Memory's codec errors and Metrics' optional native schemas
now align with Wire 0.13; no consumer type bridge or duplicate transport was added.
Metrics defaults remain disabled. Testkit separately retains its arithmetic-only
Metrics 0.23 dependency; no cross-version measurement conversion is introduced.

Current dependency-adoption evidence is `/tmp/ic-scheduling-memory-metrics.oyBtpP/`.
Published Memory, Storage and Metrics implementation sources match their preceding
versions (Metrics changes only embedded Markdown documentation); exposed supplier
identities change while codec bytes, admission and numeric behavior are retained.
Rust 1.91 strict native/Wasm Clippy, 172 Timers tests and 71 consumer tests pass.
Group/pin/direct-boundary checks, all fourteen graph contexts, formatting and local
links pass. The lock now has only Wire/Derive 0.13; no release, full CI or new
installed recovery was selected for this focused dependency adoption. Earlier
live receipts below remain bound to their original dependency graph.

The runtime graph has one registry Platform facade, macro generator and exact
runtime guard at 0.15.2, shared by Timers, Memory and the qualification consumer.
The Timers guard and its source-override rejection fixtures select the qualified
0.15 line, preserving executor/IC0-only library features, private FFI confinement
and host/protocol exclusions. Jobs remains Serde-only and independent of Timers.

Shared 0.27.20 is canonically exported from clean committed revision
`267ba25810546bd9cb622d0fb43ccc0588622b67` (131 payloads). The VERSION mismatch
in the preceding supplier commit is corrected. Group/dependency admission now
checks full current versions for all existing versioned local root dependencies;
the Timers requirement already matches 0.34.0 and is labelled with its group.
Intentionally unversioned Jobs/private path declarations remain unversioned.
No snapshot-owned payload is edited locally. No cleanup or tool installation ran.
Shared adoption evidence is `/tmp/ic-scheduling-shared020.P1MNg7/`; the earlier
release evidence below remains bound to its original source and snapshot.
Snapshot/pin/group admission, release-command fixtures, formatting and local links
pass. Both groups' real adapter metadata/rollback fixtures pass, with full gates
and registry effects substituted. Cargo metadata and lock bytes are unchanged;
that tooling-only follow-up ran no full CI or new installed recovery qualification.

Earlier release evidence is `/tmp/ic-scheduling-adopt-all.AQGpWV/`, including the complete
incoming dirty source archive, supplier index observations, exporter impact and
refresh logs, metadata, Clippy and consumer/policy checks. The full native/Wasm
Clippy gate passes. Rust 1.91 consumer library, API-wire, composition and stored
Wire tests pass. Both group plans, pins, direct boundaries, all graph contexts,
timer provider/source identity checks and their rejection fixtures pass. Tools
are prepared through the canonical host/IC/Build/Testkit setup, with old tool
selections retained. Each standard release passed complete CI, MSRV/Wasm and
packaging, then 43 installed watchdog tests and the live cohort qualification.
The released Jobs source also passed all 11 explicit live consumer recovery
tests, including endpoint refusals, upgrades, rollback, authorization and blocked
uncertain-effect disposition. Original Wasm recovery evidence is
`/tmp/jobs-recovery.dfKRF6/`; optimized variants were not selected.

Actual release/publication logs, registry observations, downloaded registry and
upload archives, VCS identities and observed remote refs are retained in the
current evidence directory. Both non-yanked registry checksums match both their
downloaded and upload archives; their embedded source commits match the tags.
Validated/prepared receipts and complete journals remain under `.git/release-state`.

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
