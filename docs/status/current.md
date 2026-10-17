# Current handoff

## Local publication admission

This compatible tooling change targets pending 0.15.1 over tagged 0.15.0
(`71761e0c`). Product versions and published release notes remain unchanged.

The publication policy now lives in `scripts/release/check-publication.sh` and
accepts only the selected remote; the package is fixed to `ic-jobs`. `publish.sh`
uses it before Cargo. Shared observers and snapshot bytes remain unchanged.
The local publication fixture covers ordering, failed observations, identity
races and real-Git source/tag admission with substitute registry transport.
The obsolete shared preflight snapshot entry awaits the next reviewed export.

## Source and pending work

Delivered Jobs **0.14.1**: `cc9587c` (Release 0.14.1).
Pending **0.15.0** hard-cuts the latest result to
`JobRecord::last_outcome: Option<RecordedOutcome>`, pairing `outcome` and `at_ns`.
The separate timestamp field and mismatched-presence admission check are removed.
The same batch restores stored consumer entries directly with their payloads,
removing the temporary job vector and positional reassembly.
Validated `Job` now serializes transparently through the canonical `JobRecord`
writer, without direct deserialization. Native notifications persist typed entries
directly and no longer need a borrowed-payload projection type parameter.
Maintenance keeps one validated job with its cache entries/counter rather than a
validated job plus a manually synchronized raw record. Reads still pass through
`Job::restore`; bounds admission and per-occurrence commit ordering remain.
Completed/Failed restoration shares its common timing/attempt checks while
retaining Once-only success, permanent failure and exhausted-retry admission.
Pending/Paused and Cancelled restoration share retained retry evidence admission;
only Pending/Paused must match the retained retry's deadline. Cancellation keeps
its independent management timestamp, and Running retry checks remain separate.
Notification management responses consume the record and payload together at one
projection boundary, after commit and full-queue deadline projection where needed.
The native notification destination owns recorded-result lookup, including
payload matching and success-before-rejection evidence precedence. The sender
still classifies those facts and explicitly resolves its retained job. Missing
evidence remains blocked. The persistence example and shared roundtrip helper
serialize Jobs directly, while reads still decode JobRecords and use `Job::restore`.
Valid-state consumer fixtures share public transition setup with explicit target
states and original timestamps; malformed-record fixtures remain independent.
Each consumer reuses its existing identity lookup for extraction/duplicate checks.
Owned extraction changes only the consumed snapshot; persisted ordering is
unchanged. Page and single-job inspection project the complete owned entry,
keeping payload and validated job together through the response boundary.
These cleanups preserve the current 0.15 JSON and transition behavior.
The delivered 0.14.1 adopted Memory 0.45's bounded fixture-cell API in the
unpublished consumer and the standalone Candid decoder for delivery replies.
Memory owns fixture allocation/bootstrap and fresh/retained Cell admission;
Jobs keeps its 24 KiB limit, stable key/configuration, snapshot codec, collection
checks and every `Job::restore` boundary. One payload header is checked before
retained bytes are loaded. Refused cell bytes are preserved; allocation metadata
may change before a later cell refusal. No reset, conversion or old reader is added.

The stored and validated queue states share `Snapshot<J>` rather than two metadata
structs. Consuming admission maps each record and payload into one validated entry;
typed writes serialize Jobs directly and borrow the same header/entry/receipt
checks without building an intermediate raw snapshot or identity-mapped output.
Every decoded record still enters `Job::restore` before that entry's metadata
checks, after header admission and before receipt checks. Encoded capacity still
precedes collection errors on writes. The entry-admission cleanup preserves
error precedence, payload ownership and transition behavior; the stored-record
cut is separate.

The maintainer explicitly selected the hard cut and retirement of earlier
installations without preserving their jobs. Earlier JSON is refused, including
the old separate timestamp field; there is no dual reader, converter or fallback.
Consumers must create fresh state under their own installation authority.
This implementation executes no storage reset or deployment. Refused cell bytes
are not replaced; Memory bootstrap may still update allocation bookkeeping.
Current-format Running/Uncertain records
still restore as blocked and require the same explicit effect disposition.
The former 0.7.5 positive record corpus is removed. A hand-authored current
uncertain record qualifies exact JSON and blocked reconstruction; frozen old
consumer JSON/physical images remain only as refusal evidence.

Delivery decoding uses an 8 KiB wire ceiling, 4 KiB header, 64 type entries and
finite decoding/skipping quotas, independently from the host management limits.
Unreadable replies stay distinct from an authoritative absent receipt and keep
uncertain effects blocked. The application still compares the exact envelope.
Jobs' actual CI fixture keeps one selected-lock drift case through its Make route;
the pinned Shared producer owns generic checkpoint/output and refusal matrices.
The 0.15 record API and JSON change; transition and error ordering remain unchanged
for current-format records. The decoder's work limits and uncertainty policy remain.
Shared Tooling is **0.16.0** at
`989752490b392f7f4c542c7652dcb84e75ecf363` (111 payloads), from canonical export
of the delivered, three-host-qualified source. Jobs selects its retained-failure
assertions explicitly, uses the standard cleanup-inclusive probes and delegates
Make skeleton copying and locked release preflight to their Shared owners.
Optional maintenance runtime/service payloads are retired; guidance and catalogs
remain. That contributor-only adoption was delivered in 0.13.3.
Release preparation now uses Shared's metadata renderer directly. Jobs no longer
requires cargo-edit, a separate release-tool catalog or local setup/check targets.
Common tool admission and metadata refusal/rollback boundaries remain in place.

The selected graph uses Memory 0.47.0, Testkit/Candid 0.51.0, Timers
0.26.1, Metrics 0.12.0, Build 0.13.0 and direct Host FS/Process 0.25.1.
Rust floor is 1.91.0; development is 1.99.0.
The cleanup preserves those incoming registry selections. Memory 0.46's narrowed
exports supply backing IO types directly, without a full substrate re-export;
its non-cloneable runtime memory lease stays with one cell owner. Only native
fixture tests name the already-selected `ic-stable-structures` package directly,
for raw Cell construction/corruption. That development dependency adds one lock
edge without changing selected package versions or published Jobs dependencies.
The standalone Candid decoder remains a direct dependency of the unpublished
consumer. Memory fixture support is enabled only there. Incoming lock updates
are preserved.
Own versions,
index, refs, release and publication remain maintainer-owned. Memory requests
follow its key-only API. Its incompatible outer ledger requires explicit owner
disposition before deployment of retained earlier installations; no reset or
migration is executed by this source update. Recovery replies now
have finite wire, header, type-table and decoding/skipping work limits. The
maximum-page and oversize-refusal proof runs natively without starting PocketIC.

`start_next` returns `(index, attempt)`. Commit the selected job record with its
immutable payload/index before external dispatch. Indexes refer to iterator order.
All restore paths retain
`Job::restore`; ordering, transition errors and uncertain-effect disposition are
unchanged. Consumers own storage and depend on Timers directly. Jobs has no
runtime Timers dependency or lifecycle exports. The capture tool owns Jobs'
optimizer recipe and uses IC Build's retained path; Jobs selects the live gate.

## Qualification

The Testkit 0.51 recovery-harness repair is qualified in
`target/qualification/testkit-051-recovery-fix/`, against clean incoming source
`56d7a6f9769fd5cac5fd14d490281d8b79aff513` and its selected registry graph. The
harness now uses public-field install/diagnostic/retry records and the current
explicit-caller Candid methods. Install sender/label/cycles, independent diagnostic
senders/default log bounds, reply decoding limits and four-attempt/ten-minute
retry settings remain unchanged. No compatibility adapters are added.

The complete recovery target compiles on actual Rust 1.91 and its bounded-reply
test passes; all seven live scenarios remain explicitly ignored. Strict Clippy
for that target passes without the unused-trait warning. Earlier consumer/capture
passes remain recorded in the supplied failure log; this repair does not relabel
them as new executions. Versions, dependency selections, shared snapshot and
primary index/refs are unchanged. No full CI, live PocketIC, native macOS,
release or publication ran. Earlier qualifications retain their source scope.

Entry/image cleanup evidence is in `target/qualification/entry-image-cleanup/`,
relative to the incoming pending 0.15 tree on `cc9587c`. The active registry caller
passes all 25 consumer and 19 composition tests on actual Rust 1.91, the separate
consumer Wasm floor check and strict native/Wasm consumer Clippy. Shared identity
lookups retain duplicate-before-restore and complete-restore-before-lookup order;
owned inspection retains state mapping, checked IDs, payloads and durable order.
Core runtime source is unchanged; its previous separate floor/Wasm evidence
retains its source scope.

[Memory PR #121 — Share bounded sparse memory image fixture loading](https://github.com/dragginzgame/ic-memory/pull/121)
proposes the fixture loader at `1b80f9c55129a5fb55ffc7839943f39368627089`, based on
published Memory 0.46.0 `578a25a15575c1e2c54439c41fc1866a847e006a`. Five producer
tests pass on Rust 1.91, with strict library/test Clippy, Rust 1.91 Wasm,
warning-denied Rust 1.91 rustdoc, formatting and changed-document links. The helper
imports only fresh zero-filled fixture backing under an explicit page ceiling;
it checks runs before allocation and never recovers authority or converts formats.

The isolated Jobs caller passes 25 consumer and 19 composition tests plus an
independent exact comparison reproducing both complete frozen backings with the
previous expansion. Its Rust 1.91 Wasm check and strict Clippy pass. Package
names/versions are unchanged; only Memory's source/checksum lock row differs in
that qualification copy. Producer/caller evidence includes the initial corrected
link-command and qualification-file documentation refusals.

The prepared `memory-adoption.patch` applies to the active tree. Jobs retains the
frozen images, provenance, schema/payload refusal and zero-write/full-backing
assertions; observation begins after fresh image import. Active registry adoption
and removal of the local expansion await reviewed publication under
[#66 — Adopt Memory sparse fixture-image loader after publication](https://github.com/dragginzgame/ic-jobs/issues/66).
There is no active path/Git dependency or copied upstream fallback. Root versions,
dependency selections, snapshot and primary index/refs are unchanged. No full CI,
live PocketIC, native macOS, reset, deployment, release or publication ran.

Evidence/fixture cleanup is qualified in
`target/qualification/evidence-fixture-cleanup/`, against the incoming pending
0.15 tree on `cc9587c`. All 47 core, 25 consumer and 19 composition tests pass on
actual Rust 1.91. The persistence example runs on Rust 1.91 and strict native
core/consumer Clippy passes. Existing cases retain success-over-rejection
precedence, payload matching, stale-attempt rejection, absent-evidence blocking,
failed commits, completion error order and idle zero-write admission. The shared
fixture builder asserts its actual target state; both callers retain their
original transition times. Formatting, documentation links and diff checks pass.
Independent writer-equivalence and corrupt-record
coverage are unchanged. This batch changes native fixtures and example/test
serialization callers; runtime and Wasm source is unchanged, so the prior
separate core/consumer Wasm qualification below retains its scope.
Own versions, dependency selections, shared snapshot and primary index/refs are
unchanged. No full CI, live PocketIC, native macOS run, reset, deployment, release
or publication ran.

Queue/retry cleanup evidence is in `target/qualification/queue-retry-cleanup/`,
relative to the incoming pending 0.15 tree on `cc9587c` and its incoming Memory
0.46.0/Timers 0.26.1/Build 0.12.1 lock selection. All 47 core, 25 consumer and 19
composition tests pass on actual Rust 1.91. Exact typed/raw snapshot bytes retain
metadata, payloads, receipts and blocked/terminal/paused records through reopening.
Collection/capacity refusal order, record restoration priority, failed commits,
deadline projection and notification record/payload pairing remain covered.
Retained retry corruption refuses across Pending/Paused/Cancelled; later
management timestamps and cancellation before the retry deadline remain valid.

Separate core/consumer Rust 1.91 Wasm checks, strict native/Wasm Clippy, recovery
harness compilation, warning-denied Rust 1.91 rustdoc and six doctests pass.
Formatting, documentation links and diff checks pass.
The first consumer attempts exposed the incoming Memory re-export and lease
changes; their failures remain retained alongside the corrected passes. The final
lock differs from the incoming selection only by the consumer's explicit raw Cell
development edge. Own versions, shared snapshot and primary index/refs remain
unchanged. No full CI, live PocketIC, native macOS qualification, storage reset,
deployment, release or publication ran. Earlier evidence retains its source scope.

Serialization/maintenance evidence is in
`target/qualification/serialization-maintenance-cleanup/`, relative to the incoming
pending 0.15 tree on `cc9587c`. All 46 core, 24 consumer and 19 composition tests
pass on actual Rust 1.91. The bounded transition-history explorer compares direct
Job serialization with its canonical record writer across every state; the exact
hand-authored JSON fixture still matches. Terminal corruption cases retain result
time, completion-kind and retry-budget refusal. Native fixtures retain exact
notification bytes, corrupt-record refusal, failed writes and partial-batch
reconstruction without repeating committed work.

Separate core/consumer Rust 1.91 Wasm checks, strict native/Wasm Clippy,
recovery-harness compilation, warning-denied Rust 1.91 rustdoc and six doctests
pass. The new compile-fail example fails specifically because Job does not
implement Deserialize, preserving record admission through `Job::restore`.
Formatting, documentation links and diff checks pass. The selected dependency
graph, own versions, shared snapshot and primary index/refs remain unchanged.
No full CI, live PocketIC, native macOS qualification, reset, deployment, release
or publication ran. Earlier qualifications below retain their source scope.

Paired-outcome cut evidence is in `target/qualification/paired-outcome-cut/`,
relative to delivered 0.14.1 `cc9587c` plus the incoming entry-admission cleanup.
The maintainer's explicit hard-cut/no-preservation answer supplies the current
retained-installation disposition. All 45 core tests, 24 consumer and 19
composition tests pass on actual Rust 1.91, including bounded transition-history
restoration, exact current-format JSON, incomplete/retired-shape refusal,
timing/counter corruption, competing admission errors, cell preservation and
unchanged zero-write refusal of incompatible outer ledgers. Running/Uncertain
states remain blocked until the existing explicit result disposition.

Separate core and consumer Rust 1.91 Wasm checks, strict native/Wasm Clippy,
recovery-target compilation, warning-denied Rust 1.91 rustdoc and all five
doctests pass. Formatting, documentation links and diff checks pass. The isolated
Build caller is refreshed with the new core/consumer Rust sources; its three
optimizer tests, recovery compilation and strict harness Clippy pass. Only
Build's source/checksum lock row differs there; the actual registry graph remains
unchanged and [#64 — Adopt Build post-processing custody after publication](https://github.com/dragginzgame/ic-jobs/issues/64)
retains its producer-publication gate. Earlier source qualifications below retain
their original format/source scope. Own versions, Cargo selections, the shared
snapshot and primary index/refs are unchanged. No full CI, live PocketIC, native
macOS qualification, storage reset, deployment, release or publication ran.

Entry/post-processing evidence is in
`target/qualification/outcome-entry-transform-cleanup/`, based on delivered
0.14.1 `cc9587c87812ef123918a64ad61d4ee729258921`. All 24 consumer and 19 composition
tests pass on Rust 1.91. The new competing-error cases preserve header-before-
restore, restore-before-entry-metadata, earlier-entry-before-later-record and
restore-before-receipt refusal order. Separate core/consumer Rust 1.91 Wasm checks,
strict native/Wasm consumer Clippy, formatting, links and diff checks pass.

[Build PR #53 — Share retained artifact custody for caller-owned post-processing](https://github.com/dragginzgame/ic-build/pull/53)
delivers the proposed helper at `5cd31f7`, based on published Build 0.12.0
`2598a417a96834d3139751adf549126add666c4b`. All 18 producer staging tests pass
on Rust 1.91, with strict library/test Clippy and warning-denied Rust 1.91 rustdoc.
The isolated Jobs caller passes three actual optimizer cases, recovery-target
compilation, strict harness Clippy and the actual consumer-tooling fixture with
optimizer/server effects substituted. All package names/versions are unchanged;
only Build's source/checksum lock row differs for source qualification. The
prepared caller patch applies to the active tree, but registry adoption remains
gated by [#64 — Adopt Build post-processing custody after publication](https://github.com/dragginzgame/ic-jobs/issues/64).
The active registry graph and local optimizer wrapper remain unchanged until
reviewed publication. No full suite, live PocketIC, native macOS run, release,
publication, reset, own-version change or primary index/ref mutation ran.

Snapshot/report/status evidence is in
`target/qualification/snapshot-report-status-cleanup/`, relative to this turn's
incoming pending 0.14.1 tree at base `242445f`. All 23 consumer and 19 composition
tests, separate Rust 1.91 core/consumer Wasm checks and strict native/Wasm consumer
Clippy pass for the active snapshot cleanup. Exact frozen JSON, incompatible-ledger
refusals, failed writes, receipt matching and uncertainty remain covered.

[Testkit PR #117](https://github.com/dragginzgame/ic-testkit/pull/117) delivers the
explicit diagnostic reporter at `7db1adc94158be0bc6116f9989a775237fe0eba8`, based on
current main `34eb5305342d86e0b55e0526379309f1c21fd54e`. It takes the exact request,
context and output writer; collection/formatting/output panics return as secondary
IO errors with their captured cause, and ordinary writer errors remain unchanged.
Seven focused diagnostic tests pass on Rust 1.91, with strict Clippy, rustdoc,
Rust 1.91 Wasm library checking, formatting and documentation links. The isolated
PR includes compatible 0.50.1 notes, without changing package versions. Earlier
formatter/test-style and preparation failures are retained alongside final passes.

Host PR #133 is closed as superseded by
[Host integration PR #172](https://github.com/dragginzgame/ic-host-tooling/pull/172),
source `85ef8b5c9f6a2e6213ab72f227e895098a7b39c8`. Its exact status module passes
two Rust 1.88 tests and strict Process Clippy on published Host 0.24.0 base
`f7ecf24557a6bbec6a6dc23ee9fb3e2052bf0251` with only its module/export added in a
separate checkout. The helper is not in the selected published Host package.
No duplicate contribution or sibling-checkout edit is made; current qualification
and the prepared caller are attached to the existing integration PR.

The combined isolated Jobs caller passes 23 consumer and 19 composition tests,
three capture tests, recovery-target compilation and strict native harness/Wasm
consumer Clippy. All package names/versions remain identical; only the five
explicit producer-source rows differ. Both caller patches pass apply-check
against the real working tree. The candidate is source qualification, not active
registry adoption: Jobs retains local diagnostic containment and numeric-status
projection until reviewed producer publication. These gates belong to
[#63 — Adopt panic-safe diagnostic reporter](https://github.com/dragginzgame/ic-jobs/issues/63)
and [#52 — Adopt shared child-status projection](https://github.com/dragginzgame/ic-jobs/issues/52).
No active dependency, package version, primary index/ref, live PocketIC, full CI,
release, publication, installation reset or native macOS qualification changed.

Fixture/decoder/runner cleanup evidence is in
`target/qualification/fixture-decoder-runner-cleanup/`, based on delivered 0.14.0
and the incoming lock selection. All 23 consumer tests and 19 composition tests
pass on Rust 1.91. They retain exact frozen Jobs JSON, unchanged incompatible
ledger backing with zero writes, restore/refusal order, failed commits, blocked
effects and receipt evidence. The new oversized-cell case bypasses the bounded
writer only to corrupt its fixture, then proves exact Cell header/payload
preservation for recovery and attempted installation. It makes no whole-backing
or blanket zero-write claim. The maximum 256-byte receipt and absent receipt are
admitted, while oversized replies remain unreadable rather than absent evidence.

Separate Rust 1.91 core/consumer Wasm checks, recovery-harness compilation and
strict native/Wasm consumer Clippy pass. Normal consumer Wasm includes
`ic-testkit-candid` and excludes PocketIC, full Testkit and Host. The only lock
change from the incoming graph is that consumer dependency edge; package versions
and other package rows remain unchanged. Formatting, pins, ShellCheck, shell
syntax, documentation links and diff checks pass.

Shared's exact pinned validation-target-runner fixture passes from a separate
Git checkout at `989752490b392f7f4c542c7652dcb84e75ecf363`; its runner matches the
active Jobs payload byte for byte. The first archive-only preparation could not
serve its nested clone fixture; that failure and its disposable fixture remain
retained. The corrected checkout passes generic mutation/deletion/redirect,
observation failure, checkpoint and timing coverage. The narrowed actual Jobs
consumer-tooling fixture passes Bash 5 with optimizer/server effects substituted;
it retains actual selected-lock admission-stop, original capture and variant/
failure evidence. No Shared source or snapshot is changed. Linux Bash 5 does not
establish Bash 3.2 or native macOS qualification.

[Jobs #50](https://github.com/dragginzgame/ic-jobs/issues/50) now tracks the actual
registry-based caller adoption, replacing its unpublished-source preparation.
Delivery and the applicable native matrix remain pending. No full CI, live
PocketIC, release, publication, installation reset or primary index/ref mutation ran.

Paired-entry cleanup evidence is in
`target/qualification/paired-entry-cleanup/`, based on local source `2e7a3f7`
with the incoming dependency updates preserved. All 44 core integration tests,
22 consumer and 19 composition tests pass on Rust 1.91, including exact frozen
Jobs JSON, incompatible-ledger zero-write refusal, full-snapshot validation,
failed commits and completion-refusal ordering. The scheduler's existing ordering
case now projects jobs from paired payload entries. Alignment state and its
synthetic mismatch test are removed because the loaded shape cannot split them.
The delivery projection constructs one full expected envelope for dispatch,
inspection and completion; comparison may copy the bounded 256-byte message,
while transitions and durable-write order remain unchanged.

The three remaining capture tests, native maximum-page/oversize-reply test,
separate Rust 1.91 core/consumer Wasm checks, strict native/Wasm Clippy, rustdoc
and all five doctests pass on the selected graph. The recovery harness compiles;
its seven live scenarios remain ignored. `check-consumer` runs the native reply
proof without selecting live execution. The actual consumer-tooling fixture
passes Bash 5 with substituted optimizer/server effects, including original
capture, producer replacement, exact variant flags and retained failure evidence.
Generic capture/retention behavior is already owned by Build's staging tests.
Initial checks exposed Memory/Testkit API fallout and concurrent graph changes;
failed receipts remain alongside passing current-graph logs. Earlier sections
below retain their historical source/graph scope and pending-version names.
No full CI, live PocketIC, native macOS, reset, release or publication ran.

Memory #105 caller evidence is in
`target/qualification/memory105-current-adoption/`. The exact producer head
`5ce93dc8903642f4a0725ceafc75a78a07bd5494` is qualified only in a separate consumer
copy, based on delivered Jobs 0.13.3 plus the pending install-diagnostics edit.
Its 24 consumer and 19 composition tests, separate Rust 1.91 core/consumer Wasm
checks, recovery compilation and strict native/Wasm consumer Clippy pass.
Only Memory's source changes in that qualification lock; all package versions
and other lock rows match the active graph. Normal consumer Wasm excludes
Testkit, PocketIC and Host. The complete caller patch passes apply-check against
the active tree and is attached to [Jobs #50](https://github.com/dragginzgame/ic-jobs/issues/50).

The candidate uses existing 24 KiB bounds for distinct fresh/retained admission.
Oversize refusal preserves the cell header and payload, while documented
allocation-bookkeeping effects remain possible. Earlier zero-write/whole-backing
assumptions failed and are retained as such; the corrected proof scopes exact
preservation to the admitted cell. The frozen incompatible-ledger cases still
prove zero writes and unchanged backing separately. Temporary preparation-path
failures are retained; final source and caller copies live under qualification.
The active Rust/registry graph and the pending 0.13.4 fix remain untouched.
No actual adoption, full CI, live recovery, native macOS Jobs run or release ran.

Recovery install-diagnostic evidence for pending 0.13.4 is in
`target/qualification/recovery-install-diagnostics-0134/`. The actual recovery
target compiles on Rust 1.91 against locked registry Testkit 0.49.0; strict recovery
Clippy, formatting, documentation links and diff checks pass. This qualifies API
integration, not live installation failure or installed recovery. No full CI,
PocketIC execution, release or publication ran for this pending change.

Delivered 0.13.3 acceptance is
[CI run 38923932920](https://github.com/dragginzgame/ic-jobs/actions/runs/38923932920),
source `684cf173af893c56f87ac462e830de960807240b`. Linux and macOS Apple Silicon
pass; macOS Intel is still running at this review. Jobs #54/#58/#59/#60/#61 remain open for that
complete matrix. Source-incorporated PRs #51 and #62 are closed without merging;
PR #55 remains open until this newer local integration is delivered. Earlier
receipts below retain their original source and pending/delivery observations.

Issue-reduction/adoption evidence is in
`target/qualification/issue-reduction-shared-adoption/`. Jobs
[#53 — Shared Cargo dependency-order fixture preparation](https://github.com/dragginzgame/ic-jobs/issues/53)
meets delivered/native acceptance at 0.13.2 `c497c0f`, with all three lanes of
[CI run 38888314944](https://github.com/dragginzgame/ic-jobs/actions/runs/38888314944)
passing. That result does not qualify the current dirty source.

The existing PR #62 caller selection is applied locally before refresh; actual
release/publication fixtures pass Bash 5 and genuine Bash 3.2 at both the old
pin and Shared 0.16.0. Canonical preview/application preserve every retained root
and retire only the three selected operational payloads. The first previews
refused the isolated clone's local/suffixed remote; selecting the exact canonical
remote corrected provenance before any export. Snapshot integrity verifies all
111 payloads. No active snapshot payload is hand-edited.

Actual release, consumer and formatting-evidence fixtures pass both shells on
the new snapshot, including selected Make paths with spaces, failed fetch/tool
admission before gate/intent and exact source/index custody. Formatting/hook
qualification, Bash 3.2's actual formatting caller, pins, release adapters,
source-following ShellCheck, documentation links and diff checks pass. Release,
registry/upload, optimizer and server effects remain substituted in disposable
fixtures. Shared's delivered source has its own
[three-host CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38906056937).
At that qualification, Jobs #54/#58/#59/#60/#61 retained delivered/native
acceptance; PR #62's green matrix covered its own earlier source.
No Rust source, incoming Cargo selections, own version or real index/ref changes
are part of this tooling batch. No full CI, live recovery, release, publication,
service installation or native macOS execution ran locally. Earlier qualification
sections below retain their original source/snapshot claims as historical evidence.

Scheduler/installer/update evidence is in
`target/qualification/schedules-installer-update-cleanup/`. All 25 scheduler/job,
ten management and one bounded history-explorer tests pass on Rust 1.91. The
recurring constructor is separately importable; expected deadlines, history
exploration order and operation-specific codec/restore diagnostics are retained.
One caller-explicit update boundary serves ordinary recovery calls and all seven
authorization/trap sites, preserving the outer structured Candid error.
Rust 1.91 recovery-target compilation and strict core/recovery Clippy pass.

One exact setup/admission substitute serves server, workflow and recovery-wrapper
qualification after the real-parser refusal cases. Input-refusal assertions now
require CLI admission before metadata access. Actual consumer-tooling fixtures
pass Bash 5 and genuine Bash 3.2 with optimizer/server effects substituted;
ShellCheck, formatting, diff and documentation links pass. The incoming Build
0.11 / Host 0.23.1 / Metrics 0.11.7 selection is retained; the capture tool is
rebuilt on Rust 1.91 and both consumer-tooling shells pass again against that
binary. The graph, snapshot, own versions and root index/refs are preserved. No full CI, live PocketIC,
release, publication or native macOS qualification ran.

Extraction/typed-management/collector evidence is in
`target/qualification/extraction-typed-management-collector/`. All 23 consumer
and 19 composition tests pass on Rust 1.91, including authorization before
corrupt decoding, per-operation refusal preservation, failed commits, returned
deadlines, exact receipts and blocked-effect recovery. Typed fixture operations
retain the same bounds, policies and stored JSON; no published API changes.
Strict native/Wasm consumer Clippy and separate Rust 1.91 core/consumer Wasm
checks pass. The initial Clippy callback-signature failure is retained; its
correction uses a named test-only function-pointer type. Formatting uses the
prepared development rustfmt; the minimum toolchain has no rustfmt component.

The exact pinned Shared producer's eight refused-store/compaction combinations
pass separately on Bash 5 and genuine Bash 3.2. The adopted collector matches
that pin byte for byte. Its generic regression already retains independent local
files, selection diagnostics and incomplete-collection refusal; no upstream
source change is required. Jobs removes its duplicate refused-store scenario
and keeps its actual CI gate/collector ordering and exact archive round trip for
formatting logs and retained inner fixtures. That caller passes both shells;
ShellCheck, formatting, diff and documentation-link checks pass. This supersedes
the earlier local collector roster below without relabelling earlier receipts.
The incoming graph, snapshot, own versions and root index/refs are preserved.
No full CI, live PocketIC, release, publication or native macOS qualification ran.

Upstream helper refresh evidence is in
`target/qualification/upstream-helper-handoffs/`. Registry inspection finds no
native Testkit package entry; published Timers 0.25.2 and Host Process 0.23.1
lack the proposed APIs. Active Jobs Rust, manifests/lock and snapshot are unchanged.
The apply-checked source patches select unpublished helpers only in a separate copy.
Its 23 consumer, 19 composition and four capture tests, recovery compilation,
separate Rust 1.91 core/consumer Wasm checks, consumer Wasm test compilation,
strict native/Wasm Clippy and actual tooling fixtures on Bash 5/genuine Bash 3.2
pass. Normal consumer Wasm dependencies exclude Testkit/PocketIC. The counter
replacement covers all four constructions and rejects overflow at all five
observations; deadline/work policy, trap-before-delivery and optimizer custody stay local.

Testkit PR #97 remains stacked on #88 at
`741e2e8e89cd1442e29dd3a8e1ff235260806bf0`; all four IO contracts, two allocator
tests, actual Rust 1.88 native/Wasm, strict Clippy and Rustdoc pass unchanged.
Host PR #133 now includes released 0.23.1, head
`68929d021232dcda14863a92af2839bf2012ee17`, with pending 0.23.2 notes and restored
published history. Both actual Rust 1.88 status tests, Clippy and Rustdoc pass.
Timers #64 has a refreshed, uncommitted patch on released 0.25.2
`19ed7612afd12f68ce2fe44ff7d96be926c9f290`, with pending 0.25.3 notes. Its nine
work/deadline combinations, actual Rust 1.91 Wasm, Clippy and Rustdoc pass;
its AGENTS.md still reserves contribution commits to the maintainer.
Jobs #52/#56/#57 retain reviewed publication, actual registry qualification and
delivered/native acceptance. No full CI, native macOS, live PocketIC, release or
publication ran. Sibling working trees and root index/refs remain unchanged.

Ticks/traps/schedules evidence is in
`target/qualification/ticks-traps-schedules-cleanup/`. Recovery reuses Testkit's
advance-then-tick helper with unchanged durations and tick counts. One recurring
schedule constructor serves five core test sites, retaining all policy cases,
one-shot management cases, zero deadlines, maximum intervals and fifty-cycle history.
All 25 Jobs and 10 management tests pass on Rust 1.91, as do recovery-target
compilation, focused core/recovery Clippy and the capture-tool build. The incoming
Testkit 0.49 manifest/lock selection is preserved; these receipts bind that graph.

Four direct EXIT traps retain their original labels and completion/failure policy.
Actual release/retention, formatting-evidence, consumer-tooling and formatting/hook
fixtures pass Bash 5 and genuine Bash 3.2. Exit probes retain nounset, premature,
failed completion and cleanup-refusal evidence; release gates and registry/upload
effects remain substituted in disposable repositories. Source-following ShellCheck,
formatting and documentation links pass. The snapshot, own versions, manifest/lock
selections and root index/refs remain unchanged. No full CI, live PocketIC, release,
publication or native macOS qualification ran.

Jobserver/corruption/policy evidence is in
`target/qualification/jobserver-corruption-policy-cleanup/`. Canonical preview and
export add only the pipe-jobserver probe at the existing reviewed revision; all
110 earlier payloads retain their exact bytes and executable modes. Formatting
now uses its descriptor identity, pipe and direction checks. Actual formatting/
workflow/archive fixtures pass Bash 5 and genuine Bash 3.2; ShellCheck, Perl
syntax, snapshot integrity and dependency pins pass.

The shared corrupted-queue assertion path preserves exact dispatch/reopen errors
and bytes, and checks summary/inspection rejection for all six corruption cases.
One valid retry-policy table retains getters, JSON and named-token contracts.
All four configuration tests and 23 consumer tests pass on Rust 1.91; focused
core/consumer Clippy, formatting and documentation links pass. The incoming lock
already selected Timers 0.25.2 / Metrics 0.11.6; an initial missing-cache refusal
is retained, followed by locked source preparation and passing offline checks.
Manifest/lock selections, own versions and the root index/refs remain unchanged.
No full CI, live PocketIC, release, publication or native macOS qualification ran.

Refusal/source cleanup evidence is in
`target/qualification/refusal-source-cleanup/`. One completion-case table retains
all six receipt/lookup/clock cases across both payloads and seven states. The live
recovery source shares management refusals across Running and Uncertain, retaining
cancellation and receipt-count checks and checking unchanged views in both states.
All 23 consumer tests, Rust 1.91 recovery-target compilation, focused consumer/
recovery Clippy and formatting pass. The live test was compiled, not executed.

[Shared PR #242](https://github.com/dragginzgame/shared-tooling/pull/242),
`75e9c9c12c9dacba602b62d1a89e5a4ee2a9a209`, adds exact lock/index preservation
after literal allowance and exact index/destination preservation after rename
refusal to the existing producer fixture. It passes Bash 5 and genuine Bash 3.2;
the production checker matches the active Jobs pin byte for byte. Jobs deletes
its duplicate direct-checker cases and retains actual release/metadata-adapter
dirty-source and failed-observation refusals before later effects. Actual Jobs
release/publication fixtures pass both shells with gate/registry/upload effects
substituted; ShellCheck and documentation links pass at both owners. Upstream
review remains with the PR. The incoming graph, snapshot and root index/refs are
unchanged. No full CI, live PocketIC, release, publication or native macOS
qualification ran; the Shared contribution used a separate clone/topic branch.

Capture/admission/target cleanup evidence is in
`target/qualification/capture-admission-target-cleanup/`. The combined capture
failure test proves producer/log retention, then exact retained bytes after
explicit producer removal. One admission table covers absent, symlink and
directory inputs at both ordered stages; capture admission receives valid Wasm.
All four capture tests, Rust 1.91 recovery-target compilation, strict capture
Clippy and the capture-tool build pass. The actual consumer-tooling fixture
passes Bash 5 and genuine Bash 3.2 with optimizer/server effects substituted;
ShellCheck, formatting and documentation links pass. Make still runs capture
tests before compiling only the recovery target with `--no-run`. The incoming
graph, active snapshot, own versions and index/refs are preserved. No full CI,
live PocketIC, release, publication or native macOS qualification ran.

Delivery/upgrade/formatting cleanup evidence is in
`target/qualification/delivery-upgrade-format-owner/`. Completion now performs
its active-attempt/task checks directly, retaining error precedence and exact
receipt comparisons. One result-returning fixture upgrade path serves success,
expected rejection and Testkit's existing selective retry. All 23 consumer tests,
Rust 1.91 recovery-harness compilation, separate core/consumer Wasm checks and
focused native/Wasm Clippy pass. Incoming Memory 0.44 / Build 0.10 edits arrived
during qualification and are preserved; `current-graph/` binds rerun receipts
to the unchanged manifest/lock across those checks. Earlier receipts retain
their original scope.

[Shared PR #240](https://github.com/dragginzgame/shared-tooling/pull/240),
`06e82e8a9cce6a7107735df4f824cf4dc6fcfe59`, owns standalone formatting success
and exact earlier-log preservation. Producer formatting and actual Jobs
workflow/archive fixtures pass Bash 5 and genuine Bash 3.2; ShellCheck,
formatting and documentation links pass. The wrapper matches the active pin
byte for byte; the transfer changes tests only and requires no runtime snapshot
adoption. Jobs retains actual Make, workflow ordering, archive round trips,
refused-store evidence and incomplete-collection checks. The active snapshot,
own versions and root index/refs remain unchanged. No full CI, live PocketIC,
release, publication or native macOS qualification ran.

Store/Make/publication cleanup evidence is in
`target/qualification/store-make-publication-cleanup/`. One raw-byte constructor
reconstructs native stores and clears volatile fault injection; consumer loads
retain bounded decoding and Job::restore. All 19 composition tests on Rust 1.91,
focused strict Clippy and formatting pass. The release fixture now uses the
copied production Makefile, substituting the validation runner instead of
maintaining another set of release recipes. Tool admission, source/metadata
refusals, recovery and exact publication arguments remain qualified locally.
Actual Jobs release/publication fixtures pass Bash 5 and genuine Bash 3.2;
source-following ShellCheck and documentation links pass.

[Shared PR #238](https://github.com/dragginzgame/shared-tooling/pull/238),
`5703391e88747e74c3eb5c863224a433943620a2`, transfers the generic publication
source/tag/destination and registry matrix into its existing producer fixture.
Both Bash 5 and genuine Bash 3.2 pass; source-following ShellCheck and documentation
links pass. All six publication runtime helpers match the active Jobs pin byte
for byte, recorded alongside the exact producer base/head. No runtime helper
or snapshot adoption is part of this test-only transfer; upstream review remains
with that PR. Actual Jobs adapter refusals and failed/uncertain upload checks
remain here. Focused fixture effects use disposable Git repositories and
substituted validation/registry/upload effects; no full CI, live recovery,
release, publication or native macOS execution was selected.

View/snapshot/probe cleanup evidence is in
`target/qualification/view-snapshot-cleanup-probes/`. Inspection/list replies now
derive their checked identity from the validated job at one view boundary; the
composition snapshot no longer derives unused cloning. All 23 consumer and
19 composition tests, separate Rust 1.91 core/consumer Wasm checks, native/Wasm
consumer Clippy and formatting pass on the preserved incoming graph.

[Shared PR #237](https://github.com/dragginzgame/shared-tooling/pull/237) adds
cleanup refusal to the standard exit-probe roster. Producer callers pass Bash 5
and genuine Bash 3.2. A canonical prospective 111-payload export at
`cd2a673353275d09be8223420b88e5a165a84534` passes integrity and the actual Jobs
release/publication fixtures on both shells with external effects substituted.
The apply-checked Jobs deletion is retained under `adoption/`; active Shared
0.15.2 still needs its explicit cleanup case. Reviewed export and delivered/native
acceptance remain with [Jobs #60](https://github.com/dragginzgame/ic-jobs/issues/60).
No active snapshot, root index/ref, own version, live recovery, release or
publication changed. The Shared contribution used its isolated PR workflow;
no sibling working checkout was edited. Linux Bash 3.2 is not macOS proof.

Capture/admission/roster cleanup evidence is in
`target/qualification/capture-admission-roster-cleanup/`. Recovery-only Candid,
Testkit, consumer and Serde edges are development dependencies; the capture
binary's normal graph excludes Testkit/PocketIC and the consumer package. Its
Rust 1.91 build and five tests pass, as does recovery-harness compilation on the
incoming Build 0.9 / Testkit 0.48 graph. Cargo discovery executes all 45 current
core integration tests without a manual target roster. The actual hosted Testkit
selection step uses the Make adapter once, keeping its checked path separate
from retained diagnostics; success/refusal and the remaining consumer-tooling
fixtures pass Bash 5 and genuine Bash 3.2 with tool/server effects substituted.
ShellCheck, manifest formatting and documentation links pass. Incoming root
manifest/lock selections and the shared snapshot are preserved. No live recovery
or full CI ran; native macOS acceptance remains separate.

[Delivered 0.13.0 CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38753013217)
passes Linux, macOS Apple Silicon and macOS Intel at
`123b8f0c9a5abcabd9498503d947d45908aa49df`. This supplies delivered/native acceptance for shared fixture allocation
and completion admission (#47/#48); it does not qualify later edits.

Current-graph focused Rust receipts are in
`target/qualification/dispatch-write-toolchain-cleanup/`: 19 composition tests,
23 consumer tests, five capture tests, harness compilation, strict Clippy and
separate Rust 1.91 core/consumer Wasm checks pass. Actual CI/recovery toolchain
inputs and release/evidence/consumer fixtures pass Bash 5 and genuine Bash 3.2.
Source snapshots bind receipts to their graph; earlier graphs remain historical.

Current tooling cleanup receipts are in
`target/qualification/release-frontend-handoff-cleanup/`. Canonical export preview
confirmed that `scripts/dev/format-frontend.sh` remains required by the adopted
`test-git-hooks.sh` frontend cases. It is retained with its explicit snapshot root;
removal requires splitting the canonical hook fixture first, not deleting a
snapshot payload. No runtime frontend formatting flow is enabled in Jobs.
Release/publication and consumer-tooling fixtures pass Bash 5 and genuine Bash
3.2 with external effects substituted. Hook qualification, snapshot integrity,
pins, release adapters, source-following ShellCheck and documentation links pass.
The first hook run caught a deleted-catalog fixture input; its failure is retained
and the corrected run passes. No Rust source changed in this tooling batch.

Dispatch/delivery/formatting cleanup evidence is in
`target/qualification/dispatch-delivery-format-cleanup/`, based on delivered
0.13.1. All 23 consumer tests pass, including idle zero-write, unchanged bytes,
exact receipts and refusal ordering. Separate Rust 1.91 core/consumer Wasm checks,
strict native/Wasm consumer Clippy and recovery-harness compilation pass. Actual
formatting/hook checks pass with prepared formatters; the Jobs caller also passes
on genuine Bash 3.2. Snapshot integrity, pins, release adapters, ShellCheck, Perl
syntax and documentation links pass. Existing test expectations follow the same
deadline/payload results; no new test flow, public API or storage layout is added.
The old graph-bound receipts above remain scoped to their recorded inputs.

Release/memory/watchdog evidence is in
`target/qualification/release-memory-watchdog-cleanup/`. Release preflight no
longer probes cargo-sort help after common-tool admission; actual release and
publication substitutes pass Bash 5 and genuine Bash 3.2, with ShellCheck passing.

The prospective memory owner is Testkit PR #97, not a second Memory API; Memory
#101 records that accepted decision. Exact Testkit head
`741e2e8e89cd1442e29dd3a8e1ff235260806bf0` passes all four memory IO contracts and
Rust 1.88 Wasm compilation. The Timers completed-batch proposal is refreshed
against delivered 0.25.0 `11e73bff33676e6d829baa502fb943ffa312b988`; its independent
work/deadline regression, Rust 1.91 Wasm and strict Clippy pass. Timers' explicit
maintainer-owned contribution-commit rule keeps that change as a tested patch
posted on its existing issue, without a contribution commit or PR.

Combined isolated Jobs adoption passes 19 composition tests, 23 consumer tests,
Rust 1.91 Wasm test compilation and strict consumer/composition Clippy. Counts
retain shared reset/backing and reject overflow; frozen image checks stay local.
The production consumer Wasm tree excludes Testkit/PocketIC. Qualification-only
manifests/lock and apply-checked source patches are under `adoption/`; the only
lock changes are the local Timers edge and optional native Testkit package.
These receipts qualify unpublished source, not active registry adoption. The
existing local counter/projection remains until reviewed delivery/publication;
[Jobs #56](https://github.com/dragginzgame/ic-jobs/issues/56) and
[Jobs #57](https://github.com/dragginzgame/ic-jobs/issues/57) own that acceptance.
No active path override, own version, primary index/ref, sibling working-tree, live
recovery, release or publication mutation ran. Incoming Host 0.21 selections
were preserved. The direct crates.io availability probe returned HTTP 403;
it establishes no package availability or absence.

Composition/restore/release cleanup evidence is in
`target/qualification/composition-restore-release-cleanup/`. The unchanged
19-scenario native composition harness and ordinary support modules now belong
to `apps/job-consumer/tests/`; core no longer has Memory/Timers dev-dependencies.
`test-consumers` and the separate composition MSRV check select that package;
`check-consumer` selects its library tests to avoid replaying composition.
The lock changes only the core local dependency row; package identities and all
external selections match this batch's incoming lock. Memory 0.43 / Build 0.8
catalog updates were incoming and are preserved.

Restore reuses its occurrence offset only after admitting origin bounds; checked
multiplication/addition and InvalidRecord refusals remain. All 45 core tests,
five doctests, 19 composition and 23 native consumer tests pass. Separate core/
composition Rust 1.91 checks, core/consumer Wasm and strict all-target Clippy pass.
Jobs retains eleven release/format/publication mode/control cases with unchanged
source/index/fetch/gate/state assertions. The exact pinned Shared producer's
exhaustive release and formatting matrices pass separately. Both actual Jobs
release fixtures pass Bash 5 and genuine Bash 3.2; ShellCheck, formatting,
snapshot integrity, pins, release adapters and documentation checks pass.
Earlier receipts and prospective caller patches retain their original graph and
paths; the watchdog caller patch is refreshed for the composition relocation.

Fixture/workflow/derive cleanup evidence is in
`target/qualification/fixture-workflow-derive-cleanup/`. The actual CI gate has a
stable `validate` ID; its run body and both collector bodies use the canonical
extractor. Jobs retains workflow ordering, temp-root, archive and failure-status
assertions. Active evidence fixtures pass Bash 5 and genuine Bash 3.2. All 23
consumer tests, Rust 1.91 consumer Wasm, recovery-harness compilation and strict
consumer/harness Clippy pass after removing five unused Clone derives. Candid
fields and stored codecs are unchanged. Release-retention fixtures, source-following
ShellCheck, formatting, snapshot, pins, release adapters and doc links pass.

Make-fixture bootstrap remains prospective: Shared PR #227 at
`aa401e37e1d87abd4329fd4c4986b5d88e7c6a4e` is still open. Both affected Jobs
fixtures pass both shells in an isolated copy using that helper and the current
common Make source; producer preparation/refusal tests also pass both shells.
The apply-checked caller patch and exact helper are retained under `adoption/`.
This does not qualify or adopt the complete proposed Shared revision. Active
copy lists await canonical export from reviewed source and actual requalification,
tracked in [Jobs #58](https://github.com/dragginzgame/ic-jobs/issues/58).

Release-owner convergence evidence is in
`target/qualification/release-owner-convergence/`. One installer substitute now
covers common leaf routing; Jobs retains late parallel admission and setup/check
refusals before gate effects. Adapter mismatch and missing-version preparation
checks reuse the existing patch-release repository. Shared PR #231's exhaustive
leaf fixture and PR #232's selected-source metadata fixture pass both shells;
the relevant production helpers match the active pin. Generic cache tests remain
active while the manual preparation block remains.

[Shared PR #233](https://github.com/dragginzgame/shared-tooling/pull/233),
`c00f3602bcdc193230acd2c54bd5a39680e23d08`, fixes shared preflight's lost
RELEASE_MAKE selection. Both producer shells pass selected paths with spaces and
failure propagation. Its prospective Jobs adapter and narrower cache fixture
are qualified through a canonical 112-payload export in isolation; the active
110-payload snapshot and preparation adapter remain unchanged pending reviewed
export, tracked in [Jobs #59](https://github.com/dragginzgame/ic-jobs/issues/59).
Earlier export/setup refusals are retained separately from corrected
qualification. No root index/ref, package version, sibling checkout, release or
publication mutation was performed; the scoped Shared contribution used its
isolated PR workflow.

Full local CI and live PocketIC were not selected for this batch. Linux Bash 3.2
execution does not establish native macOS acceptance. The last recorded live
recovery evidence is Jobs 0.8.1, source
`f9c0c0eb4e8363565be63f9434a0cf8b9940294b`,
[run 38289182820](https://github.com/dragginzgame/ic-jobs/actions/runs/38289182820):
84 passes across seven scenarios, four Wasm variants and three hosts. Current IC
recovery, upgrades and timer delivery still require explicitly selected live proof.
Retained pre-cut Memory installations still require explicit owner disposition;
no reset, migration or deployment ran.

## Upstream follow-up

GitHub owns live tracker status. Shared's dependency-order adoption (#53) has
completed delivered/native acceptance. The 0.16.0 adoption is delivered in Jobs
0.13.3; #54/#58/#59/#60/#61 retain its running native acceptance gate.

The old [Memory PR #105](https://github.com/dragginzgame/ic-memory/pull/105)
preparation is superseded by the active registry Memory 0.45 caller adoption
above. Jobs #50 still requires delivered/native acceptance; earlier exact-source
receipts retain their historical scope.
[Host PR #172](https://github.com/dragginzgame/ic-host-tooling/pull/172) supersedes
closed #133; Jobs #52 awaits reviewed publication and registry adoption.
[Testkit PR #117](https://github.com/dragginzgame/ic-testkit/pull/117) supplies the
reporter proposal, with Jobs #63 owning its publication/adoption gate.
[Testkit PR #97](https://github.com/dragginzgame/ic-testkit/pull/97) retains the
separate native memory-observer proposal; Jobs #56 still owns its adoption.
[Timers #64](https://github.com/dragginzgame/ic-timers/issues/64) retains the
completed-batch watchdog proposal; Jobs #57 awaits upstream delivery.
The source-incorporated rejection/retention preparation PRs #51/#62 are closed.
[PR #55](https://github.com/dragginzgame/ic-jobs/pull/55) remains the existing
install-diagnostics contribution; its maintained change is prepared locally for
the current pending minor on the published registry graph. It is distinct from the older PR branch's
qualification-only source overrides.

Earlier source-bound receipts and original tracker descriptions are preserved in
[history through 0.13.0](history-through-0.13.0.md). See also the
[package contract](../../crates/ic-jobs/README.md), [safety contract](../../SAFETY.md),
[consumer qualification](../../apps/job-consumer/README.md) and
[delivery procedure](../releasing.md).
