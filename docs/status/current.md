# Current handoff

## Accepted scope

Released baseline: `5be954b1704a45c9fd98d9ab6822bd92ff68970c` (Jobs 0.17.35).
Shared Tooling 0.26.25 is adopted locally and uncommitted. The compatible
changelog draft remains 0.17.36; package versions remain 0.17.35. Staging,
commits, releases and publication remain maintainer-owned.

The reviewed source is `6b6bbd9c9e5798f0ecf6105637392a7a55cd450e`. Canonical
preview/refresh retained the same entrypoints and 121-file payload closure.
The final export used a clean pinned temporary clone because the sibling had
unrelated edits in progress. No sibling working files or snapshot-owned payloads
were edited locally. The earlier description review remains applicable to Jobs'
unchanged purpose; its observation is retained with the preceding adoption.

The shared hook accepts opaque gitlinks while preserving regular-file selection,
partial-stage refusal and child-checkout custody. The formatter supports optional
explicit standalone files/editions, without caching those selections. Jobs has no
standalone adapter or runtime-scanner caller to migrate, so no additional source
roster, Clap dependency, Node setup or generic upstream test suite was introduced.
The Node guide was already selected; its new companion declaration is now retained.

Earlier uncommitted 0.26.23 caller migration remains intact: standard formatting
reports per-pass progress/timings; the local hook fixture includes the driver;
formatting-evidence assertions retain exact failing status, complete diagnostic
logs and archive comparisons. Jobs' explicit module paths conservatively disable
cache reuse. The real fmt-check still executes manifest sorting and Rustfmt.
The evidence fixture forces all passes with FORMAT_RUST_ARGS=--no-cache. Staged
hooks stay uncached, and grouped manifest/member coverage is unchanged.

## Maintained contracts and incoming work

No application Rust source, manifests, feature selections, tool pins or CI
workflow definitions changed. The core remains Serde-only with separate Rust 1.91
core/consumer qualification. Job::restore, blocked uncertainty, exact receipts,
error ordering and consumer-owned storage/timers/lifecycle remain unchanged.
No client regeneration or retained-data migration is needed. Normal releases
start fresh; only release-resume selects saved intent. The queue remains bounded.

Incoming Memory/Storage 0.51.10 lockfile changes are preserved byte for byte and
were not given fresh native/Wasm qualification here. Workspace minimums remain
0.51.9. Build/Host/Testkit remain 0.69.0, Timers 0.31.9, Platform/macros 0.7.0,
Wire/derive 0.7.18 and Metrics 0.16.3. The Platform singleton blocker is resolved.

## Qualification and evidence

`/tmp/ic-jobs-shared-02625.5rE0846k/` retains incoming changes, source review,
initial/corrected previews and refreshes, focused logs, final patch and identities.
The initial source commit 16efc62 was labelled 0.26.25 but still advertised
VERSION 0.26.24. Upstream corrected only VERSION in 6b6bbd9; all selected payload
bytes stayed identical. Passing hook/formatter checks remain applicable to those
unchanged bytes. The final 0.26.25 snapshot was refreshed and verified separately.

All 121 payloads verify. Bash syntax, ShellCheck and Perl syntax pass. Actual
Jobs formatting/hook qualification passes in disposable repositories, preserving
Rust/manifest repair, staged refresh, partial-stage refusal, failed-formatter
preservation and installer assertions. The default fmt-check passes with prepared
tools and its conservative uncached fallback. Formatting evidence checks preserve
parallel jobserver/offline behavior, sorter failure before Rustfmt, full retained
logs and actual CI collector archive contents with substitute tool effects.
Release-command routing and documentation checks pass. No formatting write ran
on this checkout; no Rust build/tests, full CI, live PocketIC or native macOS
qualification was selected for this formatting/governance change.

The earlier 0.26.23 adoption and its release-fixture evidence remain at
`/tmp/ic-jobs-shared-02623.HqRXdvzH/`. Runtime qualification at
`/tmp/ic-jobs-tooling-069.LLBQDe8r/` covers Tooling 0.69, Platform 0.7,
Memory/Storage 0.51.9 and Timers 0.31.9, not the incoming 0.51.10 graph or installed
IC recovery. Earlier evidence remains at `/tmp/ic-jobs-dependency-adoption.HHZ153GK/`
and the cleanup paths recorded in released handoffs. Linux live evidence at
`/tmp/ic-jobs-tooling-wire-shared.1SRGqz/` uses older inputs. The last recorded
three-host live run qualified Jobs 0.17.14 with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
