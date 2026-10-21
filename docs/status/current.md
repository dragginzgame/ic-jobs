# Current handoff

## Accepted scope and source

Delivered Jobs **0.17.12** is `a024cc9`; workspace metadata remains 0.17.12.
Incoming lock updates select Memory/Storage **0.50.5**, Metrics **0.15.1**,
Platform/macros **0.6.2**, Timers **0.30.1** and Wire/derive **0.5.9**.
This task preserves that incoming lockfile byte-for-byte. Package versions,
commits, releases and crates.io publication remain user-owned.

The published library retains its Serde-only API and Rust **1.91** floor. Core
owns checked records, transitions and finite-batch selection. Consumers own
storage, payloads, endpoints, lifecycle, authentication and effect disposition.
Every decoded record crosses `Job::restore`; Running/Uncertain work stays blocked
until explicit reconciliation. Error precedence and commit-before-dispatch remain.
The [root ownership map](../../README.md#scope-and-validation) is authoritative.

## Native IO observer preparation

[#56](https://github.com/dragginzgame/ic-jobs/issues/56) now tracks the consolidated
producer [Tooling #227](https://github.com/dragginzgame/ic-tooling/issues/227).
Upstream prerequisite work was explicitly authorized in isolated checkouts.
The observer was already being implemented in the Tooling checkout; qualification
reused an isolated copy and left sibling checkouts untouched. Its tested source
now matches committed Tooling `14a04da07c35abf3c2a0bddcc03c6b0bc51ab95c`.
Platform **0.6.2 already publishes** the required stable-structures boundary;
no additional Platform implementation is needed.

The source-only Jobs patch replaces `CountedMemory` with the opt-in native
`MeteredMemory`, preserving shared clones/reset, frozen ledgers, full backing
bytes, corruption refusal and zero-write assertions. Every count consumer rejects
overflow. The feature remains dev-only; both normal consumer graphs exclude it.
This preparation changes no production API, Wire frame, stored format or lifecycle.

The registry's latest `ic-testkit-native` is still **0.63.0**, with no observer
feature. Keep the active wrapper and dependency graph until the supplying release
is published and registry-qualified. The temporary external path correctly fails
the ordinary dependency-pin gate; no active path or exception was added.
Use the actual first supplying registry minimum when adopting, not an assumed
release number. Issue #56 contains the current source patch and remaining gates.

## Validation and limits

The isolated adoption passes **49 consumer tests** under Rust 1.91: 28 library,
two API Wire and 19 composition tests. Consumer native/Wasm strict Clippy,
separate core/native/Wasm MSRV checks, the five existing product dependency graph
contexts, explicit observer exclusion from both normal consumer graphs and direct
dependency boundaries pass. The incoming lock selections are preserved.

The producer snapshot passes four IO tests and two allocation tests under Rust
1.91, strict Clippy and Wasm feature checking. Its default native and opt-in Wasm
normal/build closures remain dependency-free. Native IO selects Platform 0.6.2
and stable-structures 0.7.2, sharing Memory/Storage's trait identity. Producer tests
cover actual overflow, specialized unsafe reads, refused growth and shared reset.

These are Linux/source-fixture results. They do not establish registry adoption,
native macOS support for the new source or IC callback/trap/upgrade/recovery
behavior. No full CI, live PocketIC qualification, release or publication ran.
The active checkout received documentation changes only; source replacements
and qualification-only manifests remain outside its maintained source.

Evidence: `target/qualification/native-memory-observer/` preserves the previous
handoff, source patch, producer archive/hashes, registry index, incoming and fixture
locks, graph observations and focused logs. Disposable source checkouts remain at
`/tmp/ic-jobs-issue-56.0kqmrg_e/qualification/`. Earlier Tooling 0.63 qualification
at `target/qualification/tooling-063/` remains bound to its older locked inputs.

## Existing tooling and next action

Build/Host/Testkit remain published **0.63.0**; executable selectors request
`cli`, while library dependencies retain their existing feature selections.
PocketIC's external protocols still require Candid; Jobs-owned RPCs remain Wire.
Timers 0.30.1 now shares Platform 0.6.2 with the fixture. Shared Tooling remains
**0.25.9**, revision `85aded191d85ac4a8c0e5deab207f6be18895862`.

Complete the published-observer gate on #56 before applying the replacement.
Follow [release preparation](../releasing.md) for direct `origin`/`main` delivery.
Rebuild fixture and harness together before explicitly selecting live IC
qualification. Preserve retained data and unresolved effects.
