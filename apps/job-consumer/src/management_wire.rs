//! Current management and installation Wire identities for this unpublished fixture.
//! Rebuild the canister and harness together; old Candid requests are refused.

use ic_wire::Limits;

/// Stable identities; Rust DTO declaration order is part of each native Serde contract.
#[derive(Clone, Copy, Debug)]
#[repr(u64)]
pub enum Method {
    /// Installation configuration.
    Init = 0x4a4f42534d470001,
    /// Create a job.
    Create = 0x4a4f42534d470002,
    /// Inspect a job.
    Inspect = 0x4a4f42534d470003,
    /// Page through jobs.
    List = 0x4a4f42534d470004,
    /// Cancel pending work.
    Cancel = 0x4a4f42534d470005,
    /// Pause pending work.
    Pause = 0x4a4f42534d470006,
    /// Resume paused work.
    Resume = 0x4a4f42534d470007,
    /// Replace an unstarted schedule.
    Reschedule = 0x4a4f42534d470008,
    /// Read the bounded queue summary.
    Summary = 0x4a4f42534d470009,
    /// Dispatch one bounded batch.
    DispatchDue = 0x4a4f42534d47000a,
    /// Reconcile an outstanding delivery.
    ReconcileDelivery = 0x4a4f42534d47000b,
    /// Control the deliberate watchdog trap.
    TrapWatchdogBatch = 0x4a4f42534d47000c,
    /// Trap after creating work.
    TrapAfterCreate = 0x4a4f42534d47000d,
    /// Trap after pausing work.
    TrapAfterPause = 0x4a4f42534d47000e,
    /// Trap after replacing a schedule.
    TrapAfterReschedule = 0x4a4f42534d47000f,
    /// Corrupt a retained record deliberately.
    CorruptRecord = 0x4a4f42534d470010,
    /// Mark a retained attempt uncertain.
    MarkUncertain = 0x4a4f42534d470011,
    /// Trap after confirmed delivery.
    DispatchTrapResult = 0x4a4f42534d470012,
    /// Allocate intent without delivery.
    CreateBlocked = 0x4a4f42534d470013,
    /// Inspect consumer-owned heap job observations.
    JobMetrics = 0x4a4f42534d470014,
    /// Reset diagnostics without changing durable job state.
    ResetJobMetrics = 0x4a4f42534d470015,
    /// Scan consumer-owned heap transition history.
    JobLogs = 0x4a4f42534d470016,
}

impl Method {
    /// Exact exported name; installation uses `init` only as a codec identity.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::Create => "create",
            Self::Inspect => "inspect",
            Self::List => "list",
            Self::Cancel => "cancel",
            Self::Pause => "pause",
            Self::Resume => "resume",
            Self::Reschedule => "reschedule",
            Self::Summary => "summary",
            Self::DispatchDue => "dispatch_due",
            Self::ReconcileDelivery => "reconcile_delivery",
            Self::TrapWatchdogBatch => "fixture_trap_watchdog_batch",
            Self::TrapAfterCreate => "fixture_trap_after_create",
            Self::TrapAfterPause => "fixture_trap_after_pause",
            Self::TrapAfterReschedule => "fixture_trap_after_reschedule",
            Self::CorruptRecord => "fixture_corrupt_record",
            Self::MarkUncertain => "fixture_mark_uncertain",
            Self::DispatchTrapResult => "fixture_dispatch_trap_result",
            Self::CreateBlocked => "fixture_create_blocked",
            Self::JobMetrics => "job_metrics",
            Self::ResetJobMetrics => "reset_job_metrics",
            Self::JobLogs => "job_logs",
        }
    }
}

/// Complete request frame and traversal allowances, including installation.
pub const REQUEST_LIMITS: Limits = Limits {
    bytes: 4 * 1024,
    allocation: 32 * 1024,
    nodes: 256,
    depth: 16,
};
/// Complete response allowance for eight bounded views and diagnostic errors.
pub const RESPONSE_LIMITS: Limits = Limits {
    bytes: 64 * 1024,
    allocation: 256 * 1024,
    nodes: 4096,
    depth: 32,
};
