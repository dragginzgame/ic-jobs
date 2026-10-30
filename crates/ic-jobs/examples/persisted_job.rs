//! Consumer-owned atomic commits and receipt recovery for one bounded job.
//!
//! Run: cargo run -p ic-jobs --locked --example persisted_job
//! Byte replacement models storage; the effect/receipt below are native substitutes.
//! An IC adapter supplies real storage, authenticated receipts and watchdog calls.
use std::{error::Error, io};

use ic_jobs::{
    Attempt, Job, JobError, JobId, JobRecord, JobState, Outcome, RetryPolicy, Schedule,
    next_due_ns, start_next,
};
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const MAX_BYTES: usize = 4096; // Example policy; adapters choose their own budgets.

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Entry<J> {
    job: J,
    payload: String,
    label: String, // Editable management metadata, outside immutable effect identity.
}

#[derive(Clone)]
struct Delivery {
    attempt: Attempt,
    payload: String,
}

#[derive(Default)]
struct Store {
    bytes: Vec<u8>,
    fail_next_commit: bool, // Volatile failure injection, never durable authority.
}

impl Store {
    fn load(&self) -> Result<Entry<Job>> {
        if self.bytes.len() > MAX_BYTES {
            return Err(io::Error::other("snapshot too large").into());
        }
        let stored: Entry<JobRecord> = serde_json::from_slice(&self.bytes)?;
        Ok(Entry {
            job: Job::restore(stored.job)?,
            payload: stored.payload,
            label: stored.label,
        })
    }

    fn commit(&mut self, entry: &Entry<Job>) -> Result<()> {
        // Job serializes through its canonical JobRecord representation.
        let bytes = serde_json::to_vec(entry)?;
        if bytes.len() > MAX_BYTES {
            return Err(io::Error::other("snapshot too large").into());
        }
        if std::mem::take(&mut self.fail_next_commit) {
            return Err(io::Error::other("injected commit failure").into());
        }
        self.bytes = bytes; // Atomic replacement only in this native model.
        Ok(())
    }

    fn deadline(&self) -> Result<Option<u64>> {
        let entry = self.load()?;
        Ok(next_due_ns([&entry.job]))
    }
}

fn prepare(store: &mut Store, now_ns: u64) -> Result<Option<Delivery>> {
    let mut entry = store.load()?;
    // One job is a bounded candidate set. Queue adapters validate the whole set
    // and keep any due-time index in the same atomic commit as the records.
    let Some((_, attempt)) = start_next([&mut entry.job], now_ns)? else {
        return Ok(None);
    };
    let delivery = Delivery {
        attempt,
        payload: entry.payload.clone(),
    };
    store.commit(&entry)?;
    // Publish intent only after commit. On failure, the tentative Job is dropped.
    Ok(Some(delivery))
}

fn commit_transition(
    store: &mut Store,
    delivery: &Delivery,
    transition: impl FnOnce(&mut Job) -> std::result::Result<(), JobError>,
) -> Result<()> {
    // Reload current authority after an await; never overwrite newer state with
    // a cached pre-call Job. The transition checks the exact attempt below.
    let mut entry = store.load()?;
    if entry.job.record().id != delivery.attempt.execution.job || entry.payload != delivery.payload
    {
        return Err(io::Error::other("receipt identity or payload mismatch").into());
    }
    transition(&mut entry.job)?;
    store.commit(&entry)
}

fn simulate_destination(delivery: &Delivery, effects: &mut usize) -> Delivery {
    *effects += 1;
    delivery.clone()
}

fn receipt_recovery() -> Result<()> {
    let mut store = Store::default();
    store.commit(&Entry {
        job: Job::new(JobId(1), Schedule::Once { at_ns: 100 }, RetryPolicy::NONE),
        payload: "notify alice".into(),
        label: "notification".into(),
    })?;
    let pending = store.bytes.clone();
    let mut wakeup = store.deadline()?;
    assert_eq!(wakeup, Some(100));

    store.fail_next_commit = true;
    assert!(prepare(&mut store, 100).is_err()); // No delivery escapes.
    assert_eq!(store.bytes, pending);
    assert_eq!(store.deadline()?, wakeup);

    let delivery = prepare(&mut store, 100)?.ok_or(JobError::NotDue)?;
    let committed_intent = store.bytes.clone();
    wakeup = store.deadline()?;
    assert_eq!(wakeup, None);
    // The adapter reconciles its real watchdog from committed authority here.
    // On IC, rearm failure must trap/rollback the same message before dispatch.

    let mut effects = 0;
    let receipt = simulate_destination(&delivery, &mut effects);
    store.fail_next_commit = true;
    assert!(
        commit_transition(&mut store, &delivery, |job| {
            job.finish(delivery.attempt, 101, Outcome::Success)
        })
        .is_err()
    );
    assert_eq!(store.bytes, committed_intent);
    drop(delivery);

    // Reconstruction reads only committed bytes. An effect may already have run:
    // neither Running nor Uncertain authorizes automatic redelivery.
    store = Store {
        bytes: committed_intent,
        ..Store::default()
    };
    let restored = store.load()?;
    assert_eq!(restored.job.record().state, JobState::Running);
    let recovered = Delivery {
        attempt: restored
            .job
            .active_attempt()
            .ok_or(JobError::StaleAttempt)?,
        payload: restored.payload,
    };
    assert!(prepare(&mut store, 200)?.is_none());
    commit_transition(&mut store, &recovered, |job| {
        job.finish(recovered.attempt, 201, Outcome::Uncertain)
    })?;
    assert!(prepare(&mut store, 202)?.is_none());

    let unresolved = store.bytes.clone();
    let mut stale = receipt.clone();
    stale.attempt.sequence += 1;
    assert!(
        commit_transition(&mut store, &stale, |job| {
            job.resolve(stale.attempt, 203, Outcome::Success)
        })
        .is_err()
    );
    assert_eq!(store.bytes, unresolved);
    // A real adapter obtains this evidence from its authenticated destination;
    // timeout, rejection or missing evidence alone must not manufacture success.
    // This fixture retains the destination's receipt outside the restored Store.
    commit_transition(&mut store, &receipt, |job| {
        job.resolve(receipt.attempt, 203, Outcome::Success)
    })?;
    wakeup = store.deadline()?; // Reconcile only after the result commit succeeds.
    assert_eq!(wakeup, None);
    assert_eq!(store.load()?.job.record().state, JobState::Completed);
    assert_eq!(effects, 1);

    // A duplicate authenticated receipt is no new transition or effect. An
    // adapter may return its own retained terminal result after checking identity.
    let completed = store.bytes.clone();
    let error = commit_transition(&mut store, &receipt, |job| {
        job.resolve(receipt.attempt, 204, Outcome::Success)
    })
    .unwrap_err();
    assert_eq!(
        error.downcast_ref::<JobError>(),
        Some(&JobError::NotUncertain)
    );
    assert_eq!(store.bytes, completed);
    assert_eq!(effects, 1);
    Ok(())
}

fn retry_and_late_completion() -> Result<()> {
    let mut store = Store::default();
    store.commit(&Entry {
        job: Job::new(
            JobId(2),
            Schedule::Once { at_ns: 100 },
            RetryPolicy::new(2, 5, 5)?,
        ),
        payload: "notify bob".into(),
        label: "original label".into(),
    })?;
    let first = prepare(&mut store, 100)?.ok_or(JobError::NotDue)?;
    commit_transition(&mut store, &first, |job| {
        job.finish(first.attempt, 101, Outcome::Uncertain)
    })?;
    assert!(prepare(&mut store, 102)?.is_none());
    // The application has now authenticated exact no-effect evidence. Elapsed
    // time or lease expiry alone must never manufacture this classification.
    commit_transition(&mut store, &first, |job| {
        job.resolve(first.attempt, 102, Outcome::RetryableFailure)
    })?;
    let second = prepare(&mut store, 107)?.ok_or(JobError::NotDue)?;
    assert_eq!(first.attempt.execution, second.attempt.execution);
    assert_ne!(first.attempt.sequence, second.attempt.sequence);
    assert_eq!(first.payload, second.payload);

    // An older continuation resumes while the new attempt is Running. Its
    // correlation cannot grant disposition of the successor, whatever it reports.
    let running = store.bytes.clone();
    let error = commit_transition(&mut store, &first, |job| {
        job.finish(first.attempt, 108, Outcome::Success)
    })
    .unwrap_err();
    assert_eq!(
        error.downcast_ref::<JobError>(),
        Some(&JobError::StaleAttempt)
    );
    assert_eq!(store.bytes, running);

    // Management commits while the current delivery is awaiting. Completion
    // reloads this entry and preserves its new label instead of writing a cached
    // pre-call snapshot. The immutable payload and active attempt are unchanged.
    let mut managed = store.load()?;
    managed.label = "reviewed during delivery".into();
    store.commit(&managed)?;
    let mut effects = 0;
    let receipt = simulate_destination(&second, &mut effects);
    commit_transition(&mut store, &receipt, |job| {
        job.finish(receipt.attempt, 109, Outcome::Success)
    })?;
    let completed = store.load()?;
    assert_eq!(completed.label, "reviewed during delivery");
    assert_eq!(completed.payload, second.payload);
    assert_eq!(completed.job.record().state, JobState::Completed);
    assert_eq!(effects, 1);
    assert_eq!(store.deadline()?, None);
    Ok(())
}

fn main() -> Result<()> {
    receipt_recovery()?;
    retry_and_late_completion()?;
    println!("Receipt recovery and safe retry preserve one effect and current committed state.");
    Ok(())
}
