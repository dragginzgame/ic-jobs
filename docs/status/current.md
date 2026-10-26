# Current handoff

## Selected adoption

Released baseline: `90f51ee` (Jobs 0.17.29). The incoming lockfile selects
Build/Host/Testkit 0.68.7, Timers 0.31.4, Wire/derive 0.7.16 and Platform/macros
0.6.18. Direct catalog minimums now match those selections; the incoming
lockfile is preserved byte-for-byte. Memory/Storage remain 0.51.3 and transitive
Metrics 0.16.2. Own package versions remain 0.17.29.

Reviewed release sources: Tooling `7ae301d00353e1e27e94300d492e47f9736941c3`,
Timers `b04401c713d464a38d700af9b68a8fd437acda82`, Wire `27304e5` and Platform
`98a2a8a`. Tooling retains successful Cargo completion admission and now shares
final startup deadline/server checks across Testkit constructors. Timers' library
source matches 0.31.2. Wire's u16/i16 decode optimization preserves frame bytes,
limits and malformed-varint rejection. Platform's optional BLAKE3 feature and
browser SDK are not consumed here. No Jobs caller adaptation was required.

Shared Tooling 0.26.20 at `a7f85d45d4212d4c39225f58597580560a333550` is adopted
through canonical impact/refresh export; its 120 payloads verify. Specialist
rules accompany the compact baseline. README/Make help distinguish focused
commands from aggregate gates. Detailed change accounting is opt-in. The new
npm boundary is inapplicable to this Rust-only consumer; its linked Node guide
is explicitly included so local documentation links remain complete.

The incoming 0.26.19 release changes remain: normal release targets start fresh
from clean committed metadata; only `release-resume VERSION=X.Y.Z` recovers saved
intent. Historical plans do not block fresh releases, stale candidate receipts
are cleared under the lock, and archived changelog Draft labels remain intact.
Jobs retains saved-lock recovery and refusal before source/index, gate or
preparation effects. Make help now describes explicit resume accurately.

Existing tool pins, development Rust 1.99.1, separate core/consumer Rust 1.91
floors and the optimizer-only IC selection are unchanged. The compatible next
draft is 0.17.30: public library APIs, endpoint codecs and stored formats are
unchanged. No client regeneration or retained-data migration is required.
Staging, commits, release execution and publication remain maintainer-owned.

## Maintained contracts

The published core remains Serde-only. Every decoded Job crosses `Job::restore`;
Running/Uncertain work stays blocked until explicit disposition. Storage,
payloads, authentication, lifecycle, timer integration and effect policy remain
consumer-owned. No production dependency, handler registry or timer provider.

The CLI watch loop checks interruption after sleeping and before advancing IC
time. Native Wire callers retain one projection of consumer-owned method IDs,
names and directional limits. Full receipt equality, malformed-frame refusal
and one-pass decoding are unchanged. No browser clients or stable Metrics adapter
are consumed. Exact-byte retries/signatures must retain original frames.

## Qualification and evidence

`/tmp/ic-jobs-adoption.2n_gbkqr/` retains the full incoming tracked source,
lockfile, dirty diff/status, Shared impact/refresh output and focused logs. This
separates the new adoption from incoming release-adapter work. Linux native
consumer tests pass (29 library, 2 codec and 19 composition cases); native harness
checks pass with explicit live cases ignored. Consumer/harness native Clippy,
consumer Wasm Clippy, and separate core/consumer Rust 1.91 native/Wasm checks pass.
Pins, dependency boundaries/graphs, release-command routing, formatting and local
links pass. Lock-selected Build/Testkit 0.68.7 tools are prepared and admitted.
Formatting-failure evidence retention passes. Jobs' release/publication and
consumer-tooling fixtures pass for this selected graph, including explicit
saved-lock recovery, selected-source metadata checks and refusal before tool,
build or server effects. The selected Testkit CLI admits the prepared server
offline; no server was launched.
The initial missing Node-guide link and sandbox-denied tool setup logs remain
retained; canonical refresh and authorized shared-store setup resolved them.

Earlier evidence is retained at `/tmp/ic-jobs-tooling-wire-shared.1SRGqz/`, with
its original handoff copied into the current evidence directory's
`incoming/docs/status/current.md`. That earlier batch selected Tooling 0.68.6,
Timers 0.31.2, Platform 0.6.17 and Wire 0.7.15; its Linux live queue demo, four CLI
shutdown modes and nine recovery scenarios passed. The installed module hash was
`0a7d5f3b6712c482953d5118cd6ec00d1e503e0629ab7232282bf41e16b9ff7f`.
Those observations do not qualify the newly selected dependency graph.
Full CI, fresh live PocketIC/optimized recovery and macOS qualification have not
been selected for this adoption. Live qualification retains its separate
explicit selection; native checks do not establish IC recovery.

The last recorded three-host live recovery run qualified Jobs 0.17.14 at
`7d1f808de0bc14d823bd1d3f04a5a62f06ad7a52` with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
