use super::*;
use ic_memory::ic_stable_structures::VectorMemory;

fn manager() -> Principal {
    Principal::self_authenticating(b"manager")
}
fn config() -> Init {
    Init {
        manager: manager(),
        delivery_sender: None,
    }
}
fn increment(at: u64) -> Create {
    Create {
        timing: Timing::Once(at),
        task: Task::Increment(1),
    }
}

#[test]
fn paused_rescheduled_work_retains_identity_payload_and_deadline_across_reopen() {
    let memory = VectorMemory::default();
    let mut queue = Queue::open(memory.clone(), Some(config())).unwrap();
    let id = queue
        .create(Create {
            timing: Timing::FixedRate {
                first_ns: 100,
                every_ns: 10,
                catch_up: true,
            },
            task: Task::Increment(2),
        })
        .unwrap();
    assert_eq!(queue.dispatch(100).unwrap().1, 1);
    queue.pause(id, 101).unwrap();
    assert_eq!(queue.summary().unwrap(), (None, 2, 0));
    drop(queue);
    let mut queue = Queue::open(memory.clone(), None).unwrap();
    assert_eq!(queue.inspect(id).unwrap().status, Status::Paused);
    assert_eq!(queue.dispatch(400).unwrap(), (vec![], 0));
    queue
        .reschedule(
            id,
            Timing::AfterCompletion {
                first_ns: 500,
                every_ns: 10,
            },
            400,
        )
        .unwrap();
    let before = queue.inspect(id).unwrap();
    assert_eq!((before.sequence, before.occurrence), (1, 1));
    assert_eq!(before.task, Task::Increment(2));
    assert_eq!(before.status, Status::Paused);
    drop(queue);
    let mut queue = Queue::open(memory, None).unwrap();
    assert_eq!(queue.inspect(id).unwrap(), before);
    queue.resume(id, 450).unwrap();
    assert_eq!(queue.summary().unwrap().0, Some(500));
    assert_eq!(queue.dispatch(499).unwrap().1, 0);
    assert_eq!(queue.dispatch(500).unwrap().1, 1);
    assert_eq!(queue.summary().unwrap(), (Some(510), 4, 0));
    assert_eq!(
        (
            queue.inspect(id).unwrap().sequence,
            queue.inspect(id).unwrap().occurrence
        ),
        (2, 2)
    );
    queue.pause(id, 500).unwrap();
    queue.cancel(id, 501).unwrap();
    assert_eq!(queue.inspect(id).unwrap().status, Status::Cancelled);
    assert_eq!(queue.summary().unwrap().0, None);
}

#[test]
fn resume_preserves_overdue_policy_and_bounded_batch() {
    for catch_up in [false, true] {
        let memory = VectorMemory::default();
        let mut queue = Queue::open(memory.clone(), Some(config())).unwrap();
        let id = queue
            .create(Create {
                timing: Timing::FixedRate {
                    first_ns: 100,
                    every_ns: 10,
                    catch_up,
                },
                task: Task::Increment(1),
            })
            .unwrap();
        queue.pause(id, 50).unwrap();
        drop(queue);
        let mut queue = Queue::open(memory, None).unwrap();
        queue.resume(id, 200).unwrap();
        assert_eq!(queue.summary().unwrap().0, Some(100));
        assert_eq!(queue.dispatch(200).unwrap().1, if catch_up { 2 } else { 1 });
        assert_eq!(
            queue.summary().unwrap().0,
            Some(if catch_up { 120 } else { 210 })
        );
    }
}

#[test]
fn refused_management_preserves_stable_bytes_and_blocked_delivery() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config())).unwrap();
    let id = queue
        .create(Create {
            timing: Timing::Once(0),
            task: Task::Deliver {
                destination: Principal::self_authenticating(b"receiver"),
                message: "retained".into(),
            },
        })
        .unwrap();
    let (deliveries, _) = queue.dispatch(0).unwrap();
    for uncertain in [false, true] {
        if uncertain {
            queue.finish(&deliveries[0], 1, Outcome::Uncertain).unwrap();
        }
        let before = queue.cell.get().clone();
        assert!(queue.pause(id, 2).is_err());
        assert!(queue.resume(id, 2).is_err());
        assert!(queue.reschedule(id, Timing::Once(100), 2).is_err());
        assert_eq!(*queue.cell.get(), before);
        assert_eq!(queue.dispatch(2).unwrap().1, 0);
        assert_eq!(*queue.cell.get(), before);
    }
    let pending = queue.create(increment(100)).unwrap();
    let before = queue.cell.get().clone();
    assert_eq!(queue.pause(999, 2), Err(Error::UnknownJob));
    assert!(queue.resume(pending, 2).is_err());
    assert!(
        queue
            .reschedule(
                pending,
                Timing::FixedRate {
                    first_ns: 200,
                    every_ns: 0,
                    catch_up: true
                },
                2
            )
            .is_err()
    );
    assert_eq!(*queue.cell.get(), before);
}

#[test]
fn retained_queue_restores_payloads_deadlines_cancellation_and_id_authority() {
    let memory = VectorMemory::default();
    let mut queue = Queue::open(memory.clone(), Some(config())).unwrap();
    queue.authorize(manager()).unwrap();
    assert_eq!(
        queue.authorize(Principal::anonymous()),
        Err(Error::Unauthorized)
    );
    let first = queue.create(increment(100)).unwrap();
    let second = queue.create(increment(200)).unwrap();
    queue.cancel(first, 10).unwrap();
    let expected = queue.page(None, 8).unwrap();
    drop(queue);
    let mut restored = Queue::open(memory.clone(), None).unwrap();
    assert_eq!(restored.page(None, 8).unwrap(), expected);
    assert_eq!(restored.summary().unwrap(), (Some(200), 0, 0));
    assert_eq!(restored.page(Some(first), 1).unwrap()[0].id, second);
    assert_eq!(restored.page(None, 0), Err(Error::InvalidInput));
    assert_eq!(restored.create(increment(300)).unwrap(), 3);
    assert!(Queue::open(memory, Some(config())).is_err());
}

#[test]
fn local_effects_and_bounded_overdue_work_commit_together() {
    let memory = VectorMemory::default();
    let mut queue = Queue::open(memory.clone(), Some(config())).unwrap();
    let id = queue
        .create(Create {
            timing: Timing::FixedRate {
                first_ns: 100,
                every_ns: 10,
                catch_up: true,
            },
            task: Task::Increment(2),
        })
        .unwrap();
    assert_eq!(queue.dispatch(125).unwrap(), (vec![], 2));
    assert_eq!(queue.summary().unwrap(), (Some(120), 4, 0));
    drop(queue);
    let mut queue = Queue::open(memory, None).unwrap();
    assert_eq!(queue.inspect(id).unwrap().occurrence, 2);
    queue.dispatch(125).unwrap();
    assert_eq!(queue.summary().unwrap(), (Some(130), 6, 0));
    queue.cancel(id, 125).unwrap();
    assert_eq!(queue.summary().unwrap().0, None);
}

#[test]
fn skip_and_refused_mutations_preserve_committed_bytes() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config())).unwrap();
    let id = queue
        .create(Create {
            timing: Timing::FixedRate {
                first_ns: 100,
                every_ns: 10,
                catch_up: false,
            },
            task: Task::Increment(1),
        })
        .unwrap();
    assert_eq!(queue.dispatch(125).unwrap().1, 1);
    assert_eq!(queue.inspect(id).unwrap().due_ns, Some(130));
    let before = queue.cell.get().clone();
    assert!(
        queue
            .create(Create {
                timing: Timing::FixedRate {
                    first_ns: 0,
                    every_ns: 0,
                    catch_up: true
                },
                task: Task::Increment(1)
            })
            .is_err()
    );
    assert_eq!(*queue.cell.get(), before);
    assert_eq!(queue.cancel(999, 125), Err(Error::UnknownJob));
    assert_eq!(*queue.cell.get(), before);
}

#[test]
fn known_local_failure_is_terminal_and_preserves_bounded_batch_progress() {
    for failed_first in [false, true] {
        let memory = VectorMemory::default();
        let mut queue = Queue::open(memory.clone(), Some(config())).unwrap();
        queue.create(increment(0)).unwrap();
        assert_eq!(queue.dispatch(0).unwrap(), (vec![], 1));

        let failed = Create {
            timing: Timing::FixedRate {
                first_ns: 100,
                every_ns: 10,
                catch_up: true,
            },
            task: Task::Increment(u64::MAX),
        };
        let healthy = Create {
            timing: Timing::Once(100),
            task: Task::Increment(2),
        };
        let (failed, healthy) = if failed_first {
            (
                queue.create(failed).unwrap(),
                queue.create(healthy).unwrap(),
            )
        } else {
            let healthy = queue.create(healthy).unwrap();
            (queue.create(failed).unwrap(), healthy)
        };
        let remaining = queue.create(increment(100)).unwrap();
        assert_eq!(queue.dispatch(100).unwrap(), (vec![], 2));
        assert_eq!(queue.summary().unwrap(), (Some(100), 3, 0));
        assert_eq!(queue.inspect(failed).unwrap().status, Status::Failed);
        assert_eq!(queue.inspect(healthy).unwrap().status, Status::Completed);
        assert_eq!(queue.inspect(remaining).unwrap().status, Status::Pending);
        let failed_record = queue.inspect(failed).unwrap();
        assert_eq!(failed_record.sequence, 1);
        assert_eq!(failed_record.occurrence, 0);
        assert_eq!(failed_record.due_ns, None);

        drop(queue);
        let mut restored = Queue::open(memory, None).unwrap();
        assert_eq!(restored.inspect(failed).unwrap(), failed_record);
        assert_eq!(restored.dispatch(100).unwrap(), (vec![], 1));
        assert_eq!(restored.summary().unwrap(), (None, 4, 0));
        assert_eq!(restored.inspect(failed).unwrap(), failed_record);
        assert_eq!(restored.dispatch(200).unwrap(), (vec![], 0));
        assert_eq!(restored.summary().unwrap(), (None, 4, 0));
    }
}

#[test]
fn running_and_uncertain_survive_reopen_and_require_exact_receipts() {
    for uncertain in [false, true] {
        let backing = VectorMemory::default();
        let destination = Principal::self_authenticating(b"receiver");
        let mut sender = Queue::open(backing.clone(), Some(config())).unwrap();
        let id = sender
            .create(Create {
                timing: Timing::Once(100),
                task: Task::Deliver {
                    destination,
                    message: "report".into(),
                },
            })
            .unwrap();
        let (deliveries, _) = sender.dispatch(100).unwrap();
        let delivery = &deliveries[0];
        if uncertain {
            sender.finish(delivery, 101, Outcome::Uncertain).unwrap();
        }
        drop(sender);
        let mut sender = Queue::open(backing, None).unwrap();
        assert_eq!(sender.outstanding(id).unwrap(), *delivery);
        assert!(sender.cancel(id, 102).is_err());
        assert_eq!(sender.dispatch(102).unwrap(), (vec![], 0));
        let receiver_memory = VectorMemory::default();
        let mut receiver = Queue::open(
            receiver_memory.clone(),
            Some(Init {
                manager: manager(),
                delivery_sender: Some(manager()),
            }),
        )
        .unwrap();
        assert_eq!(receiver.receipt(manager(), delivery).unwrap(), None);
        assert_eq!(
            receiver.receive(Principal::anonymous(), delivery.clone()),
            Err(Error::Unauthorized)
        );
        receiver.receive(manager(), delivery.clone()).unwrap();
        receiver.receive(manager(), delivery.clone()).unwrap();
        drop(receiver);
        let receiver = Queue::open(receiver_memory, None).unwrap();
        assert_eq!(receiver.summary().unwrap().2, 1);
        assert_eq!(
            receiver.receipt(manager(), delivery).unwrap(),
            Some(delivery.clone())
        );
        let mut stale = delivery.clone();
        stale.sequence += 1;
        let before = sender.cell.get().clone();
        assert_eq!(
            sender.finish(&stale, 102, Outcome::Success),
            Err(Error::ReceiptMismatch)
        );
        assert_eq!(*sender.cell.get(), before);
        sender.finish(delivery, 102, Outcome::Success).unwrap();
        assert_eq!(sender.inspect(id).unwrap().status, Status::Completed);
        assert_eq!(sender.dispatch(103).unwrap().1, 0);
    }
}

#[test]
fn corrupt_future_record_blocks_due_dispatch_and_recovery_without_reset() {
    let backing = VectorMemory::default();
    let mut queue = Queue::open(backing.clone(), Some(config())).unwrap();
    queue.create(increment(100)).unwrap();
    let corrupt = queue.create(increment(200)).unwrap();
    queue.corrupt_record(corrupt).unwrap();
    let bytes = queue.cell.get().clone();
    assert!(queue.dispatch(100).is_err());
    assert_eq!(*queue.cell.get(), bytes);
    drop(queue);
    assert!(Queue::open(backing, None).is_err());
}

#[test]
fn contradictory_dispatch_counts_block_the_complete_stable_queue() {
    for uncertain in [false, true] {
        let backing = VectorMemory::default();
        let mut queue = Queue::open(backing.clone(), Some(config())).unwrap();
        let corrupt = queue
            .create(Create {
                timing: Timing::Once(100),
                task: Task::Deliver {
                    destination: Principal::self_authenticating(b"receiver"),
                    message: "retained intent".into(),
                },
            })
            .unwrap();
        let healthy = queue.create(increment(200)).unwrap();
        let (deliveries, _) = queue.dispatch(100).unwrap();
        if uncertain {
            queue
                .finish(&deliveries[0], 101, Outcome::Uncertain)
                .unwrap();
        }
        let mut snapshot: Snapshot = serde_json::from_slice(queue.cell.get()).unwrap();
        let entry = snapshot
            .jobs
            .iter_mut()
            .find(|entry| entry.job.id == JobId(u128::from(corrupt)))
            .unwrap();
        entry.job.sequence += 1;
        queue.cell.set(serde_json::to_vec(&snapshot).unwrap());
        let bytes = queue.cell.get().clone();
        let expected = job_error(ic_jobs::JobError::InvalidRecord);
        assert_eq!(queue.dispatch(200), Err(expected.clone()));
        assert_eq!(queue.summary(), Err(expected.clone()));
        assert_eq!(queue.inspect(healthy), Err(expected.clone()));
        assert_eq!(*queue.cell.get(), bytes);
        let memory = queue.cell.into_memory();
        assert!(matches!(Queue::open(backing, None), Err(error) if error == expected));
        assert_eq!(Cell::init(memory, Vec::<u8>::new()).get(), &bytes);
    }
}

#[test]
fn retained_timing_corruption_blocks_the_complete_stable_queue() {
    let mut retry = Job::new(
        JobId(2),
        Schedule::Once { at_ns: 100 },
        RetryPolicy::new(3, 10, 20).unwrap(),
    );
    let first = retry.start(100).unwrap();
    retry.finish(first, 100, Outcome::RetryableFailure).unwrap();
    let mut cancelled_retry = Job::restore(retry.record()).unwrap();
    cancelled_retry.cancel(105).unwrap();
    let mut cancelled_retry = cancelled_retry.record();
    cancelled_retry.last_outcome_ns = Some(99);
    retry.start(110).unwrap();
    let mut running_retry = retry.record();
    running_retry.state = JobState::Running { started_ns: 109 };
    running_retry.updated_ns = 109;

    let mut recurring = Job::new(
        JobId(2),
        Schedule::AfterCompletion {
            first_at_ns: 100,
            every_ns: NonZeroU64::new(10).unwrap(),
        },
        RetryPolicy::NONE,
    );
    let first = recurring.start(100).unwrap();
    recurring.finish(first, 200, Outcome::Success).unwrap();
    let mut cancelled_recurring = Job::restore(recurring.record()).unwrap();
    cancelled_recurring.cancel(300).unwrap();
    let mut cancelled_recurring = cancelled_recurring.record();
    cancelled_recurring.scheduled_ns = 110;
    recurring.start(300).unwrap();
    let mut running_recurring = recurring.record();
    running_recurring.scheduled_ns = 110;

    for corrupt in [
        cancelled_retry,
        running_retry,
        cancelled_recurring,
        running_recurring,
    ] {
        let backing = VectorMemory::default();
        let mut queue = Queue::open(backing.clone(), Some(config())).unwrap();
        queue.create(increment(100)).unwrap();
        queue.create(increment(200)).unwrap();
        let mut snapshot: Snapshot = serde_json::from_slice(queue.cell.get()).unwrap();
        snapshot.jobs[1].job = corrupt;
        queue.cell.set(serde_json::to_vec(&snapshot).unwrap());
        let bytes = queue.cell.get().clone();
        assert_eq!(
            queue.dispatch(100),
            Err(job_error(ic_jobs::JobError::InvalidRecord))
        );
        assert_eq!(*queue.cell.get(), bytes);
        let memory = queue.cell.into_memory();
        assert!(matches!(
            Queue::open(backing.clone(), None),
            Err(Error::Job(_))
        ));
        assert_eq!(Cell::init(memory, Vec::<u8>::new()).get(), &bytes);
    }
}

#[test]
fn released_075_snapshot_reopens_without_reset_and_retains_blocked_effects() {
    // Produced with archived release 060fb9d226389bab702132ba23f42bd0b887a690
    // before changing configuration types. Keep the established memory key/codec.
    let bytes = include_bytes!("fixtures/snapshot-075.json").to_vec();
    let memory = VectorMemory::default();
    let mut queue = Queue::open(memory.clone(), Some(config())).unwrap();
    queue.cell.set(bytes.clone());
    drop(queue);
    let mut queue = Queue::open(memory.clone(), None).unwrap();
    assert_eq!(queue.cell.get(), &bytes);
    assert_eq!(serde_json::to_vec(&queue.load().unwrap()).unwrap(), bytes);
    assert_eq!(queue.summary().unwrap(), (Some(200), 0, 0));
    let running = queue.inspect(1).unwrap();
    let uncertain = queue.inspect(2).unwrap();
    assert_eq!(running.status, Status::Running);
    assert_eq!(uncertain.status, Status::Uncertain);
    assert_eq!(queue.inspect(3).unwrap().status, Status::Paused);
    assert_eq!(
        queue.outstanding(1).unwrap().message,
        "retained external intent"
    );
    assert_eq!(
        queue.outstanding(2).unwrap().message,
        "uncertain external effect"
    );
    assert_eq!(queue.dispatch(200).unwrap(), (vec![], 1));
    assert_eq!(queue.summary().unwrap(), (Some(210), 5, 0));
    assert_eq!(queue.inspect(1).unwrap(), running);
    assert_eq!(queue.inspect(2).unwrap(), uncertain);
    drop(queue);
    let queue = Queue::open(memory, None).unwrap();
    assert_eq!(queue.inspect(1).unwrap(), running);
    assert_eq!(queue.inspect(2).unwrap(), uncertain);
}

#[test]
fn raw_invalid_intervals_preserve_storage_and_unknown_job_precedence() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config())).unwrap();
    let id = queue.create(increment(100)).unwrap();
    for timing in [
        Timing::FixedRate {
            first_ns: 0,
            every_ns: 0,
            catch_up: false,
        },
        Timing::AfterCompletion {
            first_ns: 0,
            every_ns: 0,
        },
    ] {
        let before = queue.cell.get().clone();
        assert_eq!(
            queue.create(Create {
                timing: timing.clone(),
                task: Task::Increment(1)
            }),
            Err(Error::InvalidInput)
        );
        assert_eq!(
            queue.reschedule(id, timing.clone(), 0),
            Err(Error::InvalidInput)
        );
        assert_eq!(queue.reschedule(999, timing, 0), Err(Error::UnknownJob));
        assert_eq!(queue.cell.get(), &before);
    }
    queue.corrupt_record(id).unwrap();
    let before = queue.cell.get().clone();
    assert_eq!(queue.dispatch(100), Err(Error::InvalidStorage));
    assert_eq!(
        queue.reschedule(
            999,
            Timing::AfterCompletion {
                first_ns: 0,
                every_ns: 0
            },
            0
        ),
        Err(Error::InvalidStorage)
    );
    assert_eq!(queue.cell.get(), &before);
}

#[test]
fn released_075_stable_memory_image_opens_with_the_same_allocation_and_cell() {
    // Sparse nonzero runs are a lossless fixture representation of the old
    // producer's complete VectorMemory, including allocator and cell headers.
    #[derive(Deserialize)]
    struct FrozenMemory {
        producer: String,
        pages: u64,
        runs: Vec<(u64, String)>,
        snapshot: String,
    }
    let frozen: FrozenMemory =
        serde_json::from_str(include_str!("fixtures/stable-image-075.json")).unwrap();
    assert_eq!(frozen.producer, "060fb9d226389bab702132ba23f42bd0b887a690");
    let memory = VectorMemory::default();
    assert_eq!(memory.grow(frozen.pages), 0);
    for (offset, encoded) in frozen.runs {
        assert_eq!(encoded.len() % 2, 0);
        let bytes = (0..encoded.len())
            .step_by(2)
            .map(|index| u8::from_str_radix(&encoded[index..index + 2], 16).unwrap())
            .collect::<Vec<_>>();
        memory.write(offset, &bytes);
    }
    let mut queue = Queue::open(memory.clone(), None).unwrap();
    assert_eq!(queue.cell.get(), frozen.snapshot.as_bytes());
    assert_eq!(queue.summary().unwrap(), (Some(200), 0, 0));
    assert_eq!(queue.inspect(1).unwrap().status, Status::Uncertain);
    // MemoryRuntime republishes its allocation ledger on reopen. The existing
    // allocation and Jobs cell stay retained; no fresh installation is selected.
    assert_eq!(memory.size(), frozen.pages);
    assert_eq!(queue.dispatch(200).unwrap(), (vec![], 1));
    assert_eq!(queue.inspect(1).unwrap().status, Status::Uncertain);
    drop(queue);
    let queue = Queue::open(memory, None).unwrap();
    assert_eq!(queue.summary().unwrap(), (Some(210), 5, 0));
    assert_eq!(queue.inspect(1).unwrap().status, Status::Uncertain);
}
