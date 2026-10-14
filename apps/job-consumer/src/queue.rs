use std::num::NonZeroU64;

use candid::Principal;
use ic_jobs::{
    Attempt, Job, JobId, JobRecord, JobState, MissedRunPolicy, Outcome, RetryPolicy, Schedule,
    Scheduler,
};
use ic_memory::{
    GenericAllocationPolicy, MemoryAllocationPool, MemoryAuthority, MemoryManagerConfig,
    MemoryRequest, MemoryRuntime, RuntimeMemory, SchemaMetadata, SealedDeclarationSnapshot,
    ic_stable_structures::{Cell, Memory},
};
use serde::{Deserialize, Serialize};

use crate::api::{Create, Delivery, Error, Init, Status, Task, Timing, View};

const MAX_JOBS: usize = 32;
const MAX_BYTES: usize = 24 * 1024;
const MAX_MESSAGE: usize = 256;
pub(crate) const BATCH: usize = 2;
const KEY: &str = "test.jobs.consumer.management.v1";

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    job: JobRecord,
    task: Task,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Snapshot {
    init: Init,
    next_id: u64,
    jobs: Vec<Entry>,
    counter: u64,
    receipts: Vec<Delivery>,
}

impl Snapshot {
    // Aggregate checks are shared by stored admission and typed write admission.
    // The callback preserves restore-before-entry-check ordering on storage reads.
    fn validate(
        &self,
        mut admit_job: impl FnMut(JobRecord) -> Result<(), Error>,
    ) -> Result<(), Error> {
        if self.jobs.len() > MAX_JOBS || self.receipts.len() > MAX_JOBS || self.next_id == 0 {
            return Err(Error::InvalidStorage);
        }
        let mut previous = 0;
        for entry in &self.jobs {
            admit_job(entry.job)?;
            if entry.job.id.0 <= u128::from(previous)
                || entry.job.id.0 >= u128::from(self.next_id)
                || !task_valid(&entry.task)
            {
                return Err(Error::InvalidStorage);
            }
            previous = u64::try_from(entry.job.id.0).map_err(|_| Error::InvalidStorage)?;
        }
        for (index, receipt) in self.receipts.iter().enumerate() {
            if !delivery_valid(receipt)
                || self.receipts[..index]
                    .iter()
                    .any(|other| receipt_key(other) == receipt_key(receipt))
            {
                return Err(Error::InvalidStorage);
            }
        }
        Ok(())
    }
}

// Decoding retains validated jobs. Payloads stay in the same bounded order so
// the canonical slice scheduler can operate without restoring a second batch.
struct LoadedSnapshot {
    init: Init,
    next_id: u64,
    jobs: Vec<Job>,
    tasks: Vec<Task>,
    counter: u64,
    receipts: Vec<Delivery>,
}

/// Consumer-local reply and deadline, returned only after a successful stable write.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Committed<T> {
    pub(crate) value: T,
    pub(crate) next_due_ns: Option<u64>,
}

impl LoadedSnapshot {
    fn next_due_ns(&self) -> Option<u64> {
        self.jobs.iter().filter_map(Job::next_due_ns).min()
    }

    fn index(&self, id: u64) -> Result<usize, Error> {
        self.jobs
            .iter()
            .position(|job| job.record().id == JobId(u128::from(id)))
            .ok_or(Error::UnknownJob)
    }

    fn into_stored(self) -> Result<Snapshot, Error> {
        if self.jobs.len() != self.tasks.len() {
            return Err(Error::InvalidStorage);
        }
        Ok(Snapshot {
            init: self.init,
            next_id: self.next_id,
            jobs: self
                .jobs
                .into_iter()
                .zip(self.tasks)
                .map(|(job, task)| Entry {
                    job: job.record(),
                    task,
                })
                .collect(),
            counter: self.counter,
            receipts: self.receipts,
        })
    }

    fn take_entry(mut self, id: u64) -> Result<(Job, Task), Error> {
        let index = self.index(id)?;
        if index >= self.tasks.len() {
            return Err(Error::UnknownJob);
        }
        // This snapshot is consumed; temporary reordering never reaches storage.
        Ok((self.jobs.swap_remove(index), self.tasks.swap_remove(index)))
    }

    fn receipt_index(&self, delivery: &Delivery) -> Option<usize> {
        let key = receipt_key(delivery);
        self.receipts
            .iter()
            .position(|receipt| receipt_key(receipt) == key)
    }
}

fn delivery_valid(delivery: &Delivery) -> bool {
    delivery.job != 0 && delivery.sequence != 0 && delivery.message.len() <= MAX_MESSAGE
}

// Receipt identity is one logical occurrence within the authenticated sender.
// Exact confirmation still compares the full envelope, including its sequence.
fn receipt_key(delivery: &Delivery) -> (u64, u64) {
    (delivery.job, delivery.occurrence)
}

fn delivery(
    id: JobId,
    attempt: Attempt,
    destination: Principal,
    message: String,
) -> Result<Delivery, Error> {
    Ok(Delivery {
        job: u64::try_from(id.0).map_err(|_| Error::InvalidStorage)?,
        occurrence: attempt.execution.occurrence,
        sequence: attempt.sequence,
        destination,
        message,
    })
}

// The host owns one allocation pool/bootstrap. Cell persists the application
// snapshot; neither Memory nor Jobs owns its codec or collection semantics.
pub(crate) struct Queue<M: Memory> {
    cell: Cell<Vec<u8>, RuntimeMemory<M>>,
    init: Init,
}

fn job_error(error: ic_jobs::JobError) -> Error {
    Error::Job(error.to_string())
}
fn memory_error(error: impl std::fmt::Display) -> Error {
    Error::Memory(error.to_string())
}

fn task_valid(task: &Task) -> bool {
    match task {
        Task::Increment(_) => true,
        Task::Deliver { message, .. } => message.len() <= MAX_MESSAGE,
    }
}

fn schedule(timing: Timing) -> Result<Schedule, Error> {
    Ok(match timing {
        Timing::Once(at_ns) => Schedule::Once { at_ns },
        Timing::FixedRate {
            first_ns,
            every_ns,
            catch_up,
        } => Schedule::FixedRate {
            first_at_ns: first_ns,
            every_ns: NonZeroU64::new(every_ns).ok_or(Error::InvalidInput)?,
            missed: if catch_up {
                MissedRunPolicy::CatchUp
            } else {
                MissedRunPolicy::Skip
            },
        },
        Timing::AfterCompletion { first_ns, every_ns } => Schedule::AfterCompletion {
            first_at_ns: first_ns,
            every_ns: NonZeroU64::new(every_ns).ok_or(Error::InvalidInput)?,
        },
    })
}

fn decode(bytes: &[u8]) -> Result<LoadedSnapshot, Error> {
    if bytes.len() > MAX_BYTES {
        return Err(Error::InvalidStorage);
    }
    let snapshot: Snapshot = serde_json::from_slice(bytes).map_err(|_| Error::InvalidStorage)?;
    let mut jobs = Vec::with_capacity(snapshot.jobs.len().min(MAX_JOBS));
    snapshot.validate(|record| {
        // Every decoded JobRecord enters through the validated restore boundary.
        jobs.push(Job::restore(record).map_err(job_error)?);
        Ok(())
    })?;
    Ok(LoadedSnapshot {
        init: snapshot.init,
        next_id: snapshot.next_id,
        jobs,
        tasks: snapshot.jobs.into_iter().map(|entry| entry.task).collect(),
        counter: snapshot.counter,
        receipts: snapshot.receipts,
    })
}

impl<M: Memory> Queue<M> {
    pub(crate) fn open(memory: M, initial: Option<Init>) -> Result<(Self, Option<u64>), Error> {
        let request = MemoryRequest::new("test.jobs", KEY, SchemaMetadata::default())
            .map_err(memory_error)?;
        let declarations = SealedDeclarationSnapshot::new(vec![request]).map_err(memory_error)?;
        let pool = MemoryAllocationPool::new(
            vec![MemoryAuthority::new("test.jobs", "test.jobs.").map_err(memory_error)?],
            vec![],
        )
        .map_err(memory_error)?;
        let mut runtime = MemoryRuntime::new_with_config(
            memory,
            MemoryManagerConfig::new(1).map_err(memory_error)?,
        )
        .map_err(memory_error)?;
        runtime
            .bootstrap(&declarations, &pool, &GenericAllocationPolicy)
            .map_err(memory_error)?;
        runtime
            .verify_authority(&declarations, "test.jobs")
            .map_err(memory_error)?;
        let mut cell = Cell::init(runtime.open_memory(KEY).map_err(memory_error)?, Vec::new());
        if let Some(init) = initial {
            // Installation is distinct from recovery; never overwrite retained data.
            if !cell.get().is_empty() {
                return Err(Error::InvalidStorage);
            }
            cell.set(
                serde_json::to_vec(&Snapshot {
                    init,
                    next_id: 1,
                    jobs: vec![],
                    counter: 0,
                    receipts: vec![],
                })
                .map_err(|_| Error::InvalidStorage)?,
            );
        }
        let snapshot = decode(cell.get())?;
        let next_due_ns = snapshot.next_due_ns();
        Ok((
            Self {
                cell,
                init: snapshot.init,
            },
            next_due_ns,
        ))
    }

    pub(crate) fn authorize(&self, caller: Principal) -> Result<(), Error> {
        if caller != self.init.manager {
            return Err(Error::Unauthorized);
        }
        Ok(())
    }

    fn load(&self) -> Result<LoadedSnapshot, Error> {
        let snapshot = decode(self.cell.get())?;
        if snapshot.init != self.init {
            return Err(Error::InvalidStorage);
        }
        Ok(snapshot)
    }

    fn commit<T>(&mut self, snapshot: LoadedSnapshot, value: T) -> Result<Committed<T>, Error> {
        let next_due_ns = snapshot.next_due_ns();
        let snapshot = snapshot.into_stored()?;
        let bytes = serde_json::to_vec(&snapshot).map_err(|_| Error::InvalidStorage)?;
        if bytes.len() > MAX_BYTES {
            return Err(Error::Capacity);
        }
        // Records came from validated Jobs and maintained public transitions.
        // Check collection invariants without serializing and restoring them again.
        snapshot.validate(|_| Ok(()))?;
        self.cell.set(bytes);
        Ok(Committed { value, next_due_ns })
    }

    pub(crate) fn create(&mut self, input: Create) -> Result<Committed<u64>, Error> {
        let mut snapshot = self.load()?;
        if snapshot.jobs.len() == MAX_JOBS {
            return Err(Error::Capacity);
        }
        if !task_valid(&input.task) {
            return Err(Error::InvalidInput);
        }
        let schedule = schedule(input.timing)?;
        let id = snapshot.next_id;
        snapshot.next_id = id.checked_add(1).ok_or(Error::Capacity)?;
        snapshot
            .jobs
            .push(Job::new(JobId(u128::from(id)), schedule, RetryPolicy::NONE));
        snapshot.tasks.push(input.task);
        self.commit(snapshot, id)
    }

    pub(crate) fn page(&self, after: Option<u64>, limit: u8) -> Result<Vec<View>, Error> {
        if !(1..=8).contains(&limit) {
            return Err(Error::InvalidInput);
        }
        let snapshot = self.load()?;
        snapshot
            .jobs
            .into_iter()
            .zip(snapshot.tasks)
            .filter(|(job, _)| job.record().id.0 > u128::from(after.unwrap_or(0)))
            .take(usize::from(limit))
            .map(|(job, task)| view(job, task))
            .collect()
    }

    pub(crate) fn inspect(&self, id: u64) -> Result<View, Error> {
        let (job, task) = self.load()?.take_entry(id)?;
        view(job, task)
    }

    pub(crate) fn cancel(&mut self, id: u64, now: u64) -> Result<Committed<()>, Error> {
        self.manage(id, |job| job.cancel(now).map_err(job_error))
    }

    pub(crate) fn pause(&mut self, id: u64, now: u64) -> Result<Committed<()>, Error> {
        self.manage(id, |job| job.pause(now).map_err(job_error))
    }

    pub(crate) fn resume(&mut self, id: u64, now: u64) -> Result<Committed<()>, Error> {
        self.manage(id, |job| job.resume(now).map_err(job_error))
    }

    pub(crate) fn reschedule(
        &mut self,
        id: u64,
        timing: Timing,
        now: u64,
    ) -> Result<Committed<()>, Error> {
        self.manage(id, |job| {
            job.reschedule(schedule(timing)?, now).map_err(job_error)
        })
    }

    fn manage(
        &mut self,
        id: u64,
        operation: impl FnOnce(&mut Job) -> Result<(), Error>,
    ) -> Result<Committed<()>, Error> {
        let mut snapshot = self.load()?;
        let index = snapshot.index(id)?;
        operation(&mut snapshot.jobs[index])?;
        self.commit(snapshot, ())
    }

    pub(crate) fn summary(&self) -> Result<(Option<u64>, u64, u64), Error> {
        let snapshot = self.load()?;
        let next_due = snapshot.next_due_ns();
        Ok((next_due, snapshot.counter, snapshot.receipts.len() as u64))
    }

    // No await: the complete bounded batch commits local effects and outstanding
    // external intent before any caller is given a delivery envelope.
    pub(crate) fn dispatch(
        &mut self,
        now: u64,
    ) -> Result<(Vec<Delivery>, u64, Option<u64>), Error> {
        let mut snapshot = self.load()?;
        let mut deliveries = vec![];
        let mut work = 0;
        for _ in 0..BATCH {
            let Some((index, attempt, record)) = Scheduler::new(&mut snapshot.jobs)
                .start_next(now)
                .map_err(job_error)?
            else {
                break;
            };
            work += 1;
            match &snapshot.tasks[index] {
                Task::Increment(amount) => {
                    // Known local failure has no effect; dispose of this job
                    // without rolling back unrelated work in the same batch.
                    let outcome = if let Some(counter) = snapshot.counter.checked_add(*amount) {
                        snapshot.counter = counter;
                        Outcome::Success
                    } else {
                        Outcome::PermanentFailure
                    };
                    snapshot.jobs[index]
                        .finish(attempt, now, outcome)
                        .map_err(job_error)?;
                }
                Task::Deliver {
                    destination,
                    message,
                } => deliveries.push(delivery(record.id, attempt, *destination, message.clone())?),
            }
        }
        let next_due_ns = if work > 0 {
            self.commit(snapshot, ())?.next_due_ns
        } else {
            snapshot.next_due_ns()
        };
        // Mutated batches commit before effects escape; idle batches retain bytes.
        Ok((deliveries, work, next_due_ns))
    }

    pub(crate) fn outstanding(&self, id: u64) -> Result<Delivery, Error> {
        let (job, task) = self.load()?.take_entry(id)?;
        let attempt = job.active_attempt().ok_or(Error::InvalidInput)?;
        let Task::Deliver {
            destination,
            message,
        } = task
        else {
            return Err(Error::InvalidInput);
        };
        delivery(job.record().id, attempt, destination, message)
    }

    pub(crate) fn finish(
        &mut self,
        delivery: &Delivery,
        now: u64,
        outcome: Outcome,
    ) -> Result<Committed<()>, Error> {
        let mut snapshot = self.load()?;
        let index = snapshot.index(delivery.job)?;
        let job = &mut snapshot.jobs[index];
        let task = &snapshot.tasks[index];
        let attempt = job.active_attempt().ok_or(Error::InvalidInput)?;
        let Task::Deliver {
            destination,
            message,
        } = task
        else {
            return Err(Error::InvalidInput);
        };
        let id = u64::try_from(job.record().id.0).map_err(|_| Error::InvalidStorage)?;
        if delivery.job != id
            || delivery.occurrence != attempt.execution.occurrence
            || delivery.sequence != attempt.sequence
            || delivery.destination != *destination
            || delivery.message != *message
        {
            return Err(Error::ReceiptMismatch);
        }
        if matches!(job.record().state, JobState::Uncertain { .. }) {
            job.resolve(attempt, now, outcome).map_err(job_error)?;
        } else {
            job.finish(attempt, now, outcome).map_err(job_error)?;
        }
        self.commit(snapshot, ())
    }

    pub(crate) fn receive(
        &mut self,
        caller: Principal,
        delivery: Delivery,
    ) -> Result<Delivery, Error> {
        if self.init.delivery_sender != Some(caller) {
            return Err(Error::Unauthorized);
        }
        if !delivery_valid(&delivery) {
            return Err(Error::InvalidInput);
        }
        let mut snapshot = self.load()?;
        if let Some(index) = snapshot.receipt_index(&delivery) {
            return if snapshot.receipts[index] == delivery {
                Ok(delivery)
            } else {
                Err(Error::ReceiptMismatch)
            };
        }
        if snapshot.receipts.len() == MAX_JOBS {
            return Err(Error::Capacity);
        }
        snapshot.receipts.push(delivery.clone());
        self.commit(snapshot, ())?;
        Ok(delivery)
    }

    pub(crate) fn receipt(
        &self,
        caller: Principal,
        delivery: &Delivery,
    ) -> Result<Option<Delivery>, Error> {
        if self.init.delivery_sender != Some(caller) {
            return Err(Error::Unauthorized);
        }
        let mut snapshot = self.load()?;
        Ok(snapshot
            .receipt_index(delivery)
            .map(|index| snapshot.receipts.swap_remove(index)))
    }

    // Deliberate corruption endpoint in an unpublished qualification artifact.
    // Normal readers, including post_upgrade, reject malformed configuration
    // during bounded decoding before the aggregate Job::restore boundary.
    pub(crate) fn corrupt_record(&mut self, id: u64) -> Result<(), Error> {
        let snapshot = self.load()?.into_stored()?;
        let index = snapshot
            .jobs
            .iter()
            .position(|entry| entry.job.id == JobId(u128::from(id)))
            .ok_or(Error::UnknownJob)?;
        let mut encoded = serde_json::to_value(snapshot).map_err(|_| Error::InvalidStorage)?;
        encoded["jobs"][index]["job"]["retry"]["max_attempts"] = 0.into();
        self.cell
            .set(serde_json::to_vec(&encoded).map_err(|_| Error::InvalidStorage)?);
        Ok(())
    }
}

fn view(job: Job, task: Task) -> Result<View, Error> {
    let record = job.record();
    Ok(View {
        id: u64::try_from(record.id.0).map_err(|_| Error::InvalidStorage)?,
        task,
        status: match record.state {
            JobState::Pending { .. } => Status::Pending,
            JobState::Paused { .. } => Status::Paused,
            JobState::Running { .. } => Status::Running,
            JobState::Uncertain { .. } => Status::Uncertain,
            JobState::Completed => Status::Completed,
            JobState::Failed => Status::Failed,
            JobState::Cancelled => Status::Cancelled,
        },
        due_ns: job.next_due_ns(),
        sequence: record.sequence,
        occurrence: record.occurrence,
    })
}

#[cfg(test)]
mod tests;
