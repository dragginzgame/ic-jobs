use candid::Principal;
use ic_jobs::{
    Attempt, ExecutionId, Job, JobId, JobRecord, JobState, MissedRunPolicy, Outcome, RetryPolicy,
    Schedule, Scheduler,
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
const KEY: &str = "test.jobs.consumer.v1";

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    job: JobRecord,
    task: Task,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Snapshot {
    init: Init,
    next_id: u64,
    jobs: Vec<Entry>,
    counter: u64,
    receipts: Vec<Delivery>,
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

fn decode(bytes: &[u8]) -> Result<Snapshot, Error> {
    if bytes.len() > MAX_BYTES {
        return Err(Error::InvalidStorage);
    }
    let snapshot: Snapshot = serde_json::from_slice(bytes).map_err(|_| Error::InvalidStorage)?;
    if snapshot.jobs.len() > MAX_JOBS || snapshot.receipts.len() > MAX_JOBS || snapshot.next_id == 0
    {
        return Err(Error::InvalidStorage);
    }
    let mut previous = 0;
    for entry in &snapshot.jobs {
        // This is the sole admission boundary for decoded JobRecord values.
        Job::restore(entry.job).map_err(job_error)?;
        if entry.job.id.0 <= u128::from(previous)
            || entry.job.id.0 >= u128::from(snapshot.next_id)
            || !task_valid(&entry.task)
        {
            return Err(Error::InvalidStorage);
        }
        previous = u64::try_from(entry.job.id.0).map_err(|_| Error::InvalidStorage)?;
    }
    for (index, receipt) in snapshot.receipts.iter().enumerate() {
        if receipt.job == 0
            || receipt.sequence == 0
            || receipt.message.len() > MAX_MESSAGE
            || snapshot.receipts[..index]
                .iter()
                .any(|other| other.job == receipt.job && other.occurrence == receipt.occurrence)
        {
            return Err(Error::InvalidStorage);
        }
    }
    Ok(snapshot)
}

impl<M: Memory> Queue<M> {
    pub(crate) fn open(memory: M, initial: Option<Init>) -> Result<Self, Error> {
        let request = MemoryRequest::new("test.jobs", KEY, SchemaMetadata::default())
            .map_err(memory_error)?;
        let declarations = SealedDeclarationSnapshot::new(&[request]).map_err(memory_error)?;
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
        let init = decode(cell.get())?.init;
        Ok(Self { cell, init })
    }

    pub(crate) fn authorize(&self, caller: Principal) -> Result<(), Error> {
        if caller != self.init.manager {
            return Err(Error::Unauthorized);
        }
        Ok(())
    }

    fn load(&self) -> Result<Snapshot, Error> {
        let snapshot = decode(self.cell.get())?;
        if snapshot.init != self.init {
            return Err(Error::InvalidStorage);
        }
        Ok(snapshot)
    }

    fn commit(&mut self, snapshot: Snapshot) -> Result<(), Error> {
        let bytes = serde_json::to_vec(&snapshot).map_err(|_| Error::InvalidStorage)?;
        if bytes.len() > MAX_BYTES {
            return Err(Error::Capacity);
        }
        decode(&bytes)?;
        self.cell.set(bytes);
        Ok(())
    }

    pub(crate) fn create(&mut self, input: Create) -> Result<u64, Error> {
        let mut snapshot = self.load()?;
        if snapshot.jobs.len() == MAX_JOBS {
            return Err(Error::Capacity);
        }
        if !task_valid(&input.task) {
            return Err(Error::InvalidInput);
        }
        let schedule = match input.timing {
            Timing::Once(at_ns) => Schedule::Once { at_ns },
            Timing::FixedRate {
                first_ns,
                every_ns,
                catch_up,
            } => Schedule::FixedRate {
                first_at_ns: first_ns,
                every_ns,
                missed: if catch_up {
                    MissedRunPolicy::CatchUp
                } else {
                    MissedRunPolicy::Skip
                },
            },
        };
        let id = snapshot.next_id;
        snapshot.next_id = id.checked_add(1).ok_or(Error::Capacity)?;
        snapshot.jobs.push(Entry {
            job: Job::new(JobId(u128::from(id)), schedule, RetryPolicy::NONE)
                .map_err(job_error)?
                .record(),
            task: input.task,
        });
        self.commit(snapshot)?;
        Ok(id)
    }

    pub(crate) fn page(&self, after: Option<u64>, limit: u8) -> Result<Vec<View>, Error> {
        if !(1..=8).contains(&limit) {
            return Err(Error::InvalidInput);
        }
        self.load()?
            .jobs
            .into_iter()
            .filter(|entry| entry.job.id.0 > u128::from(after.unwrap_or(0)))
            .take(usize::from(limit))
            .map(view)
            .collect()
    }

    pub(crate) fn inspect(&self, id: u64) -> Result<View, Error> {
        view(
            self.load()?
                .jobs
                .into_iter()
                .find(|entry| entry.job.id == JobId(u128::from(id)))
                .ok_or(Error::UnknownJob)?,
        )
    }

    pub(crate) fn cancel(&mut self, id: u64, now: u64) -> Result<(), Error> {
        let mut snapshot = self.load()?;
        let entry = snapshot
            .jobs
            .iter_mut()
            .find(|entry| entry.job.id == JobId(u128::from(id)))
            .ok_or(Error::UnknownJob)?;
        let mut job = Job::restore(entry.job).map_err(job_error)?;
        job.cancel(now).map_err(job_error)?;
        entry.job = job.record();
        self.commit(snapshot)
    }

    pub(crate) fn summary(&self) -> Result<(Option<u64>, u64, u64), Error> {
        let snapshot = self.load()?;
        let mut jobs = snapshot
            .jobs
            .iter()
            .map(|entry| Job::restore(entry.job).map_err(job_error))
            .collect::<Result<Vec<_>, _>>()?;
        Ok((
            Scheduler::new(&mut jobs).next_due_ns(),
            snapshot.counter,
            snapshot.receipts.len() as u64,
        ))
    }

    // No await: the complete bounded batch commits local effects and outstanding
    // external intent before any caller is given a delivery envelope.
    pub(crate) fn dispatch(&mut self, now: u64) -> Result<(Vec<Delivery>, u64), Error> {
        let mut snapshot = self.load()?;
        let mut jobs = snapshot
            .jobs
            .iter()
            .map(|entry| Job::restore(entry.job).map_err(job_error))
            .collect::<Result<Vec<_>, _>>()?;
        let mut deliveries = vec![];
        let mut work = 0;
        for _ in 0..BATCH {
            let Some((attempt, record)) = Scheduler::new(&mut jobs)
                .start_next(now)
                .map_err(job_error)?
            else {
                break;
            };
            work += 1;
            let index = snapshot
                .jobs
                .iter()
                .position(|entry| entry.job.id == record.id)
                .ok_or(Error::InvalidStorage)?;
            match &snapshot.jobs[index].task {
                Task::Increment(amount) => {
                    snapshot.counter = snapshot
                        .counter
                        .checked_add(*amount)
                        .ok_or(Error::InvalidInput)?;
                    jobs[index]
                        .finish(attempt, now, Outcome::Success)
                        .map_err(job_error)?;
                }
                Task::Deliver {
                    destination,
                    message,
                } => deliveries.push(Delivery {
                    job: u64::try_from(record.id.0).map_err(|_| Error::InvalidStorage)?,
                    occurrence: attempt.execution.occurrence,
                    sequence: attempt.sequence,
                    destination: *destination,
                    message: message.clone(),
                }),
            }
        }
        for (entry, job) in snapshot.jobs.iter_mut().zip(jobs) {
            entry.job = job.record();
        }
        self.commit(snapshot)?;
        Ok((deliveries, work))
    }

    pub(crate) fn outstanding(&self, id: u64) -> Result<Delivery, Error> {
        let entry = self
            .load()?
            .jobs
            .into_iter()
            .find(|entry| entry.job.id == JobId(u128::from(id)))
            .ok_or(Error::UnknownJob)?;
        if !matches!(
            entry.job.state,
            JobState::Running { .. } | JobState::Uncertain { .. }
        ) {
            return Err(Error::InvalidInput);
        }
        let Task::Deliver {
            destination,
            message,
        } = entry.task
        else {
            return Err(Error::InvalidInput);
        };
        Ok(Delivery {
            job: id,
            occurrence: entry.job.occurrence,
            sequence: entry.job.sequence,
            destination,
            message,
        })
    }

    pub(crate) fn finish(
        &mut self,
        delivery: &Delivery,
        now: u64,
        outcome: Outcome,
    ) -> Result<(), Error> {
        if self.outstanding(delivery.job)? != *delivery {
            return Err(Error::ReceiptMismatch);
        }
        let mut snapshot = self.load()?;
        let entry = snapshot
            .jobs
            .iter_mut()
            .find(|entry| entry.job.id == JobId(u128::from(delivery.job)))
            .ok_or(Error::UnknownJob)?;
        let mut job = Job::restore(entry.job).map_err(job_error)?;
        let attempt = Attempt {
            execution: ExecutionId {
                job: entry.job.id,
                occurrence: delivery.occurrence,
            },
            sequence: delivery.sequence,
        };
        if matches!(entry.job.state, JobState::Uncertain { .. }) {
            job.resolve(attempt, now, outcome).map_err(job_error)?;
        } else {
            job.finish(attempt, now, outcome).map_err(job_error)?;
        }
        entry.job = job.record();
        self.commit(snapshot)
    }

    pub(crate) fn receive(
        &mut self,
        caller: Principal,
        delivery: Delivery,
    ) -> Result<Delivery, Error> {
        if self.init.delivery_sender != Some(caller) {
            return Err(Error::Unauthorized);
        }
        if delivery.job == 0 || delivery.sequence == 0 || delivery.message.len() > MAX_MESSAGE {
            return Err(Error::InvalidInput);
        }
        let mut snapshot = self.load()?;
        if let Some(receipt) = snapshot.receipts.iter().find(|receipt| {
            receipt.job == delivery.job && receipt.occurrence == delivery.occurrence
        }) {
            return if *receipt == delivery {
                Ok(receipt.clone())
            } else {
                Err(Error::ReceiptMismatch)
            };
        }
        if snapshot.receipts.len() == MAX_JOBS {
            return Err(Error::Capacity);
        }
        snapshot.receipts.push(delivery.clone());
        self.commit(snapshot)?;
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
        Ok(self.load()?.receipts.into_iter().find(|receipt| {
            receipt.job == delivery.job && receipt.occurrence == delivery.occurrence
        }))
    }

    // Deliberate corruption endpoint in an unpublished qualification artifact.
    // Normal readers, including post_upgrade, still reject this at Job::restore.
    pub(crate) fn corrupt_record(&mut self, id: u64) -> Result<(), Error> {
        let mut snapshot = self.load()?;
        let entry = snapshot
            .jobs
            .iter_mut()
            .find(|entry| entry.job.id == JobId(u128::from(id)))
            .ok_or(Error::UnknownJob)?;
        entry.job.retry.max_attempts = 0;
        self.cell
            .set(serde_json::to_vec(&snapshot).map_err(|_| Error::InvalidStorage)?);
        Ok(())
    }
}

fn view(entry: Entry) -> Result<View, Error> {
    let job = Job::restore(entry.job).map_err(job_error)?;
    Ok(View {
        id: u64::try_from(entry.job.id.0).map_err(|_| Error::InvalidStorage)?,
        task: entry.task,
        status: match entry.job.state {
            JobState::Pending { .. } => Status::Pending,
            JobState::Running { .. } => Status::Running,
            JobState::Uncertain { .. } => Status::Uncertain,
            JobState::Completed => Status::Completed,
            JobState::Failed => Status::Failed,
            JobState::Cancelled => Status::Cancelled,
        },
        due_ns: job.next_due_ns(),
        sequence: entry.job.sequence,
        occurrence: entry.job.occurrence,
    })
}

#[cfg(test)]
mod tests;
