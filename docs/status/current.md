# Current handoff

## Source and pending adoption

Jobs **0.17.17** is delivered at `47201d7`;
workspace metadata remains 0.17.17. The **0.17.18** draft records the current IC
adoption. Staging, commits, version changes, releases and publication remain user-owned.

The incoming lockfile already selected Platform/macros **0.6.8**, Memory/Storage
**0.50.11**, Build/Host/Testkit/Testkit Native **0.64.1** and Wire **0.5.22**.
This task preserves the patch selections, records their minimums in the catalog
and adopts published Wire/derive **0.6.0**. Timers stays **0.30.3**; Metrics and
other non-Wire lockfile selections are unchanged from the incoming tree.

Jobs uses Wire's retained `native` Serde/Postcard framing only; the removed finite
schema, typed-method and TypeScript APIs have no Jobs callers. No Rust adaptation,
new codec, binding regeneration or application migration is required. Method IDs,
limits and DTO layouts remain unchanged. The queue client and canister must still
be rebuilt together. No borrowed metadata or unreleased Wire additions are selected.

Testkit's native management feature selects Platform's Candid and management DTOs.
Those dependencies remain outside the production canister and core graphs.
PocketIC retains management types 0.8 while Platform selects 0.11 on native tests;
these separate upstream contracts are not exchanged by Jobs-owned APIs.
No direct external IC dependencies or local facades are introduced.

The published library retains its Serde-only API and Rust **1.91** floor.
Consumers own storage, payloads, endpoints, lifecycle, authentication and effect
disposition. Every decoded record crosses `Job::restore`; Running/Uncertain work
stays blocked until explicit reconciliation. Error precedence, stored formats
and commit-before-dispatch remain. The [root ownership map](../../README.md#scope-and-validation)
is authoritative. The consumer Wasm graph requires one Platform identity.

## Current qualification

Starting source, incoming lockfile/patch, previous handoff and qualification logs
are retained in `target/qualification/ic-deps-01718/`. Strict workspace-selected
Clippy, consumer composition/unit/Wire tests, capture tests and harness compilation
pass. Separate core/consumer Rust 1.91 and Wasm checks, all five graph contexts,
boundaries, pins, formatting and documentation links pass. `make test-demo` passes
against the local canister for seconds/minutes/weeks, recurrence, pause/resume and
cancellation. Prepared Build/Testkit CLIs pass offline admission, and release,
publication and consumer tooling fixtures pass with their documented substitute
effects. No actual release or upload was performed.
No full CI, new native macOS or nine-scenario recovery result is claimed.

Selected Build and Testkit executables require `cli` during preparation; library
dependencies retain their selected features. Prepare with
`make install-release-tools install-testkit-tools`, then admit offline with
`make release-tools-check testkit-tools-check`. PocketIC setup/admission stays
Testkit-owned. Shared Tooling remains **0.25.9**, revision
`85aded191d85ac4a8c0e5deab207f6be18895862`.

## Local canister queue client

[The terminal demo](../../crates/jobs-canister-tests/README.md) shipped in 0.17.15.
`make demo` installs a fresh fixture canister in local PocketIC. Queue seconds,
minutes or weeks; inspect jobs, advance IC time, watch countdowns and manage
recurrence through existing Wire endpoints. The host retains display labels only;
queue storage, scheduling and effects stay in the canister. Quit/EOF/Ctrl+C release
the owned server. Interactive execution stays in the foreground; the batch runner
is used for the noninteractive `make test-demo` live scenario.

Earlier Linux qualifications are retained in `target/qualification/queue-demo/`,
`target/qualification/host-064/` and `target/qualification/timers-0303/`.
They cover their respective source/graph selections, not the current adoption.
The memory IO observer remains consumer dev-only, with exact counts and overflow
rejection; production graphs exclude it.

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
