# Current handoff

## Source and selected adoption

Released baseline: `04ae957` (Jobs 0.17.28). The incoming lockfile selects
Build/Host/Testkit 0.68.5, Memory/Storage 0.51.3, Platform/macros 0.6.16,
Timers 0.31.2 and Wire/derive 0.7.12. This batch aligns direct catalog minimums
and preserves the incoming lockfile byte-for-byte. Metrics remains transitive at
0.16.2. The published core remains Serde-only.

Reviewed supplier release sources: Tooling `52457b72`, Persistence `6cb8082`,
Wire `974b6ff`, Platform `0b84c2e` and Timers `50ee4ca`. Later supplier drafts,
including Wire's 0.7.13 notes at `a806197`, are excluded. Tooling simplifies
startup deadlines, cache completion and native tooling; Persistence simplifies
bootstrap/allocation flows. Wire 0.7.11 changes its host TypeScript-generation
API, which Jobs does not call; 0.7.12 strengthens qualification without changing
runtime frames. Jobs' public APIs, endpoint codecs/IDs and stored formats remain
unchanged. No Rust caller change, generated-client update or data migration.

Shared Tooling remains 0.26.16 at
`2077c934efb82b181bea58fe50078dbad289c1aa`; its 118 payloads verify.
Existing tool pins, development Rust 1.99.1 and separate core/consumer Rust 1.91
floors are unchanged. IC tooling selects only `wasm-opt`. The compatible draft
remains 0.17.29; package versions, staging, commits, release delivery and
publication remain maintainer-owned.

## Maintained contracts

The CLI watch loop checks interruption after sleeping and before advancing IC
time. The real process fixture covers quit, EOF, idle Ctrl+C and watch Ctrl+C
without dispatching work after interruption. Its pre-fix reproduction is retained
under `target/qualification/deps-watch-01728/`.

Every decoded Job crosses `Job::restore`; Running/Uncertain work stays blocked
until explicit disposition. Storage, payloads, authentication, lifecycle, timer
integration and effect policy remain consumer-owned. No new persisted formats,
handler registry, timer provider or production dependency is introduced.

Native management calls use Testkit's Wire adapter and one projection of
consumer-owned method names, IDs and request/reply limits. Delivery/management
budgets remain in the consumer modules. Full typed receipt equality, malformed
frame checks, one-pass decoding and directional limits are retained. Jobs has no
generated browser clients and consumes no stable Metrics adapter. Future exact-byte
retries/signatures must retain original frames; successful decode does not prove
identical re-encoding.

## Qualification and evidence

`target/qualification/deps-20261026.zBqNbK/` retains incoming source/lock/catalog/handoff,
focused logs and the final patch/source identities. The evidence binds the
released base, its incoming dependency update and this catalog/documentation
patch. Consumer/composition tests, strict native/Wasm Clippy, separate
core/consumer Rust 1.91 native/Wasm checks, dependency graphs/boundaries,
pins/snapshot, formatting, documentation links and release/consumer tooling
fixtures pass. Lock-selected release/Testkit CLIs pass offline admission.
The consumer Wasm graph excludes Platform's `wire-schema` and `pocket-ic` features.

The explicit Linux live run passes the queue demo, all four CLI shutdown modes
and all nine original-Wasm recovery scenarios. Its retained recovery directory
is copied into this qualification directory as `recovery-evidence/`; the
installed Wasm hash is
`a0612f050f0a3fd6f84e277f54c1869dc81ffb2a577eeb0a9b4af09cb283c42b`.
Full CI, optimized recovery variants and fresh macOS qualification were not run.

The prior Tooling 0.68.3 graph is retained under
`target/qualification/deps-01729/`, including passing focused tests, separate
core/consumer Rust 1.91 native/Wasm checks, Clippy, tooling fixtures and Linux
live queue/CLI/recovery evidence. Its installed Wasm hash was
`23801bf701f5989216afe7784f1deddfd27c45456316c70b644f61000161aec9`.

Earlier evidence remains under `target/qualification/deps-watch-01728/` and
`target/qualification/native-cleanup-metrics-wire/`; failed attempts, original
artifacts, release plans and unresolved effects remain retained.

The last recorded three-host live recovery run qualified Jobs 0.17.14 at
`7d1f808de0bc14d823bd1d3f04a5a62f06ad7a52` with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
Its artifacts remain in `target/qualification/recovery-01714/`. Linux compilation
and native tests alone do not qualify fresh macOS or installed-IC recovery.
