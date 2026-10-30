![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# IcyDB adoption record

This page preserves dated adoption evidence, not current dependency or issue
status. Current integration guidance is in [runtime architecture](../architecture.md#consumer-integration)
and [the public API](../../../crates/ic-timers/README.md#runtime-ownership). Consumer owners qualify
their selected graphs and deployed artifacts; historical receipts below qualify
only their named sources. See [the release guide](../development.md)
for this repository's host-evidence limits.

Status: historical exact-0.5.0 IcyDB adoption record.
This is maintained downstream evidence supplied by the IcyDB owner; this
repository did not modify IcyDB. Historical tagged and intermediate evidence
is retained separately below.

This record preserves the supplied exact-0.5.0 evidence; it is not a fresh
inspection of downstream HEAD or qualification of 0.8.0 adoption. See the
[current handoff](../../status/current.md) and
[dated Toko Miner evidence](toko-miner.md) for later downstream context,
including scoped 0.8.0 composition qualification.

## Recorded exact-0.5.0 integration

The recorded IcyDB dependency graph resolves exactly one `ic-timers` 0.5.0
package. `ic-cdk-timers` 1.0.0 is private and transitive beneath it; IcyDB has
no direct provider dependency or alternate provider path.

The generated database actor owns one retained `WatchdogRegistration` with
the fixed identity `icydb/startup/recovery` and a one-second cadence. The
lifecycle owner initializes and reconciles it before application
post-upgrade work. IcyDB continues to own durable readiness and terminal
database-failure authority; it does not persist timer handles, generations,
snapshots, epochs, or library policy.

The 0.5 hard cut replaced IcyDB's sole explicit `TimerContext` use directly
with `WatchdogContext`. It retained no alias, compatibility shim, policy
probe, dual version, fallback registry, or parallel timer state machine. The
SQL-performance timer consumer also compiles against exact 0.5.0.

The result mapping remains:

| IcyDB result | Timer completion | Watchdog decision |
| --- | --- | --- |
| Recovery page made progress | success | continue |
| Returned retryable recovery failure | retryable failure | continue |
| Database is ready | no work | stop |
| Durable terminal database failure | invariant failure | stop |

A normally returned mutation error that retains recovery controls invokes the
synchronous idempotent ensure seam before returning. Durable demand always
calls `ensure_scheduled()` unconditionally; `has_armed_wakeup()` remains a
reporting observation and does not create a check-then-arm race.

## Recorded exact-0.5.0 downstream validation

IcyDB reports the following checks passing against exact 0.5.0:

- current Rust and Rust 1.88 compilation;
- warning-denied Clippy;
- one resolved `ic-timers` package and a private transitive provider;
- explicit Watchdog work-trap recovery and later progress;
- actual 40-billion-instruction exhaustion with successor survival;
- overdue cadence coalescing;
- synchronous upgrade reconstruction and lifecycle composition;
- retry after a trapped upgrade;
- rejection of external timer-executor ingress; and
- compilation of the SQL-performance timer consumer.

IcyDB's Candid remains byte-identical. No public IcyDB API, persisted format,
compatibility path, or recovery-authority boundary changed during the 0.5
adoption.

These are owner-maintained downstream results, not a claim that the
`ic-timers` repository reran IcyDB's suite. Combined Canic/IcyDB/application
qualification must still resolve one exact `ic-timers` package in the final
Wasm and inventory every remaining direct provider user canister-wide.

## Historical tagged evidence

Tagged IcyDB 0.226.1 at commit
`cd388cad96383f7c4c56054a8f27de608e9371e3` first adopted exact `ic-timers`
0.3.4. That hard cut removed IcyDB's startup `TimerId`, active flag, cadence
guard, serial interval, and zero-delay cleanup path.

The tagged subject demonstrated successor survival and later progress after
an explicit trap and 40-billion-instruction exhaustion, external-ingress
rejection, synchronous upgrade reconstruction before downstream hooks,
terminal unregistration, overdue coalescing, and independent application
timer progress while recovery work trapped. Its optimized raw Wasm was
4,164,071 bytes.

Its normally completed Watchdog sample was 1,163 instructions and two
application callbacks were 1,986 instructions total. Candid was 60,348 bytes
with SHA-256
`a3a396639a0b809cf8865fc838ec9f69ada7ee291b3fdbdedd4c3f55525e97e5`.

## Historical post-tag 0.3.8 evidence

A validated post-tag IcyDB worktree advanced from snapshot-derived wake-up
reporting to `WatchdogRegistration::has_armed_wakeup()` and exact
`ic-timers` 0.3.8. It resolved one package, kept `ic-cdk-timers` private and
transitive, compiled to Wasm on Rust 1.88, and reran the focused real-canister
recovery evidence.

That subject reported 4,164,625 optimized raw Wasm bytes, 4,767,940 compiler
artifact bytes, and 1,606,620 deterministic gzip bytes. Relative to the
4,125,495-byte direct-provider subject, the shared runtime added 39,130 raw
bytes and remained within IcyDB's 65,536-byte owner budget. These numbers are
historical 0.3.8 measurements and are not relabeled as 0.5 results.

## Memory-observation boundary

The exact-0.5.0 runtime records allocation-free start/end Wasm and stable
memory page extents for normally completed scheduler and work callbacks. The
bounded summaries retain the latest extents and maximum observed non-negative
growth; they do not total absolute page counts or fabricate samples for
trapped or instruction-exhausted work.

Page extent is a runtime-epoch-local high-water observation, not exact live
bytes. IcyDB still owns any allocator-derived sub-page byte bound and its
maximum 64-index fanout probe. Watchdog scheduler and work are synchronous, so
their extent intervals avoid the interleaved-await qualification that applies
to ordinary async callbacks.

The focused [0.5 overhead probe](../audits/0.5-memory-sampling-overhead-2026-08-15.md)
reports the page-read cost separately from callback instruction aggregates.
In PocketIC 15.0.0, the empty and sampled brackets both measured 200
call-context instructions, for an observed four-read delta of zero. This is a
local regression subject rather than a promise about future IC metering.

## Historical exact-0.6 adoption requirements

At the time of the exact-0.5.0 record, exact-0.6 adoption evidence was not yet
supplied. The resulting requirement remains relevant to cross-release
comparisons: 0.228 watchdog measurements must be rebaselined rather than
directly compared with 0.5 totals. The documented meaning is the accepted
`ic-timers` work interval, not application callback code alone. The runtime's counter placement
did not change between 0.5 and 0.6; the 0.6 documentation corrects the broader
envelope label. Cross-release reports must identify the exact artifact and
measurement interpretation.

IcyDB must retain external PocketIC evidence for its hard 40-billion-
instruction exhaustion claim because the library aggregate excludes provider
entry/exit and post-interval observation accounting. The final combined Wasm
must also resolve exactly one `ic-timers` package across IcyDB, Canic, and
application owners.

The [0.6 calibration record](../audits/0.6-message-instruction-calibration-2026-08-15.md)
keeps unavailable full-message instruction totals explicit and does not
convert cycle-balance deltas into instruction estimates.

## Progress-sensitive integration guidance (available since 0.7.0)

`ic-timers` 0.7.0 added the Watchdog-specific immediate continuation described
in the [design record](../design/immediate-watchdog-continuation.md).
Read-only IcyDB source inspection on 2026-10-02 finds generated lifecycle
reconciliation using `ScheduledImmediately` and successful recovery progress
using `ContinueImmediately`, with registry 0.8.0 in its lockfile. Its owner
records 11 focused 0.8.0 PocketIC integration tests passing in
`docs/design/0.261-entity-rename/0.261-status.md` (D1 final evidence);
this repository did not rerun them. The integration mapping is:

| IcyDB condition | API |
| --- | --- |
| Inactive actionable debt during lifecycle or commit reconciliation | `WatchdogReconcileState::ScheduledImmediately` |
| New actionable debt on an existing inactive or cadence-scheduled claim | `WatchdogRegistration::ensure_scheduled_immediately()` |
| Successful bounded journal progress with more complete batches | `WatchdogDecision::ContinueImmediately` |
| Returned retryable failure | `WatchdogDecision::Continue` |
| Quiescence | `WatchdogDecision::Stop` |
| Durable terminal failure | invariant-failure completion with `Stop` |

Further adoption must remain an atomic single-package hard cut and retain
IcyDB's own backlog, trap, exhaustion, lifecycle and performance evidence.
It must not add a second watchdog or ordinary-timer drain loop.
