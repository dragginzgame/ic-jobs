# Current handoff

## Source and pending adoption

Jobs **0.17.15** is delivered at
`0a1341eff1c5bebe23464e2bf04b70575f1efa6d`. Workspace metadata remains 0.17.15;
the **0.17.16** draft records the pending Host adoption. Staging, commits,
version changes, release execution and publication remain user-owned.

The working graph selects Host FS/Process/Artifacts, Build and Testkit **0.64.0**.
Build and Testkit move together because their previous line requires Host 0.63.
Only those five lockfile packages changed. Testkit Native stays **0.63.3**;
Memory/Storage, Platform and Wire retain their incoming selections.
Recovery tests and the queue example now import upstream PocketIC types from
`ic_platform::pocket_ic`, with `pocket-ic` enabled only in harness dev-dependencies.
Testkit retains startup/lifecycle extensions. Jobs does not use the removed
Testkit principal/performance facades, diagnostics batch error or Wire call helper.

The published library retains its Serde-only API and Rust **1.91** floor.
Consumers retain storage, payloads, endpoints, lifecycle, authentication and effect
disposition. Every decoded record crosses `Job::restore`; Running/Uncertain work
stays blocked until explicit reconciliation. Error precedence, Wire frames,
stored formats and commit-before-dispatch are unchanged.
The [root ownership map](../../README.md#scope-and-validation) remains authoritative.

## Local canister queue client

[The terminal demo](../../crates/jobs-canister-tests/README.md) shipped in 0.17.15.
`make demo` installs a fresh fixture canister in local PocketIC. Queue seconds,
minutes or weeks; inspect jobs, fast-forward IC time, watch countdowns and manage
recurrence through existing Wire endpoints. The host retains display labels only;
queue storage, scheduling and effects stay in the canister. Quit/EOF/Ctrl+C release
the owned server. Interactive execution stays in the foreground; the batch runner
is used for the noninteractive test. `make test-demo` selects the live CLI case.

Earlier Linux qualification of the 0.63.3 graph includes the actual terminal
route, six-second countdown, week advance, normal quit and interrupt cleanup.
It is retained in `target/qualification/queue-demo/`; it does not qualify 0.64.

## Current focused qualification

Host adoption evidence, original tracked source and the previous handoff are in
`target/qualification/host-064/`. Rust 1.91 passes the four capture tests, recovery
reply-limit test and queue parser test; nine recovery cases and the live queue
case remain ignored in that ordinary native run. All five dependency graph
contexts, dependency boundaries and pin declarations pass. Strict Clippy, separate
core/consumer MSRV and Wasm checks, consumer and release tooling fixtures,
formatting and documentation links pass. Prepared Build/Testkit executables pass
offline admission. `make test-demo` passes against the local PocketIC canister,
covering seconds/minutes/weeks, recurrence, pause/resume and cancellation.
This live CLI case does not rerun the separate nine-scenario recovery suite.

The selected Build and Testkit executables require `cli` during preparation;
library dependencies retain their selected features. Use
`make install-release-tools install-testkit-tools` followed by the offline
`make release-tools-check testkit-tools-check`. PocketIC provisioning/admission
stays Testkit-owned. Candid remains transitive for PocketIC's external protocols;
Jobs-owned messages use Wire. No full CI or new native macOS result is claimed.

## Retained live recovery evidence

The [three-host live recovery run](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793)
passed on **2026-10-21** against Jobs **0.17.14**, commit
`7d1f808de0bc14d823bd1d3f04a5a62f06ad7a52`, with Build/Host/Testkit **0.63.2**.
Nine scenarios passed for original Wasm and Binaryen `-O3`, `-Os`, `-Oz` on Linux,
macOS Intel and Apple Silicon: **108 successful scenario executions**.
The [regular CI matrix](https://github.com/dragginzgame/ic-jobs/actions/runs/39591155917)
also passed for that delivered source. These results do not qualify newer graphs.

`target/qualification/recovery-01714/` retains downloaded artifacts, job logs,
source/lock/snapshot selections and verified Wasm hashes; preserve them.
The tested lock SHA256 is
`0fb02c5ce5128281f0183b7911fde2c6196bd018fb478fff1b939299201dd6b9`.
Coverage includes watchdog delivery, upgrades, overdue policies, rollback, trap
recovery and receipt-based uncertain-effect reconciliation. It does not establish
production deployment, instruction/cycle costs or untested consumer behavior.

## Existing tooling and next delivery

[#56 — Adopt canonical Testkit memory IO observer after publication](https://github.com/dragginzgame/ic-jobs/issues/56)
is completed in 0.17.14. The observer remains consumer dev-only, with exact counts
and overflow rejection; production graphs exclude it.
Shared Tooling remains **0.25.9**, revision
`85aded191d85ac4a8c0e5deab207f6be18895862`.
Follow [release preparation](../releasing.md) for direct `origin`/`main` delivery.
Rebuild fixture and harness together for any explicitly selected live recovery
gate. Preserve retained data and unresolved effects.
