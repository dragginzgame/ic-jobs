# Current handoff

## Source and accepted scope

Baseline: `796b68181b7820245d1785e869be82f2d46c420e` (Release 0.17.50).
The ten-reduction cleanup, Metrics 0.18 adoption and subsequent Logging 0.18
migration are prepared locally with pending 0.17.51 notes. Package versions and
Git index/refs remain unchanged. No release, publication, deployment, data reset
or conversion was performed. Published core runtime source/API remain Serde-only.

The cleanup uses one post-commit diagnostic event flow and one manager-authorized
queue accessor. Host transport/error handling, controller snapshots, native
corruption setup and typed capture arguments are simplified. Release-fixture
phases run in isolated subshells. Cargo's declared Rust floor supplies Make,
launchers and hosted setup through `scripts/dev/minimum-rust.sh`, without setup
side effects; development Rust comes from `rust-toolchain.toml`. Rust stays 1.91.
Shared Tooling stays reviewed 0.26.25 at
`6b6bbd9c9e5798f0ecf6105637392a7a55cd450e`; its snapshot files are unchanged.

Wire's published codec hooks are Wasm-only. **Keep the explicit Platform
attributes and local guards:** automatic Wire attributes trap oversized RPC input,
whereas this fixture rejects it. IDs, shapes, limits, rejection codes and
synchronous reply-failure rollback remain; codec trap text comes from Wire.

## Logging and Metrics contracts

[#79 — Adopt ic-logging 0.18 in the job-consumer logging adapter](https://github.com/dragginzgame/ic-jobs/issues/79)
is implemented locally with published Logging 0.18.0. The retained UTF-8 text
ceiling is `128 * (192 + 2 * 32)` = 32 KiB, preserving the existing 128-row,
192-byte message and 32-byte per-label capacity. It excludes metadata, spare
capacity and transient buffers; it is not a full heap cap. Paging explicitly
uses TopicMatch::Any without changing the request contract. Successful append
receipts retain sequence/truncation on rows; normal eviction is not an omitted
event. All typed append refusals, including TextBudgetExceeded, increment omitted
observations without changing committed job outcomes or retry authority.

Logs are heap-only. No Logging checkpoint is persisted, so the 0.18 checkpoint
layout requires no retained-data transition here. Reconstruction still creates
empty history/omissions; Metrics resets do not clear logs or reuse log IDs.
The issue remains open pending delivery and source-matching acceptance.

The consumer selects Metrics 0.18.0 with unchanged counter/window arithmetic and
numeric reports. Timers keeps its required Metrics 0.17.7; no Metrics values cross
those Rust identities. Incoming Memory/Storage 0.56.2, Timers 0.32.4 and Tooling
0.70.5 selections were preserved. Logging is the only package changed by this
latest update. The lock-selected Testkit CLI was prepared through normal setup.

## Latest qualification

Evidence: `/tmp/ic-jobs-logging018.rB7SPK4d/`, including incoming inputs, final
source/lock/graph, command logs, CLI setup/admission and actual tested Wasm.
Use the retained dirty-source identities, not baseline HEAD alone, for this proof.

Six focused native tests pass on Rust 1.91: full UTF-8 row capacity with no early
byte eviction, successful eviction/truncation versus omission, new typed text
refusal with unchanged history, redaction, bounded/empty-filter pagination and
refused diagnostics after committed work. Strict consumer native/Wasm lint,
actual minimum-compiler Wasm build, pin/boundary/five graph checks, manifest order
and documentation links pass. The Serde-only core/native DTO boundaries remain.

All eleven Linux PocketIC recovery tests pass on the original Wasm using
Testkit 0.70.5/PocketIC 16.1.0. Evidence: `live/jobs-recovery.LeLCoF/` within that
root. Wasm SHA-256:
`5c09d3ce958cc9ef65f3d0d4785cecc4063b8e138c8147452970c0901188abad`.
This includes manager-only log/metric queries, reconstruction with jobs retained,
real watchdog trap rollback, exact receipt recovery and blocked unknown effects.
No full CI, new macOS, optimized-variant, provider or deployment acceptance ran.

Earlier Metrics-only qualification (eight native checks, no live run) remains at
`/tmp/ic-jobs-metrics018.1RGk9ddy/`. Cleanup qualification (74 native checks,
release/consumer tooling fixtures, eleven live recovery cases and four terminal
shutdown scenarios) remains at `/tmp/ic-jobs-complexity-cleanup.haF9H0W6/`.
Keep those earlier source/graphs distinct; do not relabel their artifacts.

## Retained state and earlier Canic work

Release 0.17.50 contains
[#78 — Replace job-consumer JSON queue persistence with bounded IC Wire payloads](https://github.com/dragginzgame/ic-jobs/issues/78).
Current queue bytes remain `JQW\x01` bounded Wire. Retained JSON has no fallback,
reset or converter. Preserve old binaries/backups, attempts and receipts and
explicitly disposition Running/Uncertain work before retiring an installation;
missing evidence never establishes retry safety. Memory's ledger is separately
owned: never clear only a queue or ledger. Follow the
[consumer guide](../../apps/job-consumer/README.md) and
[fixture provenance](../../crates/ic-jobs/tests/fixtures/README.md). Prior JSON
producer evidence remains at `/tmp/ic-jobs-78-upstream.Mso2Zod0/`.

The private Canic contribution remains isolated at
`/tmp/canic-jobs-adapter.dPFVZ7FD/source`, branch `codex/jobs-application-adapter`,
commit `fc14bbe1baa3c0fdc09b0550e25a1d3fdcd9d2d1` on base
`d09d1692e29878221e53f5e5b71479025c09e7f2`, with its patch/evidence alongside.
No sibling checkout was edited. Its release base was unpublished at that review;
integrate onto published source and qualify that resulting graph before delivery.
[Canic #502 — Assess one IC Jobs metadata boundary before adopting it](https://github.com/dragginzgame/canic/issues/502)
records the placement lease/domain-operation assessment; leave its owner unchanged.
The [integration guide](../canic-integration.md) does not advertise a shipped mount
API. Historical all-host proofs remain in earlier handoffs/fixture guides; this
Logging qualification does not qualify Canic or instruction/cycle costs.
