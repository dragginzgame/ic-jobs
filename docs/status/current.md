# Current handoff

## Source and selected adoption

Released baseline: `647dd65` (Jobs 0.17.27). The incoming lockfile selects Build/
Host/Testkit 0.68.2, Memory/Storage 0.51.1, transitive Metrics 0.16.2, Timers
0.31.1 and Wire/derive 0.7.9. This batch raises the direct catalog minimums to
those reviewed published releases and preserves the incoming lock byte-for-byte.
Platform/macros remain 0.6.15; Platform's pending 0.6.16 source is not adopted.
Supplier release sources reviewed: Tooling `4357e0ab`, Persistence `fa5cfd7`,
Metrics `0d60020`, Timers `6dd07a1` and Wire `00b1686`.

Shared Tooling 0.26.16 at `2077c934efb82b181bea58fe50078dbad289c1aa` is adopted
through its clean-source impact/refresh exporter. Its 118 payloads verify.
Busy-tool admission is distinct from missing setup; no other owner's locks or
retained generations are removed. Existing tool pins, development Rust 1.99.1
and separate core/consumer Rust 1.91 floors are unchanged. IC tooling still
selects only `wasm-opt`. The compatible draft is 0.17.28; package versions,
staging, commits, release delivery and publication remain maintainer-owned.

## Local improvement and maintained contracts

The CLI's watch loop now checks interruption after sleeping and before advancing
IC time. A watch-start message confirms the loop was entered; the real process
fixture queues a job due at the first advance, interrupts that wait, and checks
that no dispatch occurred. Quit, EOF and idle interruption remain covered.
The initial regression failure and final focused evidence are retained below.

The published core stays Serde-only. Every decoded Job crosses `Job::restore`;
Running/Uncertain work stays blocked until explicit disposition. Storage,
payloads, authentication, lifecycle, timer integration and effect policy remain
consumer-owned. No new persisted formats, handler registry or timer provider.

Native management calls still use Testkit's Wire adapter and one projection of
consumer-owned method names, IDs and request/reply limits. Delivery/management
budgets remain in the consumer modules; no generated browser clients exist.
Shared's new cross-runtime budget rule therefore adds no browser generation or
second policy here. Full typed receipt equality, raw malformed-frame tests,
Wire's one-pass decoder and explicit directional limits are unchanged. Future
exact-byte retries/signatures must retain original frames; decode success does
not prove identical re-encoding. Metrics remains transitive and no stable adapter
is consumed by Jobs. Current DTOs and stored JSON require no migration/reset.

## Qualification and evidence

`target/qualification/deps-watch-01728/` retains starting source/lock/snapshot,
prior handoff, exporter impact/refresh output, focused logs and final identities.
These bind the released baseline above, incoming dependency selections and the
current local patch. Native consumer/Wire, capture, parser, composition and
memory-observer tests pass. Separate core/consumer Rust 1.91/Wasm checks,
native/Wasm strict Clippy, all five graph contexts, dependency boundaries/pins,
snapshot integrity, formatting/hooks, release routing, documentation links and
release/publication/consumer/evidence fixtures pass.

The pre-fix CLI fails the new live watch test with an observed dispatch after
Ctrl+C. Its executable, source and hash remain in this qualification directory.
The final explicit Linux `make test-demo test-demo-cli test-canister` passes:
seconds/minutes/weeks and recurrence management, four shutdown modes including
active-watch interruption with no dispatch, and all nine original-Wasm recovery
scenarios. The installed Wasm SHA256 is
`a6bb47086ac9ebfa796c0097492e9e2dc82f64ac093faf94fbaf1ef6d141364a`.
Recovery evidence is retained at `/tmp/jobs-recovery.oWUrxD` and copied here.

Both refreshed CLIs pass offline admission. The first Testkit setup wait expired
while another checkout owned its slot; a supported bounded retry reused the
completed tool without cleanup or interference. The timeout and pre-fix test
failure remain retained. Full CI, optimizer variants and fresh native macOS
qualification were not run; historical host results do not establish them.

Prior Linux native/live evidence remains under
`target/qualification/native-cleanup-metrics-wire/`: the queue demo, three CLI
shutdown modes and all nine original-Wasm recovery cases passed for the preceding
graph. Its recovery artifact hash is
`bd6c9884c935827556e00975d9f09f922c5778967997b0a98973762ce596e55f`.
That result does not qualify new dependency selections. Earlier qualification
directories, failed attempts, release plans and unresolved effects stay retained.

The last recorded three-host live recovery run qualified Jobs 0.17.14 at
`7d1f808de0bc14d823bd1d3f04a5a62f06ad7a52` with Tooling 0.63.2:
[108 successful scenario executions](https://github.com/dragginzgame/ic-jobs/actions/runs/39618054793).
Its artifacts remain in `target/qualification/recovery-01714/`. Linux compilation
and native tests alone do not qualify fresh macOS or installed-IC recovery.
