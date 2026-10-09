//! Durable job metadata and checked scheduling policies above IC timer primitives.
//!
//! Applications persist the canonical [JobRecord] beside their payload, commit
//! dispatch intent before an external effect, and restore records with [Job::restore].
//! This crate owns no database, endpoints, lifecycle exports, clock or global queue.
//! Enable the timers feature for the optional IC Timers adapter (Rust 1.88+).
//!
//! An uncertain effect blocks further dispatch until the application reconciles
//! it or explicitly establishes that repeating it is safe.

#![forbid(unsafe_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

mod job;
mod policy;
mod scheduler;
#[cfg(feature = "timers")]
pub mod timers;

pub use job::{Attempt, ExecutionId, Job, JobError, JobId, JobRecord, JobState, Outcome};
pub use policy::{MissedRunPolicy, RetryPolicy, Schedule};
pub use scheduler::Scheduler;
