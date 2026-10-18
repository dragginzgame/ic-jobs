# Current handoff

## Source and pending work

Delivered Jobs **0.17.2**: `177a4dd4aa9a22492830839fec0ea6e42c5d234c`.
Pending **0.17.3** adopts published Timers **0.28.1**'s completed-batch projection
in the qualification canister and native maintenance consumer
([#57](https://github.com/dragginzgame/ic-jobs/issues/57)), and selects development
Rust **1.99.1** in the local toolchain and both workflows/evidence
([#69](https://github.com/dragginzgame/ic-jobs/issues/69)). Work/deadline policies
and trap-before-delivery ordering remain consumer-owned. Core and consumer MSRV
selectors remain **1.91**, with separate native/Wasm qualification. Public APIs,
package versions, locks and wire/storage formats are unchanged. No release,
upload, reset, conversion or deployment is selected.

Linux Rust 1.99.1 qualification passes 48 core, 46 consumer/composition/wire,
four capture/recovery-decoder tests and six doctests. Rust 1.91 native/Wasm
checks, the release Wasm build, strict native/Wasm Clippy, formatting/hook/evidence
fixtures, all four graph contexts, dependency boundaries/pins and links pass.
The seven live recovery tests remain ignored; hosted native acceptance is separate.

Delivered 0.17.2's complete Linux/Apple Silicon/Intel macOS native matrix now passes
on its exact source in [CI run 39139517213](https://github.com/dragginzgame/ic-jobs/actions/runs/39139517213).
That delivered evidence does not qualify the pending Timers/compiler changes or
installed-IC recovery; their PR/native gates remain separate.

The #57 caller adoption passes 45 consumer/composition tests on Rust 1.91,
the Rust 1.91 consumer Wasm check, strict native/Wasm Clippy, all four graph
contexts, direct dependency boundaries, formatting and documentation links.
Required hosted native acceptance remains separate from these Linux checks.

The [root ownership map](../../README.md#scope-and-validation) is the maintained
boundary reference; the [workspace manifest](../../Cargo.toml) and
[lockfile](../../Cargo.lock) own requirements and resolved versions. Earlier
migration decisions, source-bound receipts and cross-repository work are preserved
verbatim in [the previous handoff](history-through-0.17.1.md), which links the
older archives. Historical pending states are not current instructions.

## Ownership and integration

Core owns checked records, attempts, transitions and finite-batch selection through
`Job::restore`, `Job` methods, `next_due_ns` and `start_next`. Its only normal
dependency is Serde. Consumers own payloads, authentication, storage codecs/indexes,
endpoints, lifecycle and disposition of uncertain effects. Every retained record
still crosses `Job::restore`; Running/Uncertain work remains blocked until explicit
resolution. Commit-before-dispatch ordering and error precedence are unchanged.

The qualification app uses IC Memory's bounded fixture cell for durable bytes,
IC Storage's governed cells in native integration tests, and IC Timers directly
for volatile watchdogs. Platform owns Principal, Candid and the CDK frontend.
Testkit owns PocketIC transport/startup; Build owns capture/retention and Host
supplies native mechanics. Jobs retains scenarios, limits, optimizer flags,
evidence names and exit/error policy. Shared Tooling owns common setup/checkers;
Jobs owns concrete gates and release/publication admission.

The separate fixture paths are intentional:

- Native notification/maintenance consumers use an atomic byte-replacement store
  to inject commit failures independently of real stable memory.
- Native stable cells exercise allocation authority, cold reopen and unchanged-byte
  refusal. Memory's sparse-image loader expands frozen images; Jobs checks their
  producer/schema and work ceiling. Frozen old formats are negative inputs, not
  compatibility readers.
- The canister/harness exercise IC calls, traps, timers and upgrades only when the
  explicit live gate is selected. Native models do not establish IC rollback.

## Dependency and tooling boundaries

Platform `candid` serves DTOs and finite decoders; `cdk` is enabled only on Wasm.
Ordinary native consumer builds expose only API DTOs with Platform and Serde.
Jobs, Memory, Timers and JSON are declared for Wasm and development only, matching
the existing queue/reply module gates. The native normal/build graph now excludes
these integrations and their runtime/storage branches. Recovery uses Platform's
Serde contract export instead of a separate direct Serde declaration. The full
IC executable bundle remains explicitly selected for installation and admission.
The published generator routes generated paths through Platform and preserves
CDK's protected-spawn unfinished-task panic. Testkit shares Platform's decoder
identities. There are no direct Candid/CDK, Platform generator, raw stable-substrate
or retired Testkit decoder declarations. Upstream packages remain transitive;
the qualification Wasm still uses CDK 0.20.3, executor 2.0.0 and ic0 1.2.0.
Host parsing, Agent and unused crypto/certification capabilities are not selected.
Core and consumer Rust floors remain **1.91**; development is **1.99.1**.

The snapshot keeps its 0.24.0 annotation and exact reviewed revision
`6eb8c41785c5156aef62f082702bd54e8ce04606`. Canonical payloads remain managed
through Shared Tooling, with product IC tools explicitly selected in Jobs.
`ci/release-delivery.txt` is the sole direct-delivery declaration. Publication
remains a separately authorized operation.

`make check-dependency-boundaries` rejects canonical direct names, including
aliases in member normal/build/dev/target tables. Four resolved graph contexts
check native/Wasm core exclusions, the qualification Wasm's single runtime
identities and native consumer exclusion of the queue/runtime/storage dependencies
and CDK frontend. These are selected
known-name policies, not exhaustive dependency-family or feature admission. They do not
inspect unused catalog entries or independent roots, authenticate registry sources,
or admit deployed artifacts. This repo has one workspace and no production-canister
deployment command; downstream deployment owners must qualify their own graph and
artifact. Role comments alone do not enforce deployment exclusions.

## Qualification and remaining limits

Platform 0.1.11 and its paired macro archive match public registry checksums.
The generator and Candid implementation are unchanged from 0.1.10. Its source-bound
receipt is in `target/qualification/platform-0111/`: 28 focused Rust 1.91 tests,
separate core/consumer MSRV/Wasm checks, release Wasm compilation, strict
native/Wasm Clippy and dependency/format/link checks pass. The DID and all 28 Wasm
export names/kinds match the preceding build. The earlier 0.1.10 receipt separately
records 50 focused tests; it is not a later-source or installed-IC qualification.

The preceding ownership audit changed guidance only. It removed the stale Testkit
decoder exception, delegated guide version selection to manifests and separated
current facts from historical receipts. No further runtime deletion or upstream
move was justified. Its tracked non-document inputs remained byte-identical to
that audit's incoming working tree. On Rust 1.91, the core restoration-history matrix and 19 native
composition cases pass (20 tests). Four graph contexts, dependency boundaries,
pins, formatting, snapshot integrity and 313 local documentation references pass.
Unchanged runtime builds retain the source-bound Platform receipt above; no new
Wasm/Clippy claim is inferred from document changes. Audit evidence is in
`target/qualification/ownership-sanity-0172/`.

The subsequent dependency-scope cleanup passes 50 focused Rust 1.91 tests through
`make check-consumer test-consumers`, including capture failure/retention and the
bounded recovery decoder; the seven live tests remain ignored. Separate ordinary
native DTO compilation, `make check-msrv check-wasm`, release Wasm compilation and
strict native/Wasm Clippy pass. All four dependency graph policies, direct
boundaries, pins, formatting and release-adapter substitution checks pass. The
native consumer normal/build graph shrinks from 74 to 54 package identities;
both macOS target graphs also contain 54, while Wasm remains at 89. These are
resolved graphs, not native macOS execution or artifact-size measurements. No
locked package identity/version changed; only the redundant harness Serde edge
was removed. Formatting-hook qualification now selects an existing dependency
pair and overlays current member manifests/Rust inputs together, so pending
dependency migrations do not mix current root declarations with released callers.
Its formatting, preservation and installation scenarios pass on Linux.

The release-tooling manifest-order fixture also referenced the removed direct
Candid row. It now perturbs the adjacent `ic-build`/`ic-host-fs` declarations.
`make test-release-tooling` passes on Linux, including selected-commit sorting,
release/resume and publication eligibility/ordering with disposable Git repositories
and substituted validation/registry effects. No actual release or upload ran.

The small optimizer-status projection and panic containment around install-failure
diagnostics remain necessary with selected Host/Testkit APIs. Keep those policies
local until an available upstream interface preserves their contracts; existing
proposals are [#52](https://github.com/dragginzgame/ic-jobs/issues/52) and
[#63](https://github.com/dragginzgame/ic-jobs/issues/63). Available sibling sources
verify capability ownership, not deployed usage or final Canic/IcyDB composition.

Seven live recovery scenarios remain explicitly gated. Full CI, native macOS and
installed-IC recovery are not run by this audit. Earlier receipts qualify only
their tested sources and do not establish those outstanding observations.
