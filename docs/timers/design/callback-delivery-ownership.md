# Callback delivery ownership

The [architecture](../architecture.md) and [safety boundary](../../../crates/ic-timers/SAFETY.md)
define current behavior. The first section summarizes the delivered 0.32.2
cleanup; later sections preserve historical implementation and evidence records
for their named releases, including APIs and dependencies that no longer exist.
They do not select current versions or establish deployment qualification.

## Effect-selected handle cleanup

The 0.32.2 cleanup supersedes the historical detachment/restoration stages below.
The registry applies a transition before selecting the handles that its effect
will clear: replacement arms select the wakeup, explicit clears select their
named roles, and Watchdog dispatch selects both. Initial arms and no-effect
transitions leave existing capabilities bound. A removed entry transfers all
its handles through the transition so runtime cleanup still precedes capture
release outside registry access.

Scheduler delivery consumes its exact fired handle before the transition, but
remaining handle selection follows the transition, including terminal removal.
Work completion uses the same selection, retaining its separate trap/IC rollback
contract. The runtime has no provider-handle restoration path. Retained successors
and queued work are never reinstalled merely to finish another transition. Cleanup
carries raw linear handles, without constructing tokens solely for restoration;
only native stale-delivery fixtures construct a token when taking a live handle.

Native regressions inject a binding failure after the one real scheduler repair
and before no-effect completion; unaffected work/successor ownership must survive
without consuming it. Installed recovery, cancellation and callback-cleanup
subjects remain required evidence for the IC paths.

## Prepared Metrics 0.10 adoption

The maintainer-selected root graph now uses Metrics 0.10. Its public
`MeasurementSummary` removes the count/total/maximum/mean forwarding methods;
instruction reports use `summary.aggregate()` while ordered `latest()` remains
summary-owned. Library tests, native projections and both Wasm probes use that
same split. Memory-page summaries and already aggregated interval values retain
their own accessors. No provider, recording or callback-admission logic changes.

Because Timers publicly re-exports this shared type, the prepared release moves
to 0.23.0 rather than patching the published 0.22 API contract. The existing
pending tooling batch stays in that one section; Cargo versions and lockfile
selections remain maintainer-owned. Consumers should use the matching Metrics
type or `ic_timers::MeasurementSummary`, with no forwarding compatibility layer.
The earlier 0.9 qualification below remains bound to its original graph.

The selected Metrics 0.10.0/Testkit 0.43.0 graph passes 155 native library tests,
eight doctests, strict library/probe Clippy and Rust 1.88 native/test-target and
Wasm compilation, including the runtime and all four policy probes. Format and
diff checks pass. Commands and before-images stay at
`/tmp/ic-timers-metrics010.6r6g297m/`. These are focused native/compile results;
no live PocketIC, full release gate or new footprint measurement ran. The
maintainer's Cargo inputs remain unchanged.

## Prepared 0.22.6 identity sharing and probe cleanup

Timer identities share one immutable component allocation through `Arc`. Cloning
no longer copies the three strings. Validation, component access, value-based
equality/ordering/hashing, debug fields and public auto-traits remain unchanged.
One private component type preserves named fields; registration claims and
callback generations still supply authority independently of storage sharing.
Snapshots remain inert and retain no callbacks or provider handles.

The cancellation cohort now queries its exact registration's armed wake-up,
trapping on lookup failure instead of silently treating it as an absent snapshot.
Its changed flag describes an armed wake-up, not every possible logical deadline.
The maintained cohort boundary observes completed work before cancellation;
repeated cancellation and subsequent time advancement must remain inactive.
Failure probes share checked window construction and one pre-failure schedule
proposal, retaining independent half-open windows and trap precedence.

Compared with `6ced4341bacce7d9f08acfd9a261a6d222755b93` using the preserved
incoming lock (`5f9ed11441775b7eb6dc20eade12a1dfe29ac06dafb65dcf4d5e78526896a019`),
Rust 1.88.0 and the unchanged thin-LTO `timer-probe` profile:

| Raw linked artifact | Before bytes | After bytes | Delta |
| --- | ---: | ---: | ---: |
| Inactive baseline | 263,403 | 265,217 | +1,814 |
| Once | 324,779 | 321,107 | -3,672 |
| AfterCompletion | 325,524 | 321,955 | -3,569 |
| Watchdog | 326,859 | 322,864 | -3,995 |
| Runtime recovery probe | 639,519 | 626,924 | -12,595 |

On admitted PocketIC 16.1.0, cancellation intervals fell from 16,335 to 6,862
instructions for Once, 26,003 to 11,845 for AfterCompletion, and 23,020 to 9,039
for Watchdog. Watchdog scheduler samples fell from 21,494 to 15,321 instructions.
All other measured active-cohort start, duplicate ensure, snapshot, inventory and
work intervals decreased. These are probe subjects, not whole-application savings
or a new optimizer policy. The graph selects Metrics 0.9.6 and Testkit 0.42.0.

A separate native Rust 1.99 allocation measurement for 1,000 clones of
`("owner", "subsystem", "name")` fell from 3,000 allocations/18,000 requested
bytes to zero. The native identity value shrank from 48 to 8 bytes. Initial
construction instead uses four allocations/82 requested bytes versus three/18;
preallocated vector storage is excluded. These are native allocation/layout
observations, not IC heap-page measurements. Sharing each string separately was
also measured and rejected because all four Wasm artifacts grew.

The 155 native library tests, strict library/probe Clippy, minimum-Rust native/Wasm
compilation, policy cohorts and all 16 runtime PocketIC subjects pass. Final cohort
hashes equal the measured identity candidate's hashes. Artifacts, failed sandbox
socket attempt, allocation source, commands and logs remain at
`/tmp/ic-timers-identity-reduction.y7d3dr0q/`. The real Cargo inputs are preserved;
no full release gate, macOS qualification, setup or release execution ran.

## Prepared 0.22.1 relative-deadline cleanup

Callback retries now use the existing relative `TimerSchedule` resolution path,
then retain their `Retry` mode. Both paths encode `Duration` before checked
deadline addition; callback resolution still maps failures into terminal control
errors, while explicit scheduling returns its input-boundary `ScheduleError`.
The supplied time sample and requested relative delay are unchanged. Registry
arbitration still discards losing or terminal proposals before resolution.

The existing schedule/directive fixture covers a normal relative retry, zero
delay at `u64::MAX`, and a maximal encodable delay from zero, alongside its prior
out-of-range duration and deadline-overflow cases. Expected absolute deadlines,
requested-delay metadata and Retry mode remain explicit. No new helper, type,
public API, provider operation or persistence path is introduced. Native/test
target and Wasm compile evidence belongs in the current handoff; test execution
and full owner qualification remain separate. No Wasm or instruction saving is
claimed from this source simplification.

## Whole-canister Wasm experiments

The maintainer approved isolated optimized builds on 2026-10-13 to investigate
code complexity, types and Wasm size. All four size-probe cohorts were compared
at released 0.21.4 source `44935d6188a0da82314e00a56f5f68a9d0c59638`, using
Rust 1.88.0, the frozen release lockfile (Metrics 0.8.3), one source path, and
locked/offline inputs. Baseline, Once, AfterCompletion and Watchdog were built
separately with `timer-probe`, whose other settings remained unchanged.

Full LTO reduced raw linked-canister Wasm by 18.67–18.85%, or 49,849–61,093 bytes.
Export names matched. These artifacts include Candid, CDK and dependency code;
the result does not isolate ic-timers' contribution or qualify larger apps.
There was no final optimizer, PocketIC execution or instruction/heap measurement.
The experiment left local profiles and runtime unchanged. The incoming Metrics 0.8.4
selection is outside this frozen comparison and remains preserved.

The reusable recipe and measurement table belong to
[IC Build's deployable-Wasm guidance](https://github.com/dragginzgame/ic-build/blob/main/docs/artifacts.md#deployable-wasm-lto).
Complete applications should compare their final transformed artifacts and
instruction costs before a common build adapter adopts full LTO. IC Timers keeps
its existing policy/recovery cohorts as timer-semantic evidence; a probe size
win does not establish an application release default.

Two private source experiments produced byte-identical Wasm to baseline in all
four cohorts: direct recording from the callback measurement start, and a
non-generic identity validation core. The former is a possible small source
simplification; neither supplies size evidence. No public state/type collapse is
justified by these measurements. The retained experiment directory is
`/tmp/ic-timers-reductions.NeY8gjCm/`, with `results.tsv`, complete build logs,
variant patches, compiler identity, artifacts and preservation checks. It is
local evidence, not a required input to normal builds or deployment.

## Prepared 0.22 callback measurement simplification

The runtime now records directly from the captured instruction and memory start
after callback completion. It still closes the instruction interval before
reading the end memory extent, then takes the same registry mutation boundary.
Only accepted callback paths reach recording, and stale registration claims are
still rejected by the registry before any accounting mutation. Ordinary work,
Watchdog scheduling and Watchdog work retain their existing recording sites and
trap context. There is no new public API, state, dependency or profile selection.

The identity-reuse fixture now reaches this production recording method using
the native platform's end counters. It retains checks after declaration removal,
after replacement scheduling and after replacement completion, plus the normal
seven-instruction/two-Wasm-page/three-stable-page callback assertions. The earlier
four-cohort experiment produced byte-identical Wasm for this production change
at frozen 0.21.4 inputs; it does not qualify the newer selected dependency graph.
Current compile-only checks and any owner-authorized test execution are recorded
in the handoff. Full native/IC/release qualification remains user-owned.

## Prepared Metrics 0.9 adoption

The incoming root catalog and lock select registry Metrics 0.9.0. The cached
official archive matches lock checksum
`f37740e17c25ed7792cad59abf4f72670ca29b85b2ae9d1f8cd022ff1b6a7e18`
and records producer source `fe4d14f2a24138794878a17a354885a1e7222487`.
Timers' public `MeasurementSummary` re-export now has that package identity.
Consumers exchanging it must coordinate the Metrics 0.9 selection, or use the
Timers re-export. Actual-overflow accessors use `*_overflowed`, and error matches
must follow Metrics' renamed cases and deleted unreachable mean/interval cases.
No old-name adapter or second Metrics identity is retained.

Existing zero-sample and wide-total fixtures now assert the new overflow flags,
retaining exact totals, means, latest/maximum ordering and scheduler/work
attribution. Timers' separate conservative event/page-counter saturation rules
are unchanged. The runtime still records through the same registration claim
and preserves sampling order. These Metrics changes alter the public dependency
contract, so the complete prepared cleanup/tooling batch uses undated 0.22.0
notes; Cargo package versions remain maintainer-owned. Timers retains no stable
measurement format to migrate. New compile evidence belongs in the handoff;
prior Metrics 0.8 executions do not qualify this graph.

## Consume the delivered handle through the selected entry

Runtime work delivery previously called registry-level handle consumption and
then work acceptance. Each operation independently selected the same entry.
`begin_ordinary` and `begin_watchdog_work` now consume the delivered handle through
their selected entry before accepting work. Runtime dispatch calls that one
operation and still releases the registry borrow before invoking consumer work.

The shared private `Entry::consume_provider_handle` owner verifies claim, policy
role and owned slot generation. It consumes a fired handle independently of
whether control state accepts work, preserving the previous delivery ordering.
An older generation or superseded claim cannot consume the replacement's handle.
Acceptance still independently checks work role, claim, generation and state,
records rejection or start, and returns only the matching callback. The old
registry-level consumption method and single-caller slot accessor are deleted.

## Detach Watchdog scheduler capabilities before the transition

Watchdog scheduler delivery must detach the remaining handles before its registry
transition can remove a transient declaration. This remains a separate stage from
work acceptance. `take_watchdog_scheduler_handles` now consumes the delivered
scheduler handle and detaches those remaining capabilities through one selected
entry, replacing the runtime's two registry operations and stale-claim match.

Missing and superseded claims still yield empty handles as normal stale delivery;
other registry errors retain their typed propagation. The scheduler transition
still follows detachment under the same runtime borrow, with unchanged time
sampling, allocation order, unacknowledged accounting and terminal cleanup.
Only runtime invokes provider clearing or arming after releasing the borrow.
Public control restoration, effect confirmation and Watchdog callback rollback
remain distinct and unchanged.

The change removes repeated entry selection and a separately callable consumption
phase, without new state, token types or a callback framework. No timing, size or
performance result is claimed. It is private and behavior-preserving: public APIs,
snapshots, dependency versions, generation sequences and persisted formats are
unchanged. No generated artifact or downstream adapter needs updating, and
downstream work remains deferred.

## Focused verification

Existing fixtures were updated rather than adding a test suite:

- `provider_roles_keep_paired_handles_distinct_and_reject_policy_mismatches`
  exercises the entry's consumption owner for all three provider roles. It now
  rejects a mismatched claim with the same role and callback generation, alongside
  its existing stale-generation and cross-policy role cases. It retains paired
  slot identity, rejected-binding cleanup and mock provider-count assertions.
  The expanded ownership matrix has a justified local line-limit expectation.
- `stale_reused_identity_callback_cannot_change_handles_or_measurements` queues
  the old delivery through the mock provider and real runtime dispatch. It
  requires a stale event without a work start, replacement handle consumption or
  measurement changes, then executes the replacement normally. Its independent
  late-measurement boundary check remains.
- `suspended_after_completion_uses_completion_time_and_exact_reconciliation`
  now checks that the registration owns no wakeup after work suspends. Existing
  suspended Once coverage already checks this ownership boundary.

Existing Watchdog prearming, continuation, cancellation, dispatched recovery,
terminal failure/lifetime and provider failure fixtures remain applicable. Native
mock behavior does not establish IC rollback or provider-heap results.

Tests, builds, lint gates and deployment validation remain maintainer-owned and
have not been run. Before the Shared Tooling adoption, changed Rust formatting,
diff whitespace, the then-current prepared release metadata and release-truth
checks, and locked offline metadata checks passed. Those checks do not validate
runtime behavior or the subsequently changed release tooling; its verification
is scoped in the [adoption record](../development.md). Cargo versions and lockfiles
remain unchanged. The root changelog owns the maintainer's release selection;
this document does not select a version or establish delivery.
All version bumps and Git/release execution remain maintainer-owned.


## Capture removal and coalesced requests

The 2026-10-06 source review at release commit
`134899f1620b29f51711ff479c37c681db3fb9ec` found that ordinary abandonment
returned removed callbacks after its registry borrow, but normal cancellation,
unregistration, transient control failure and rejected registration could drop
consumer captures inside `RUNTIME`'s mutable borrow. A nontrapping destructor
calling `timer_inventory` observed `RuntimeBusy`; cleanup that consumed another
claim could lose its control capability after that rejection.

The released 0.13.3 implementation carries a removed entry in its existing
`RegistryTransition`. Runtime applies provider effects before dropping that
transition, outside registry access. Rejected public registration retains a
local `Rc` until registration access ends. This is temporary destructor custody,
not a retained timer, retry queue or new mutation authority. Ordinary abandonment
keeps its separate provider-call-free cleanup contract. Capture destructors
must still be bounded and nontrapping, and must not schedule provider work from
CDK cleanup.

Public control now validates its claim before applying the pure transition.
Rejected requests leave owned handles intact. A no-effect transition retains
those handles in the selected entry; a removal transfers the entry and detaches
its handles from that returned owner; other effects detach handles after the
transition. No user callback runs in between. Watchdog callback completion keeps
its existing explicit detachment and trap/rollback rule.
The obsolete detached-error restoration finalizer is removed: input rejection
no longer creates detached capabilities needing restoration. Binding/restoration
errors after an actual provider effect retain their existing cleanup owners.

The release baseline's successful Apple Silicon gate reports these instruction
subjects from the maintained size probe:

| Policy | Initial arm | Duplicate ensure |
| --- | ---: | ---: |
| Once | 24,006 | 9,512 |
| AfterCompletion | 27,695 | 11,496 |
| Watchdog | 24,924 | 10,872 |

These are operation intervals from the 0.13.2 artifact. A bound handle previously
copied three boxed identity components to
construct its detached token. An ordinary coalesced ensure removes that temporary
allocation/deallocation and reinstallation; paired Watchdog requests remove two
such copies. The same platform operations were already avoided by coalescing.
The subsequent 0.13.3 measurements below quantify operation costs; linked Wasm
savings are not established by either run.
A removal now makes one temporary boxed-entry allocation so ordinary transition
values stay small; the allocation lasts only until provider effects finish.
Successful registration adds one short-lived `Rc` increment/decrement, with no
additional callback allocation. There is no persistent heap-growth claim.

Native fixtures cover removed captures inspecting the absent entry, reusing its
identity, and mutating timers after provider cleanup across all three policies;
rejected factories; transient binding/scheduler failures; and rejected/coalesced
requests retaining installation-fault injections until a real arm. PocketIC adds
same-message armed removal for all three policies and checks registry access
from capture Drop in existing ordinary abandonment subjects. No provider work is
scheduled by those PocketIC destructors.

Initial preparation had source and Rust formatting/parsing review only. The
maintainer's released source `864397a7c21eec4f396fe9617dbb8d8e1f9cfc73`
subsequently passed [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37430538342),
including Linux checks, MSRV and both native macOS complete release gates.
The [Apple Silicon job](https://github.com/dragginzgame/ic-timers/actions/runs/37430538342/job/112160114006)
records 142 native tests, doctests, all 14 maintained PocketIC runtime subjects,
and policy cohorts passing with the audited PocketIC 16.0.0 artifact. This
includes the added capture-release subjects; native substitute and actual
canister evidence retain their separate contracts.

| Policy | Initial arm, 0.13.3 | Duplicate ensure, 0.13.3 | Cancel, 0.13.2 → 0.13.3 |
| --- | ---: | ---: | ---: |
| Once | 24,934 | 6,617 | 16,800 → 16,806 |
| AfterCompletion | 28,837 | 8,344 | 25,313 → 26,600 |
| Watchdog | 25,880 | 7,878 | 21,344 → 22,492 |

Compared with the preceding source-bound probe, duplicate ensures cost about
27–30% fewer instructions, while initial arming costs about 4% more and recurring
cancellation about 5% more. These are individual probe operation intervals,
including their brackets, not total-message or application throughput results.
Snapshot/inventory subjects are unchanged; executed-work subjects are slightly
lower. The correctness fix has an observable cold/control-path cost, so it
should not be described as a universal speedup. No heap bound or linked Wasm
reduction follows from these results.

Neither hosted log emitted linked byte sizes. The 0.13.4 host-only cohort change
adds `wasm_bytes` from the exact vector installed into PocketIC to the existing
row, including the baseline cohort. The four artifacts retain the same build
profile/toolchain and can be compared without a second file read or extra build.
At preparation, that output had not been executed; it cannot retroactively
establish sizes for 0.13.2 or 0.13.3. No target Wasm or runtime instrumentation changes.
Public APIs, snapshots, recurrence, generation allocation, dependency selection
and upgrade reconstruction are unchanged; 0.13.3 is a compatible fix. No new
feature, persistence, interval provider, scheduler data structure or global
control API is justified by the reviewed consumers.

## Released 0.13.5 cohort review

The completed [Apple Silicon gate](https://github.com/dragginzgame/ic-timers/actions/runs/37442170942/job/112198306109)
and [Intel gate](https://github.com/dragginzgame/ic-timers/actions/runs/37442170942/job/112198305932)
for `98c4b296d7461525c15a01e30adbe33b75bcfa38` record exact installed byte
sizes for the four existing cohort artifacts, built with Rust 1.88 and the
testing workspace's `opt-level = "z"`, thin LTO and stripped release profile.
The probe lock selects registry `ic-metrics 0.1.6`; the root lock separately
selects compatible 0.1.7. Neither graph qualifies subsequent worktree edits.

| Cohort | Apple Silicon Wasm bytes | Intel Wasm bytes | Bytes above each host's baseline |
| --- | ---: | ---: | ---: |
| Baseline | 263,433 | 262,094 | 0 |
| Once | 316,941 | 315,602 | 53,508 |
| AfterCompletion | 317,563 | 316,224 | 54,130 |
| Watchdog | 318,293 | 316,954 | 54,860 |

Baseline already imports IC Timers initialization, snapshot and inventory APIs,
as well as the shared Candid/CDK probe surface. These are feature-reachability
comparisons within this probe, not the total size added by linking IC Timers into
an arbitrary canister. No historical Wasm reduction can be calculated from logs
that did not record byte sizes.

Every previously emitted cohort subject matches the inspected 0.13.3 Apple
Silicon row exactly: initial arm, duplicate ensure, cancellation, snapshot,
inventory, measured scheduler/work, memory-sampling brackets and dispatch cycles.
Both Watchdog calibration rows also match. The maintained probes therefore show
no instruction regression or new instruction saving in these intervals. Runtime
and probe canister sources are unchanged between those commits; this supports
keeping the current design rather than attributing release-tooling work to a
runtime optimization.

The Intel instruction and sampling fields match Apple Silicon, including both
calibration rows. Absolute Wasm sizes differ by 1,339 bytes across every cohort,
but the baseline-relative differences match. Recorded dispatch cycles differ:
Once/AfterCompletion/Watchdog are 16,780,004/16,788,704/33,578,100 on Intel versus
15,700,004/15,708,704/31,418,100 on Apple Silicon. The calibration cycle rows
also differ. These host-specific observations are retained separately; this
review establishes neither the cause nor a cross-host cycle regression.

The zero measured memory-sampling difference is limited to its bracket, and
dispatch cycles include the driven-round interval. Neither establishes zero
measurement cost universally or total-message instruction consumption, which
the calibration explicitly reports as unavailable. Watchdog is only 730 bytes
larger than AfterCompletion in this build; deleting its distinct prearmed recovery
semantics is not justified by that size difference. No further runtime change
or performance release is supported by this review.

## IC Metrics 0.6 released graph

Released IC Timers **0.17.7**
`1d21b9f55a8d9121355d216471495f92b0db7865` selects one registry Metrics **0.6.0**,
Testkit **0.33.2**, all four Host packages at **0.12.10** and PocketIC **16.1.0**.
The root catalog and lock already contain those selections. Consumers exchanging
instruction summaries use that same Metrics 0.6 identity or
`ic_timers::MeasurementSummary`; the old Metrics 0.5 type is distinct.

Metrics 0.6 tracks actual count/total overflow in ordered summaries. An exactly
reached `u64::MAX` can still yield a valid mean. After overflow the checked mean
fails while latest/maximum observations remain available. Timers retains ordered
scheduler/work summaries; no unordered aggregate replaces them. Its event counters
and page-sample counts keep their conservative numeric-cap contract. Comparing
roles, units, registration identity and append-only continuity is still required
before deriving an interval; numeric admission cannot detect replacement followed
by larger values. These are inert observations, not runtime mutation authority.

The change of public Metrics identity and mean semantics was delivered in the
**0.17.7 patch**, contrary to the local pre-1.0 minor-boundary rule. Record this
mistake without restoring a shim or rewriting released history. Future public
type or semantic hard cuts require the next minor line. Released **0.17.8** adds
tests and accurate guidance for the behavior already selected; preparation did
not make another public change or alter Cargo metadata. The maintainer's delivered
lock selects Metrics **0.6.1**, Testkit **0.33.3** and Host **0.13.0**.

[Issue #43](https://github.com/dragginzgame/ic-timers/issues/43) owns consumer
qualification, with synthetic instruction-accumulator boundaries and a native
completion/control/replacement/reset sequence. Metrics owns private count-overflow
construction and arithmetic internals. Native runtime reset is deterministic
test evidence, not an IC upgrade/recovery guarantee. Existing PocketIC recovery
and cohort gates remain required for their own claims. The
[0.17.8 release evidence](../evidence/release-history.md#0178-release-evidence) records actual
execution of the new cases on Linux and both native macOS architectures. No local
tests are rerun during inspection; earlier Metrics 0.5 qualification remains
source-bound below.

## IC Metrics 0.5 released graph

Released IC Timers **0.17.0** at
[`5e0d0865248f6ebfc1f98c896581f21e2ce67831`](https://github.com/dragginzgame/ic-timers/commit/5e0d0865248f6ebfc1f98c896581f21e2ce67831)
selects one registry Metrics **0.5.0**, Testkit **0.31.0**, all four Host
**0.11.0** packages and PocketIC **16.1.0**. This supersedes the 0.4/0.30
preparation below; the finalized historical release notes retain their original
description. Current consumers exchanging measurement values must select the
same Metrics package identity or use `ic_timers::MeasurementSummary` directly.
A matching struct definition in Metrics 0.4 is still a different Rust type.

All seven published Metrics 0.5 Rust files match committed
[`01549632c0c3fa6e1ff315ce4de803ddfae904ad`](https://github.com/dragginzgame/ic-metrics/commit/01549632c0c3fa6e1ff315ce4de803ddfae904ad);
its arithmetic source is unchanged from 0.4. All 40 published Testkit 0.31 Rust
files match committed
[`f1ae9e6d3b0f3f20ec1e1f1b49c8b1dea3155e0a`](https://github.com/dragginzgame/ic-testkit/commit/f1ae9e6d3b0f3f20ec1e1f1b49c8b1dea3155e0a).
Its packaged CLI lock also selects Host 0.11. No timer adapter, arithmetic reader
or direct Host dependency is needed; Host/Testkit remain outside the production
library's dependency closure. Host 0.12's availability does not change Testkit's
selected 0.11 requirement or justify a second native dependency route.

Full locked offline metadata and source comparisons are retained under
`/tmp/ic-timers-shared031.aFkkRB/`. Read-only inspection of clean IcyDB source
`736c583ebf27097826517a0010b8e6ae87a811ad` finds declarations for Timers 0.17 and
Metrics 0.5; its root lock selects one Timers 0.17.0 and one Metrics 0.5.0. This
removes the reported duplicate Metrics selection in that graph without claiming
new downstream compilation or changing sibling files. Existing released Linux library, recovery/cohort,
MSRV and tag results are recorded by the
[release owner](../evidence/release-history.md#0170-release-acceptance). Both macOS jobs now pass;
their downloaded logs establish complete source-bound qualification for
[#37](https://github.com/dragginzgame/ic-timers/issues/37). No Wasm/instruction delta is
measured by these source/graph checks, and no contributor test/build ran.

## IC Metrics 0.4 adoption

Incoming maintainer catalog/lock edits after released **0.16.7** select Metrics
**0.4.0**. Its reviewed published release is
[`97ec991776bb16efbab59faeb43bc72ce9ebcefa`](https://github.com/dragginzgame/ic-metrics/commit/97ec991776bb16efbab59faeb43bc72ce9ebcefa).
All seven published Rust source files match that commit and 0.3.7 byte for byte.
The dependency-free arithmetic and consumer-owned instruction reader require no
adapter. [#37](https://github.com/dragginzgame/ic-timers/issues/37) records the
real downstream mixed 0.3/0.4 graph that motivates package convergence.

IC Timers publicly re-exports `MeasurementSummary`; its 0.3 and 0.4 identities
are different Rust types despite identical arithmetic. The then-pending
**0.17.0** covered that public hard cut and the tooling-contract change. Its final
Metrics 0.5 selection is recorded above; this paragraph records earlier preparation.
That preparation selected one Metrics package, with no
alias, second dependency route, persisted layout or timer lifecycle change.

Complete locked offline metadata at `/tmp/ic-timers-shared030.roCrPG/metadata.json`
selects Metrics 0.4.0, Testkit 0.30.0, Host 0.11.0 and PocketIC 16.1.0. The incoming
catalog and all four local lock entries report 0.16.6 although the released HEAD
is 0.16.7; these maintainer-owned bytes are preserved. Contributor preparation
does not resolve that release metadata difference or bump package versions.
The user-operated minor preparation must select 0.17.0 before release.

Source equality gives no expected algorithmic instruction or heap change, but
no Wasm/instruction delta is measured for the new package graph. Source/metadata
review is not execution qualification; the normal user-operated gate retains
library, MSRV, probe, recovery/cohort and native host checks. #37 stays open for
delivery and qualification; no product test/build/lint ran for this review.
[Alignment feedback](https://github.com/dragginzgame/ic-timers/issues/37#issuecomment-6096552294)
records the preserved incoming selections and minor boundary.

## IC Metrics 0.3 adoption

The incoming root catalog selects registry `ic-metrics 0.3`; the one root lock
selects 0.3.0. This adoption is prepared for **0.16.0**. Released 0.15.0 at
`1410415d41212385234509524480186074972764` still selects Metrics 0.2.20.
The four local packages remain at 0.15.0 until maintainer-owned version preparation.

The downloaded registry 0.3.0 package's eight `src` files are byte-identical to
the owner's tagged 0.2.20 and 0.3.0 source trees. Metrics 0.3.0's release commit is
`070c768de881a6e966b93651e242e16e4f9f1111`. The crate remains dependency-free
and `no_std`; summary arithmetic, attribution and the consumer-owned instruction
reader require no implementation changes.

IC Timers publicly re-exports `MeasurementSummary` and returns it from
`TimerPerformance::scheduler_instructions` and `work_instructions`. Rust treats
the 0.2 and 0.3 package types as distinct despite identical source. Consumers
exchanging these values with a direct Metrics dependency must select 0.3 or
use `ic_timers::MeasurementSummary`. The next minor boundary covers that public
identity change. There is no old-type alias, dual dependency route or fallback;
this change introduces no persisted layout or timer lifecycle change.

Locked offline metadata on 2026-10-09 resolves exactly one registry Metrics
0.3.0 and all four local members through the root graph. Source comparison and
metadata evidence are retained at `/tmp/ic-timers-metrics03-current`'s referenced
directory. The checks preserve the incoming manifest and lock bytes. No new
test, build, lint, PocketIC, cohort or release execution ran for this preparation.
The maintainer-operated complete release gate must qualify the selected graph;
0.15.0's hosted results do not qualify Metrics 0.3.0.

Unchanged arithmetic gives no expected algorithmic instruction or heap delta.
No measured Wasm or instruction delta is available for this graph; package
identity and build inputs prevent claiming binary identity from source equality.
The release adds no runtime mechanism or speculative Metrics API.

The maintainer subsequently released the cut as **0.16.0** at
`984c2f0a92f7e3ebde604f88895b12fb2b78cb2c`, selecting registry Metrics **0.3.1**.
Its eight `src` files match tagged 0.3.0 byte for byte, preserving the reviewed
arithmetic. Exact-source [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37923550864)
passes Linux/MSRV and [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37923551348)
passes; both complete native macOS gates remain queued at inspection. No fresh
cohort measurement is claimed. The earlier 0.3.0 metadata evidence remains
preparation evidence with its original graph.

## IC Metrics 0.2 adoption

The maintainer requested registry `ic-metrics 0.2` after publication. The root
catalog and lock already selected 0.2.0 when this continuation inspected them.
The inherited library declaration stays `workspace = true`, without the removed
`ic` feature. The nested probe workspace reaches that same declaration through
its path dependency; its separate lock is updated narrowly from 0.1.6 to 0.2.0
with `cargo update --manifest-path testing/Cargo.toml -p ic-metrics@0.1.6
--precise 0.2.0 --offline`. Every unrelated testing lock record and the existing
root lock are preserved. Both select registry checksum
`e6df432373e44c1956cbaa15548625efbebdbad4070a916e9fe0c7e4359f6265`.

The downloaded registry package is dependency-free, `no_std`, edition 2024 and
MSRV 1.88. Its `src/summary/mod.rs` is byte-identical to downloaded registry
0.1.6. The previously prepared production adapter reads
`ic0::performance_counter(1)` directly; the test-only native substitute remains
consumer-owned. This is source comparison, not new execution or measurement.
The earlier 0.1 extraction checks and released 0.13.5 cohorts retain their
original dependency identities and do not qualify this selection.

IC Timers exposes the shared type through its facade and the
`TimerPerformance::scheduler_instructions` / `work_instructions` return values.
Despite identical arithmetic, `ic_metrics 0.1::MeasurementSummary` and the 0.2
type are different Rust types. For example, passing a timer performance summary
to a function accepting the old direct-dependency type can stop compiling.
The [Cargo resolver's version-incompatibility guidance](https://doc.rust-lang.org/cargo/reference/resolver.html#version-incompatibility-hazards)
explains this boundary. The complete unpublished batch moves to 0.14.0 under
the pre-1.0 compatibility rule. Consumers exchanging these values must align
their direct dependency to 0.2 or use `ic_timers::MeasurementSummary`; no old
identity alias, dual reader or fallback is retained.

Both owning locked/offline metadata checks pass and dependency-tree inspection
finds one registry 0.2.0 package per workspace. Diff/source checks pass. No new
test, build, lint, PocketIC, cohort or release command was run during this
adoption; those remain maintainer-owned. There is no claimed Wasm or instruction
saving from moving the reader or updating the dependency. The scope addresses
[ic-metrics #10](https://github.com/dragginzgame/ic-metrics/issues/10).

The maintainer subsequently released this adoption as 0.14.0 at
`902323a9e896ce3771044fdc23a7a2d03d49cf28`. Cargo, both local-package lock
entries and tag `v0.14.0` agree. Its matching tag CI, main Linux/MSRV and Apple
Silicon complete gate passed; Intel was unfinished when inspected. The
[host record](../development.md) keeps exact run links and scope.
The earlier preparation checks remain separate from that hosted execution.

The completed [0.14.0 Apple Silicon gate](https://github.com/dragginzgame/ic-timers/actions/runs/37476635415/job/112313644296)
subsequently supplied that comparison. Its four cohort and two calibration rows
match the recorded 0.13.5 Apple Silicon rows exactly, including Wasm sizes,
operation instruction fields, sampling brackets and dispatch cycles. The new
registry 0.2.0 graph therefore shows zero observed delta in these maintained
subjects after consumer ownership of the counter reader. This is a same-host,
source-bound result, not a universal size/cost guarantee or qualification for
the subsequent structured-checker adoption.

## Histogram evaluation

The maintainer requested working through
[#22](https://github.com/dragginzgame/ic-timers/issues/22) after the upstream
histogram primitive became available. Evaluation at released 0.14.6
`0c90c391dff5960a7502fc15a0718b03631f2515`, with repository-only 0.14.7 preparation,
concludes **no histogram adoption**. The issue closes as not planned, with no
runtime, public snapshot or dependency change. The current root/testing locks
select ic-metrics 0.2.3/0.2.0 independently. Upstream publication/identity
evidence for 0.2.4 is linked in the issue; primitive availability is resolved.

Actual maintained consumers in the size/runtime probes and PocketIC subjects
project sample counts and totals through the existing work summary. They are
scheduler qualification workloads, not a production distribution requirement;
they establish neither useful application bounds nor an acceptable heap and
recording-cost budget. No repeated histogram/percentile accumulator is being
removed or replaced. Additional diagnostics alone do not justify growing every
registration and copied inventory snapshot.

`TimerPerformance::record_work` remains the owning sample boundary. A future
histogram must replace that role's summary internally and project the existing
summary from it, rather than keeping two aggregates. Its field payload adds
approximately `16*N + 8` bytes per timer: eight bounds add 136 bytes, or 1.36 MB
over 10,000 retained declarations, before layout/allocation and snapshot-copy
effects. Recording adds at most N bound comparisons and a saturating bucket
update. These are source estimates, not Wasm, heap or instruction measurements.

The runtime finishes the accepted callback-envelope interval before registry
recording; unchanged reported work instructions cannot qualify bucket overhead.
The population contains retained, normally completed accepted envelopes. Traps,
missing/superseded claims and terminal remove-on-stop declarations do not produce
retained samples; these are not all attempted jobs or exclusive application cost.
Existing summary counters are sufficient for the maintained contracts.

Reopen for a named application's distribution question, caller-selected useful
bounds and a storage/recording budget. Qualification must include actual IC
recording cost and raw Wasm with identical source/lock/workload inputs, plus
native boundary/saturation and claim/reset/removal tests. Native fake timings
and callback totals alone cannot establish that cost. No such measurement or
test ran during this evaluation. Keep this research outside the 0.14.7 tooling
release; do not add arbitrary buckets, a second registry or instrumentation modes.


## Prepared IC Metrics 0.7 adoption

The cut is now delivered in Timers **0.19.0** at
`81b82589ca08aeb966edaf77a09f3f611bcce079`. Its
[released Linux/MSRV jobs](https://github.com/dragginzgame/ic-timers/actions/runs/38221325703)
pass, with both native macOS lanes still incomplete at inspection. Preparation
observations below retain their original source and graph scope.

The pending **0.19.0** cut implements [Timers #47](https://github.com/dragginzgame/ic-timers/issues/47).
The maintainer-selected root catalog/lock now use registry Metrics **0.7.0**;
its cached official archive matches selected SHA-256
`d526f945450ce67a517b36673d215809ce6e089e7809c4192db23600fe75d4ca`
and identifies source `384046884365e3d73696e0813a43640d3b1f2cb4`.

`MeasurementSummary::total()` and checked integer means are `u128`. Individual
observations, count, latest and maximum stay `u64`; ordering and sticky actual
saturation flags remain owned by Metrics. Two maximal observations fit exactly,
so exceeding the former `u64` total cap no longer makes a mean unavailable.
The public re-export changes Rust identity and return types. This is the next
minor line, with no narrow wrapper, old-type alias or compatibility reader.

Both maintained canister probes and their PocketIC consumers widen instruction
total fields and cohort calibration returns together. Candid changes those
fields from `nat64` to `nat`; rebuild both sides and retain wide values in logs
and downstream codecs. The updated existing projection fixture exercises a
Canic-style report above the old cap. The runtime-probe fixture also round-trips
synthetic wide totals through its actual Candid report shape, separately from
its small real measured instruction deltas. That synthetic boundary does not
claim that one IC execution consumes `u128` instructions.

Timers stores these observations only in its volatile runtime registry. There
is no persisted timer metric format to migrate and no stable-map package added.
Applications that persist their own report shape decide its hard-cut disposition.
Reset/epoch, scheduler/work attribution, trap rejection and upgrade reconstruction
remain unchanged. Interval subtraction still requires owner-established identity
and append-only continuity.

The source preparation replaces the old-cap saturation expectation with exact
wide totals, integer means and unchanged latest/maximum ordering for both roles.
The maintainer explicitly approved offline compile-only development checks on
2026-10-12, now recorded in AGENTS.md. The library/all-targets check passes,
including its test targets without executing them. No product tests, lint gates,
PocketIC or measurements were run for these changed inputs. Qualification
requires the existing library/MSRV/probe lint and real
runtime/policy-cohort gates against the selected root graph. Source review,
formatting, snapshot integrity and cheap locked metadata are preparation evidence.

Wide arithmetic and larger inline totals may add instructions and heap bytes;
they add no per-observation allocation or platform read. No Wasm or instruction
saving is claimed. Use the existing matched policy cohorts to measure the delta
against released 0.18.0 rather than reusing an older graph's size/cost figures.

## Prepared IC Metrics 0.8 adoption

Delivered in Timers 0.21.0 at `a6b518722438d2f7e2dccd7ccfef07debc35b5b3`.
[#50 is closed](https://github.com/dragginzgame/ic-timers/issues/50#issuecomment-6129282698)
from its completed standard release acceptance; the [delivery owner](../evidence/release-history.md#021-delivery-and-issue-50-acceptance)
separates that graph from later contributor dependencies and hosted CI gaps.
Preparation observations below retain their original scope.

Released Timers 0.20.0 at `01031c912058089756aed87fafa4918476c12bd4`
selects Metrics 0.7.3. Incoming maintainer changes to the one root catalog and
lock now select published registry Metrics **0.8.0** for all maintained members
([Timers #50](https://github.com/dragginzgame/ic-timers/issues/50)). Its cached
archive SHA-256 matches the selected checksum
`8ea44e32777b165cd60bdc0316d11891088ec87592648ba2bb7ca6949dfbfc37`;
the resolved package identifies released source
`71a29d8a4d7ecf6774d9715895ce9abd13b9e5d7`. Locked/offline metadata resolves one
Metrics package and one Timers package. All direct member dependencies still
inherit the root catalog; no sibling path fallback or second workspace is added.

Timers already records through tracked `MeasurementSummary` values. No local
raw recording helper or raw-only mean-error match needs replacement. Upstream
recording state, `record`, integer means and ordered observations retain their
existing implementation. The new `scaled_mean(scale)` is available directly
from the public summary; no Timers reporting wrapper or new recording path is
needed. Totals/means stay `u128`, samples/counts/latest/maximum stay `u64`, and
empty, measured zero, exact caps and sticky actual overflow remain distinct.

The public summary's Rust identity changes from Metrics 0.7 to 0.8. Its mean
error removes the raw-only variant and gains a typed projection cause. Consumers
exchanging direct Metrics types or exhaustively matching errors must update
together; select the next minor **0.21.0**, without aliases or dual readers.
The timer runtime, callback roles, attribution, epoch/reset ownership and probe
Candid report fields are unchanged. Timers metrics remain volatile; no stable
format or persistence dependency is introduced.

Library/all-target/all-feature compile-only checks pass with Rust 1.99.0;
the library Wasm path passes at Rust 1.88.0, all with locked/offline prepared
inputs. The selected graph also contains Testkit 0.34.7, Host 0.14.10 and
PocketIC 16.1.0. Inputs, metadata, commands and logs are retained at
`target/development/metrics080.ehT3o5JK/`. Cargo files/index entries are preserved.
These initial checks execute no tests. Subsequent explicitly approved focused
0.21 checks on the atomic lifecycle source pass all 154 native subjects, nine
doctests on Rust 1.99.0 and 1.88.0, library/probe Clippy and runtime/four-cohort
Wasm compilation; the [release owner](../evidence/release-history.md#prepared-021-first-lifecycle-reconciliation)
retains their exact source/graph/commands at
`/tmp/ic-timers-021-reconciliation.sFVjKHjS/`. IC runtime/policy cohorts and the
complete release gate remain outstanding; #50 stays open pending that acceptance.
Upstream's arithmetic experiment is not Timers cost or IC recovery evidence.
No Wasm, heap or instruction savings are claimed.

## Prepared 0.21.4 callback ownership simplification

The maintainer reports this cleanup delivered/live at 0.21.4. The
[delivery evidence](../evidence/release-history.md#0214-delivery-and-prepared-0215-tooling)
records the completed full-gate receipt separately from initial focused checks.

Internal callback aliases store their erased closures directly as
`Rc<RefCell<dyn FnMut(...) -> ...>>`, without a nested `Box`. The same shared
owner remains in registration, dispatch and abandonment; mutable borrowing and
the last-strong-owner drop still govern captures. Ordinary returned futures
remain separately boxed and pinned across suspension. This removes an ownership
layer, not a callback capability or policy boundary.

Lifecycle reconciliation selects the empty/existing slot with one `let ... else`.
It no longer checks `is_none` and then re-reads the same exclusive slot with an
impossible `ReconciliationConflict` branch. Fresh authority is still published
only after successful reconciliation; existing identity/policy/lifetime checks,
creation-error cleanup and unused-factory destruction ordering remain.

Retain distinct ordinary abandonment and watchdog trap/rollback paths. Retain
detached handle restoration, confirmed-delivery tracking and boxed removed
entries: each protects an independent cleanup or execution boundary. No new
allocation counter, generic scheduler or public API is introduced.

Existing native subjects cover mutable/repeated callbacks, suspended work,
reentrant capture destruction, first-reconciliation cleanup/retry and later
callback ownership. All 154 pass under the maintainer's focused approval;
library Clippy also passes with warnings denied. The
[release owner](../evidence/release-history.md#0213-delivery-and-prepared-0214-cleanup) retains
exact compiler/test commands and evidence; full IC/cohort qualification was
still outstanding at that preparation. Source changes require no public migration.
No measured Wasm, heap or instruction savings are claimed; removing
nested boxing also changes internal pointer layout.
