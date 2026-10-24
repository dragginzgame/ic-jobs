# Current handoff

## Pending local cleanup

Against released `34c536a` (0.17.21), the 0.17.22 draft narrows IC tools to
`wasm-opt`, reuses post-advance CLI views, and adds `make test-demo-cli` for real
terminal quit/EOF/idle-interrupt cleanup. Public APIs and stored formats remain
unchanged. The incoming Cargo.lock updates are preserved, not independently
qualified by this cleanup. Rerun `make install-ic-tools` for the new selection.

Focused checks pass: Rust 1.91 parser test and compilation of both ignored live
tests, strict Clippy for the queue example/process test, consumer tooling
fixtures, formatting and documentation links. No fresh live result is claimed:
`make consumer-server-check` refuses because the lock-selected Testkit 0.66.3
CLI has not been prepared. Prepare it with `make install-testkit-tools`, then
`make install-consumer-server`; explicitly run `make test-demo test-demo-cli`.
Full CI and macOS execution were not run.

## Previous qualification handoff (historical)

Jobs **0.17.20** is delivered at `2b867b8`; workspace metadata remains 0.17.20.
The **0.17.21** draft records qualification of the pending Platform, Metrics and
Timers selections. Staging, commits, version changes, releases and publication
remain user-owned.

The incoming lockfile already selected Platform/macros **0.6.10**, Metrics
**0.15.7** and Timers **0.30.6**. These are the latest published releases reviewed
for this task. No further lockfile or manifest edits are needed. Metrics remains
transitive through Timers rather than becoming a new direct Jobs dependency.

Published Platform, Metrics and Timers `src` trees match their prior selected
versions, respectively 0.6.9, 0.15.6 and 0.30.5. The releases adopt tooling and
metadata selections upstream. Jobs does not enable Platform's optional native
`wire-schema` metadata; no feature, entrypoint, codec or timer policy is added.
Platform's advertised Rust 1.91 floor matches the existing Jobs requirement.

Host/Build/Testkit/Testkit Native stay **0.66.0**. PocketIC access uses `instance()`
while retaining the owner for the whole session. Memory/Storage stays **0.50.12**
and Wire/derive stays **0.6.7**. Shared Tooling stays **0.26.8** at
`0b07c2a3163a5a66328e100b553baf93afd8ed07`, including the integrity audit document.
Common Cargo tools and selected CLIs were prepared after the build-policy change;
this lockfile qualification does not require another executable installation.
IC tools remain `quill icp didc ic-wasm wasm-opt`; provisioning stays Testkit-owned.

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

Starting source, incoming patch/lockfile, previous handoff and focused logs are in
`target/qualification/platform-metrics-timers-01721/`. Strict Clippy, consumer
composition/unit/Wire tests, capture tests and harness compilation pass. Separate
core and consumer Rust 1.91/Wasm checks, all five graph contexts, boundaries,
pins, formatting and documentation links pass. Source comparisons also confirm
the Platform macros are unchanged. The incoming lockfile remains byte-identical;
the Wasm feature graph excludes Platform's optional schema metadata.
No full CI, new native macOS or live PocketIC result is claimed for this graph.

Earlier qualifications remain in `target/qualification/host-shared-01720/`,
`target/qualification/host-shared-01719/` and `target/qualification/doc-link-repair/`.
They bind their respective tested sources and graphs, not this selection.
The 0.17.20 dependency graph passed the live queue scenario and focused native,
Wasm, MSRV, graph and tooling checks; retain the source-bound evidence there.
Hosted native/integration runs follow the adopted asynchronous cadence; pending
results do not establish qualification or bypass enforced checks.

## Local canister queue client

[The terminal demo](../../crates/jobs-canister-tests/README.md) shipped in 0.17.15.
`make demo` installs a fresh fixture canister in local PocketIC. Queue seconds,
minutes or weeks; inspect jobs, advance IC time, watch countdowns and manage
recurrence through existing Wire endpoints. The host retains display labels only;
queue storage, scheduling and effects stay in the canister. Quit/EOF/Ctrl+C release
the owned server. Interactive execution stays in the foreground; the batch runner
is used for the separately selected `make test-demo` live scenario. Earlier terminal
and interrupt cleanup evidence remains in `target/qualification/queue-demo/`.
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
