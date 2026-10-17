//! Durable job metadata and checked scheduling policies above IC timer primitives.
//!
//! Applications persist the canonical [JobRecord] beside their payload, commit
//! dispatch intent before an external effect, and restore records with [Job::restore].
//! This crate owns no database, endpoints, lifecycle exports, clock or global queue.
//! Applications integrate directly with IC Timers when they need wakeups.
//!
//! An uncertain effect blocks further dispatch until the application reconciles
//! it or explicitly establishes that repeating it is safe.

#![forbid(unsafe_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

mod job;
mod policy;
mod scheduler;

pub use job::{
    Attempt, ExecutionId, Job, JobError, JobId, JobRecord, JobState, Outcome, RecordedOutcome,
};
pub use policy::{MissedRunPolicy, RetryPolicy, Schedule};
pub use scheduler::{next_due_ns, start_next};
