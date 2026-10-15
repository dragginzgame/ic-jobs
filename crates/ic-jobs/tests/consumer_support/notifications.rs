use super::{Result, Store};
use ic_jobs::{
    Attempt, ExecutionId, Job, JobError, JobId, JobRecord, Outcome, RetryPolicy, Schedule,
    next_due_ns, start_next,
};
use ic_memory::{MemoryRequest, SchemaMetadata};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt, io};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Notification {
    pub(crate) recipient: String,
    pub(crate) body: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct NotificationSnapshot<P = Notification> {
    pub(crate) job: JobRecord,
    pub(crate) payload: P,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Delivery {
    pub(crate) attempt: Attempt,
    pub(crate) payload: Notification,
}

// The synchronous scheduler produces committed intent for a separate delivery
// continuation. Neither the scheduler nor a watchdog callback calls a receiver.
pub(crate) struct Notifications {
    pub(crate) store: Store,
}

// The fixture receives a trusted caller from its application boundary. A real
// canister obtains that identity from the IC caller API, never request payloads.
pub(crate) const NOTIFICATION_MANAGER: &str = "notification-manager";

#[derive(Clone)]
pub(crate) enum ManagementCommand {
    Enqueue {
        id: JobId,
        at_ns: u64,
        payload: Notification,
    },
    Inspect(JobId),
    List {
        after: Option<JobId>,
        limit: usize,
    },
    Cancel(JobId),
    DispatchDue,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ManagementValue {
    Job(NotificationSnapshot),
    Page(Vec<NotificationSnapshot>),
    Dispatch(Option<Delivery>),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ManagementReply {
    pub(crate) value: ManagementValue,
    // The application projects this committed queue deadline into its watchdog.
    // Native execution does not register or qualify a platform timer.
    pub(crate) next_due_ns: Option<u64>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ManagementError {
    Unauthorized,
    UnknownJob,
    InvalidPageLimit,
}

impl fmt::Display for ManagementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Unauthorized => "caller cannot manage this notification queue",
            Self::UnknownJob => "notification job is not retained",
            Self::InvalidPageLimit => "page limit must be between one and four",
        })
    }
}

impl Error for ManagementError {}

impl Notifications {
    fn index(jobs: &[Job], id: JobId) -> Option<usize> {
        jobs.iter().position(|job| job.record().id == id)
    }

    // One consumer-owned store holds records and immutable payloads together.
    // The eventual host supplies allocation policy and owns one bootstrap.
    pub(crate) fn memory_request() -> Result<MemoryRequest> {
        Ok(MemoryRequest::new(
            "test.notifications",
            "test.notifications.queue.v1",
            SchemaMetadata::default(),
        )?)
    }

    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            store: Store::new(&vec![NotificationSnapshot {
                job: Job::new(
                    JobId(1),
                    Schedule::Once { at_ns: 100 },
                    RetryPolicy::new(2, 10, 10).unwrap(),
                )
                .record(),
                payload: Notification {
                    recipient: "alice".into(),
                    body: "Your report is ready".into(),
                },
            }])?,
        })
    }

    pub(crate) fn load(&self) -> Result<(Vec<Job>, Vec<Notification>)> {
        let snapshots: Vec<NotificationSnapshot> = self.store.read()?;
        if snapshots.len() > 4 {
            return Err(
                io::Error::new(io::ErrorKind::InvalidData, "too many notifications").into(),
            );
        }
        let mut jobs = Vec::with_capacity(snapshots.len());
        let mut payloads = Vec::with_capacity(snapshots.len());
        for snapshot in snapshots {
            if jobs
                .iter()
                .any(|job: &Job| job.record().id == snapshot.job.id)
            {
                return Err(
                    io::Error::new(io::ErrorKind::InvalidData, "duplicate job identity").into(),
                );
            }
            jobs.push(Job::restore(snapshot.job)?);
            payloads.push(snapshot.payload);
        }
        Ok((jobs, payloads))
    }

    pub(crate) fn commit(&mut self, jobs: &[Job], payloads: &[Notification]) -> Result<()> {
        if jobs.len() != payloads.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unaligned notification payloads",
            )
            .into());
        }
        let snapshots: Vec<_> = jobs
            .iter()
            .zip(payloads)
            .map(|(job, payload)| NotificationSnapshot {
                job: job.record(),
                payload,
            })
            .collect();
        self.store.write(&snapshots)
    }

    pub(crate) fn enqueue(&mut self, id: JobId, at_ns: u64, payload: Notification) -> Result<()> {
        self.manage(
            NOTIFICATION_MANAGER,
            0,
            ManagementCommand::Enqueue { id, at_ns, payload },
        )
        .map(|_| ())
    }

    pub(crate) fn manage(
        &mut self,
        caller: &str,
        now_ns: u64,
        command: ManagementCommand,
    ) -> Result<ManagementReply> {
        // Authorization precedes decoding, reads of private payloads and writes.
        if caller != NOTIFICATION_MANAGER {
            return Err(ManagementError::Unauthorized.into());
        }
        if matches!(&command, ManagementCommand::List { limit, .. } if !(1..=4).contains(limit)) {
            return Err(ManagementError::InvalidPageLimit.into());
        }
        let (mut jobs, mut payloads) = self.load()?;
        let value = match command {
            ManagementCommand::Enqueue { id, at_ns, payload } => {
                if jobs.len() == 4 || jobs.iter().any(|job| job.record().id == id) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "queue full or identity reused",
                    )
                    .into());
                }
                let job = Job::new(id, Schedule::Once { at_ns }, RetryPolicy::NONE);
                let record = job.record();
                payloads.push(payload);
                jobs.push(job);
                self.commit(&jobs, &payloads)?;
                ManagementValue::Job(NotificationSnapshot {
                    job: record,
                    payload: payloads.pop().unwrap(),
                })
            }
            ManagementCommand::Inspect(id) => {
                let index = Self::index(&jobs, id).ok_or(ManagementError::UnknownJob)?;
                ManagementValue::Job(NotificationSnapshot {
                    job: jobs[index].record(),
                    payload: payloads.swap_remove(index),
                })
            }
            ManagementCommand::List { after, limit } => {
                let mut snapshots: Vec<_> = jobs
                    .iter()
                    .zip(payloads)
                    .filter(|(job, _)| after.is_none_or(|id| job.record().id > id))
                    .map(|(job, payload)| NotificationSnapshot {
                        job: job.record(),
                        payload,
                    })
                    .collect();
                snapshots.sort_by_key(|snapshot| snapshot.job.id);
                snapshots.truncate(limit);
                ManagementValue::Page(snapshots)
            }
            ManagementCommand::Cancel(id) => {
                let index = Self::index(&jobs, id).ok_or(ManagementError::UnknownJob)?;
                jobs[index].cancel(now_ns)?;
                self.commit(&jobs, &payloads)?;
                ManagementValue::Job(NotificationSnapshot {
                    job: jobs[index].record(),
                    payload: payloads.swap_remove(index),
                })
            }
            ManagementCommand::DispatchDue => {
                ManagementValue::Dispatch(self.dispatch_batch(now_ns, &mut jobs, &payloads)?)
            }
        };
        Ok(ManagementReply {
            value,
            next_due_ns: next_due_ns(&jobs),
        })
    }

    pub(crate) fn next_due_ns(&self) -> Result<Option<u64>> {
        let (jobs, _) = self.load()?;
        Ok(next_due_ns(&jobs))
    }

    pub(crate) fn prepare(&mut self, now_ns: u64) -> Result<Option<Delivery>> {
        let (mut jobs, payloads) = self.load()?;
        self.dispatch_batch(now_ns, &mut jobs, &payloads)
    }

    fn dispatch_batch(
        &mut self,
        now_ns: u64,
        jobs: &mut [Job],
        payloads: &[Notification],
    ) -> Result<Option<Delivery>> {
        let Some((index, attempt)) = start_next(jobs, now_ns)? else {
            return Ok(None);
        };
        let payload = payloads[index].clone();
        self.commit(jobs, payloads)?;
        // No delivery capability escapes before the intent and payload commit.
        Ok(Some(Delivery { attempt, payload }))
    }

    pub(crate) fn outstanding(&self) -> Result<Vec<Delivery>> {
        // Inspect unresolved envelopes for reconciliation, never for automatic replay.
        let (jobs, payloads) = self.load()?;
        Ok(jobs
            .iter()
            .zip(payloads)
            .filter_map(|(job, payload)| {
                job.active_attempt()
                    .map(|attempt| Delivery { attempt, payload })
            })
            .collect())
    }

    pub(crate) fn finish(&mut self, attempt: Attempt, now_ns: u64, outcome: Outcome) -> Result<()> {
        let (mut jobs, payloads) = self.load()?;
        let index = Self::index(&jobs, attempt.execution.job).ok_or(JobError::StaleAttempt)?;
        jobs[index].finish(attempt, now_ns, outcome)?;
        self.commit(&jobs, &payloads)
    }

    pub(crate) fn reconcile(
        &mut self,
        attempt: Attempt,
        destination: &Destination,
        now_ns: u64,
    ) -> Result<bool> {
        let (mut jobs, payloads) = self.load()?;
        let index = Self::index(&jobs, attempt.execution.job).ok_or(JobError::StaleAttempt)?;
        let outcome = if destination.receipts.iter().any(|(execution, payload)| {
            *execution == attempt.execution && payload == &payloads[index]
        }) {
            Outcome::Success
        } else if destination
            .rejections
            .iter()
            .any(|(rejected, payload)| *rejected == attempt && payload == &payloads[index])
        {
            // Exact destination evidence confirms this attempt had no effect.
            Outcome::RetryableFailure
        } else {
            // A missing receipt is not proof that delivery failed.
            return Ok(false);
        };
        jobs[index].resolve(attempt, now_ns, outcome)?;
        self.commit(&jobs, &payloads)?;
        Ok(true)
    }
}

// A separate destination owns receipts/idempotency, outside the sender's
// reconstructed history. This fixture retains at most four logical deliveries.
#[derive(Default)]
pub(crate) struct Destination {
    pub(crate) receipts: Vec<(ExecutionId, Notification)>,
    pub(crate) rejections: Vec<(Attempt, Notification)>,
    pub(crate) reject_next: bool,
    pub(crate) calls: usize,
}

impl Destination {
    pub(crate) fn deliver(&mut self, delivery: &Delivery) -> bool {
        self.calls += 1;
        if let Some((_, payload)) = self
            .receipts
            .iter()
            .find(|(execution, _)| *execution == delivery.attempt.execution)
        {
            assert_eq!(payload, &delivery.payload);
            return true;
        }
        if let Some((_, payload)) = self
            .rejections
            .iter()
            .find(|(attempt, _)| *attempt == delivery.attempt)
        {
            assert_eq!(payload, &delivery.payload);
            return false;
        }
        if std::mem::take(&mut self.reject_next) {
            // At most two attempts for each of this fixture's four jobs.
            assert!(self.rejections.len() < 8);
            self.rejections
                .push((delivery.attempt, delivery.payload.clone()));
            return false;
        }
        assert!(self.receipts.len() < 4);
        self.receipts
            .push((delivery.attempt.execution, delivery.payload.clone()));
        true
    }
}
