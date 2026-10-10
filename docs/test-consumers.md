# Test consumers

Run `make test-consumers` for two small application-owned consumers in
[the integration fixture](../crates/ic-jobs/tests/consumers.rs). The target tests
core and optional timers separately on Rust 1.88, using the selected locked graph.
Configured CI also runs this target on the three supported hosts.

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

The notification queue supplies all its bounded records to `Scheduler`, so the
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
neither. Its bounded wake result uses `timers::complete_batch` when timers are
enabled. Tests check future and overdue scheduling decisions and stopping after
cancellation; the native fixture does not call the IC timer provider.
The store can also refuse the second write in a batch. The first committed
occurrence survives; reconstruction resumes at the second occurrence without
repeating the first. This byte-store fault injection does not model an IC trap's
message-wide rollback.

Reconstruction keeps only committed bytes and discards volatile consumer state.
The tests cover refused intent/result commits, lost replies, receipt reconciliation,
and corrupt metadata rejection before work. This is native composition evidence.
Live IC message rollback, stable-memory persistence, actual lifecycle upgrades,
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
response or delivery envelope and retain the previous deadline. The timers
feature tests projection of those deadlines into ScheduleAt/Stop decisions;
it does not register provider wakeups. Cancellation retains terminal history,
refuses Running/Uncertain work and cannot undo an effect. An early dispatch is
idle; restored intent stays blocked until exact destination evidence resolves it.

These flows use the existing byte-replacement store and immutable payloads.
They establish consumer composition evidence only. Stable allocation and the
real canister lifecycle must adopt the replacement contract coordinated by
[IC Memory #44](https://github.com/dragginzgame/ic-memory/issues/44); no provisional
range-based bootstrap is added here.

## Consumer memory requests

IC Memory is a development dependency selected from the root workspace catalog
(`0.33`, locked to 0.33.4). Each fixture contributes one checked `MemoryRequest`
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

The fixtures still commit to their bounded byte store. The published IC Memory
0.33 contract requires host authority ranges for actual allocation; the requested
shared-pool hard cut is pending in
[IC Memory #44](https://github.com/dragginzgame/ic-memory/issues/44).
This dependency/declaration step neither implements that allocator nor establishes
stable persistence or upgrades. The final host must gather declarations and
recovered-metadata admission, bootstrap once, and open stores only after the
allocation commit. Jobs' library acquires no storage dependency or lifecycle hook.

## Proposed durable canister consumer

The next composition requested by the maintainer is a small canister consumer
with a management API and actual upgrade recovery. Its implementation and
qualification are tracked in [#11](https://github.com/dragginzgame/ic-jobs/issues/11).
This section records the intended canister architecture. The management flow is
exercised natively above; Candid endpoints, stable-memory persistence and actual
IC timer reconstruction remain unimplemented.

The consumer API should authorize task creation/scheduling, job inspection,
bounded queue listing, pending cancellation and bounded dispatch of eligible
due work. Startup reconstructs retained tasks. `Job::start` still requires a
due Pending job; cancellation cannot undo Running or Uncertain effects.
Exact-attempt reconciliation remains required before retry. API names and the
stable byte format are to be selected during implementation; force-run,
arbitrary rescheduling and replay are not implied by this management surface.

The consumer owns authentication, immutable execution payloads, handlers, storage,
codec, due-time index, bounds and lifecycle hooks. Jobs supplies checked records
and scheduling; IC Timers supplies volatile wakeups. IC Memory should validate
stable-region ownership before stores open. The selected stable structure and
codec then retain `JobRecord`, payload and queue/index metadata; IC Memory does
not serialize jobs or validate their transitions.

An admitted API mutation applies the checked Jobs transition, updates stable
record/payload/index state consistently, derives the earliest global pending
deadline and calls `timers::reconcile` on the consumer's single watchdog.
Enqueue, cancellation, completion and explicit disposition all rederive this
wakeup; no pending work leaves it inactive. Stable writes and timer changes
belong to the same synchronous message segment and a trap rolls that segment
back. Refused mutations and timer-reconciliation failures must not report a
successful persisted schedule. The implementation must qualify that boundary.

Commit Running intent before exposing an external effect, using a consumer-owned
continuation because the synchronous watchdog cannot await. Persist the exact
result and update scheduling afterward. Lost replies and interrupted result
writes keep work blocked until receipt evidence or explicit disposition resolves
it; upgrading does not authorize replay.

During `post_upgrade`, validate IC Memory allocations, open retained stores,
boundedly decode records through `Job::restore`, initialize IC Timers and
reconstruct required wakeups from durable deadlines before downstream hooks.
Provider handles, closures and the timer registry are recreated. Overdue work
follows the existing bounded CatchUp/Skip behavior; Running and Uncertain work
remains blocked.

Use IC Testkit for the separate host harness and PocketIC setup/check ownership.
Test real wakeup changes, dispatch, retained jobs/payloads/deadlines across an
upgrade, overdue bounds, invalid restored records, message rollback and
lost-reply reconciliation without duplicate effects. Testkit already depends on
the Host crates; a direct Host dependency needs a concrete additional caller.
Keep host dependencies outside the Jobs library's Wasm graph. Rust 1.88,
native macOS qualification and live IC recovery remain separate requirements.
