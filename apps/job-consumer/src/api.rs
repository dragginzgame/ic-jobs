//! Consumer-owned Candid boundary. JobRecord remains an internal stored value.

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

/// Installation configuration, retained across upgrades.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize, Serialize)]
pub struct Init {
    /// Identity permitted to manage tasks and fixture controls.
    pub manager: Principal,
    /// Optional sender permitted to write this canister's delivery receipts.
    pub delivery_sender: Option<Principal>,
}

/// Immutable effect payload.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize, Serialize)]
pub enum Task {
    /// A local effect committed atomically with its job completion.
    Increment(u64),
    /// Delivery to the independent consumer receipt endpoint.
    Deliver {
        /// Independent receipt canister.
        destination: Principal,
        /// Immutable UTF-8 payload, at most 256 bytes.
        message: String,
    },
}

/// Consumer's supported scheduling policies.
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum Timing {
    /// One occurrence.
    Once(u64),
    /// Fixed rate with bounded catch-up or skip behavior.
    FixedRate {
        /// Original first deadline.
        first_ns: u64,
        /// Positive interval.
        every_ns: u64,
        /// Catch up two occurrences per batch; otherwise skip missed intervals.
        catch_up: bool,
    },
}

/// Task creation request. Identities are allocated by the retained queue.
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Create {
    /// Scheduling policy.
    pub timing: Timing,
    /// Immutable execution payload.
    pub task: Task,
}

/// Projection of the checked job state.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Status {
    /// Pending work with a durable deadline.
    Pending,
    /// Outstanding committed attempt, never replayed during restore.
    Running,
    /// Unknown effect, blocked until receipt reconciliation.
    Uncertain,
    /// Successful one-shot completion.
    Completed,
    /// Terminal failure.
    Failed,
    /// Cancelled pending work.
    Cancelled,
}

/// Authorized inspection response.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct View {
    /// Consumer-allocated job identity.
    pub id: u64,
    /// Retained payload.
    pub task: Task,
    /// Checked state projection.
    pub status: Status,
    /// Pending dispatch time, absent for blocked or terminal work.
    pub due_ns: Option<u64>,
    /// Exact allocated dispatch sequence.
    pub sequence: u64,
    /// Recurring occurrence sequence.
    pub occurrence: u64,
}

/// Exact delivery identity and payload. Correlation is not authentication.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize, Serialize)]
pub struct Delivery {
    /// Job identity within the authenticated sending canister.
    pub job: u64,
    /// Logical occurrence, shared by safe retries.
    pub occurrence: u64,
    /// Exact dispatch sequence.
    pub sequence: u64,
    /// Destination canister.
    pub destination: Principal,
    /// Immutable bounded payload.
    pub message: String,
}

/// Bounded state summary used by the host harness.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct Summary {
    /// Earliest pending deadline over the entire bounded queue.
    pub next_due_ns: Option<u64>,
    /// Total local counter effects.
    pub counter: u64,
    /// Number of independently retained logical delivery receipts.
    pub receipts: u64,
    /// Whether the volatile watchdog currently owns an armed wakeup.
    pub armed: bool,
}

/// Refused management, storage or execution boundary.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum Error {
    /// Caller lacks management or receiver authority.
    Unauthorized,
    /// Job identity was not retained.
    UnknownJob,
    /// Bounded history or receipts are full.
    Capacity,
    /// Payload, page or arithmetic input is invalid.
    InvalidInput,
    /// Stored bytes contradict the current consumer contract.
    InvalidStorage,
    /// Canonical Jobs transition refused the operation.
    Job(String),
    /// Allocation bootstrap or opening failed.
    Memory(String),
    /// Receipt does not identify this immutable delivery.
    ReceiptMismatch,
}
