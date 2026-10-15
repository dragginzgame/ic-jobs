# IC Jobs safety contract

IC Jobs owns checked transitions of one durable metadata record. Applications
own its storage, payload, effect execution, authorization, clock and lifecycle.
Applications delegate timer runtime coordination directly to IC Timers.

## Persistence and effects

Persist each transition atomically with any associated payload/index changes.
There must be one canonical record for each job and one immutable payload per
execution identity. Commit Running intent before issuing an external effect.
On IC, writes in a message segment commit only when that segment completes;
a write before a synchronous trap is not independently durable.

An external result and the corresponding record update may be separated by
an interruption. Restoring Running or Uncertain state never redispatches it.
Reconcile receipts, or establish destination-side idempotency, before explicitly
classifying it as safe to retry. A watchdog's recovery successor does not
establish the result of an external operation.

Attempt correlation rejects old replies within one retained history. It does not
authenticate clients, prevent a consumer from overwriting a record, or distinguish
forked/restored histories. Restoring an older backup can rewind counters: the
application must reconcile external effects and preserve globally unique effect
identity before dispatch from such a history. Reinstalls and recreated jobs need
fresh application-assigned job IDs.

Pending cancellation prevents future dispatch. Running and Uncertain work cannot
be cancelled through this crate because cancellation cannot undo an external
effect. Retry exhaustion stops the whole job, including a recurring job; it does
not silently advance to its next occurrence.

All transitions use checked deadline and identity arithmetic. Failures leave the
record unchanged. If successor calculation overflows after an effect succeeds,
the record retains its active attempt. The application must retain that successful
receipt and explicitly dispose of the unschedulable job without repeating the
effect. Full execution history and operator receipts remain consumer-owned.

## Reconstruction and bounds

JobRecord contains fixed-size numeric/enumerated metadata. Decode at a bounded,
authenticated storage boundary, then validate through Job::restore. Validation
checks structural invariants; it is not a signature, schema migration, trusted
history proof or a byte-format versioning protocol. Consumers own their selected
codec and any retained storage-format contract.

Recurring intervals cannot be zero in a `Schedule`. `RetryPolicy` construction
and deserialization share the same positive-budget/delay and cap checks. These
local configuration guarantees do not establish a valid `JobRecord` history:
all decoded aggregates still require `Job::restore`. Malformed configuration
fails at decoding; valid 0.7 records retain their existing serialized shape and
need no reset for the 0.8 source API cut. The consumer's bounded JSON codec is
qualified against independently frozen 0.7.5 records and a reopened stable cell;
consumers with other codecs own their retained-byte qualification.

That JSON guarantee does not cover IC Memory's physical ledger envelope.
Memory 0.39 refuses pre-cut two-slot images without writing, including when
installation is requested. Retained installations need explicit owner disposition;
this repository supplies no compatibility reader, automatic reset or migration.

The retry writer and checked reader retain the same public Serde structure name
`RetryPolicy`, including formats that preserve names. The private admission
projection does not define another wire identity; constructor checks still own
its numeric constraints.

Retained retry completions must be at or after the occurrence's scheduled time.
Pending/paused retries retain a representable backoff deadline even when cancelled.
A running retry must start at or after the preceding retry deadline.
Every attempted occurrence retains enough elapsed time for its cumulative
exponential/capped retry delays. Total historical retries also require at least
their initial delays in retained start/completion evidence, including after a
reschedule. These are necessary bounds, not reconstruction of discarded history.
Wide arithmetic and an aggregated capped tail bound restore work independently
of untrusted attempt counters (at most 65 delay steps).
A zero-attempt successor must follow a completed recurring occurrence,
with its deadline matching the recorded outcome timestamp and scheduling policy,
unless an explicit unstarted-occurrence reschedule establishes a new cadence
origin. That successor remains validated after cancellation or first dispatch
while the preceding success is retained. Management time must not replace
outcome/backoff evidence.
Successful one-shot work cannot be reconstructed as a fresh pending occurrence.
The first occurrence of each scheduling policy retains that policy's original
deadline through dispatch, uncertainty, retries and cancellation. Cancellation timestamps must remain
consistent with the preceding completed occurrence; cancellation may still
precede the next due time.

CatchUp exposes only one occurrence per successful completion. Consumers bound
each dispatch batch, indexed query, payload, outstanding-call set and retained
history. Skip advances from the current schedule to the first aligned deadline
strictly after successful completion. AfterCompletion waits from that completion.
Retry backoff does not change the logical occurrence's original scheduled time.

Paused work retains its deadline and retry evidence but has no dispatch deadline.
Resume preserves the original policy and does not grant repeat permission or
allocate an attempt. Rescheduling replaces only an unstarted occurrence's future
cadence, preserving logical effect identity, sequence, payload and retry budget.
Already-attempted, Running, Uncertain and terminal work cannot be rescheduled.
Failed management leaves the complete record unchanged. The consumer commits
management and due-index changes before reconciling its watchdog.

The 0.6 record changes are an incompatible stored contract. Consumers must
explicitly dispose of old snapshots before fresh current state; no old reader,
default-filled metadata or migration engine is supplied. The maintainer approved
retirement/reset of this repository's disposable 0.5 test installations. The
canister fixture uses a distinct storage key; old bytes are not interpreted under
the current layout, and this change does not erase retained memory itself.

`next_due_ns` reads a bounded candidate slice of validated `Job` values.
`start_next` starts one earliest due job through `Job::start`, returning its batch
index and attempt. The application reads the exact Running record from
`jobs[index].record()` and commits it with immutable payload/index changes before
dispatching an external effect.
It does not restore raw records, dispatch effects or admit unresolved jobs. On a
selected job's transition error, all records remain unchanged and no other job
is started. The application owns unique job identities, batch bounds and its
global due-time index; a batch's deadline describes only the jobs supplied.

The core creates no threads, platform timers, handler registry or hidden storage.
The application reconstructs its consumer-owned watchdog and projects next
deadlines directly into IC Timers. The callback remains synchronous and
bounded. Timer handles/claims are volatile and are never persisted as job authority.

A callback budget does not bound the number of future catch-up callbacks or
guarantee fairness to newer jobs. Earliest pending deadlines remain authoritative;
applications select Skip when replaying every missed occurrence has no value.
Timer callback success is not confirmation of an external job effect. Known
no-effect handler failures may be explicitly recorded as PermanentFailure without
stopping unrelated jobs; storage corruption and uncertain effects must retain
their separate fail-closed boundaries. A recovered watchdog can otherwise repeat
the same failing callback indefinitely without committing progress.

Native tests qualify pure scheduling, transitions and reconstruction. They do
not prove IC message rollback, real upgrades, provider delivery, instruction/cycle
behavior or database integration. A final application needs its own PocketIC
composition/recovery evidence and native macOS qualification.
