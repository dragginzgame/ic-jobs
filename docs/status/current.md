# Current handoff

## Source and pending work

Delivered Jobs **0.17.0**: `312ff2486f7d15d60be92a14c58cc2bbd45e3d05`.
Pending **0.17.1** is a compatible ownership cleanup. Package versions, published
APIs, current wire/storage formats and installation authority are unchanged.
The earlier 0.16 stored-record and 0.17 management-response cuts are delivered;
see the [package guide](../../crates/ic-jobs/README.md) and
[consumer guide](../../apps/job-consumer/README.md) for their required actions.
No storage reset, conversion, compatibility reader or deployment is added.

## Maintained ownership

The [root ownership map](../../README.md#scope-and-validation) identifies the
public boundaries. Core owns validated records, attempts, transitions and slice
scheduling; its normal dependency tree contains only Serde. Consumers own payloads,
authentication, storage codecs, indexes, endpoints, lifecycle and effect disposition.
Every decoded job still enters through `Job::restore`. Running/Uncertain intent
stays blocked until explicit resolution; commit-before-dispatch ordering remains.

The unpublished consumer uses Memory's bounded fixture Cell for durable bytes and
Timers directly for volatile watchdogs. IC Testkit owns PocketIC transport/startup;
Platform owns the supported Candid boundary. Jobs owns test scenarios, limits and exact receipt
comparison. Build owns capture/retention and Host supplies native mechanics;
Jobs' capture binary owns optimizer flags, evidence names and exit policy.

This batch replaces local sparse-hex image expansion with the already-selected
Memory helper. Jobs retains producer/schema checks, its three-page fixture bound,
write-count observation and unchanged-byte refusal proofs. Frozen old JSON,
physical images and Candid replies remain negative inputs rather than old readers.
The small byte-replacement store remains intentional fault injection, distinct
from the real stable-memory fixture and IC message rollback.

Shared's delivery checker now reads Jobs' single direct-only declaration in
`ci/release-delivery.txt` from Make and the standalone metadata adapter. The
validation checkpoint includes it. The local publication policy remains Jobs-owned.
`formatting-hook-check` keeps the actual Jobs input/Make/hook preservation check.
Shared owns its general hook/frontend regression matrix; the unused fixture and
frontend formatter payloads are retired through canonical snapshot refresh.

## Selected dependencies and tooling

The native consumer fixtures now select published IC Storage 0.5.3 as a dev
dependency instead of directly importing stable-structures. Its registry archive
checksum matches the index, and its governed `StableCell` preserves the selected
upstream cell format and operations with Memory 0.48 allocation leases. The
oversized corruption case uses Memory's explicit larger fixture write ceiling;
normal admission and exact retained header/payload checks are unchanged. Core
remains Serde-only; Storage is excluded from core and qualification Wasm graphs.
Stable-structures 0.7.2 remains transitive through Memory in the qualification
Wasm and through Memory/Storage in native tests. Direct substrate dependencies,
including aliases in normal/build/dev/target-specific member tables, are refused
by the Shared boundary checker. Four disposable negative fixtures verify those
refusals. The unused catalog and independent roots remain outside that check;
this repository has one workspace and its catalog has no substrate declaration.
Evidence is in `target/qualification/storage-adoption/`. This adoption passes
46 Rust 1.91 consumer tests, separate core/consumer MSRV and Wasm checks, strict
native consumer Clippy, formatting, dependency pins and all three graph contexts.
Full CI, native macOS and live IC qualification are not selected by this change.

The lock selects Memory 0.48.2, Timers 0.27.1, its Metrics 0.13.4, Testkit
0.54.0 with native Metrics 0.14.1, Build 0.17.3 and direct Host FS/Process 0.27.3.
Platform remains at published 0.1.9, upgraded earlier from locked 0.1.8.
The retired Testkit decoder is absent from the resolved graph. Core and consumer
Rust floors remain 1.91; development is 1.99. Shared Tooling retains the 0.24.0 version annotation
but now selects committed topic revision `6eb8c41785c5156aef62f082702bd54e8ce04606`,
with 115 selected payloads, for the graph checker in
[Shared PR #285](https://github.com/dragginzgame/shared-tooling/pull/285). This is
unmerged source, not a published 0.24.1 release. Adoption uses the canonical
exporter independently of the dirty sibling checkout. Product IC
tools remain explicitly selected, including ICP CLI 1.7.0; setup/check and live
PocketIC selection remain separate.

The [dependency exposure map](../../README.md#dependency-exposure) distinguishes
workspace declarations, the qualification Wasm, native recovery tests and the
capture binary. The maintainer wants to phase out Candid/CDK use, including
indirect runtime paths. Current direct users are annotated in manifests; moving
the fixture or wrapping the codec would not meet that goal. Qualification stays
in place pending the qualified runtime replacement. Published Platform 0.1.9
supplies DTO derives, Principal, encoding and the delivery decoder with the
existing five limits. Pre-adoption API bytes are frozen in the new `api_wire`
test; native cold-reopen and corruption tests retain the storage boundary.
The qualification app and native harness select only Platform `candid`, with
defaults disabled. No parser, certification or Platform executor is enabled.
Testkit separately declares Platform `ic0` for its own Wasm library; Testkit is
not in the qualification canister graph. Wasm Candid/CDK remains
for generated macro paths and runtime behavior under
[Platform #1](https://github.com/dragginzgame/ic-platform/issues/1#issuecomment-6247576553).
Published Testkit 0.54 now consumes Platform's decoder identities, completing
Jobs' adoption for [Platform #2](https://github.com/dragginzgame/ic-platform/issues/2).
The harness has no old decoder declaration, conversion or compatibility re-export.
CDK spawn's panic-on-outliving-method behavior is not replaced with the weaker
raw protected spawn. Calls, callbacks, traps, lifecycle and memory code stay intact.

Canonical dependency-role labels identify core as the sole production package;
Serde serves production and tests, while all other catalog entries serve tests
and tooling here. Both formatter commands use grouped sorting. The release
adapter selects grouped manifest checks for current and committed source, with
disposable fixtures proving ordering refusal and selected-commit isolation.
The common Rust selection now has two tools and no Candid extractor; Jobs has
no extractor caller. Dependency requirements, lockfile and package versions stay
unchanged by the earlier tooling adoption. Its evidence is retained in
`target/qualification/shared-tooling-0240/`; the subsequent Platform adoption has
its own source/registry/graph/test evidence in `target/qualification/platform-adoption/`.

Native Linux and Wasm all-feature normal/build core exclusions now run through
`make check-dependency-graphs` in the CI/release roster. Its third context admits
the qualification Wasm without host-only packages and requires one Timers and one
CDK executor rendered identity. Shared owns graph observation/matching; Jobs owns
the explicit policy in Make. These selected known-name policies do not authenticate
sources or cover every feature combination. No production-canister deployment
entrypoint exists here; downstream applications must bind their own target/features,
exclusions, successful locked build and final artifact identity to deployment.
Role comments, sorting and direct pin checks do not supply deployment admission.
The newly selected Shared direct-boundary checker is now in the CI/release roster
and its policy is a checkpoint input. It checks renamed direct dependencies across
all member tables, with the narrow package exceptions above. It does not enforce
transitive/feature isolation, exception target placement, unused catalog entries
or other workspace roots; those are explicitly inspected for this adoption.

## Qualification and remaining decisions

The published Platform 0.1.9 archive was independently downloaded and its SHA-256
matched the public registry index. Its actual features, module exports and packaged
migration guidance described the typed-CDK and then-current Testkit 0.53 exceptions
under issues #1/#2. Only the CDK exception remains after Testkit 0.54 adoption. Bounded decoder types and implementation match selected 0.1.8 exactly;
the added contract decoder reexports retain their upstream configuration policy. Jobs keeps
only the `candid` feature with defaults disabled; new default-decoder, parser,
Agent, certification, management, ledger, raw-system and executor capabilities
are not selected. No runtime wrapper, changed endpoint visibility or decoder-policy
substitution is introduced. Current qualification is retained separately in
`target/qualification/platform-019/`; earlier receipts keep their original sources.
That earlier Platform selection passed 50 focused native tests (26 consumer library, one
frozen API wire test, 19 composition, three capture and one harness decoder),
separate core/consumer Rust 1.91 native/Wasm checks, the release consumer Wasm
build and strict native/Wasm Clippy. Shared graph/direct-boundary/pin checks,
formatting, snapshot integrity, release command admission and links pass. The
Wasm graph excludes host parser/Testkit and retains one executor 2.0.0 shared by
CDK and Timers. Seven live recovery scenarios remain ignored; native macOS and
installed-IC qualification are still outstanding. Issue #1 covers the
remaining direct Candid/CDK dependencies; no new unsupported capability was found. The new
management deletion DTO and raw IC0 calls are unused and their features stay
disabled. The Candid-path patch proposal is upstream work, not a published CDK
macro capability; selected CDK 0.20.3 still needs its direct Candid declaration.
No candidate macro, Git override, default decoder or runtime replacement is used.

The Testkit decoder publication blocker is resolved. Published Testkit 0.54.0's
archive checksum matches the public index. Its call helpers accept Platform's
limits and expose Platform decoder errors directly; Jobs uses those exact types
and keeps every byte/work/header/type budget unchanged. The old decoder is
removed from the member manifest and lockfile, and the shared direct-boundary
policy now forbids it without exceptions. The root already selected the 0.54
requirement when this completion began. Only Testkit and its required native
Metrics 0.14.1 enter the lock delta; Timers retains Metrics 0.13.4 in the Wasm.
No other package version, runtime, storage or wire format changes.
Published-source and graph evidence is in `target/qualification/testkit-054/`.
On Linux, all four focused native harness/capture tests pass on Rust 1.91 and
strict harness Clippy passes; seven live scenarios compile and remain ignored.
Direct-boundary, all three core/qualification graph contexts, pin, formatting and
link checks pass. Complete all-feature workspace metadata has no retired decoder,
and a renamed dev dependency is rejected in a disposable negative fixture.
The earlier disposable Platform 0.1.6 producer/adoption patches in
`target/qualification/fleet-convergence/` remain historical and are not required
for the registry dependency. Native macOS and installed-IC qualification remain
separate; no live recovery is selected by this decoder adoption.

Shared PR #285 now preserves consumer graph depth, package-family exclusions,
forbidden dependency-feature edges and optional-package identity caps. The
producer fixture covers their real offline Cargo behavior and failure admission.
The maintainer explicitly approved scoped normal-checkout adoption in Canic,
Auth and Query after reviewing the prepared patches. Their 52 actual graph
contexts pass: one Canic runtime union, 26 Auth and 25 Query selections. Canic's
source/lifecycle policy remains in Rust; 16 isolated source-guard tests pass on
Rust 1.91 against the reviewed copy. This does not compile or qualify Canic Core.
Query keeps its graph-only matrix separate from compilation; Auth keeps its
policy sets while full rendered identities replace name/version collapsing.
Canic's concurrent Build/Auth documentation updates were preserved by applying
only cleanly matching reviewed hunks. Initial Query feature-policy refusal and
Canic sandbox cache-preparation refusal are retained alongside passing reruns.
Current receipts are in `target/qualification/graph-adoption/`; prior proofs in
`target/qualification/fleet-convergence/` retain their original inputs.

The Testkit 0.53 harness repair replaces the removed `CandidCallExt` methods with
`update_candid_args`/`query_candid_args`, selecting `(T,)` for one wire reply and
unpacking that outer tuple without changing the application's Result. All five
limits and Testkit's transport/rejection/decoder error ownership stay intact.
The existing native reply test now exercises this tuple shape, inner application
errors, bounded skipping and oversized/old-response refusal. No compatibility
shim or upstream change is needed; seven live recovery cases retain their explicit gate.

Earlier Platform 0.1.4 adoption passed 50 distinct focused native tests (26 queue/reply, 19
composition, one frozen wire, three capture and one harness decoder). Seven live
recovery cases remain ignored. Separate core/consumer Rust 1.91 and Wasm checks,
the release Wasm build, native/Wasm Clippy and formatting/hook/release fixtures
pass. The direct-boundary checker rejects a forbidden renamed Candid dependency
in disposable normal, build, dev and target-specific build tables. The Wasm
graph has no host parser or old Testkit decoder, and CDK/Timers still resolve one
executor 2.0.0 identity. Native macOS and installed IC behavior are unqualified
for this source; no live execution or deployment was selected.

The ownership audit reviewed manifests/imports, public exports, history, current
source and available sibling/cached package APIs under the Shared flow-convergence
and module-cleanup methods. Its focused receipt and source hashes are retained in
`target/qualification/ownership-sediment-0171/`. Source-only inspection of siblings
does not establish their deployed behavior or qualify unpublished changes.
On Linux, 56 focused tests pass: 26 consumer library, 19 composition, seven core
record and four native harness/capture tests. Seven live scenarios remain ignored.
Separate core/consumer Rust 1.91 and Wasm checks, consumer Clippy, the real-Git
release/publication fixtures, actual hook and consumer-tooling fixtures, formatting,
snapshot/pin/link checks and release-command dispatch checks pass. Initial missing
fixture inputs and an unprepared standalone test prerequisite were corrected;
their failed evidence is retained separately from the passing reruns.

A second ownership pass corrected stale Timers/Metrics version requirements in
the packaged API guide and removed the consumer guide's obsolete workflow-delivery
condition. Dependency identity stays application-owned through Timers' measurement
re-export. No further runtime removal was justified; the first pass's source-bound
Rust evidence remains unchanged. This documentation pass has its own receipt in
`target/qualification/ownership-sanity-0171/`.

The selected Host package still has no child-status projection export, and the
selected Testkit package has no panic-safe diagnostic reporter. Keep the local
policy/containment adapters until an available API preserves those behaviors;
[#52](https://github.com/dragginzgame/ic-jobs/issues/52) and
[#63](https://github.com/dragginzgame/ic-jobs/issues/63) own those proposals. IC Relay
is an unpublished transport scaffold; it cannot replace the receipt test model.
IC Wire is a different protocol, not a compatible replacement for Candid. Neither
is adopted without a concrete consumer requirement and separate contract review.

Native macOS and live IC recovery need evidence for their exact source and graph.
Earlier receipts do not qualify this pending batch; the seven live scenarios
retain their separate explicit gate. No full CI, live recovery, release, package
upload or reset is selected. Shared PR #285 is the isolated canonical-owner
contribution. Canic/Auth/Query graph adoption has the explicit scoped checkout
exception above; other sibling source remains unchanged.

Historical receipts and former pending/proposal state are preserved in
[history through 0.17.0](history-through-0.17.0.md), with earlier archives linked
there. They are evidence of their original inputs, not the current architecture
or an active issue-status ledger.
