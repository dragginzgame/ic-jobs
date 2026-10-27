# Current handoff

## Selected adoption

Released baseline: `f79db55` (Jobs 0.17.30); local HEAD remains `b3aaae5` after
changelog-only PR [#74 — Use IcyDB-style sections in the latest root changelog](https://github.com/dragginzgame/ic-jobs/pull/74).
The compatible next draft remains 0.17.31. Own package versions stay 0.17.30.

The incoming lockfile selects Build/Host/Testkit 0.68.9, Memory/Storage 0.51.5,
Timers 0.31.5, Platform/macros 0.6.19 and Wire/derive 0.7.18. Direct minimums now
match those selections. Metrics is transitive at 0.16.3; the previous handoff's
0.16.2 description was stale. The incoming lockfile, including rustix 1.1.6 and
openssl-probe 0.2.2, is preserved byte-for-byte.

Reviewed new supplier release sources: Tooling
`a748463715f77d83029c22654ffad8c75a585187`, Persistence
`e51d6588f889ac491f7177221c0bcf2a17f6f6ec` and Wire
`c748379325266d7fa614fefee9d745eb4a88acca`. Consumed library implementations match
0.68.8, 0.51.4 and 0.7.17 respectively; changes adopt supplier dependencies and
governance. Timers and Platform selections are unchanged. Wire's uncommitted
0.7.19 allocation documentation/tests remain excluded. No Jobs caller changes,
client regeneration or retained-data migration are required.

The pending Shared Tooling 0.26.21 adoption at
`66c91c4412856baa147482e648fc80428db8b179` is preserved; all 120 payloads verify.
Current/future root notes retain emoji category sections, with older published
entries unchanged. Existing tool pins, development Rust 1.99.1, separate
core/consumer Rust 1.91 floors and optimizer-only IC selection remain unchanged.
Staging, commits, release execution and publication remain maintainer-owned.

## Accepted cleanup

The ranked review's first nine changes are implemented: receiver installation is
explicit in the three delivery scenarios; restore admission delegates to three
ordered private validators; transition exploration skips codec work for already
visited records; CLI watch carries only its preceding step's views; receipt
confirmation borrows payloads; CLI RPC helpers name query/update explicitly;
release fixtures expose five independent groups; validated snapshots project
deadlines through one helper; duration parsing/display share their unit table.
The bounded 32-job ID lookup remains linear pending a measured need, as recommended.

The release-fixture aggregate still selects all groups and the existing
publication eligibility fixture. Focused targets are documented in README.
Public APIs, RPC/stored formats, error precedence and uncertain-effect blocking
are unchanged. The current draft remains 0.17.31; no release effects are selected.

## Maintained contracts

The published core remains Serde-only. Every decoded Job crosses `Job::restore`;
Running/Uncertain work stays blocked until explicit disposition. Storage,
payloads, authentication, lifecycle, timer integration and effect policy remain
consumer-owned. No production dependency, handler registry or timer provider.

The CLI watch loop checks interruption after sleeping and before advancing IC
time. Native Wire callers retain one projection of consumer-owned method IDs,
names and directional limits. Full receipt equality, malformed-frame refusal
and one-pass decoding are unchanged. No browser clients, optional BLAKE3 or
stable Metrics adapter are consumed. Exact-byte retries/signatures retain frames.
Normal release targets start fresh; only explicit `release-resume` recovers a
saved attempt. Metadata changes do not authorize release or publication effects.

## Qualification and evidence

`/tmp/ic-jobs-simplify.c9lx1yg4/` retains incoming source/diff, focused logs and
before/after consumer Wasm. A token comparison proves every restore check remains
in its original order within the history/progression/state phases. Core tests,
consumer library/codec/composition tests and native harness/CLI checks pass;
live cases remain ignored. Native/Wasm Clippy and separate core/consumer Rust
1.91 qualifications pass. ShellCheck 0.11.0 with sourced helpers, shell syntax,
formatting, documentation links and release-command routing pass.

Each new release-fixture group passes independently: retention, delivery,
recovery, metadata and publication. Effects stay in disposable Git repositories,
with substituted complete gates and registry/upload effects. All prior scenario
bodies remain intact; the default aggregate selects them all. The initial
ShellCheck invocation omitted sourced helpers; the corrected `-x` run passes.

With the same lock, Rust 1.91 and release build settings, raw qualification Wasm
falls from 949,475 to 942,978 bytes; its code section falls from 705,209 to 698,794.
These are artifact-size observations, not IC execution or throughput measurements.
The native history test's observed execution times were 0.22 s before and 0.12 s
after; one pair is not a performance benchmark. Live fixture setup is reduced
from 18 to 12 canister installations per nine-scenario source roster, by inspection.
The live demo now includes two normal watch steps when explicitly selected.

Dependency-adoption evidence remains at `/tmp/ic-jobs-deps-20261027.7jaw3_x2/`;
its handoff is preserved in this batch's `incoming/docs/status/current.md`.
The incoming lock, catalogs and Shared snapshot remain byte-identical. No source
was staged or committed and no real-checkout release/publication was executed.

Earlier Linux live recovery evidence remains attributed to Tooling 0.68.6,
Timers 0.31.2, Platform 0.6.17 and Wire 0.7.15 in
`/tmp/ic-jobs-tooling-wire-shared.1SRGqz/`; it does not qualify this selected graph.
Full CI, fresh live/optimized recovery and fresh macOS qualification have not
been selected. Native checks do not establish IC rollback or upgrade recovery.

The last recorded three-host live recovery run qualified Jobs 0.17.14 at
`7d1f808de0bc14d823bd1d3f04a5a62f06ad7a52` with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
