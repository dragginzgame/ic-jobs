//! Optional adapter to the canonical IC Timers watchdog runtime.
//!
//! A single consumer-owned watchdog can service many jobs. Restore records first,
//! derive the earliest pending deadline, then reconcile its volatile registration
//! during the application's install/upgrade hooks. Handlers remain bounded;
//! watchdog work itself cannot cross an await.

use ic_timers::{
    TimerCadence, TimerCompletion, TimerError, TimerIdentity, WatchdogReconcileState,
    WatchdogRegistration, WatchdogRunResult, reconcile_watchdog,
};

/// Reconstruct a consumer-owned watchdog at the next durable job deadline.
///
/// None retains an inactive declaration. Call again after inserting, cancelling
/// or completing jobs. Within active work, return [complete_batch] instead.
/// The caller initializes the shared IC Timers runtime before lifecycle recovery.
/// No registration, provider handle or timer snapshot is durable job authority.
pub fn reconcile<F>(
    registration: &mut Option<WatchdogRegistration>,
    identity: &TimerIdentity,
    recovery_cadence: TimerCadence,
    next_due_ns: Option<u64>,
    callback: F,
) -> Result<(), TimerError>
where
    F: FnMut(ic_timers::WatchdogContext) -> WatchdogRunResult + 'static,
{
    let desired = next_due_ns.map_or(
        WatchdogReconcileState::Inactive,
        WatchdogReconcileState::ScheduledAt,
    );
    reconcile_watchdog(registration, identity, recovery_cadence, desired, callback)
}

/// Finish one bounded scheduler batch and select its next durable deadline.
///
/// Pass the real completion classification. Pending overdue work requests a later
/// immediate message. An empty queue stops until the application reconciles it.
/// Unresolved external effects need a separate application reconciliation path.
#[must_use]
pub fn complete_batch(completion: TimerCompletion, next_due_ns: Option<u64>) -> WatchdogRunResult {
    let decision = next_due_ns.map_or(
        ic_timers::WatchdogDecision::Stop,
        ic_timers::WatchdogDecision::ScheduleAt,
    );
    WatchdogRunResult::new(completion, decision)
}
