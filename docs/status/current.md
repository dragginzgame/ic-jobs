# Current handoff

## Accepted scope and source

Delivered Jobs **0.17.13** is `a734659`; workspace metadata remains 0.17.13.
Pending **0.17.14** adopts the published Testkit native IO observer for #56.
Incoming Memory/Storage **0.50.6** and Wire/derive **0.5.12** lock updates are
preserved and included in qualification. Existing Platform/macros **0.6.3**,
Timers **0.30.1** and Metrics **0.15.1** selections remain.
Source and notes are prepared; staging, commits, package versions, release
execution and crates.io publication remain user-owned.

The published library retains its Serde-only API and Rust **1.91** floor. Core
owns checked records, transitions and finite-batch selection. Consumers own
storage, payloads, endpoints, lifecycle, authentication and effect disposition.
Every decoded record crosses `Job::restore`; Running/Uncertain work stays blocked
until explicit reconciliation. Error precedence and commit-before-dispatch remain.
The [root ownership map](../../README.md#scope-and-validation) is authoritative.

## Published native IO observer adoption

[#56](https://github.com/dragginzgame/ic-jobs/issues/56) now uses registry
`ic-testkit-native` **0.63.2**, with its actual supplying minimum in the root
catalog and `stable-memory` selected only by the consumer dev-dependency.
The supplying archive is non-yanked, declares Rust 1.91 and records release source
`e5b4095150f49069fc51c3b497e192a8f8eda016`. Its SHA256 matches the canonical index:
`b8948c336c3785c5dbd9251a5086779757f3010048684287705669dbf1bfc6ee`.
Platform's published stable-structures feature supplies the shared Memory trait;
no new Platform implementation or external path exception is needed.

The local write-counter wrapper is removed. Canonical shared clones/reset,
frozen ledgers, full backing-byte checks, corruption refusal and zero-write
assertions remain; count consumers reject overflow before reporting exact writes.
Normal native and deployed Wasm graphs explicitly exclude the observer.
Published APIs, Wire frames, stored formats and lifecycle policy are unchanged.

## Validation and evidence

Linux passes **49 consumer tests** under Rust 1.91: 28 library, two API Wire and
19 composition tests, against the actual registry package. All three existing
strict Clippy commands pass, compiling native harness/capture callers as well
as core, native consumer and Wasm code. Separate core/consumer Rust 1.91 native
and Wasm checks, all five product dependency graph contexts, direct dependency
boundaries and pin checks pass. The lock delta beyond incoming work adds only
the observer and its selected Platform substrate edge.

Upstream archive qualification covers all four IO tests and two allocator tests,
including shared reset, refused growth, specialized unsafe reads and actual
overflow. Its recorded registry evidence is linked from
[Tooling #227](https://github.com/dragginzgame/ic-tooling/issues/227).
These are native IO observations, not IC instruction or cycle costs.

The exact producer release's browser, optimizer and PocketIC runtime matrices
passed. Its native Intel CI setup failed on a Rust download DNS error; required
native qualification remains tracked by
[Tooling #230](https://github.com/dragginzgame/ic-tooling/issues/230).
Linux checks here establish neither native macOS acceptance nor IC callback,
trap, upgrade or uncertain-effect recovery behavior. No local full CI, live
PocketIC qualification, release or publication ran in this task.

Evidence: `target/qualification/native-memory-observer-0632/` retains the starting
tree archive, previous handoff, incoming diff/lock, registry provenance, source
hashes and focused logs. Prior path-only rehearsal is historical preparation;
this batch selects a checksum-authenticated registry dependency.

## Delivery and existing tooling

Keep #56 open until the adoption is delivered and its required native acceptance
is verified. Reconcile it on the next reported push; do not treat publication
alone as consumer adoption or close Tooling's separate qualification tracker.

Other Build/Host/Testkit selections remain **0.63.0**; executable selectors request
`cli`, while library dependencies retain their existing feature selections.
PocketIC's external protocols still require Candid; Jobs-owned RPCs remain Wire.
Timers 0.30.1 shares Platform 0.6.3 with the fixture. Shared Tooling remains
**0.25.9**, revision `85aded191d85ac4a8c0e5deab207f6be18895862`.
Follow [release preparation](../releasing.md) for direct `origin`/`main` delivery.
Rebuild fixture and harness together before explicitly selecting live IC
qualification. Preserve retained data and unresolved effects.
