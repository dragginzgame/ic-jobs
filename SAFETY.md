# IC Jobs safety contract

IC Jobs owns checked transitions of one durable metadata record. Applications
own its storage, payload, effect execution, authorization, clock and lifecycle.
The optional timer adapter delegates runtime coordination to IC Timers.

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

CatchUp exposes only one occurrence per successful completion. Consumers bound
each dispatch batch, indexed query, payload, outstanding-call set and retained
history. Skip advances from the current schedule to the first aligned deadline
strictly after successful completion. AfterCompletion waits from that completion.
Retry backoff does not change the logical occurrence's original scheduled time.

`Scheduler` borrows a bounded candidate batch of validated `Job` values. It
derives pending deadlines afresh, starts one earliest due job through `Job::start`,
and returns the exact Running record for the application's persistence boundary.
It does not restore raw records, dispatch effects or admit unresolved jobs. On a
selected job's transition error, all records remain unchanged and no other job
is started. The application owns unique job identities, batch bounds and its
global due-time index; a batch's deadline describes only the jobs supplied.

The core creates no threads, platform timers, handler registry or hidden storage.
The timers feature reconstructs one consumer-owned watchdog and projects next
deadlines into the canonical runtime. The callback remains synchronous and
bounded. Timer handles/claims are volatile and are never persisted as job authority.

Native tests qualify pure scheduling, transitions and reconstruction. They do
not prove IC message rollback, real upgrades, provider delivery, instruction/cycle
behavior or database integration. A final application needs its own PocketIC
composition/recovery evidence and native macOS qualification.
