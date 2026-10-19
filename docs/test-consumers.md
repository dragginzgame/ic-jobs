# Test consumers

Run `make test-consumers` for two small application-owned consumers in
[the integration fixture](../apps/job-consumer/tests/composition.rs). The target tests
the public scheduling API and direct Timers composition on Rust 1.91, using the
selected locked graph.
Configured CI also runs this target on the three supported hosts. The harness
and its support modules belong to the unpublished consumer package; the core
package has no Memory or Timers development dependency. `check-consumer` selects
the consumer library and frozen API-wire tests separately, avoiding a second
composition run. Management Candid derives/encoding use published Platform;
delivery requests/replies use published Wire with consumer-owned identities and
limits. Testkit uses Platform's Candid decoder limits/error types; the harness has no
separate decoder dependency.

These consumers exercise the public API without adding storage, payloads,
handlers or lifecycle ownership to the Jobs library. Each application decodes a
bounded JSON snapshot and passes every restored record through `Job::restore`.
The byte store models atomic replacement and can refuse a commit while retaining
the previous bytes. It is a test storage owner, not a stable-memory implementation.

| Consumer | Application requirement | Exercised behavior |
| --- | --- | --- |
| Deferred notifications | Deliver a queue of at most four immutable messages at their deadlines, retry only a known rejection, reconcile a lost reply against destination receipts | Commit intent before exposing delivery; retain payload and execution identity across retries; reject stale replies; block unresolved work after reconstruction while other due jobs proceed; complete without redelivery after a confirmed receipt |
| Cache maintenance | Remove expired entries locally on a fixed-rate schedule, with at most two occurrences per wake and 32 cache entries | Commit local effects and metadata together; reconstruct pending deadlines; bound overdue CatchUp work; Skip missed intervals; cancel pending work; derive the next timer decision |

The notification scheduler produces an outstanding delivery for a separate
application continuation. The synchronous scheduling step makes no external call.
Its destination is an independent receipt owner: repeated execution identities
retain the original payload and produce one logical delivery. An absent receipt
leaves the uncertain job blocked. A known rejection before the effect permits an
explicit retryable classification; a missing reply does not.

The destination retains rejection evidence for the exact `Attempt` and payload,
so a lost rejection reply can be reconciled to a safe retry after sender
reconstruction. That disposition must commit before the next attempt becomes
eligible. Repeated requests retain the destination's recorded result; a confirmed
successful execution takes precedence over an earlier rejected attempt. Stale
attempt evidence cannot resolve the current dispatch. Exhausting the original
retry budget leaves the job terminal rather than allocating another attempt.

The notification queue supplies all its bounded records to `next_due_ns`, so the
derived deadline covers its earliest pending work. Equal deadlines retain queue
order. Enqueue and cancellation update the committed queue before the application
derives its wakeup again. Duplicate identities and excess records are rejected on
both insertion and reconstruction; a corrupt future record blocks dispatch of
an otherwise valid due record. Outstanding envelopes are inspected for
reconciliation, never automatically replayed. An interruption after intent
commit but before delivery still reconstructs blocked work: the receiver's absent
receipt alone does not establish retry safety.

Cache maintenance has only local effects, so it commits the pruned entries and
completed job transition in the same replacement. A failed replacement commits
neither. The loaded snapshot holds one validated job with its entries/counter;
there is no second record copy to synchronize. Notification and maintenance
snapshots serialize validated jobs directly through the canonical `JobRecord`
writer. Reads still decode records and pass through `Job::restore`.
Its bounded wake result constructs a direct IC Timers decision.
Tests check future and overdue scheduling decisions and stopping after
cancellation; the native fixture does not call the IC timer provider.
The store can also refuse the second write in a batch. The first committed
occurrence survives; reconstruction resumes at the second occurrence without
repeating the first. This byte-store fault injection does not model an IC trap's
message-wide rollback.

Reconstruction keeps only committed bytes and discards volatile consumer state.
The tests cover refused intent/result commits, lost replies, receipt reconciliation,
and corrupt metadata rejection before work. This is native composition evidence.
Live IC message rollback, actual lifecycle upgrades,
watchdog registration/provider delivery and external canister calls remain
unqualified. A live harness must use IC Testkit's selected server setup/check
contract rather than adding another PocketIC installer here.

## Native management fixture

The notification consumer now exercises an application management boundary:
create a one-shot notification, inspect it, list a bounded page, cancel pending
work or dispatch one eligible due notification. All calls require the fixture's
configured manager identity before decoding storage, reading private payloads
or writing. The supplied caller is trusted boundary input; a real canister must
obtain it from the IC caller API, never from a request field. This native policy
check does not qualify Candid endpoints or IC caller authentication.

Listing accepts limits from one to four and an optional exclusive job-ID cursor.
Pages sort by job ID independently of queue insertion order; each page observes
the current committed queue, without claiming a snapshot across requests.
Every read and mutation validates the entire bounded queue through `Job::restore`.
Unknown jobs, invalid page limits, corrupt future records, duplicate identities
and oversized snapshots are refused without changing committed records.

Successful responses include the earliest deadline derived from committed
records. Failed create/cancel/dispatch writes expose no successful management
response or delivery envelope and retain the previous deadline. Direct IC Timers
composition tests projection of those deadlines into ScheduleAt/Stop decisions;
it does not register provider wakeups. Cancellation retains terminal history,
refuses Running/Uncertain work and cannot undo an effect. An early dispatch is
idle; restored intent stays blocked until exact destination evidence resolves it.

These flows use the existing byte-replacement store and immutable payloads.
They establish consumer composition evidence only. The native host fixture below
uses committed IC Memory allocations; the separate canister fixture owns its
lifecycle integration. Neither path assigns component memory-ID ranges.

## Consumer memory requests

The unpublished consumer selects IC Memory from the root workspace catalog;
`Cargo.lock` records the exact release. Each fixture contributes one checked `MemoryRequest`
with a distinct authority and permanent logical key: `test.notifications.queue.v1`
for job records and immutable notification payloads, and
`test.maintenance.state.v1` for the maintenance record, entries and counter.
These names describe the fixture's intended stores; no numeric slots or private
ranges are selected by the consumers.

The host composes both requests into one `SealedDeclarationSnapshot`. Native
checks establish order-independent declaration meaning, refusal of a foreign
request reusing a consumer key, and unchanged retained job bytes/deadlines after
that refusal. A fresh native `MemoryRuntime` refuses opens before bootstrap;
sealed requests alone do not grant committed allocation authority. Authority
labels express ownership policy, not caller authentication.

The fault-injection consumers retain their bounded byte store. A separate native
host fixture uses IC Memory allocation leases and IC Storage cells: one pool grants the
two namespaces without component ranges or numeric IDs; the host bootstraps once
before component adoption/open. Stable cells persist the same bounded snapshot
bytes. Cold reopens retain allocation IDs, payloads and pending deadlines; reads
restore through the existing `Job::restore` boundary. Running/Uncertain intent
remains blocked, and cancellation persists across a subsequent cold reopen.
Foreign-owner bootstrap fails without changing backing bytes or granting opens.
This is actual stable-structure persistence on native VectorMemory, not an IC
upgrade, message rollback, final Canic/IcyDB composition or live timer evidence.
Canic/IcyDB composition needs those applications' own qualification. No old
allocation API or fallback remains in Jobs' consumers. The current format and
installation disposition are documented with the
[canister consumer](../apps/job-consumer/README.md); Jobs' library acquires no
storage dependency or lifecycle hook.

## Durable canister consumer

The unpublished [application canister](../apps/job-consumer/README.md) now
implements an authenticated management API, bounded stable queue, local counter
tasks, external delivery receipts and lifecycle-owned timer reconstruction.
All decoded records still cross `Job::restore`; restored Running/Uncertain work
stays blocked. It owns a new test installation rather than changing these native
consumers' existing snapshots or any deployed format.

`make check-consumer` runs native stable-memory tests, builds the Rust 1.91 Wasm,
compiles the separate Testkit harness and checks native/Wasm Clippy. Ordinary CI
includes these checks without launching a server. Explicit Testkit CLI/server
preparation and `make test-canister` select the live upgrade, rollback, watchdog
and lost-result scenarios. Durable management now covers pause/resume,
unstarted-occurrence rescheduling, wakeup reconstruction and real management trap
rollback. The current eight-case harness also checks Wire endpoint refusal,
authorization and receipt persistence. It needs source-bound live qualification;
historical receipts do not establish that its current graph passes. See
[#23](https://github.com/dragginzgame/ic-jobs/issues/23). Completed
[#11](https://github.com/dragginzgame/ic-jobs/issues/11) acceptance covers the older
five-scenario v0.5.6 harness. Current inputs require their own source-bound
execution and host evidence, including the additional watchdog scenario in
[#20](https://github.com/dragginzgame/ic-jobs/issues/20). The original small native
consumers remain focused models; they do not substitute for those IC observations.

The queue now contains known no-effect local counter failures as terminal jobs,
while keeping the two-attempt bound and corrupt-record refusal. The additional
real-watchdog scenario traps after its stable batch write, observes the actual
canister trap log, checks rollback and an armed successor, and clears the fault
without rearming before requiring subsequent progress. Failed and completed jobs
must remain terminal after upgrade, with no duplicate committed local effect.
This case requires matching live execution evidence; prior five-scenario evidence
does not qualify it. See [#20](https://github.com/dragginzgame/ic-jobs/issues/20).

The current [configuration tests](../crates/ic-jobs/tests/configuration.rs)
round-trip current records and retain unresolved dispatch as blocked. Frozen
pre-cut JSON, physical images and Candid replies are negative inputs: their
presence is refusal evidence, not a compatibility promise. IC Memory's selected
`testing::memory_from_sparse_hex` expands the old physical images; Jobs owns their
producer/schema assertions and three-page fixture ceiling. Both retained opening
and fresh creation refuse those ledgers with zero writes and identical backing
bytes. Current-format corruption remains a queue-wide refusal. Raw zero intervals
return `InvalidInput` without committing, and the fault endpoint deliberately
mutates encoded bytes without adding an unchecked configuration constructor.
These native checks do not establish live IC upgrade qualification.
