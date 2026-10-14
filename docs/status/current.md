# Current handoff

## Source and pending work

Delivered Jobs **0.9.5**: `7f6edbe157ee7a7ee8617a1d57309a408af8d8bd`.
The working tree prepares **0.10.0**. `Scheduler::start_next` now returns
`(index, attempt, running_record)`: update tuple destructuring. The index belongs
only to the supplied batch while its order is unchanged; persist job identity
and the running record as before. Both maintained consumers use that selected
index directly instead of searching again. Stored records and effect-disposition
rules are unchanged; no storage reset is required.

The preceding native fixture cleanup remains included: notification fixtures
keep one validated job-state collection and borrow payloads for commit;
maintenance loads once but commits each occurrence separately. General Memory
contract tests stay with published IC Memory; Jobs retains stable-cell reopen,
blocked-effect and partial-commit recovery coverage.

Local lockfile identities and expected versions share one admission parse.
Disposable release fixtures reject wrong local-row versions at that boundary.
Optimized recovery producers are removed only after successful exact capture;
failed capture retains producer bytes and the frozen original. Qualified bytes,
hashes and optimizer/server logs remain retained.

Incoming IC Build 0.3 and Testkit 0.39 manifest/lock selections are preserved.
Own package versions, staging, Git refs, release and publication remain
maintainer-owned. No release or upload ran.

## Shared adoption

Shared Tooling **0.12.5** is pinned to
`4256171b5b7ec4c4cc4078548c1c1617ee150699`, verified against upstream main and
canonically exported from a clean exact-source clone. The snapshot has 93 files;
existing roots and tool versions/digests are preserved. Common setup and offline
admission pass. The public repository description remains accurate.

AGENTS links to the shared validation cadence: focused checks qualify ordinary
handoff; full suites retain explicit local selection or configured CI. Its
explicit sibling read-only restriction remains in force. The local toolchain
action remains consumer-owned pending
[Shared #177](https://github.com/dragginzgame/shared-tooling/issues/177).

## Focused proof

Evidence: `target/qualification/shared-0125/` and
`target/qualification/scheduler-index/`. Rust 1.88 core and timers tests pass
separately, including 19 native consumer scenarios, frozen records and transition
histories. Scheduler tests verify selected indices in deadline/tie order and
unchanged refusal behavior. All 17 stable-consumer tests, release Wasm, capture
tests, harness compilation, native/Wasm minimum checks, strict Clippy and
rustdoc pass against the selected graph.

Formatting-hook/evidence, consumer-tooling and disposable release/publication
fixtures pass. Consumer/release fixtures also pass under genuine Bash 3.2,
including child shells. Success retires duplicate optimized producers; injected
capture failure retains them and refuses optimized server dispatch. Wrong local
lock versions refuse without changing the selected graph. Tool, registry,
optimizer/server and complete-release gates are substituted in those fixtures.
ShellCheck, Actionlint, formatting, pins, snapshot integrity and documentation
links pass. Incoming Cargo.toml/Cargo.lock remain byte-identical to task entry.

## Qualification boundaries

Full `make ci` and live PocketIC recovery were not selected. These local Linux
checks do not establish the required macOS Intel/Apple Silicon matrix or live IC
recovery for this source/graph. The old delivered 0.9.3 Host 0.17.7 failures are
historical evidence, not a qualification of the currently selected graph.

Current issue acceptance remains with
[#41](https://github.com/dragginzgame/ic-jobs/issues/41),
[#42](https://github.com/dragginzgame/ic-jobs/issues/42),
[#43](https://github.com/dragginzgame/ic-jobs/issues/43), and
[#44](https://github.com/dragginzgame/ic-jobs/issues/44).
Last inspected live recovery remains Jobs 0.8.1 source
`f9c0c0eb4e8363565be63f9434a0cf8b9940294b`,
[run 38289182820](https://github.com/dragginzgame/ic-jobs/actions/runs/38289182820):
84 passes across seven scenarios, four Wasm variants and three hosts.
It does not qualify the pending cut.

See [frozen history](history-through-0.9.1.md), [architecture](../../README.md),
[safety](../../SAFETY.md), [consumer qualification](../../apps/job-consumer/README.md),
and [delivery procedure](../releasing.md).
