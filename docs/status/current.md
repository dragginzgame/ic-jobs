# Current handoff

## Delivered source and pending adoption

Jobs **0.17.18** is delivered at `7d3f78b`; workspace metadata remains 0.17.18.
The **0.17.19** draft records Host and Shared Tooling adoption. Staging, commits,
version changes, releases and publication remain user-owned.

Host FS/Process/Artifacts, Build, Testkit and Testkit Native now select **0.65.0**.
The incoming lockfile's Platform/macros **0.6.9**, Timers **0.30.4** and
Wire/derive **0.6.5** selections are preserved, along with other incoming versions.
Only the six Tooling packages change relative to that incoming lockfile.
Jobs uses Wire's retained native Serde framing, not Testkit's optional Wire call
helpers; the 0.65 signature change requires no local Rust adaptation.

Shared Tooling **0.26.7** is exported from reviewed revision
`3e8d7cd3e7e9b7b79a5e800c30daf0be2710dab0` through the canonical exporter,
retaining the existing entrypoint selection. The baseline adds asynchronous
hosted validation guidance, self-owned dependency repairs and explicit IC tool
selection. Host/Cargo/IC version and digest pins are unchanged.

Make, the optimized recovery wrapper and CI evidence callers explicitly select
`quill icp didc ic-wasm wasm-opt`, preserving the previous full set. Local release
and consumer fixtures admit those arguments. New IC installation receipts require
explicit preparation; old tool bundles and evidence are preserved. Shared setup
waits at most 60 seconds for another slot owner; offline checks remain immediate.
Use `make install-ic-tools install-release-tools install-testkit-tools`, then the
matching offline checks. PocketIC setup/admission stays Testkit-owned.

## Ownership and compatibility

The published library retains its Serde-only API and Rust **1.91** floor.
Consumers own storage, payloads, endpoints, lifecycle, authentication and effect
disposition. Every decoded record crosses `Job::restore`; Running/Uncertain work
stays blocked until explicit reconciliation. Error precedence, stored formats,
method IDs, Wire budgets and commit-before-dispatch remain unchanged.
The [root ownership map](../../README.md#scope-and-validation) is authoritative.
The consumer Wasm graph requires one Platform identity and excludes Candid/Host.
PocketIC's Candid management dependencies remain native-only.

## Current qualification

Starting source, incoming patch/lockfile, previous handoff and logs are retained
in `target/qualification/host-shared-01719/`. Snapshot integrity, release-command
adapters, dependency graphs/boundaries/pins, strict Clippy, consumer unit/Wire and
composition tests, capture tests and harness compilation pass. Separate core and
consumer Rust 1.91/Wasm checks, formatter/hook and local evidence fixtures pass.
Build, Testkit and IC tools are prepared. Shared-slot contention was safely
refused at 60 seconds; retry reused the verified result after its owner finished.
Release/publication and consumer tooling fixtures pass with substitute effects;
no actual release or upload occurred. Prepared CLIs pass offline admission, and
`make test-demo` passes live seconds/minutes/weeks, recurrence, pause/resume and
cancellation checks against the local canister with this graph.
No full CI, new native macOS or nine-scenario recovery result is claimed.
GitHub's repository description was reviewed and matches current ownership.

Release validation exposed a missing audit-index target. The snapshot now selects
`audits/ci-release-integrity.md` at the same pinned revision. Documentation links
and snapshot integrity pass after repair; the earlier link-check claim was wrong.
Repair evidence and the starting handoff are in `target/qualification/doc-link-repair/`.

Earlier qualifications remain in `target/qualification/ic-deps-01718/`,
`target/qualification/host-064/` and `target/qualification/timers-0303/`.
They bind their respective tested sources and graphs, not this adoption.
New hosted native/integration runs follow the adopted asynchronous cadence;
pending results do not establish qualification or bypass enforced checks.

## Local canister queue client

[The terminal demo](../../crates/jobs-canister-tests/README.md) shipped in 0.17.15.
`make demo` installs a fresh fixture canister in local PocketIC. Queue seconds,
minutes or weeks; inspect jobs, advance IC time, watch countdowns and manage
recurrence through existing Wire endpoints. The host retains display labels only;
queue storage, scheduling and effects stay in the canister. Quit/EOF/Ctrl+C release
the owned server. Interactive execution stays in the foreground; the batch runner
is used for the noninteractive `make test-demo` live scenario. Earlier terminal
qualification and interrupt cleanup evidence remain in `target/qualification/queue-demo/`.
The memory IO observer remains consumer dev-only; production graphs exclude it.

## Retained live recovery evidence

The [three-host live recovery run](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793)
passed on **2026-10-21** against Jobs **0.17.14**, commit
`7d1f808de0bc14d823bd1d3f04a5a62f06ad7a52`, with Build/Host/Testkit **0.63.2**.
Nine scenarios passed for original Wasm and Binaryen `-O3`, `-Os`, `-Oz` on Linux,
macOS Intel and Apple Silicon: **108 successful scenario executions**.
The [regular CI matrix](https://github.com/dragginzgame/ic-jobs/actions/runs/39591155917)
also passed for that source. These results do not qualify newer graphs.

`target/qualification/recovery-01714/` retains artifacts, job logs, selections and
verified Wasm hashes; preserve them. The tested lock SHA256 is
`0fb02c5ce5128281f0183b7911fde2c6196bd018fb478fff1b939299201dd6b9`.
Coverage includes watchdog delivery, upgrades, overdue policies, rollback, trap
recovery and receipt-based uncertain-effect reconciliation. It does not establish
production deployment, instruction/cycle costs or untested consumer behavior.
Follow [release preparation](../releasing.md) for direct `origin`/`main` delivery.
Rebuild fixture and harness together for an explicitly selected live recovery
gate. Preserve retained data and unresolved effects.
