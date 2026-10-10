//! Small application-owned consumers using only the public Jobs API.
//!
//! Byte replacement models an atomic commit for fault injection; a host fixture
//! reopens actual stable cells on VectorMemory. These native fixtures do not
//! model IC message rollback, installed upgrades or live timers.

use std::{error::Error, fmt, io};

use ic_jobs::{
    Attempt, ExecutionId, Job, JobError, JobId, JobRecord, JobState, MissedRunPolicy, Outcome,
    RetryPolicy, Schedule, Scheduler,
};
use ic_memory::{
    GenericAllocationPolicy, MemoryAllocationPool, MemoryAuthority, MemoryManagerConfig,
    MemoryRequest, MemoryRuntime, RuntimeOpenError, SchemaMetadata, SealedDeclarationSnapshot,
    StaticMemoryDeclarationError,
    ic_stable_structures::{Cell, VectorMemory},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

// Both fixture applications have tiny, bounded snapshots. Storage is not a Jobs
// trait, and a failed write keeps the previously committed bytes unchanged.
struct Store {
    bytes: Vec<u8>,
    writes_before_failure: Option<usize>,
}

impl Store {
    fn new(value: &impl Serialize) -> Result<Self> {
        let mut store = Self {
            bytes: Vec::new(),
            writes_before_failure: None,
        };
        store.write(value)?;
        Ok(store)
    }

    fn read<T: DeserializeOwned>(&self) -> Result<T> {
        if self.bytes.len() > 4096 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "snapshot too large").into());
        }
        Ok(serde_json::from_slice(&self.bytes)?)
    }

    fn write(&mut self, value: &impl Serialize) -> Result<()> {
        let bytes = serde_json::to_vec(value)?;
        if bytes.len() > 4096 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "snapshot too large").into());
        }
        if let Some(remaining) = self.writes_before_failure {
            if remaining == 0 {
                self.writes_before_failure = None;
                return Err(io::Error::other("injected commit failure").into());
            }
            self.writes_before_failure = Some(remaining - 1);
        }
        self.bytes = bytes;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Notification {
    recipient: String,
    body: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct NotificationSnapshot {
    job: JobRecord,
    payload: Notification,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Delivery {
    attempt: Attempt,
    payload: Notification,
}

// The synchronous scheduler produces committed intent for a separate delivery
// continuation. Neither the scheduler nor a watchdog callback calls a receiver.
struct Notifications {
    store: Store,
}

// The fixture receives a trusted caller from its application boundary. A real
// canister obtains that identity from the IC caller API, never request payloads.
const NOTIFICATION_MANAGER: &str = "notification-manager";

#[derive(Clone)]
enum ManagementCommand {
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
enum ManagementValue {
    Job(NotificationSnapshot),
    Page(Vec<NotificationSnapshot>),
    Dispatch(Option<Delivery>),
}

#[derive(Debug, Eq, PartialEq)]
struct ManagementReply {
    value: ManagementValue,
    // The application projects this committed queue deadline into its watchdog.
    // Native execution does not register or qualify a platform timer.
    next_due_ns: Option<u64>,
}

#[derive(Debug, Eq, PartialEq)]
enum ManagementError {
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
    // One consumer-owned store holds records and immutable payloads together.
    // The eventual host supplies allocation policy and owns one bootstrap.
    fn memory_request() -> Result<MemoryRequest> {
        Ok(MemoryRequest::new(
            "test.notifications",
            "test.notifications.queue.v1",
            SchemaMetadata::default(),
        )?)
    }

    fn new() -> Result<Self> {
        Ok(Self {
            store: Store::new(&vec![NotificationSnapshot {
                job: Job::new(
                    JobId(1),
                    Schedule::Once { at_ns: 100 },
                    RetryPolicy {
                        max_attempts: 2,
                        initial_delay_ns: 10,
                        max_delay_ns: 10,
                    },
                )?
                .record(),
                payload: Notification {
                    recipient: "alice".into(),
                    body: "Your report is ready".into(),
                },
            }])?,
        })
    }

    fn load(&self) -> Result<(Vec<Job>, Vec<NotificationSnapshot>)> {
        let snapshots: Vec<NotificationSnapshot> = self.store.read()?;
        if snapshots.len() > 4 {
            return Err(
                io::Error::new(io::ErrorKind::InvalidData, "too many notifications").into(),
            );
        }
        let mut jobs = Vec::with_capacity(snapshots.len());
        for snapshot in &snapshots {
            if jobs
                .iter()
                .any(|job: &Job| job.record().id == snapshot.job.id)
            {
                return Err(
                    io::Error::new(io::ErrorKind::InvalidData, "duplicate job identity").into(),
                );
            }
            jobs.push(Job::restore(snapshot.job)?);
        }
        Ok((jobs, snapshots))
    }

    fn enqueue(&mut self, id: JobId, at_ns: u64, payload: Notification) -> Result<()> {
        let (jobs, mut snapshots) = self.load()?;
        if snapshots.len() == 4 || jobs.iter().any(|job| job.record().id == id) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "queue full or identity reused",
            )
            .into());
        }
        snapshots.push(NotificationSnapshot {
            job: Job::new(id, Schedule::Once { at_ns }, RetryPolicy::NONE)?.record(),
            payload,
        });
        self.store.write(&snapshots)
    }

    fn inspect(&self, id: JobId) -> Result<NotificationSnapshot> {
        self.load()?
            .1
            .into_iter()
            .find(|snapshot| snapshot.job.id == id)
            .ok_or_else(|| ManagementError::UnknownJob.into())
    }

    fn manage(
        &mut self,
        caller: &str,
        now_ns: u64,
        command: ManagementCommand,
    ) -> Result<ManagementReply> {
        // Authorization precedes decoding, reads of private payloads and writes.
        if caller != NOTIFICATION_MANAGER {
            return Err(ManagementError::Unauthorized.into());
        }
        let value = match command {
            ManagementCommand::Enqueue { id, at_ns, payload } => {
                self.enqueue(id, at_ns, payload)?;
                ManagementValue::Job(self.inspect(id)?)
            }
            ManagementCommand::Inspect(id) => ManagementValue::Job(self.inspect(id)?),
            ManagementCommand::List { after, limit } => {
                if !(1..=4).contains(&limit) {
                    return Err(ManagementError::InvalidPageLimit.into());
                }
                let (_, mut snapshots) = self.load()?;
                snapshots.sort_by_key(|snapshot| snapshot.job.id);
                ManagementValue::Page(
                    snapshots
                        .into_iter()
                        .filter(|snapshot| after.is_none_or(|id| snapshot.job.id > id))
                        .take(limit)
                        .collect(),
                )
            }
            ManagementCommand::Cancel(id) => {
                self.inspect(id)?;
                self.cancel(id, now_ns)?;
                ManagementValue::Job(self.inspect(id)?)
            }
            ManagementCommand::DispatchDue => ManagementValue::Dispatch(self.prepare(now_ns)?),
        };
        Ok(ManagementReply {
            value,
            next_due_ns: self.next_due_ns()?,
        })
    }

    fn next_due_ns(&self) -> Result<Option<u64>> {
        let (mut jobs, _) = self.load()?;
        Ok(Scheduler::new(&mut jobs).next_due_ns())
    }

    fn prepare(&mut self, now_ns: u64) -> Result<Option<Delivery>> {
        let (mut jobs, mut snapshots) = self.load()?;
        let Some((attempt, running)) = Scheduler::new(&mut jobs).start_next(now_ns)? else {
            return Ok(None);
        };
        let snapshot = snapshots
            .iter_mut()
            .find(|s| s.job.id == running.id)
            .unwrap();
        snapshot.job = running;
        let payload = snapshot.payload.clone();
        self.store.write(&snapshots)?;
        // No delivery capability escapes before the intent and payload commit.
        Ok(Some(Delivery { attempt, payload }))
    }

    fn outstanding(&self) -> Result<Vec<Delivery>> {
        // Inspect unresolved envelopes for reconciliation, never for automatic replay.
        let (jobs, snapshots) = self.load()?;
        Ok(jobs
            .iter()
            .zip(snapshots)
            .filter_map(|(job, snapshot)| {
                job.active_attempt().map(|attempt| Delivery {
                    attempt,
                    payload: snapshot.payload,
                })
            })
            .collect())
    }

    fn finish(&mut self, attempt: Attempt, now_ns: u64, outcome: Outcome) -> Result<()> {
        let (mut jobs, mut snapshots) = self.load()?;
        let index = jobs
            .iter()
            .position(|job| job.record().id == attempt.execution.job)
            .ok_or(JobError::StaleAttempt)?;
        jobs[index].finish(attempt, now_ns, outcome)?;
        snapshots[index].job = jobs[index].record();
        self.store.write(&snapshots)
    }

    fn reconcile(
        &mut self,
        attempt: Attempt,
        destination: &Destination,
        now_ns: u64,
    ) -> Result<bool> {
        let (mut jobs, mut snapshots) = self.load()?;
        let index = jobs
            .iter()
            .position(|job| job.record().id == attempt.execution.job)
            .ok_or(JobError::StaleAttempt)?;
        let outcome = if destination
            .receipts
            .contains(&(attempt.execution, snapshots[index].payload.clone()))
        {
            Outcome::Success
        } else if destination
            .rejections
            .contains(&(attempt, snapshots[index].payload.clone()))
        {
            // Exact destination evidence confirms this attempt had no effect.
            Outcome::RetryableFailure
        } else {
            // A missing receipt is not proof that delivery failed.
            return Ok(false);
        };
        jobs[index].resolve(attempt, now_ns, outcome)?;
        snapshots[index].job = jobs[index].record();
        self.store.write(&snapshots)?;
        Ok(true)
    }

    fn cancel(&mut self, id: JobId, now_ns: u64) -> Result<()> {
        let (mut jobs, mut snapshots) = self.load()?;
        let index = jobs
            .iter()
            .position(|job| job.record().id == id)
            .ok_or(JobError::InvalidRecord)?;
        jobs[index].cancel(now_ns)?;
        snapshots[index].job = jobs[index].record();
        self.store.write(&snapshots)
    }
}

// A separate destination owns receipts/idempotency, outside the sender's
// reconstructed history. This fixture retains at most four logical deliveries.
#[derive(Default)]
struct Destination {
    receipts: Vec<(ExecutionId, Notification)>,
    rejections: Vec<(Attempt, Notification)>,
    reject_next: bool,
    calls: usize,
}

impl Destination {
    fn deliver(&mut self, delivery: &Delivery) -> bool {
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

#[derive(Deserialize, Serialize)]
struct MaintenanceSnapshot {
    job: JobRecord,
    // The application owns cache entries and its local effect counter.
    expires_ns: Vec<u64>,
    completed_runs: u64,
}

struct Maintenance {
    store: Store,
}

#[derive(Debug, Eq, PartialEq)]
struct Wake {
    completed: u64,
    next_due_ns: Option<u64>,
}

#[cfg(feature = "timers")]
impl Wake {
    fn timer_result(&self) -> ic_timers::WatchdogRunResult {
        let completion = if self.completed == 0 {
            ic_timers::TimerCompletion::no_work()
        } else {
            ic_timers::TimerCompletion::success(self.completed)
        };
        ic_jobs::timers::complete_batch(completion, self.next_due_ns)
    }
}

impl Maintenance {
    fn memory_request() -> Result<MemoryRequest> {
        Ok(MemoryRequest::new(
            "test.maintenance",
            "test.maintenance.state.v1",
            SchemaMetadata::default(),
        )?)
    }

    fn new(missed: MissedRunPolicy) -> Result<Self> {
        Ok(Self {
            store: Store::new(&MaintenanceSnapshot {
                job: Job::new(
                    JobId(2),
                    Schedule::FixedRate {
                        first_at_ns: 100,
                        every_ns: 10,
                        missed,
                    },
                    RetryPolicy::NONE,
                )?
                .record(),
                expires_ns: vec![90, 100, 125, 200],
                completed_runs: 0,
            })?,
        })
    }

    fn load(&self) -> Result<(Job, MaintenanceSnapshot)> {
        let snapshot: MaintenanceSnapshot = self.store.read()?;
        if snapshot.expires_ns.len() > 32 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "too many entries").into());
        }
        Ok((Job::restore(snapshot.job)?, snapshot))
    }

    fn wake(&mut self, now_ns: u64) -> Result<Wake> {
        let mut completed = 0;
        // Exactly two runs at most, even when CatchUp leaves overdue work.
        for _ in 0..2 {
            let (mut job, mut snapshot) = self.load()?;
            let Some((attempt, _)) =
                Scheduler::new(std::slice::from_mut(&mut job)).start_next(now_ns)?
            else {
                break;
            };
            snapshot.expires_ns.retain(|expires| *expires > now_ns);
            snapshot.completed_runs = snapshot
                .completed_runs
                .checked_add(1)
                .ok_or_else(|| io::Error::other("maintenance counter exhausted"))?;
            job.finish(attempt, now_ns, Outcome::Success)?;
            snapshot.job = job.record();
            // Local effects and metadata commit together. No external call or await.
            self.store.write(&snapshot)?;
            completed += 1;
        }
        Ok(Wake {
            completed,
            next_due_ns: self.load()?.0.next_due_ns(),
        })
    }

    fn cancel(&mut self, now_ns: u64) -> Result<()> {
        let (mut job, mut snapshot) = self.load()?;
        job.cancel(now_ns)?;
        snapshot.job = job.record();
        self.store.write(&snapshot)
    }
}

#[test]
fn consumer_memory_requests_compose_without_granting_open_authority() -> Result<()> {
    let notifications = Notifications::memory_request()?;
    let maintenance = Maintenance::memory_request()?;
    let composed = SealedDeclarationSnapshot::new(&[notifications.clone(), maintenance.clone()])?;
    let reordered = SealedDeclarationSnapshot::new(&[maintenance, notifications])?;
    assert_eq!(composed, reordered);
    assert_eq!(composed.requests().len(), 2);
    assert_ne!(
        composed.requests()[0].authority(),
        composed.requests()[1].authority(),
    );

    // Sealed requests establish checked names, not committed allocation access.
    // No range-based bootstrap or provisional allocator is introduced here.
    let runtime = MemoryRuntime::new(VectorMemory::default())?;
    for request in composed.requests() {
        assert!(matches!(
            runtime.open_memory(request.stable_key().as_str()),
            Err(RuntimeOpenError::NotBootstrapped)
        ));
    }
    Ok(())
}

#[test]
fn consumer_memory_collision_refuses_without_changing_retained_jobs() -> Result<()> {
    let notifications = Notifications::new()?;
    let maintenance = Maintenance::new(MissedRunPolicy::Skip)?;
    let notification_bytes = notifications.store.bytes.clone();
    let maintenance_bytes = maintenance.store.bytes.clone();
    let requested = Notifications::memory_request()?;
    let foreign = MemoryRequest::new(
        Maintenance::memory_request()?.authority(),
        requested.stable_key().as_str(),
        SchemaMetadata::default(),
    )?;
    assert!(matches!(
        SealedDeclarationSnapshot::new(&[requested.clone(), foreign]),
        Err(StaticMemoryDeclarationError::DuplicateRequest { stable_key })
            if stable_key == *requested.stable_key()
    ));
    assert_eq!(notifications.store.bytes, notification_bytes);
    assert_eq!(maintenance.store.bytes, maintenance_bytes);
    assert_eq!(notifications.load()?.0[0].next_due_ns(), Some(100));
    assert_eq!(maintenance.load()?.0.next_due_ns(), Some(100));
    Ok(())
}

#[test]
fn host_pool_cells_reopen_committed_jobs_without_replaying_blocked_effects() -> Result<()> {
    let requests = [
        Notifications::memory_request()?,
        Maintenance::memory_request()?,
    ];
    let declarations = SealedDeclarationSnapshot::new(&requests)?;
    let pool = MemoryAllocationPool::new(
        vec![
            MemoryAuthority::new("test.notifications", "test.notifications.")?,
            MemoryAuthority::new("test.maintenance", "test.maintenance.")?,
        ],
        vec![],
    )?;
    let config = MemoryManagerConfig::new(1)?;
    for uncertain in [false, true] {
        let backing = VectorMemory::default();
        let mut sender = Notifications::new()?;
        sender.enqueue(
            JobId(3),
            200,
            Notification {
                recipient: "bob".into(),
                body: "Later".into(),
            },
        )?;
        let delivery = sender.prepare(100)?.unwrap();
        if uncertain {
            sender.finish(delivery.attempt, 101, Outcome::Uncertain)?;
        }
        let mut maintenance = Maintenance::new(MissedRunPolicy::Skip)?;
        maintenance.wake(125)?;
        let ids;
        {
            let mut host = MemoryRuntime::new_with_config(backing.clone(), config)?;
            host.bootstrap(&declarations, &pool, &GenericAllocationPolicy)?;
            ids = [
                host.memory_id(requests[0].stable_key().as_str())?,
                host.memory_id(requests[1].stable_key().as_str())?,
            ];
            assert_ne!(ids[0], ids[1]);
            for (request, bytes) in requests
                .iter()
                .zip([&sender.store.bytes, &maintenance.store.bytes])
            {
                // The host opens only after commitment. Components verify their
                // requirements without bootstrapping or selecting physical IDs.
                host.verify_authority(
                    &SealedDeclarationSnapshot::new(std::slice::from_ref(request))?,
                    request.authority(),
                )?;
                Cell::init(
                    host.open_memory(request.stable_key().as_str())?,
                    bytes.clone(),
                );
            }
        }
        drop(sender);
        drop(maintenance);
        // A foreign claim cannot commit or open either store, nor change bytes.
        let before = backing.borrow().clone();
        let foreign = SealedDeclarationSnapshot::new(&[MemoryRequest::new(
            "test.maintenance",
            requests[0].stable_key().as_str(),
            SchemaMetadata::default(),
        )?])?;
        {
            let mut rejected = MemoryRuntime::new_with_config(backing.clone(), config)?;
            assert!(
                rejected
                    .bootstrap(&foreign, &pool, &GenericAllocationPolicy)
                    .is_err()
            );
            assert!(matches!(
                rejected.open_memory(requests[0].stable_key().as_str()),
                Err(RuntimeOpenError::NotBootstrapped)
            ));
        }
        assert_eq!(*backing.borrow(), before);
        // Discard runtime, cells and consumer state. Reopen retained cells from
        // the same backing, including the original payload and blocked attempt.
        for reopen in 0..2 {
            let mut host = MemoryRuntime::new_with_config(backing.clone(), config)?;
            host.bootstrap(&declarations, &pool, &GenericAllocationPolicy)?;
            assert_eq!(host.memory_id(requests[0].stable_key().as_str())?, ids[0]);
            assert_eq!(host.memory_id(requests[1].stable_key().as_str())?, ids[1]);
            let mut queue = Cell::init(
                host.open_memory(requests[0].stable_key().as_str())?,
                Vec::<u8>::new(),
            );
            let state = Cell::init(
                host.open_memory(requests[1].stable_key().as_str())?,
                Vec::<u8>::new(),
            );
            let mut restored = Notifications {
                store: Store {
                    bytes: queue.get().clone(),
                    writes_before_failure: None,
                },
            };
            let restored_maintenance = Maintenance {
                store: Store {
                    bytes: state.get().clone(),
                    writes_before_failure: None,
                },
            };
            assert_eq!(restored.outstanding()?, vec![delivery.clone()]);
            assert_eq!(restored.prepare(150)?, None);
            assert!(!restored.reconcile(delivery.attempt, &Destination::default(), 150)?);
            assert_eq!(restored_maintenance.load()?.0.next_due_ns(), Some(130));
            assert_eq!(restored_maintenance.load()?.1.completed_runs, 1);
            assert_eq!(
                restored.next_due_ns()?,
                if reopen == 0 { Some(200) } else { None }
            );
            if reopen == 0 {
                restored.manage(
                    NOTIFICATION_MANAGER,
                    150,
                    ManagementCommand::Cancel(JobId(3)),
                )?;
                queue.set(restored.store.bytes);
                // Unrelated consumer bytes remain intact after the queue commit.
                assert_eq!(state.get(), &restored_maintenance.store.bytes);
            }
        }
    }
    Ok(())
}

#[test]
fn notification_commit_failure_never_exposes_delivery() -> Result<()> {
    let mut sender = Notifications::new()?;
    let before = sender.store.bytes.clone();
    assert_eq!(sender.prepare(99)?, None);
    sender.store.writes_before_failure = Some(0);
    assert!(sender.prepare(100).is_err());
    assert_eq!(sender.store.bytes, before);
    assert!(sender.outstanding()?.is_empty());
    assert_eq!(sender.next_due_ns()?, Some(100));

    let delivery = sender.prepare(100)?.unwrap();
    assert_eq!(sender.outstanding()?, vec![delivery]);
    assert_eq!(sender.next_due_ns()?, None);
    Ok(())
}

#[test]
fn lost_reply_and_failed_result_commit_reconcile_without_redelivery() -> Result<()> {
    for failed_result_commit in [false, true] {
        let mut sender = Notifications::new()?;
        let mut destination = Destination::default();
        let delivery = sender.prepare(100)?.unwrap();
        assert!(destination.deliver(&delivery));
        if failed_result_commit {
            sender.store.writes_before_failure = Some(0);
            assert!(
                sender
                    .finish(delivery.attempt, 101, Outcome::Success)
                    .is_err()
            );
        }

        // Only committed bytes survive reconstruction; the destination is separate.
        let mut restored = Notifications {
            store: Store {
                bytes: sender.store.bytes,
                writes_before_failure: None,
            },
        };
        assert_eq!(restored.outstanding()?, vec![delivery.clone()]);
        assert_eq!(restored.prepare(200)?, None);
        restored.finish(delivery.attempt, 200, Outcome::Uncertain)?;
        assert_eq!(restored.prepare(201)?, None);
        assert!(!restored.reconcile(delivery.attempt, &Destination::default(), 201)?);
        assert_eq!(
            restored.load()?.0[0].record().state,
            JobState::Uncertain { started_ns: 100 }
        );
        let unresolved = restored.store.bytes.clone();
        restored.store.writes_before_failure = Some(0);
        assert!(
            restored
                .reconcile(delivery.attempt, &destination, 202)
                .is_err()
        );
        assert_eq!(restored.store.bytes, unresolved);
        assert_eq!(restored.prepare(202)?, None);
        assert!(restored.reconcile(delivery.attempt, &destination, 202)?);
        assert_eq!(restored.load()?.0[0].record().state, JobState::Completed);
        assert_eq!(restored.prepare(300)?, None);
        assert_eq!(destination.calls, 1);
        assert_eq!(destination.receipts.len(), 1);
    }
    Ok(())
}

#[test]
fn known_rejection_retries_same_payload_and_rejects_old_reply() -> Result<()> {
    let mut sender = Notifications::new()?;
    let mut destination = Destination {
        reject_next: true,
        ..Destination::default()
    };
    let first = sender.prepare(100)?.unwrap();
    assert!(!destination.deliver(&first));
    assert!(destination.receipts.is_empty());
    sender.finish(first.attempt, 101, Outcome::RetryableFailure)?;
    assert_eq!(sender.next_due_ns()?, Some(111));
    assert_eq!(sender.prepare(110)?, None);
    let second = sender.prepare(111)?.unwrap();
    assert_eq!(first.attempt.execution, second.attempt.execution);
    assert_ne!(first.attempt.sequence, second.attempt.sequence);
    assert_eq!(first.payload, second.payload);

    let before = sender.store.bytes.clone();
    let stale = sender
        .finish(first.attempt, 112, Outcome::Success)
        .unwrap_err();
    assert_eq!(
        stale.downcast_ref::<JobError>(),
        Some(&JobError::StaleAttempt)
    );
    assert_eq!(sender.store.bytes, before);
    assert!(destination.deliver(&second));
    destination.reject_next = true;
    assert!(destination.deliver(&second)); // Destination-side duplicate suppression.
    sender.finish(second.attempt, 112, Outcome::Success)?;
    assert_eq!(destination.receipts.len(), 1);
    assert_eq!(sender.next_due_ns()?, None);
    Ok(())
}

#[test]
fn maintenance_reconstruction_bounds_catch_up_and_rebuilds_deadlines() -> Result<()> {
    let maintenance = Maintenance::new(MissedRunPolicy::CatchUp)?;
    let mut restored = Maintenance {
        store: Store {
            bytes: maintenance.store.bytes,
            writes_before_failure: None,
        },
    };
    let early = restored.wake(99)?;
    assert_eq!(
        early,
        Wake {
            completed: 0,
            next_due_ns: Some(100)
        }
    );
    let first = restored.wake(135)?;
    assert_eq!(
        first,
        Wake {
            completed: 2,
            next_due_ns: Some(120)
        }
    );
    let (_, snapshot) = restored.load()?;
    assert_eq!(snapshot.completed_runs, 2);
    assert_eq!(snapshot.expires_ns, vec![200]);
    let second = restored.wake(135)?;
    assert_eq!(
        second,
        Wake {
            completed: 2,
            next_due_ns: Some(140)
        }
    );
    assert_eq!(restored.load()?.1.completed_runs, 4);

    #[cfg(feature = "timers")]
    {
        use ic_timers::WatchdogDecision;
        assert_eq!(
            early.timer_result().decision(),
            WatchdogDecision::ScheduleAt(100)
        );
        assert_eq!(
            first.timer_result().decision(),
            WatchdogDecision::ScheduleAt(120)
        );
        assert_eq!(
            second.timer_result().decision(),
            WatchdogDecision::ScheduleAt(140)
        );
    }
    restored.cancel(136)?;
    let stopped = restored.wake(200)?;
    assert_eq!(
        stopped,
        Wake {
            completed: 0,
            next_due_ns: None
        }
    );
    #[cfg(feature = "timers")]
    assert_eq!(
        stopped.timer_result().decision(),
        ic_timers::WatchdogDecision::Stop
    );
    Ok(())
}

#[test]
fn local_maintenance_effect_rolls_back_with_failed_commit_and_skip_avoids_backlog() -> Result<()> {
    let mut maintenance = Maintenance::new(MissedRunPolicy::Skip)?;
    let before = maintenance.store.bytes.clone();
    maintenance.store.writes_before_failure = Some(0);
    assert!(maintenance.wake(135).is_err());
    assert_eq!(maintenance.store.bytes, before);
    assert_eq!(maintenance.load()?.1.completed_runs, 0);
    assert_eq!(maintenance.load()?.1.expires_ns, vec![90, 100, 125, 200]);
    assert_eq!(
        maintenance.wake(135)?,
        Wake {
            completed: 1,
            next_due_ns: Some(140)
        }
    );
    assert_eq!(maintenance.load()?.1.expires_ns, vec![200]);
    Ok(())
}

#[test]
fn consumers_reject_corrupt_records_before_effects() -> Result<()> {
    let mut sender = Notifications::new()?;
    let mut notifications: Vec<NotificationSnapshot> = sender.store.read()?;
    notifications[0].job.state = JobState::Completed;
    sender.store.write(&notifications)?;
    let before = sender.store.bytes.clone();
    let error = sender.prepare(100).unwrap_err();
    assert_eq!(
        error.downcast_ref::<JobError>(),
        Some(&JobError::InvalidRecord)
    );
    assert_eq!(sender.store.bytes, before);

    let mut maintenance = Maintenance::new(MissedRunPolicy::Skip)?;
    let mut snapshot: MaintenanceSnapshot = maintenance.store.read()?;
    snapshot.job.state = JobState::Running { started_ns: 100 };
    maintenance.store.write(&snapshot)?;
    let before = maintenance.store.bytes.clone();
    let error = maintenance.wake(100).unwrap_err();
    assert_eq!(
        error.downcast_ref::<JobError>(),
        Some(&JobError::InvalidRecord)
    );
    assert_eq!(maintenance.store.bytes, before);
    Ok(())
}

#[test]
fn shared_notification_wakeup_preserves_order_and_progress_past_unresolved_work() -> Result<()> {
    let mut sender = Notifications::new()?;
    for (id, at_ns) in [(2, 90), (3, 95), (4, 90)] {
        sender.enqueue(
            JobId(id),
            at_ns,
            Notification {
                recipient: format!("recipient-{id}"),
                body: format!("report-{id}"),
            },
        )?;
    }
    assert_eq!(sender.next_due_ns()?, Some(90));
    assert_eq!(sender.prepare(89)?, None);
    let first = sender.prepare(90)?.unwrap();
    assert_eq!(first.attempt.execution.job, JobId(2));

    // Reconstruct the whole bounded queue with job 2's intent still unresolved.
    let mut restored = Notifications {
        store: Store {
            bytes: sender.store.bytes,
            writes_before_failure: None,
        },
    };
    let mut destination = Destination::default();
    let tied = restored.prepare(90)?.unwrap();
    assert_eq!(tied.attempt.execution.job, JobId(4));
    restored.finish(first.attempt, 91, Outcome::Uncertain)?;
    assert!(destination.deliver(&tied));
    restored.finish(tied.attempt, 91, Outcome::Success)?;
    assert_eq!(restored.next_due_ns()?, Some(95));

    let before = restored.store.bytes.clone();
    assert!(restored.cancel(JobId(2), 91).is_err());
    assert_eq!(restored.store.bytes, before);
    restored.cancel(JobId(3), 91)?;
    assert_eq!(restored.next_due_ns()?, Some(100));
    assert_eq!(restored.prepare(99)?, None);
    let last = restored.prepare(100)?.unwrap();
    assert_eq!(last.attempt.execution.job, JobId(1));
    assert_eq!(last.payload.recipient, "alice");
    assert!(destination.deliver(&last));
    restored.finish(last.attempt, 101, Outcome::Success)?;
    assert_eq!(restored.next_due_ns()?, None);
    assert_eq!(restored.prepare(1000)?, None);
    assert_eq!(restored.outstanding()?, vec![first.clone()]);
    assert!(!restored.reconcile(first.attempt, &destination, 1000)?);
    assert_eq!(destination.calls, 2);
    assert_eq!(destination.receipts.len(), 2);
    #[cfg(feature = "timers")]
    assert_eq!(
        ic_jobs::timers::complete_batch(
            ic_timers::TimerCompletion::no_work(),
            restored.next_due_ns()?
        )
        .decision(),
        ic_timers::WatchdogDecision::Stop
    );
    Ok(())
}

#[test]
fn interruption_before_delivery_keeps_committed_intent_blocked() -> Result<()> {
    let mut sender = Notifications::new()?;
    let delivery = sender.prepare(100)?.unwrap();
    let mut restored = Notifications {
        store: Store {
            bytes: sender.store.bytes,
            writes_before_failure: None,
        },
    };
    let destination = Destination::default();
    assert_eq!(restored.prepare(200)?, None);
    assert_eq!(restored.outstanding()?, vec![delivery.clone()]);
    restored.finish(delivery.attempt, 200, Outcome::Uncertain)?;
    assert!(!restored.reconcile(delivery.attempt, &destination, 201)?);
    assert_eq!(restored.prepare(1000)?, None);
    assert_eq!(destination.calls, 0);
    assert_eq!(
        restored.load()?.0[0].record().state,
        JobState::Uncertain { started_ns: 100 }
    );
    Ok(())
}

#[test]
fn notification_queue_rejects_reused_identities_and_excess_work() -> Result<()> {
    let mut sender = Notifications::new()?;
    let payload = sender.load()?.1[0].payload.clone();
    let before = sender.store.bytes.clone();
    assert!(sender.enqueue(JobId(1), 200, payload.clone()).is_err());
    assert_eq!(sender.store.bytes, before);
    for id in 2..=4 {
        sender.enqueue(JobId(id), 200, payload.clone())?;
    }
    let before = sender.store.bytes.clone();
    assert!(sender.enqueue(JobId(5), 200, payload.clone()).is_err());
    assert_eq!(sender.store.bytes, before);

    let mut snapshots: Vec<NotificationSnapshot> = sender.store.read()?;
    snapshots.push(NotificationSnapshot {
        job: Job::new(JobId(5), Schedule::Once { at_ns: 200 }, RetryPolicy::NONE)?.record(),
        payload,
    });
    sender.store.write(&snapshots)?;
    let before = sender.store.bytes.clone();
    assert!(sender.prepare(100).is_err());
    assert_eq!(sender.store.bytes, before);
    snapshots.pop();

    // Reject duplicate retained identities independently at the restore boundary.
    snapshots[3].job = snapshots[0].job;
    sender.store.write(&snapshots)?;
    let before = sender.store.bytes.clone();
    assert!(sender.prepare(100).is_err());
    assert_eq!(sender.store.bytes, before);

    // A corrupt future record must also block dispatch of the valid due record.
    snapshots[3].job =
        Job::new(JobId(4), Schedule::Once { at_ns: 200 }, RetryPolicy::NONE)?.record();
    snapshots[3].job.state = JobState::Completed;
    sender.store.write(&snapshots)?;
    let before = sender.store.bytes.clone();
    let error = sender.prepare(100).unwrap_err();
    assert_eq!(
        error.downcast_ref::<JobError>(),
        Some(&JobError::InvalidRecord)
    );
    assert_eq!(sender.store.bytes, before);
    Ok(())
}

#[test]
fn lost_rejection_reply_uses_exact_evidence_and_retains_retry_budget() -> Result<()> {
    for second_succeeds in [false, true] {
        let mut sender = Notifications::new()?;
        let mut destination = Destination {
            reject_next: true,
            ..Destination::default()
        };
        let first = sender.prepare(100)?.unwrap();
        assert!(!destination.deliver(&first));
        // The receiver retains its rejection even if an old delivery is repeated.
        assert!(!destination.deliver(&first));
        let mut restored = Notifications {
            store: Store {
                bytes: sender.store.bytes,
                writes_before_failure: None,
            },
        };
        assert_eq!(restored.prepare(101)?, None);
        restored.finish(first.attempt, 101, Outcome::Uncertain)?;

        let before = restored.store.bytes.clone();
        assert!(!restored.reconcile(first.attempt, &Destination::default(), 102)?);
        assert_eq!(restored.store.bytes, before);
        restored.store.writes_before_failure = Some(0);
        assert!(
            restored
                .reconcile(first.attempt, &destination, 102)
                .is_err()
        );
        assert_eq!(restored.store.bytes, before);
        assert_eq!(restored.prepare(102)?, None);
        assert!(restored.reconcile(first.attempt, &destination, 102)?);
        assert_eq!(restored.next_due_ns()?, Some(112));
        assert_eq!(restored.prepare(111)?, None);
        let second = restored.prepare(112)?.unwrap();
        assert_eq!(second.attempt.execution, first.attempt.execution);
        assert_ne!(second.attempt.sequence, first.attempt.sequence);
        assert_eq!(second.payload, first.payload);
        destination.reject_next = !second_succeeds;
        assert_eq!(destination.deliver(&second), second_succeeds);
        restored.finish(second.attempt, 113, Outcome::Uncertain)?;

        let before = restored.store.bytes.clone();
        let error = restored
            .reconcile(first.attempt, &destination, 114)
            .unwrap_err();
        assert_eq!(
            error.downcast_ref::<JobError>(),
            Some(&JobError::StaleAttempt)
        );
        assert_eq!(restored.store.bytes, before);
        assert!(restored.reconcile(second.attempt, &destination, 114)?);
        assert_eq!(
            restored.load()?.0[0].record().state,
            if second_succeeds {
                JobState::Completed
            } else {
                JobState::Failed
            }
        );
        assert_eq!(restored.next_due_ns()?, None);
        assert_eq!(restored.prepare(1000)?, None);
        assert!(restored.outstanding()?.is_empty());
        assert_eq!(destination.receipts.len(), usize::from(second_succeeds));
    }
    Ok(())
}

#[test]
fn maintenance_resumes_after_partial_batch_commit_without_repeating_work() -> Result<()> {
    let mut maintenance = Maintenance::new(MissedRunPolicy::CatchUp)?;
    maintenance.store.writes_before_failure = Some(1);
    let error = maintenance.wake(135).unwrap_err();
    assert_eq!(
        error.downcast_ref::<io::Error>().unwrap().kind(),
        io::ErrorKind::Other
    );
    let (job, snapshot) = maintenance.load()?;
    assert_eq!(snapshot.completed_runs, 1);
    assert_eq!(snapshot.expires_ns, vec![200]);
    assert_eq!(job.record().occurrence, 1);
    assert_eq!(job.record().sequence, 1);
    assert_eq!(job.next_due_ns(), Some(110));

    let mut restored = Maintenance {
        store: Store {
            bytes: maintenance.store.bytes,
            writes_before_failure: None,
        },
    };
    assert_eq!(
        restored.wake(135)?,
        Wake {
            completed: 2,
            next_due_ns: Some(130)
        }
    );
    let (job, snapshot) = restored.load()?;
    assert_eq!(snapshot.completed_runs, 3);
    assert_eq!(job.record().occurrence, 3);
    assert_eq!(job.record().sequence, 3);
    assert_eq!(
        restored.wake(135)?,
        Wake {
            completed: 1,
            next_due_ns: Some(140)
        }
    );
    assert_eq!(restored.load()?.1.completed_runs, 4);
    assert_eq!(restored.load()?.1.expires_ns, vec![200]);
    Ok(())
}

#[test]
fn management_creation_inspection_and_pages_survive_reconstruction() -> Result<()> {
    let mut sender = Notifications::new()?;
    for (id, at_ns, body, next_due_ns) in [(8, 80, "eight", 80), (3, 200, "three", 80)] {
        let reply = sender.manage(
            NOTIFICATION_MANAGER,
            0,
            ManagementCommand::Enqueue {
                id: JobId(id),
                at_ns,
                payload: Notification {
                    recipient: "bob".into(),
                    body: body.into(),
                },
            },
        )?;
        assert_eq!(reply.next_due_ns, Some(next_due_ns));
        let ManagementValue::Job(snapshot) = reply.value else {
            panic!("creation must return the committed notification");
        };
        assert_eq!(snapshot.job.id, JobId(id));
        assert_eq!(snapshot.job.state, JobState::Pending { due_ns: at_ns });
        assert_eq!(snapshot.payload.body, body);
    }
    let mut restored = Notifications {
        store: Store {
            bytes: sender.store.bytes,
            writes_before_failure: None,
        },
    };
    let before = restored.store.bytes.clone();
    let reply = restored.manage(
        NOTIFICATION_MANAGER,
        0,
        ManagementCommand::Inspect(JobId(8)),
    )?;
    let ManagementValue::Job(snapshot) = reply.value else {
        panic!("inspection must return the retained notification");
    };
    assert_eq!(snapshot.payload.body, "eight");
    assert_eq!(reply.next_due_ns, Some(80));

    // Cursor order follows stable job identity, independently of insertion order.
    let mut after = None;
    for expected in [1, 3, 8] {
        let reply = restored.manage(
            NOTIFICATION_MANAGER,
            0,
            ManagementCommand::List { after, limit: 1 },
        )?;
        assert_eq!(reply.next_due_ns, Some(80));
        let ManagementValue::Page(page) = reply.value else {
            panic!("listing must return a bounded page");
        };
        assert_eq!(page.len(), 1);
        assert_eq!(page[0].job.id, JobId(expected));
        after = Some(page[0].job.id);
    }
    let reply = restored.manage(
        NOTIFICATION_MANAGER,
        0,
        ManagementCommand::List { after, limit: 1 },
    )?;
    assert_eq!(reply.value, ManagementValue::Page(vec![]));
    assert_eq!(restored.store.bytes, before);
    Ok(())
}

#[test]
fn management_refuses_unauthorized_calls_before_decoding_or_writing() -> Result<()> {
    let commands = [
        ManagementCommand::Enqueue {
            id: JobId(2),
            at_ns: 80,
            payload: Notification {
                recipient: "bob".into(),
                body: "private".into(),
            },
        },
        ManagementCommand::Inspect(JobId(1)),
        ManagementCommand::List {
            after: None,
            limit: 4,
        },
        ManagementCommand::Cancel(JobId(1)),
        ManagementCommand::DispatchDue,
    ];
    for corrupt in [false, true] {
        let mut sender = Notifications::new()?;
        if corrupt {
            sender.store.bytes = b"not a snapshot".to_vec();
        }
        let before = sender.store.bytes.clone();
        sender.store.writes_before_failure = Some(0);
        for command in &commands {
            let error = sender
                .manage("other-caller", 100, command.clone())
                .unwrap_err();
            assert_eq!(
                error.downcast_ref::<ManagementError>(),
                Some(&ManagementError::Unauthorized)
            );
            assert_eq!(sender.store.bytes, before);
            assert_eq!(sender.store.writes_before_failure, Some(0));
        }
    }
    Ok(())
}

#[test]
fn management_commit_failures_preserve_queue_and_derived_wakeup() -> Result<()> {
    for (command, expected_deadline) in [
        (
            ManagementCommand::Enqueue {
                id: JobId(2),
                at_ns: 80,
                payload: Notification {
                    recipient: "bob".into(),
                    body: "committed before dispatch".into(),
                },
            },
            Some(80),
        ),
        (ManagementCommand::Cancel(JobId(1)), None),
        (ManagementCommand::DispatchDue, None),
    ] {
        let mut sender = Notifications::new()?;
        let before = sender.store.bytes.clone();
        sender.store.writes_before_failure = Some(0);
        let error = sender
            .manage(NOTIFICATION_MANAGER, 100, command.clone())
            .unwrap_err();
        assert_eq!(
            error.downcast_ref::<io::Error>().unwrap().kind(),
            io::ErrorKind::Other
        );
        assert_eq!(sender.store.bytes, before);
        assert!(sender.outstanding()?.is_empty());
        assert_eq!(sender.next_due_ns()?, Some(100));

        let reply = sender.manage(NOTIFICATION_MANAGER, 100, command)?;
        assert_eq!(reply.next_due_ns, expected_deadline);
        #[cfg(feature = "timers")]
        {
            let result = ic_jobs::timers::complete_batch(
                ic_timers::TimerCompletion::no_work(),
                reply.next_due_ns,
            );
            assert_eq!(
                result.decision(),
                expected_deadline.map_or(
                    ic_timers::WatchdogDecision::Stop,
                    ic_timers::WatchdogDecision::ScheduleAt,
                ),
            );
        }
    }
    Ok(())
}

#[test]
fn management_preserves_due_admission_and_blocks_unresolved_effects() -> Result<()> {
    let mut sender = Notifications::new()?;
    let before = sender.store.bytes.clone();
    let early = sender.manage(NOTIFICATION_MANAGER, 99, ManagementCommand::DispatchDue)?;
    assert_eq!(early.value, ManagementValue::Dispatch(None));
    assert_eq!(early.next_due_ns, Some(100));
    assert_eq!(sender.store.bytes, before);

    let reply = sender.manage(NOTIFICATION_MANAGER, 100, ManagementCommand::DispatchDue)?;
    let ManagementValue::Dispatch(Some(delivery)) = reply.value else {
        panic!("a due dispatch must return its committed intent");
    };
    assert_eq!(reply.next_due_ns, None);
    assert_eq!(sender.outstanding()?, vec![delivery.clone()]);
    let before = sender.store.bytes.clone();
    let error = sender
        .manage(
            NOTIFICATION_MANAGER,
            101,
            ManagementCommand::Cancel(JobId(1)),
        )
        .unwrap_err();
    assert_eq!(error.downcast_ref::<JobError>(), Some(&JobError::InFlight));
    assert_eq!(sender.store.bytes, before);

    let mut restored = Notifications {
        store: Store {
            bytes: sender.store.bytes,
            writes_before_failure: None,
        },
    };
    assert_eq!(
        restored
            .manage(NOTIFICATION_MANAGER, 200, ManagementCommand::DispatchDue)?
            .value,
        ManagementValue::Dispatch(None)
    );
    restored.finish(delivery.attempt, 200, Outcome::Uncertain)?;
    let before = restored.store.bytes.clone();
    let error = restored
        .manage(
            NOTIFICATION_MANAGER,
            201,
            ManagementCommand::Cancel(JobId(1)),
        )
        .unwrap_err();
    assert_eq!(error.downcast_ref::<JobError>(), Some(&JobError::InFlight));
    assert_eq!(restored.store.bytes, before);
    assert!(!restored.reconcile(delivery.attempt, &Destination::default(), 202)?);
    assert_eq!(
        restored
            .manage(NOTIFICATION_MANAGER, 203, ManagementCommand::DispatchDue)?
            .value,
        ManagementValue::Dispatch(None)
    );

    let mut destination = Destination::default();
    assert!(destination.deliver(&delivery));
    assert!(restored.reconcile(delivery.attempt, &destination, 204)?);
    let inspected = restored.manage(
        NOTIFICATION_MANAGER,
        204,
        ManagementCommand::Inspect(JobId(1)),
    )?;
    let ManagementValue::Job(snapshot) = inspected.value else {
        panic!("inspection must return the resolved job");
    };
    assert_eq!(snapshot.job.state, JobState::Completed);
    assert_eq!(inspected.next_due_ns, None);
    assert_eq!(destination.calls, 1);
    Ok(())
}

#[test]
fn management_refused_inputs_preserve_committed_jobs_and_deadlines() -> Result<()> {
    let mut sender = Notifications::new()?;
    let before = sender.store.bytes.clone();
    for (command, expected) in [
        (
            ManagementCommand::Inspect(JobId(99)),
            ManagementError::UnknownJob,
        ),
        (
            ManagementCommand::Cancel(JobId(99)),
            ManagementError::UnknownJob,
        ),
        (
            ManagementCommand::List {
                after: None,
                limit: 0,
            },
            ManagementError::InvalidPageLimit,
        ),
        (
            ManagementCommand::List {
                after: None,
                limit: 5,
            },
            ManagementError::InvalidPageLimit,
        ),
    ] {
        let error = sender.manage(NOTIFICATION_MANAGER, 0, command).unwrap_err();
        assert_eq!(error.downcast_ref::<ManagementError>(), Some(&expected));
        assert_eq!(sender.store.bytes, before);
        assert_eq!(sender.next_due_ns()?, Some(100));
    }
    for (id, body) in [(1, "duplicate".into()), (2, "x".repeat(4097))] {
        assert!(
            sender
                .manage(
                    NOTIFICATION_MANAGER,
                    0,
                    ManagementCommand::Enqueue {
                        id: JobId(id),
                        at_ns: 80,
                        payload: Notification {
                            recipient: "bob".into(),
                            body
                        },
                    }
                )
                .is_err()
        );
        assert_eq!(sender.store.bytes, before);
        assert_eq!(sender.next_due_ns()?, Some(100));
    }
    let cancelled = sender.manage(
        NOTIFICATION_MANAGER,
        90,
        ManagementCommand::Cancel(JobId(1)),
    )?;
    let ManagementValue::Job(snapshot) = cancelled.value else {
        panic!("cancellation must return the committed record");
    };
    assert_eq!(snapshot.job.state, JobState::Cancelled);
    assert_eq!(cancelled.next_due_ns, None);
    let before = sender.store.bytes.clone();
    let error = sender
        .manage(
            NOTIFICATION_MANAGER,
            100,
            ManagementCommand::Cancel(JobId(1)),
        )
        .unwrap_err();
    assert_eq!(
        error.downcast_ref::<JobError>(),
        Some(&JobError::NotPending)
    );
    assert_eq!(sender.store.bytes, before);
    assert_eq!(
        sender
            .manage(NOTIFICATION_MANAGER, 100, ManagementCommand::DispatchDue)?
            .value,
        ManagementValue::Dispatch(None)
    );
    Ok(())
}

#[test]
fn management_validates_future_records_before_reads_or_mutation() -> Result<()> {
    let mut sender = Notifications::new()?;
    sender.enqueue(
        JobId(2),
        200,
        Notification {
            recipient: "bob".into(),
            body: "future".into(),
        },
    )?;
    let mut snapshots: Vec<NotificationSnapshot> = sender.store.read()?;
    snapshots[1].job.state = JobState::Completed;
    sender.store.write(&snapshots)?;
    let before = sender.store.bytes.clone();
    for command in [
        ManagementCommand::Enqueue {
            id: JobId(3),
            at_ns: 80,
            payload: Notification {
                recipient: "bob".into(),
                body: "new".into(),
            },
        },
        ManagementCommand::Inspect(JobId(1)),
        ManagementCommand::List {
            after: None,
            limit: 1,
        },
        ManagementCommand::Cancel(JobId(1)),
        ManagementCommand::DispatchDue,
    ] {
        let error = sender
            .manage(NOTIFICATION_MANAGER, 100, command)
            .unwrap_err();
        assert_eq!(
            error.downcast_ref::<JobError>(),
            Some(&JobError::InvalidRecord)
        );
        assert_eq!(sender.store.bytes, before);
    }
    Ok(())
}
