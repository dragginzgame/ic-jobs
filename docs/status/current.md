# Current handoff

## Accepted scope and source

Issue [#82 — Version Jobs and Timers independently](https://github.com/dragginzgame/ic-scheduling/issues/82)
approves two release groups in the existing nine-member root workspace.
`jobs` is primary: `ic-jobs`, `ic-jobs-test-consumer`,
`ic-jobs-canister-test-support`, `ic-jobs-canister-tests` inherit the root version.
`timers`: `ic-timers`, `ic-timers-pocketic`, `ic-timers-runtime-probe`,
`ic-timers-size-probe`, `ic-timers-test-protocol` use explicit matching versions.
Only the two public libraries are publishable. Both groups currently remain
0.33.1; no numeric bump, release, real commit/tag/push or publication was executed.
The retained coordinated release is `v0.33.1`, commit `03b166f`.

Policy: `ci/cargo-release-groups.json`; complete comment map: root Cargo.toml.
Jobs ledger: `CHANGELOG.md`, pending 0.33.2 for compatible tooling/integration work.
Timers ledger: `crates/ic-timers/CHANGELOG.md`, pending 0.34.0 for Metrics 0.22's
exposed identity/native-metadata change. Shared history through 0.33.1 is retained
once at `docs/changelog/shared-through-0.33.1.md`; older group entries are preserved.
The user's staged CI change and unmerged changelog index entries remain intact.
Resolving/staging that existing merge remains user-owned.

## Release and dependency state

Shared Tooling commit `39ba9836a269ee7ea7f073a3f23d9cefe3c7d03c` is adopted
through its exporter (snapshot version metadata 0.27.16, including the committed
0.27.17 shared-metadata support; no publication claim). Build 0.73.0 supplies the installed group-capable editor.
Build, Host and Testkit now select 0.73.0. Earlier tool roots and evidence remain
retained. Shared's committed HEAD is unchanged; its supplier checkout has only
an uncommitted VERSION edit, so no dirty snapshot was exported.

Default `RELEASE_GROUP=jobs`; select `timers` for version/package/publication and
standard release/resume commands. Shared owns exact group file plans, metadata
transactions, group tags/journals and source custody. Local adapters retain
source/tool/publication policy and the complete combined release gate. Group tags
are `jobs-vX.Y.Z` and `timers-vX.Y.Z`; historical coordinated tags remain unchanged.
Shared receipt mechanics bind validated identity/group/policy and prepared exact
metadata payloads under each group's retained state directory. Receipt admission
precedes preparation and subsequent prepared/commit/tag/push checks.
No new release system or extra lockfile was added. See `docs/releasing.md`.

Direct runtime selections now use Platform 0.12.0 / Wire 0.11.0, Metrics/Logging
0.23.0, Testkit 0.73.0 and Memory/Storage 0.58.1. The deployed graph has one
Platform 0.12 facade/guard, shared with Memory. Native Testkit retains Platform
0.13 without another runtime owner; its Wire 0.11 limits align with the harnesses.
The newest composition still fails: Platform 0.15 conflicts with Endpoint 0.1 ->
Platform 0.14 -> runtime guard 0.14, and Testkit 0.73 still accepts Wire 0.11,
not the latest core 0.12.1. No aliases or type/transport copies were introduced.
Release grouping does not isolate Cargo resolution or waive this compatibility.
Failure evidence: `/tmp/ic-scheduling-stack-latest/latest-resolution.log`.
Metrics defaults are disabled here; schemas require its explicit `native-wire`
feature and Wire 0.12. Jobs remains Serde-only and independent of this stack.

## Qualification and remaining work

Current stack adoption evidence is under `/tmp/ic-scheduling-stack-latest/`:
locked metadata and both group plans, fourteen graph contexts and direct
boundaries, Rust 1.91 strict native/Wasm Clippy, 258 native tests and consumer/
runtime-probe release Wasm builds pass. Timer boundary fixtures, release-depth
consumer fixtures, dependency pins and both selected CLI admission checks pass.
Both release-group metadata/receipt/rollback fixtures pass on the final lockfile
(`group-metadata-fixtures.log`), preserving the unrelated group's identities.
No full CI or live recovery was run.

Issue #82 focused evidence is under `/tmp/ic-scheduling-82/`. Disposable Git/bare
remote fixtures use the actual consumer adapter, shared editor and Cargo metadata;
full release validation, tool installation and registry/upload effects are
substituted. They are not live product releases or installed recovery evidence.
Both group plans admit all nine members. Metadata preparation/rollback preserves
unselected versions, ledgers and requirements. Timers-only preparation retains
a Jobs 0.33.1 consumer and updates only the declared Timers requirement.
Runtime singleton, provider, native/Wasm and Rust 1.91 obligations remain intact.
The final receipt-enabled `make test-release-tooling` passes for both groups
(`groups-receipts-final.log`), including tampered-receipt refusal, both editor
rollback boundaries, a copied cross-group journal refusal and successful resume.
Strict Rust 1.91 harness Clippy, 11 focused native capture/build tests, core MSRV,
Timers/consumer Wasm checks, dependency policies/pins, both group plans, shared
snapshot integrity, release-command fixtures, manifest order, ShellCheck and
documentation checks pass. The release-depth consumer fixture also passes.
Shared history was compared byte-for-byte against the pre-change root entry.

The combined `release-verify` gate still runs complete CI, explicit Testkit/server
setup and live Timers watchdog/cohort tests. Ordinary CI packages both public
crates. No full local CI, live PocketIC or new macOS matrix was run for #82.
Preceding dependency checks/tests remain source-bound historical evidence at
`/tmp/ic-scheduling-cohort-adoption/`, `/tmp/ic-scheduling-persistence-latest/`,
`/tmp/ic-scheduling-tooling-latest/` and `/tmp/ic-scheduling-latest-adoption/`.
Earlier native/live qualification cannot establish recovery on this new graph.

## Provenance and recovery constraints

Jobs baseline: `50fac19a55624c2110778e21ba5bb15ef8bf469a` (0.17.51).
Timers import: dirty sibling at `651efce370bd6aec6290b74118eb709488062d0a`
(0.32.4), including pending Platform/Metrics changes. The sibling is read-only
provenance, never a build input. Migration evidence and incoming patches remain
at `/tmp/ic-scheduling-merge.7PRSi3Nq/`. Shared snapshots are exporter-owned.

Consumers own durable metadata, storage, lifecycle reconstruction and handler
selection. Timers owns its sole private volatile IC timer provider. Every restore
uses `Job::restore`; Running/Uncertain effects stay blocked until explicit owner
disposition. Queue bytes remain bounded Wire `JQW\x01`, with no JSON fallback,
reset or converter. Preserve prior binaries/backups, attempts and receipts before
retiring installations. Memory's allocation ledger is separately owned: never
clear only a queue or ledger. See the consumer guide and fixture provenance.
Logging remains heap-only, bounded to 32 KiB and recorded after committed work;
Metrics reset does not clear logs or reuse IDs. Preserve explicit Platform
attributes and local size guards and their rejection behavior.

The private Canic contribution remains `/tmp/canic-jobs-adapter.dPFVZ7FD/source`,
branch `codex/jobs-application-adapter`, commit
`fc14bbe1baa3c0fdc09b0550e25a1d3fdcd9d2d1`. It requires separate integration and
qualification; Jobs tests do not establish Canic readiness or a shipped mount API.
