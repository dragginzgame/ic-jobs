![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Immediate progress-sensitive Watchdog continuation

Status: released in `0.7.0`.

## Consumer requirement

One successful bounded Watchdog callback may prove that more actionable work
remains. Waiting for the configured cadence in that case creates avoidable
backlog pressure. Retryable failure must still retain the cadence delay, and a
trap must still leave the cadence successor committed before consumer work.

“Immediate” means an absolute deadline equal to current IC message time. The
provider invokes the scheduler in a later replicated message. It never means a
direct callback call, synchronous recursion, or an ordinary timer beside the
Watchdog.

## Public API

The extension is policy-specific:

- `WatchdogDecision::Continue` retains the pre-armed cadence successor;
- `WatchdogDecision::ContinueImmediately` replaces that successor with a
  zero-delay scheduler after normal completion;
- `WatchdogRegistration::ensure_scheduled_immediately()` applies immediate
  demand outside work;
- `WatchdogContext::ensure_scheduled_immediately()` applies immediate demand
  to the exact running work attempt; and
- `WatchdogReconcileState::{Inactive, Scheduled, ScheduledImmediately}` gives
  lifecycle reconstruction one closed Watchdog-specific desired-state type.

`reconcile_watchdog` now accepts `WatchdogReconcileState` rather than the
generic two-state `TimerReconcileState`. This is a pre-1.0 hard cut: no alias,
forwarder, or dual reconciliation path remains. Because the public type
changes, the hard cut was released on the next minor line, `0.7.0`.

The API does not infer progress from work count and does not add a configurable
retry policy. The consumer classifies its result: successful progress with
more work chooses `ContinueImmediately`, retryable failure chooses `Continue`,
and quiescence or terminal failure chooses `Stop`.

## Scheduling transitions

| Current Watchdog state | Immediate request |
| --- | --- |
| Inactive | Allocate one scheduler generation and arm one zero-delay wake-up. |
| Scheduled later than now | Rotate the scheduler generation and replace the exact owned wake-up at deadline now. |
| Scheduled at or before now | Coalesce; do not duplicate or delay the earlier/equivalent wake-up. |
| Work dispatched | Coalesce because the already queued zero-delay work message satisfies current demand. The cadence successor remains the safety wake-up. |
| Work running | Store one immediate pending command for that exact attempt. Normal completion applies it to that attempt's pre-armed successor. |

Every accepted scheduler message still computes and commits one successor at
`scheduler time + configured cadence`, queues one zero-delay work callback,
and returns. An immediate completion rotates only the successor scheduler
generation, changes its authoritative deadline to work-message time, and emits
one replacement arm. It never creates a second Watchdog declaration, ordinary
timer registration, attempt generation, provider path, or pending-command
machine.

After the immediate scheduler later runs, it pre-arms cadence safety again
before dispatching the next work attempt. A further successful unit can again
request immediate continuation; a retryable failure leaves that cadence
successor unchanged.

## Arbitration

The existing one-command Watchdog arbitration remains authoritative:

1. An invariant-failure completion always stops. On the 0.10 line it also wins
   the inactive-reason classification over pending cancellation, so a retained
   declaration reports `Failed` and does not count a losing cancellation.
2. Unregistration is sticky and cannot be superseded by a later ensure.
3. A later cancellation overrides either continuation request.
4. As before, a later explicit ensure can re-enable a pending cancellation on
   a retained declaration. Immediate ensure re-enables it immediately; cadence
   ensure re-enables it at cadence.
5. Among pending continuation requests, immediate is the earliest deadline and
   cannot be downgraded by a cadence ensure.
6. The selected pending command overrides the callback's returned decision.
   With no pending command, the returned decision applies.

Cancellation or unregistration while scheduled clears the exact successor and
any queued work. While work runs, terminal arbitration clears its pre-armed
successor after normal completion. A trapping work message rolls its nested
requests back with all other message-local mutations.

## Failure and ownership boundary

The cadence successor is committed by the scheduler message before fallible
work begins. A work trap or instruction exhaustion before return therefore
leaves it available for the next attempt.

Normal `ContinueImmediately` temporarily detaches the exact successor handle,
mutates the same canonical state to a checked replacement generation, clears
the old provider timer, and installs one zero-delay provider timer. An
unexpected installation or confirmation failure traps the work message.
Replicated-message rollback then restores the earlier committed registry state
and cadence provider handle together. The runtime must not catch that failure
and commit a state that lost both successors.

Public immediate requests outside a callback retain the existing public
provider-failure behavior: a failed binding retires the declaration to
`ProviderBindingFailed` rather than claiming a wake-up it does not own.

Old scheduler tokens fail generation and claim checks. Old work tokens fail the
paired attempt checks. Neither can restore state, arm a timer, or invoke
consumer work.

## Observation

No snapshot field or persisted format is added. While an immediate successor
is authoritative:

- `next_deadline_ns` is the selected deadline at or before request time;
- `scheduling_mode` is `Continuation`;
- `latest_requested_delay_ns` and `latest_armed_delay_ns` are `Some(0)` after
  the replacement commits;
- each explicit request increments `schedule_requests`;
- an actual initial or replacement provider arm increments `wakeups_armed`;
  and
- equivalent, already-earlier, dispatched, and running requests increment
  `coalesced` instead of fabricating another arm.

When the immediate scheduler runs, the newly pre-armed successor is again a
Watchdog cadence successor, so the mode and latest armed delay describe that
current authority. Callback decisions do not increment `schedule_requests`,
matching ordinary directive accounting.

## State-space and cost

The closed Watchdog runtime states, provider-handle bound, registry capacity,
callback roles, snapshot shape, and persisted-state count are unchanged. The
state-space delta is one public decision, one public reconciliation state, one
private pending-command variant, and one private cadence/immediate request
selector.

Each immediate continuation performs one additional clear/set replacement
inside the normally completed work message. It also causes the next existing
two-message scheduler/work pair to run earlier; it does not remove the cadence
pre-arm required for trap safety. Binary, instruction, and cycle impact is
reported in the
[focused evidence record](../audits/immediate-watchdog-continuation-2026-08-28.md)
rather than inferred.
