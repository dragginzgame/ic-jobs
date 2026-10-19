# Current handoff

## Source and pending work

Delivered Jobs **0.17.5**: `5d1ba09`. Pending **0.17.6** adopts published
Platform **0.2.1** and its matching macros. All owned fixture messages now use
bounded Wire frames: installation, management, unit RPCs and fault controls join
the existing delivery endpoints. Candid derives, management decoder plumbing
and service generation are removed. Method names, authorization, stored JSON,
delivery frame identities and blocked Running/Uncertain semantics are preserved.

This is a protocol cut for the unpublished fixture and native harness only.
Quiesce activity, dispose of outstanding Candid management/init requests and
rebuild both together. There is no compatibility reader or dual endpoint.
Post-upgrade takes no arguments. Init checks raw length before copying because
Platform lifecycle macros do not support guard attributes. The remaining codecs
use Platform's explicit hooks and pre-copy length guards, with identities and
budgets owned by `management_wire` and `delivery_wire`.

The pending manifest/lock updates are preserved, including Build 0.22, Host 0.30,
Memory 0.49, Testkit 0.55.2 and Wire 0.4.9. Storage **0.5.7** now supports Memory
0.49, resolving the earlier native test conflict without reverting Memory or
adding a version bridge. Native DTOs use Principal, Wire and Serde; no Candid
capability is selected by this application. The qualification Wasm excludes
Candid and full `ic-cdk`; Platform 0.2's owned runtime and Timers' released
Platform 0.1 dependency share registry executor 2.0.0 and ic0 1.2.0.

Rust 1.91 passes 28 queue/codec tests, one management protocol test, 19 native
composition cases, four capture/CLI tests and one bounded management-reply test.
Separate core/consumer MSRV and Wasm checks, release Wasm compilation, strict
native/Wasm Clippy and all four graph/boundary/pin checks pass. The live recovery
suite now has nine scenarios, adding installed rejection of retired Candid init
and management frames, invalid unit frames and oversized arguments. All nine pass
on original, `-O3`, `-Os`, `-Oz` Wasm: **36 successful Linux scenario runs**.
The source is the working tree based on `5d1ba09`, including the new management
codec module; saved non-document fingerprints still match after qualification.
`target/qualification/platform-021-0176/` retains the patch, source hashes, focused
and tooling logs. Its `live/jobs-recovery.tGksOx/` directory retains all four tested
artifacts/hashes, Testkit selection, lock/snapshot and server/test logs. Release,
publication and consumer-tooling fixtures, formatting and dependency checks pass.

Earlier Host 0.29 qualification passed eight live scenarios on original, `-O3`,
`-Os`, `-Oz` Wasm (32 runs), retained in `target/qualification/host-029-0176/`.
The subsequent Host 0.30 focused evidence is in
`target/qualification/host-030-0176/`. Those receipts retain their tested sources;
they do not qualify this later runtime/protocol cut. Native macOS, hosted matrices
and full CI remain separate from local focused and explicit live checks.

The reviewed Shared Tooling **0.25.0** snapshot remains bound to committed source
`b66071bcd14da7086cc234a7ddf6c1223b41c3b9`; this is not a claim of a tagged
Shared release. The canonical exporter refreshed 114 files, retired both Perl
Cargo editors and added Shared's preparation transaction. Jobs selects the
released IC Build editor through `install-release-tools`/`release-tools-check`,
with automatic offline-admitted absolute-path selection and a 1 MiB per-document
allowance. Fresh release preflight prepares the editor after source admission;
maintainers need no shell exports. Saved preparation selects from its admitted
source's committed lock without installing or fetching against live metadata.
CI and standalone offline fixtures prepare it separately from common tools.
The adapter and actual local release fixtures use that editor. This batch ran no
release or upload in the real checkout.

The capture CLI now uses Clap; original/optimizer invocations and evidence
ordering remain intact. Shared owns grouped-formatting defaults, so the duplicate
Make commands are removed. Dependency-role headers live only in the root catalog.
Tracked files and package assets have no symlinks; admitted tool-store pointers
and isolated rejection fixtures retain their existing purpose.

Earlier adoption evidence separately covers hooks, formatting-evidence collection,
ShellCheck, package inventory and the regular LICENSE copy. The current release
fixture rerun still proves setup without exports, failed-check refusal despite a
printed path, saved-lock selection with unreadable live metadata and recovery
without installation, fetching or repeated validation. Historical receipts below
remain bound to their own source and dependency selections.

### Delivered delivery cut and earlier evidence

The **0.17.4** change switches the unpublished fixture's `receive_delivery` and
`delivery_receipt` RPCs to published Wire **0.4.4** native Serde frames. There is
one current protocol with separate method identities and finite request/response
limits; Candid delivery input is refused. Installation and management stay
Candid. The two Wire methods keep their callable exports but are omitted from
the Candid service. Quiesce deliveries and dispose of in-flight calls before
upgrading both fixture canisters together; retained JSON needs no reset.

That change preserves the published Jobs API, error transitions, consumer-owned
storage and Running/Uncertain blocking rules. This is a patch to the released
library's qualification tooling, not a core API break. The earlier receipt used
package version 0.17.3; it does not qualify the later graph automatically.

The delivery-cut receipt preserved Build **0.18.0**, Platform **0.1.15**, cc **1.9.0**
and find-msvc-tools **0.1.15**. Wire added its published
codec/derive and Postcard graph without changing earlier locked identities.
Rust floors remain **1.91**, separately qualified for core and consumer; the
development compiler remains **1.99.1**.

Rust 1.91 passes 48 native consumer/codec/composition tests and the management
reply-decoder test. All eight live tests compile but remain explicitly ignored.
Separate core/consumer Rust 1.91 native/Wasm checks, the release Wasm build,
strict native/Wasm Clippy, all four graph contexts, dependency boundaries/pins,
formatting and documentation links pass. The failed
attempt to use Rust 1.91 rustfmt made no changes; formatting uses development
Rust 1.99.1 as selected by the repository.

### Earlier acceptance (before the pending Wire cut)

The earlier Timers caller and compiler changes are integrated on `main`:
[PR #71](https://github.com/dragginzgame/ic-jobs/pull/71)
and [PR #72](https://github.com/dragginzgame/ic-jobs/pull/72). Their complete
Linux/Apple Silicon/Intel macOS matrices pass in [run 39140917132](https://github.com/dragginzgame/ic-jobs/actions/runs/39140917132)
and [run 39141391926](https://github.com/dragginzgame/ic-jobs/actions/runs/39141391926).
Retargeting #72 after #71 preserved its tested tree exactly.

Delivered 0.17.3's lockfile selected Platform 0.1.13, Build 0.17.6,
Host 0.27.6, Storage 0.5.4, Testkit 0.54.1 and Metrics 0.14.4. Those selections
were outside the two caller/compiler PR graphs; their passing checks do not
qualify a later dependency graph.

Linux Rust 1.99.1 qualification passes 48 core, 46 consumer/composition/wire,
four capture/recovery-decoder tests and six doctests. Rust 1.91 native/Wasm
checks, the release Wasm build, strict native/Wasm Clippy, formatting/hook/evidence
fixtures, all four graph contexts, dependency boundaries/pins and links pass.
The seven live recovery tests remain ignored; native CI does not qualify them.

Delivered 0.17.2's complete Linux/Apple Silicon/Intel macOS native matrix now passes
on its exact source in [CI run 39139517213](https://github.com/dragginzgame/ic-jobs/actions/runs/39139517213).
That delivered evidence remains separate from the later caller/compiler PR
matrices and does not qualify installed-IC recovery.

The #57 caller adoption passes 45 consumer/composition tests on Rust 1.91,
the Rust 1.91 consumer Wasm check, strict native/Wasm Clippy, all four graph
contexts, direct dependency boundaries, formatting and documentation links.
The hosted caller acceptance is recorded above, separately from these Linux checks.

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
for volatile watchdogs. Platform owns Principal, management Candid and the CDK
frontend; Wire owns the delivery codec. Jobs owns its two method IDs and limits.
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

Platform `cdk` is enabled only on Wasm; the application selects no Candid
capability. Jobs, Memory, Timers and JSON remain Wasm/development dependencies.
The native consumer normal/build graph excludes those integrations and runtime
branches. The recovery harness uses direct Serde bounds and Testkit-owned raw
PocketIC transport with Wire frames; no decoder-type conversion crosses the two
released Platform versions. Principal retains its upstream identity.

The generator/runtime preserve protected spawning and trap behavior while
removing full `ic-cdk` from the qualification Wasm. The graph requires two
Platform versions (the owned 0.2 runtime and released Timers' 0.1 exports), but
only one Platform macro, Timers, registry executor and raw-system identity.
Candid, full CDK, parser, Agent and host tooling are refused in that Wasm graph.
There are no direct Candid/CDK, generator or retired decoder declarations.
Host 0.30 direct dependencies disable defaults; only development test support is
selected. Testkit retains Host 0.29, while Build 0.22 uses Host 0.30. The full IC
bundle remains explicit. Core and consumer floors stay **1.91**; development is
**1.99.1**. Published Jobs' Serde-only API/runtime boundary is unchanged.

The snapshot records reviewed 0.25.0 source
`b66071bcd14da7086cc234a7ddf6c1223b41c3b9`. Canonical payloads remain managed
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

The selected Host/Testkit releases now supply the status projection and contained
diagnostic writer previously tracked in
[#52](https://github.com/dragginzgame/ic-jobs/issues/52) and
[#63](https://github.com/dragginzgame/ic-jobs/issues/63). Jobs keeps its CLI fallback,
error precedence and optimizer recipe. This local adoption changes neither
sibling ownership nor deployed consumer composition.

Live recovery retains its explicit gate; the current results and source identities
are recorded above. Earlier receipts qualify only their tested
sources and do not establish current native macOS or hosted acceptance.
