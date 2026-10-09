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
