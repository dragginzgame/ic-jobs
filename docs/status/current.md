# Current handoff

## Delivered source and ownership

Jobs **0.17.14**, tag `v0.17.14`, is delivered at
`7d1f808de0bc14d823bd1d3f04a5a62f06ad7a52`; workspace metadata remains 0.17.14.
The released graph selects Build/Host/Testkit **0.63.2**, Memory/Storage **0.50.7**,
Metrics **0.15.2**, Platform/macros **0.6.4**, Timers **0.30.1** and Wire **0.5.14**.
Newer dependency selections remain in the working-tree lockfile. The queue demo
preserves those incoming selections and adds only dev-only Ctrl+C support and its
platform dependencies. Release-tag results do not qualify the newer selections.
Staging, commits, version changes, release execution and publication remain user-owned.

The published library retains its Serde-only API and Rust **1.91** floor. Core
owns checked records, transitions and finite-batch selection. Consumers own
storage, payloads, endpoints, lifecycle, authentication and effect disposition.
Every decoded record crosses `Job::restore`; Running/Uncertain work stays blocked
until explicit reconciliation. Error precedence and commit-before-dispatch remain.
The [root ownership map](../../README.md#scope-and-validation) is authoritative.

## Pending local canister queue demo

The **0.17.15** draft adds [the terminal demo](../../crates/jobs-canister-tests/README.md)
as an example in the existing native harness package. No published API, canister
endpoint or scheduling policy changes. `make demo` installs a fresh fixture
canister in local PocketIC; `sample`, `add`, `list`, `advance`, `watch`, `pause`,
`resume` and `cancel` operate through its existing Wire management endpoints.
The host retains display labels only. Actual queue storage, due times, effects
and watchdog callbacks stay in the canister. Weeks can be fast-forwarded;
`watch` displays one-second advances with real waits. Quit/EOF/Ctrl+C release
the owned server. The interactive client runs in the foreground; the Testkit
batch runner remains appropriate for the noninteractive test.

Linux Rust 1.91 passes the input test and explicit `make test-demo` live case,
covering seconds/minutes/weeks, recurrence, pause/resume and cancellation.
The actual terminal route also passed six-second countdown, week fast-forward,
quit and interrupt checks. Package strict Clippy, product dependency graphs,
boundary/pin checks and affected tooling fixtures pass. Ctrl+C support remains
dev-only; the published library and capture binary normal graphs exclude it.
Ordinary consumer checks run input tests and compile the ignored live case.
The new CLI has not been qualified natively on macOS or through full CI.
Evidence and the starting tree are in `target/qualification/queue-demo/`.

## Current qualification

The [regular CI matrix](https://github.com/dragginzgame/ic-jobs/actions/runs/39591155917)
passed Linux, macOS Intel and Apple Silicon for the exact delivered commit.
Each host runs the maintained `make ci` gate, including strict Clippy, native
tests, separate core/consumer Rust 1.91 and Wasm checks, dependency graphs,
boundary/pin checks, packaging and contributor-tooling fixtures.

The explicitly selected [live recovery run](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793)
also passed all three hosts on **2026-10-21**, attempt 1, against the release tag.
Each host passed all **nine scenarios** on the original Wasm and Binaryen
`-O3`, `-Os` and `-Oz` variants: **108 successful scenario executions**.
This is live PocketIC evidence for the tested fixture, source, graph and artifacts.

Coverage includes management/Wire admission, real watchdog delivery, upgrades,
overdue policies, synchronous storage/timer rollback, watchdog trap recovery,
lost sender results with retained destination receipts, missing-receipt blocking,
and invalid-record upgrade refusal. Repeated effects remain blocked without
matching destination evidence or explicit disposition. These results do not
establish IC instruction/cycle costs, production deployment or untested consumers.

## Retained evidence

Local `target/qualification/recovery-01714/` preserves all three host artifacts,
job logs, run/job/artifact manifests, the released lock/snapshot, verification
results and the previous handoff. GitHub artifacts have 14-day retention; downloaded
copies retain the tested Wasm, optimizer/server diagnostics and source selections.
Every host's recorded source and lock/snapshot matched the selected release.
Each of the twelve retained Wasm variants matched its recorded hash, and every
per-variant log records nine passes with zero failures.

The released Cargo.lock SHA256 is
`0fb02c5ce5128281f0183b7911fde2c6196bd018fb478fff1b939299201dd6b9`.
Evidence is source-bound; do not transfer it to pending lockfile upgrades.

## Observer adoption and existing tooling

[#56](https://github.com/dragginzgame/ic-jobs/issues/56) is completed in 0.17.14.
Registry `ic-testkit-native` **0.63.2** replaces the local write counter, with
`stable-memory` selected only by the consumer dev-dependency. Exact count checks
reject overflow; shared reset, backing-byte, corruption and zero-write proofs remain.
Normal native and deployed Wasm graphs explicitly exclude the observer.
Published APIs, Wire frames, stored formats and lifecycle authority are unchanged.

Executable selectors request `cli`; library dependencies keep their selected
features. PocketIC's external protocols require transitive Candid; Jobs-owned
RPCs remain Wire. Shared Tooling remains **0.25.9**, revision
`85aded191d85ac4a8c0e5deab207f6be18895862`.
Follow [release preparation](../releasing.md) for direct `origin`/`main` delivery.
Rebuild fixture and harness together for any future explicitly selected live gate.
Preserve retained data and unresolved effects.
