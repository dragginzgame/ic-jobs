# Current handoff

## Source and pending work

Delivered Jobs **0.10.0**: `d01f868f176469d94014d462c0e518c06ea010b4`.
The working tree prepares compatible **0.10.1** tooling cleanup. Jobs delegates
fixture exit injection and retained-evidence assertions to the canonical Shared
helper; its actual fixture roster and archive/command assertions remain local.
Release metadata preparation, rollback and selected-source checks now delegate
to Shared Tooling; Jobs retains direct-delivery admission and exact tool selection.
Recovery fixtures use Testkit's nanosecond clock extension and retain its 0.40
instance owner rather than a bare PocketIC client.
Delivered `Scheduler::start_next` returns
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

Incoming IC Memory 0.37, IC Build 0.3 and Testkit 0.40 manifest/lock selections
are preserved, including the selected Timers 0.22.3 lock entry.
Own package versions, staging, Git refs, release and publication remain
maintainer-owned. No release or upload ran.

## Shared adoption

Shared Tooling **0.12.7** is pinned to
`9364ffe3f770a6d5d94809f0bf82c33202054bf8`, verified against upstream main and
canonically exported from a clean exact-source clone. The snapshot has 100 files
after adding the canonical Cargo release-metadata entrypoint;
existing roots and tool versions/digests are preserved. Previously prepared tool
selections remain admitted; snapshot and pin checks pass. The public repository
description remains accurate.

AGENTS links to the shared validation cadence: focused checks qualify ordinary
handoff; full suites retain explicit local selection or configured CI. Its
explicit sibling read-only restriction remains in force. The local toolchain
action remains consumer-owned pending
[Shared #177](https://github.com/dragginzgame/shared-tooling/issues/177).

## Focused proof

Latest cleanup evidence: `target/qualification/metadata-owner/`. Disposable
release/publication fixtures pass on Bash 5 and genuine Bash 3.2, including
patch/minor/major preparation, rollback, retained selected-source failures and
resume checks. Both Rust 1.88 native/Wasm feature qualifications pass separately;
19 native consumer scenarios and all 17 stable-consumer tests pass against the
incoming graph. Release Wasm, two capture tests, recovery harness compilation,
strict native/Wasm Clippy, formatting, ShellCheck, snapshot, pin and documentation
checks pass. The initial harness compilation failure exposed Testkit 0.40's
removed builder extension; it is repaired by retaining the published instance
owner. No live recovery ran. Incoming Cargo.toml/Cargo.lock remain byte-identical
to task entry.

The capture-finalization proposal is implemented only as an isolated IC Build
0.3.0 draft, tracked in
[IC Build #26](https://github.com/dragginzgame/ic-build/issues/26).
`CapturedArtifact::retain_to` preserves exact bytes and reports original or
destination custody on rename/cleanup failure. Nine focused staging tests,
strict Clippy and Rust 1.88 rustdoc pass; the proposed Jobs caller passes its two
capture tests in a disposable package. Reviewable upstream and caller patches
are retained in `target/qualification/metadata-owner/`. Active Jobs capture
finalization stays local until the owner API is reviewed and published. Sibling
working trees were untouched; no upstream commit, PR or publication ran.

Latest tooling evidence: `target/qualification/fixture-probes/`. The expanded
entrypoint closure and pins pass. Disposable release/publication fixtures pass
on Bash 5 and genuine Bash 3.2, including function nounset, failed cleanup,
premature exits and contradictory command assertions. Archive round trips stay
consumer-owned. ShellCheck and documentation checks pass. That earlier fixture-probe follow-up
did not change product Rust, Cargo selections or release versions.

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

Delivered 0.10.0's [CI 38549616375](https://github.com/dragginzgame/ic-jobs/actions/runs/38549616375)
has passed Linux; both native macOS jobs remain queued at inspection. Issues
#41–#44 have updated delivered-source evidence and remain open for that matrix.

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
