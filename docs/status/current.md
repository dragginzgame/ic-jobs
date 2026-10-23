# Current handoff

## Delivered source and pending adoption

Jobs **0.17.19** is delivered at `eab5a56`; workspace metadata remains 0.17.19.
The **0.17.20** draft records the next Host and Shared Tooling adoption. Staging,
commits, version changes, releases and publication remain user-owned.

Host FS/Process/Artifacts, Build, Testkit and Testkit Native select **0.66.0**.
Only those six packages change relative to the incoming lockfile. Incoming
Memory/Storage **0.50.12**, Timers **0.30.5**, Wire/derive **0.6.7** and other
selections are preserved. Platform/macros remains **0.6.9**.

Testkit's `PocketIcInstance` now hides client/server fields. The recovery harness
and queue example use `instance()` for shared borrows; both retain the owner for
the entire session. No ownership extraction, new server lifetime or async drop
path is introduced. Client deletion still precedes managed server shutdown.

Shared Tooling **0.26.8** is exported at reviewed revision
`0b07c2a3163a5a66328e100b553baf93afd8ed07` with the existing entrypoints,
including `audits/ci-release-integrity.md`. It fixes lock-cleanup contention and
initializes Cargo's source cache without a global setup lock. Previous generations
and old preparation slots remain retained. Pins and IC selections are unchanged.

Rerun `make install-rust-tools install-release-tools install-testkit-tools` once
for the changed Cargo-tool build policy, then admit the selections offline.
Make, optimized recovery and CI evidence still select
`quill icp didc ic-wasm wasm-opt`. PocketIC provisioning remains Testkit-owned.

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
in `target/qualification/host-shared-01720/`. Strict Clippy, consumer composition,
unit/Wire tests, capture tests and harness compilation pass. Separate core and
consumer Rust 1.91/Wasm checks, all five graph contexts, boundaries, pins and
release-command checks pass. Snapshot integrity, all documentation links,
formatting/hook checks and local failure-evidence fixtures pass. Common Cargo
tools and Build/Testkit CLIs are prepared and admitted offline. Release/publication
and consumer tooling fixtures pass with substitute effects; no release or upload
occurred. `make test-demo` passes live seconds/minutes/weeks, recurrence,
pause/resume and cancellation checks against the local canister on this graph.
No full CI, new native macOS or nine-scenario recovery result is claimed.

Earlier qualifications remain in `target/qualification/host-shared-01719/`,
`target/qualification/doc-link-repair/` and `target/qualification/ic-deps-01718/`.
They bind their respective tested sources and graphs, not this adoption.
The prior missing audit document was fixed through canonical entrypoint selection;
retain link checks on the complete source copy, including new snapshot payloads.
Hosted native/integration runs follow the adopted asynchronous cadence; pending
results do not establish qualification or bypass enforced checks.

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
