use super::*;
use ic_memory::VectorMemory;
use ic_stable_structures::Cell;

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

// These fixtures start with a fresh Once job. Build only valid states through
// the public transitions; malformed-record fixtures remain independent.
fn set_job_state(job: &mut Job, state: JobState, at_ns: u64) {
    match state {
        JobState::Pending { .. } => {}
        JobState::Paused { .. } => job.pause(at_ns).unwrap(),
        JobState::Cancelled => job.cancel(at_ns).unwrap(),
        _ => {
            let attempt = job.start(at_ns).unwrap();
            match state {
                JobState::Running => {}
                JobState::Uncertain { .. } => {
                    job.finish(attempt, at_ns, Outcome::Uncertain).unwrap()
                }
                JobState::Completed => job.finish(attempt, at_ns, Outcome::Success).unwrap(),
                JobState::Failed => job
                    .finish(attempt, at_ns, Outcome::PermanentFailure)
                    .unwrap(),
                _ => unreachable!(),
            }
        }
    }
    assert_eq!(job.record().state, state);
}

#[test]
fn paused_rescheduled_work_retains_identity_payload_and_deadline_across_reopen() {
    let memory = VectorMemory::default();
    let mut queue = Queue::open(memory.clone(), Some(config())).unwrap().0;
    let id = queue
        .create(Create {
            timing: Timing::FixedRate {
                first_ns: 100,
                every_ns: 10,
                catch_up: true,
            },
            task: Task::Increment(2),
        })
        .unwrap()
        .value;
    assert_eq!(queue.dispatch(100).unwrap().value.1, 1);
    queue.pause(id, 101).unwrap();
    assert_eq!(queue.summary().unwrap(), (None, 2, 0));
    drop(queue);
    let mut queue = Queue::open(memory.clone(), None).unwrap().0;
    assert_eq!(queue.inspect(id).unwrap().status, Status::Paused);
    assert_eq!(
        queue.dispatch(400).unwrap(),
        Committed {
            value: (vec![], 0),
            next_due_ns: None
        }
    );
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
    let mut queue = Queue::open(memory, None).unwrap().0;
    assert_eq!(queue.inspect(id).unwrap(), before);
    queue.resume(id, 450).unwrap();
    assert_eq!(queue.summary().unwrap().0, Some(500));
    assert_eq!(queue.dispatch(499).unwrap().value.1, 0);
    assert_eq!(queue.dispatch(500).unwrap().value.1, 1);
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
        let mut queue = Queue::open(memory.clone(), Some(config())).unwrap().0;
        let id = queue
            .create(Create {
                timing: Timing::FixedRate {
                    first_ns: 100,
                    every_ns: 10,
                    catch_up,
                },
                task: Task::Increment(1),
            })
            .unwrap()
            .value;
        queue.pause(id, 50).unwrap();
        drop(queue);
        let mut queue = Queue::open(memory, None).unwrap().0;
        queue.resume(id, 200).unwrap();
        assert_eq!(queue.summary().unwrap().0, Some(100));
        assert_eq!(
            queue.dispatch(200).unwrap().value.1,
            if catch_up { 2 } else { 1 }
        );
        assert_eq!(
            queue.summary().unwrap().0,
            Some(if catch_up { 120 } else { 210 })
        );
    }
}

#[test]
fn refused_management_preserves_stable_bytes_and_blocked_delivery() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    let id = queue
        .create(Create {
            timing: Timing::Once(0),
            task: Task::Deliver(DeliveryPayload {
                destination: Principal::self_authenticating(b"receiver"),
                message: "retained".into(),
            }),
        })
        .unwrap()
        .value;
    let Committed {
        value: (deliveries, _),
        ..
    } = queue.dispatch(0).unwrap();
    for uncertain in [false, true] {
        if uncertain {
            queue.finish(&deliveries[0], 1, Outcome::Uncertain).unwrap();
        }
        let before = queue.cell.bytes().to_vec();
        assert!(queue.pause(id, 2).is_err());
        assert!(queue.resume(id, 2).is_err());
        assert!(queue.reschedule(id, Timing::Once(100), 2).is_err());
        assert_eq!(queue.cell.bytes(), before);
        assert_eq!(queue.dispatch(2).unwrap().value.1, 0);
        assert_eq!(queue.cell.bytes(), before);
    }
    let pending = queue.create(increment(100)).unwrap().value;
    let before = queue.cell.bytes().to_vec();
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
    assert_eq!(queue.cell.bytes(), before);
}

#[test]
fn retained_queue_restores_payloads_deadlines_cancellation_and_id_authority() {
    let memory = VectorMemory::default();
    let mut queue = Queue::open(memory.clone(), Some(config())).unwrap().0;
    queue.authorize(manager()).unwrap();
    assert_eq!(
        queue.authorize(Principal::anonymous()),
        Err(Error::Unauthorized)
    );
    let first = queue.create(increment(100)).unwrap().value;
    let second = queue.create(increment(200)).unwrap().value;
    queue.cancel(first, 10).unwrap();
    let expected = queue.page(None, 8).unwrap();
    drop(queue);
    let mut restored = Queue::open(memory.clone(), None).unwrap().0;
    assert_eq!(restored.page(None, 8).unwrap(), expected);
    assert_eq!(restored.summary().unwrap(), (Some(200), 0, 0));
    assert_eq!(restored.page(Some(first), 1).unwrap()[0].id, second);
    assert_eq!(restored.page(None, 0), Err(Error::InvalidInput));
    assert_eq!(restored.create(increment(300)).unwrap().value, 3);
    assert!(Queue::open(memory, Some(config())).is_err());
}

#[test]
fn local_effects_and_bounded_overdue_work_commit_together() {
    let memory = VectorMemory::default();
    let mut queue = Queue::open(memory.clone(), Some(config())).unwrap().0;
    let id = queue
        .create(Create {
            timing: Timing::FixedRate {
                first_ns: 100,
                every_ns: 10,
                catch_up: true,
            },
            task: Task::Increment(2),
        })
        .unwrap()
        .value;
    assert_eq!(
        queue.dispatch(125).unwrap(),
        Committed {
            value: (vec![], 2),
            next_due_ns: Some(120)
        }
    );
    assert_eq!(queue.summary().unwrap(), (Some(120), 4, 0));
    drop(queue);
    let mut queue = Queue::open(memory, None).unwrap().0;
    assert_eq!(queue.inspect(id).unwrap().occurrence, 2);
    queue.dispatch(125).unwrap();
    assert_eq!(queue.summary().unwrap(), (Some(130), 6, 0));
    queue.cancel(id, 125).unwrap();
    assert_eq!(queue.summary().unwrap().0, None);
}

#[test]
fn skip_and_refused_mutations_preserve_committed_bytes() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    let id = queue
        .create(Create {
            timing: Timing::FixedRate {
                first_ns: 100,
                every_ns: 10,
                catch_up: false,
            },
            task: Task::Increment(1),
        })
        .unwrap()
        .value;
    assert_eq!(queue.dispatch(125).unwrap().value.1, 1);
    assert_eq!(queue.inspect(id).unwrap().due_ns, Some(130));
    let before = queue.cell.bytes().to_vec();
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
    assert_eq!(queue.cell.bytes(), before);
    assert_eq!(queue.cancel(999, 125), Err(Error::UnknownJob));
    assert_eq!(queue.cell.bytes(), before);
}

#[test]
fn known_local_failure_is_terminal_and_preserves_bounded_batch_progress() {
    for failed_first in [false, true] {
        let memory = VectorMemory::default();
        let mut queue = Queue::open(memory.clone(), Some(config())).unwrap().0;
        queue.create(increment(0)).unwrap();
        assert_eq!(
            queue.dispatch(0).unwrap(),
            Committed {
                value: (vec![], 1),
                next_due_ns: None
            }
        );

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
                queue.create(failed).unwrap().value,
                queue.create(healthy).unwrap().value,
            )
        } else {
            let healthy = queue.create(healthy).unwrap().value;
            (queue.create(failed).unwrap().value, healthy)
        };
        let remaining = queue.create(increment(100)).unwrap().value;
        assert_eq!(
            queue.dispatch(100).unwrap(),
            Committed {
                value: (vec![], 2),
                next_due_ns: Some(100)
            }
        );
        assert_eq!(queue.summary().unwrap(), (Some(100), 3, 0));
        assert_eq!(queue.inspect(failed).unwrap().status, Status::Failed);
        assert_eq!(queue.inspect(healthy).unwrap().status, Status::Completed);
        assert_eq!(queue.inspect(remaining).unwrap().status, Status::Pending);
        let failed_record = queue.inspect(failed).unwrap();
        assert_eq!(failed_record.sequence, 1);
        assert_eq!(failed_record.occurrence, 0);
        assert_eq!(failed_record.due_ns, None);

        drop(queue);
        let mut restored = Queue::open(memory, None).unwrap().0;
        assert_eq!(restored.inspect(failed).unwrap(), failed_record);
        assert_eq!(
            restored.dispatch(100).unwrap(),
            Committed {
                value: (vec![], 1),
                next_due_ns: None
            }
        );
        assert_eq!(restored.summary().unwrap(), (None, 4, 0));
        assert_eq!(restored.inspect(failed).unwrap(), failed_record);
        assert_eq!(
            restored.dispatch(200).unwrap(),
            Committed {
                value: (vec![], 0),
                next_due_ns: None
            }
        );
        assert_eq!(restored.summary().unwrap(), (None, 4, 0));
    }
}

#[test]
fn running_and_uncertain_survive_reopen_and_require_exact_receipts() {
    for uncertain in [false, true] {
        let backing = VectorMemory::default();
        let destination = Principal::self_authenticating(b"receiver");
        let mut sender = Queue::open(backing.clone(), Some(config())).unwrap().0;
        let id = sender
            .create(Create {
                timing: Timing::Once(100),
                task: Task::Deliver(DeliveryPayload {
                    destination,
                    message: "report".into(),
                }),
            })
            .unwrap()
            .value;
        let Committed {
            value: (deliveries, _),
            ..
        } = sender.dispatch(100).unwrap();
        let delivery = &deliveries[0];
        if uncertain {
            sender.finish(delivery, 101, Outcome::Uncertain).unwrap();
        }
        drop(sender);
        let mut sender = Queue::open(backing, None).unwrap().0;
        assert_eq!(sender.outstanding(id).unwrap(), *delivery);
        assert!(sender.cancel(id, 102).is_err());
        assert_eq!(
            sender.dispatch(102).unwrap(),
            Committed {
                value: (vec![], 0),
                next_due_ns: None
            }
        );
        let receiver_memory = VectorMemory::default();
        let mut receiver = Queue::open(
            receiver_memory.clone(),
            Some(Init {
                manager: manager(),
                delivery_sender: Some(manager()),
            }),
        )
        .unwrap()
        .0;
        assert_eq!(receiver.receipt(manager(), delivery).unwrap(), None);
        assert_eq!(
            receiver.receive(Principal::anonymous(), delivery.clone()),
            Err(Error::Unauthorized)
        );
        receiver.receive(manager(), delivery.clone()).unwrap();
        receiver.receive(manager(), delivery.clone()).unwrap();
        drop(receiver);
        let receiver = Queue::open(receiver_memory, None).unwrap().0;
        assert_eq!(receiver.summary().unwrap().2, 1);
        assert_eq!(
            receiver.receipt(manager(), delivery).unwrap(),
            Some(delivery.clone())
        );
        let mut stale = delivery.clone();
        stale.sequence += 1;
        let before = sender.cell.bytes().to_vec();
        assert_eq!(
            sender.finish(&stale, 102, Outcome::Success),
            Err(Error::ReceiptMismatch)
        );
        assert_eq!(sender.cell.bytes(), before);
        sender.finish(delivery, 102, Outcome::Success).unwrap();
        assert_eq!(sender.inspect(id).unwrap().status, Status::Completed);
        assert_eq!(sender.dispatch(103).unwrap().value.1, 0);
    }
}

#[test]
fn completion_refusals_preserve_input_receipt_and_clock_precedence() {
    let destination = Principal::self_authenticating(b"receiver");
    for task in [
        Task::Increment(1),
        Task::Deliver(DeliveryPayload {
            destination,
            message: "report".into(),
        }),
    ] {
        for state in [
            JobState::Pending { due_ns: 10 },
            JobState::Paused { due_ns: 10 },
            JobState::Running,
            JobState::Uncertain { started_ns: 10 },
            JobState::Completed,
            JobState::Failed,
            JobState::Cancelled,
        ] {
            let mut queue = Queue::open(VectorMemory::default(), Some(config()))
                .unwrap()
                .0;
            let id = queue
                .create(Create {
                    timing: Timing::Once(10),
                    task: task.clone(),
                })
                .unwrap()
                .value;
            let mut snapshot = queue.load().unwrap();
            let job = &mut snapshot.jobs[0].job;
            set_job_state(job, state, 10);
            let record = job.record();
            queue.commit(snapshot, ()).unwrap();
            let delivery = Delivery {
                job: id,
                occurrence: record.occurrence,
                sequence: record.sequence,
                payload: DeliveryPayload {
                    destination,
                    message: "report".into(),
                },
            };
            let outstanding = matches!(task, Task::Deliver(_))
                && matches!(state, JobState::Running | JobState::Uncertain { .. });
            let before = queue.cell.bytes().to_vec();
            let mismatch = if outstanding {
                Error::ReceiptMismatch
            } else {
                Error::InvalidInput
            };
            for (candidate, expected) in [
                (
                    Delivery {
                        occurrence: delivery.occurrence + 1,
                        ..delivery.clone()
                    },
                    mismatch.clone(),
                ),
                (
                    Delivery {
                        sequence: delivery.sequence + 1,
                        ..delivery.clone()
                    },
                    mismatch.clone(),
                ),
                (
                    Delivery {
                        payload: DeliveryPayload {
                            destination: Principal::anonymous(),
                            ..delivery.payload.clone()
                        },
                        ..delivery.clone()
                    },
                    mismatch.clone(),
                ),
                (
                    Delivery {
                        job: id + 1,
                        ..delivery.clone()
                    },
                    Error::UnknownJob,
                ),
                (
                    Delivery {
                        payload: DeliveryPayload {
                            message: "changed".into(),
                            ..delivery.payload.clone()
                        },
                        ..delivery.clone()
                    },
                    mismatch,
                ),
                (
                    delivery,
                    if outstanding {
                        job_error(ic_jobs::JobError::TimeWentBackwards)
                    } else {
                        Error::InvalidInput
                    },
                ),
            ] {
                assert_eq!(queue.finish(&candidate, 9, Outcome::Success), Err(expected));
                assert_eq!(queue.cell.bytes(), before);
            }
        }
    }
}

#[test]
fn unrelated_corruption_precedes_completion_lookup_receipt_and_clock_checks() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    queue
        .create(Create {
            timing: Timing::Once(10),
            task: Task::Deliver(DeliveryPayload {
                destination: Principal::self_authenticating(b"receiver"),
                message: "report".into(),
            }),
        })
        .unwrap();

    let corrupt = queue.create(increment(20)).unwrap().value;
    let delivery = queue.dispatch(10).unwrap().value.0.remove(0);
    queue.corrupt_record(corrupt).unwrap();
    let before = queue.cell.bytes().to_vec();
    for candidate in [
        Delivery {
            job: corrupt + 1,
            ..delivery.clone()
        },
        Delivery {
            sequence: delivery.sequence + 1,
            ..delivery.clone()
        },
        delivery,
    ] {
        assert_eq!(
            queue.finish(&candidate, 9, Outcome::Success),
            Err(Error::InvalidStorage)
        );
        assert_eq!(queue.cell.bytes(), before);
    }
}

#[test]
fn corrupt_future_record_blocks_due_dispatch_and_recovery_without_reset() {
    let backing = VectorMemory::default();
    let mut queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
    queue.create(increment(100)).unwrap();
    let corrupt = queue.create(increment(200)).unwrap().value;
    queue.corrupt_record(corrupt).unwrap();
    let bytes = queue.cell.bytes().to_vec();
    assert!(queue.dispatch(100).is_err());
    assert_eq!(queue.cell.bytes(), bytes);
    drop(queue);
    assert!(Queue::open(backing, None).is_err());
}

fn assert_invalid_record_blocks_queue(
    mut queue: Queue<VectorMemory>,
    backing: VectorMemory,
    now: u64,
    healthy: u64,
) {
    let bytes = queue.cell.bytes().to_vec();
    let expected = job_error(ic_jobs::JobError::InvalidRecord);
    assert_eq!(queue.dispatch(now), Err(expected.clone()));
    assert_eq!(queue.summary(), Err(expected.clone()));
    assert_eq!(queue.inspect(healthy), Err(expected.clone()));
    assert_eq!(queue.cell.bytes(), bytes);
    let memory = queue.cell.into_memory();
    assert!(matches!(Queue::open(backing, None), Err(error) if error == expected));
    assert_eq!(Cell::init(memory, Vec::<u8>::new()).get(), &bytes);
}

#[test]
fn contradictory_dispatch_counts_block_the_complete_stable_queue() {
    for uncertain in [false, true] {
        let backing = VectorMemory::default();
        let mut queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
        let corrupt = queue
            .create(Create {
                timing: Timing::Once(100),
                task: Task::Deliver(DeliveryPayload {
                    destination: Principal::self_authenticating(b"receiver"),
                    message: "retained intent".into(),
                }),
            })
            .unwrap()
            .value;
        let healthy = queue.create(increment(200)).unwrap().value;
        let Committed {
            value: (deliveries, _),
            ..
        } = queue.dispatch(100).unwrap();
        if uncertain {
            queue
                .finish(&deliveries[0], 101, Outcome::Uncertain)
                .unwrap();
        }
        let mut snapshot: Snapshot = serde_json::from_slice(queue.cell.bytes()).unwrap();
        let entry = snapshot
            .jobs
            .iter_mut()
            .find(|entry| entry.job.id == JobId(u128::from(corrupt)))
            .unwrap();
        entry.job.sequence += 1;
        queue.cell.set(serde_json::to_vec(&snapshot).unwrap());
        assert_invalid_record_blocks_queue(queue, backing, 200, healthy);
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
    cancelled_retry.last_outcome.as_mut().unwrap().at_ns = 99;
    retry.start(110).unwrap();
    let mut running_retry = retry.record();
    running_retry.state = JobState::Running;
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
        let mut queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
        let healthy = queue.create(increment(100)).unwrap().value;
        queue.create(increment(200)).unwrap();
        let mut snapshot: Snapshot = serde_json::from_slice(queue.cell.bytes()).unwrap();
        snapshot.jobs[1].job = corrupt;
        queue.cell.set(serde_json::to_vec(&snapshot).unwrap());
        assert_invalid_record_blocks_queue(queue, backing, 100, healthy);
    }
}

#[test]
fn snapshot_admission_preserves_restore_and_collection_error_order() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    queue.create(increment(100)).unwrap();
    queue.create(increment(200)).unwrap();
    let bytes = queue.cell.bytes().to_vec();
    for case in 0..4 {
        let mut snapshot: Snapshot = serde_json::from_slice(&bytes).unwrap();
        let bad_record = if case == 2 { 1 } else { 0 };
        snapshot.jobs[bad_record].job.state = JobState::Completed;
        let expected = match case {
            0 => {
                snapshot.next_id = 0;
                Error::InvalidStorage
            }
            1 => {
                snapshot.jobs[0].job.id = JobId(0);
                job_error(ic_jobs::JobError::InvalidRecord)
            }
            2 => {
                snapshot.jobs[0].task = Task::Deliver(DeliveryPayload {
                    destination: manager(),
                    message: "x".repeat(MAX_MESSAGE + 1),
                });
                Error::InvalidStorage
            }
            3 => {
                snapshot.receipts.push(Delivery {
                    job: 0,
                    occurrence: 0,
                    sequence: 0,
                    payload: DeliveryPayload {
                        destination: manager(),
                        message: String::new(),
                    },
                });
                job_error(ic_jobs::JobError::InvalidRecord)
            }
            _ => unreachable!(),
        };
        assert_eq!(
            decode(&serde_json::to_vec(&snapshot).unwrap()).map(|_| ()),
            Err(expected),
            "case {case}"
        );
    }
    assert_eq!(queue.cell.bytes(), bytes);
}

#[test]
fn view_projection_preserves_large_ids_and_refuses_unadmitted_identities() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    queue.create(increment(100)).unwrap();
    let original = queue.cell.bytes().to_vec();
    let id = u64::MAX - 1;
    let mut snapshot: Snapshot = serde_json::from_slice(&original).unwrap();
    snapshot.next_id = u64::MAX;
    snapshot.jobs[0].job.id = JobId(u128::from(id));
    queue.cell.set(serde_json::to_vec(&snapshot).unwrap());
    assert_eq!(queue.inspect(id).unwrap().id, id);
    assert_eq!(queue.page(None, 8).unwrap()[0].id, id);

    for invalid_id in [u128::from(u64::MAX), u128::from(u64::MAX) + 1, u128::MAX] {
        snapshot.jobs[0].job.id = JobId(invalid_id);
        let bytes = serde_json::to_vec(&snapshot).unwrap();
        queue.cell.set(bytes.clone());
        assert_eq!(queue.inspect(id), Err(Error::InvalidStorage));
        assert_eq!(queue.page(None, 8), Err(Error::InvalidStorage));
        assert_eq!(queue.cell.bytes(), bytes);
    }
}

#[test]
fn typed_commit_preserves_canonical_snapshot_bytes_and_metadata() {
    let memory = VectorMemory::default();
    let mut queue = Queue::open(
        memory.clone(),
        Some(Init {
            manager: manager(),
            delivery_sender: Some(manager()),
        }),
    )
    .unwrap()
    .0;
    queue.create(increment(100)).unwrap();
    queue
        .create(Create {
            timing: Timing::Once(100),
            task: Task::Deliver(DeliveryPayload {
                destination: manager(),
                message: "paired payload".into(),
            }),
        })
        .unwrap();
    let delivery = queue.dispatch(100).unwrap().value.0.pop().unwrap();
    queue.receive(manager(), delivery.clone()).unwrap();
    queue.finish(&delivery, 101, Outcome::Uncertain).unwrap();
    let later = queue.create(increment(200)).unwrap().value;
    queue.pause(later, 102).unwrap();
    // The raw canonical writer and typed writer must retain the same metadata,
    // payloads, receipt and blocked/terminal/paused records byte for byte.
    let stored: Snapshot = serde_json::from_slice(queue.cell.bytes()).unwrap();
    let expected = serde_json::to_vec(&stored).unwrap();
    let loaded = queue.load().unwrap();
    assert_eq!(queue.commit(loaded, ()).unwrap().next_due_ns, None);
    assert_eq!(queue.cell.bytes(), expected);
    drop(queue);
    let (queue, due) = Queue::open(memory, None).unwrap();
    assert_eq!(due, None);
    assert_eq!(queue.summary().unwrap(), (None, 1, 1));
    assert_eq!(queue.outstanding(delivery.job).unwrap(), delivery);
    assert_eq!(queue.inspect(later).unwrap().status, Status::Paused);
}

#[test]
fn typed_commit_refuses_invalid_collection_metadata_without_changing_bytes() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    queue.create(increment(100)).unwrap();
    let receipt = Delivery {
        job: 1,
        occurrence: 0,
        sequence: 1,
        payload: DeliveryPayload {
            destination: manager(),
            message: "confirmed".into(),
        },
    };
    for case in 0..8 {
        let before = queue.cell.bytes().to_vec();
        let mut loaded = queue.load().unwrap();
        match case {
            0 => loaded.next_id = 0,
            1 => loaded.next_id = 1,
            2 => {
                loaded.jobs.push(Entry {
                    job: Job::restore(loaded.jobs[0].job.record()).unwrap(),
                    task: Task::Increment(1),
                });
            }
            3 => {
                loaded.jobs[0].task = Task::Deliver(DeliveryPayload {
                    destination: manager(),
                    message: "x".repeat(MAX_MESSAGE + 1),
                })
            }
            4 => loaded.receipts.push(Delivery {
                job: 0,
                ..receipt.clone()
            }),
            5 => loaded.receipts.push(Delivery {
                sequence: 0,
                ..receipt.clone()
            }),
            6 => loaded.receipts.push(Delivery {
                payload: DeliveryPayload {
                    message: "x".repeat(MAX_MESSAGE + 1),
                    ..receipt.payload.clone()
                },
                ..receipt.clone()
            }),
            7 => {
                loaded.receipts.push(receipt.clone());
                loaded.receipts.push(Delivery {
                    sequence: 2,
                    ..receipt.clone()
                });
            }
            _ => unreachable!(),
        }
        assert_eq!(
            queue.commit(loaded, ()),
            Err(Error::InvalidStorage),
            "case {case}"
        );
        assert_eq!(queue.cell.bytes(), before, "case {case}");
    }
    // Encoded capacity still takes precedence over invalid collection metadata.
    let before = queue.cell.bytes().to_vec();
    let mut loaded = queue.load().unwrap();
    loaded.next_id = 0;
    loaded.jobs[0].task = Task::Deliver(DeliveryPayload {
        destination: manager(),
        message: "x".repeat(MAX_BYTES),
    });
    assert_eq!(queue.commit(loaded, ()), Err(Error::Capacity));
    assert_eq!(queue.cell.bytes(), before);
}

#[test]
fn retired_snapshot_is_refused_without_replacing_the_cell() {
    // Exact old producer bytes remain only as a refusal/preservation fixture.
    for frozen in [
        include_bytes!("fixtures/snapshot-075.json").as_slice(),
        include_bytes!("fixtures/snapshot-running-015.json").as_slice(),
        include_bytes!("fixtures/snapshot-receipt-015.json").as_slice(),
    ] {
        let bytes = frozen.to_vec();
        let backing = VectorMemory::default();
        let mut queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
        queue.cell.set(bytes.clone());
        let memory = queue.cell.into_memory();
        for initial in [None, Some(config())] {
            assert!(matches!(
                Queue::open(backing.clone(), initial),
                Err(Error::InvalidStorage)
            ));
            assert_eq!(
                ic_memory::decode_stable_cell_payload(&memory).unwrap(),
                bytes
            );
        }
    }
}

#[test]
fn raw_invalid_intervals_preserve_storage_and_unknown_job_precedence() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    let id = queue.create(increment(100)).unwrap().value;
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
        let before = queue.cell.bytes().to_vec();
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
        assert_eq!(queue.cell.bytes(), before);
    }
    queue.corrupt_record(id).unwrap();
    let before = queue.cell.bytes().to_vec();
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
    assert_eq!(queue.cell.bytes(), before);
}

#[test]
fn prior_memory_ledgers_are_refused_without_writes_or_reset() {
    // Sparse nonzero runs are a lossless fixture representation of the old
    // producer's complete VectorMemory, including allocator and cell headers.
    #[derive(Deserialize)]
    struct FrozenMemory {
        producer: String,
        pages: u64,
        runs: Vec<(u64, String)>,
        snapshot: String,
    }
    for (encoded, producer) in [
        (
            include_str!("fixtures/stable-image-075.json"),
            "060fb9d226389bab702132ba23f42bd0b887a690",
        ),
        (
            include_str!("fixtures/stable-image-memory040.json"),
            "ic-memory 0.40.0",
        ),
    ] {
        let frozen: FrozenMemory = serde_json::from_str(encoded).unwrap();
        assert_eq!(frozen.producer, producer);
        // Both layers are retired. The outer Memory ledger must refuse before
        // payload admission, publishing an allocation or overwriting old bytes.
        assert!(matches!(
            decode(frozen.snapshot.as_bytes()),
            Err(Error::InvalidStorage)
        ));
        let backing = VectorMemory::default();
        let memory = CountedMemory {
            backing,
            writes: std::rc::Rc::new(std::cell::Cell::new(0)),
        };
        assert_eq!(memory.grow(frozen.pages), 0);
        for (offset, encoded) in frozen.runs {
            assert_eq!(encoded.len() % 2, 0);
            let bytes = (0..encoded.len())
                .step_by(2)
                .map(|index| u8::from_str_radix(&encoded[index..index + 2], 16).unwrap())
                .collect::<Vec<_>>();
            memory.write(offset, &bytes);
        }
        memory.writes.set(0);
        let before = memory.backing.borrow().clone();
        for initial in [None, Some(config())] {
            assert!(matches!(
                Queue::open(memory.clone(), initial),
                Err(Error::Memory(_))
            ));
            assert_eq!(memory.writes.get(), 0);
            assert_eq!(memory.size(), frozen.pages);
            assert_eq!(*memory.backing.borrow(), before);
        }
    }
}

#[derive(Clone)]
struct CountedMemory {
    backing: VectorMemory,
    writes: std::rc::Rc<std::cell::Cell<u64>>,
}

impl Memory for CountedMemory {
    fn size(&self) -> u64 {
        self.backing.size()
    }
    fn grow(&self, pages: u64) -> i64 {
        self.backing.grow(pages)
    }
    fn read(&self, offset: u64, dst: &mut [u8]) {
        self.backing.read(offset, dst);
    }
    fn write(&self, offset: u64, src: &[u8]) {
        self.writes.set(self.writes.get() + 1);
        self.backing.write(offset, src);
    }
}

#[test]
fn idle_dispatch_validates_the_complete_snapshot_without_stable_writes() {
    for state in [
        None,
        Some((JobState::Pending { due_ns: 100 }, 0)),
        Some((JobState::Paused { due_ns: 100 }, 0)),
        Some((JobState::Running, 100)),
        Some((JobState::Uncertain { started_ns: 100 }, 100)),
        Some((JobState::Completed, 100)),
    ] {
        let memory = CountedMemory {
            backing: VectorMemory::default(),
            writes: std::rc::Rc::new(std::cell::Cell::new(0)),
        };
        let mut queue = Queue::open(memory.clone(), Some(config())).unwrap().0;
        let id = state.map(|_| queue.create(increment(100)).unwrap().value);
        let mut loaded = queue.load().unwrap();
        if let Some((target, at_ns)) = state {
            set_job_state(&mut loaded.jobs[0].job, target, at_ns);
        }
        queue.commit(loaded, ()).unwrap();
        let expected_due = queue.summary().unwrap().0;
        let before = queue.cell.bytes().to_vec();
        memory.writes.set(0);
        assert_eq!(
            queue.dispatch(99).unwrap(),
            Committed {
                value: (vec![], 0),
                next_due_ns: expected_due
            },
            "state {state:?}"
        );
        assert_eq!(memory.writes.get(), 0, "state {state:?}");
        assert_eq!(queue.cell.bytes(), before, "state {state:?}");
        if let Some(id) = id {
            queue.corrupt_record(id).unwrap();
            let before = queue.cell.bytes().to_vec();
            memory.writes.set(0);
            assert!(queue.dispatch(99).is_err(), "state {state:?}");
            assert_eq!(memory.writes.get(), 0);
            assert_eq!(queue.cell.bytes(), before);
        }
    }
    // Real due work still writes its metadata and local effect together.
    let memory = CountedMemory {
        backing: VectorMemory::default(),
        writes: std::rc::Rc::new(std::cell::Cell::new(0)),
    };
    let mut queue = Queue::open(memory.clone(), Some(config())).unwrap().0;
    queue.create(increment(100)).unwrap();
    memory.writes.set(0);
    assert_eq!(
        queue.dispatch(100).unwrap(),
        Committed {
            value: (vec![], 1),
            next_due_ns: None
        }
    );
    assert!(memory.writes.get() > 0);
    assert_eq!(queue.summary().unwrap().1, 1);
}

#[test]
fn opening_returns_the_admitted_deadline_without_rewriting_retained_records() {
    let memory = VectorMemory::default();
    let (mut queue, due) = Queue::open(memory.clone(), Some(config())).unwrap();
    assert_eq!(due, None);
    queue.create(increment(200)).unwrap();
    let first = queue.create(increment(100)).unwrap().value;
    let before = queue.cell.bytes().to_vec();
    drop(queue);
    let (mut queue, due) = Queue::open(memory.clone(), None).unwrap();
    assert_eq!(due, Some(100));
    assert_eq!(queue.cell.bytes(), before);
    queue.pause(first, 0).unwrap();
    drop(queue);
    let (mut queue, due) = Queue::open(memory.clone(), None).unwrap();
    assert_eq!(due, Some(200));
    queue.corrupt_record(first).unwrap();
    drop(queue);
    // Corruption in inactive work still blocks the entire restoration result.
    assert!(Queue::open(memory, None).is_err());
}

#[test]
fn mutation_replies_return_the_fresh_committed_deadline() {
    let memory = VectorMemory::default();
    let mut queue = Queue::open(memory.clone(), Some(config())).unwrap().0;
    let later = queue.create(increment(300)).unwrap();
    assert_eq!(later.next_due_ns, Some(300));
    let early = queue.create(increment(100)).unwrap();
    assert_eq!(early.next_due_ns, Some(100));
    assert_eq!(queue.pause(early.value, 0).unwrap().next_due_ns, Some(300));
    assert_eq!(
        queue
            .reschedule(later.value, Timing::Once(200), 0)
            .unwrap()
            .next_due_ns,
        Some(200)
    );
    assert_eq!(queue.resume(early.value, 0).unwrap().next_due_ns, Some(100));
    assert_eq!(queue.cancel(early.value, 0).unwrap().next_due_ns, Some(200));
    assert_eq!(
        queue.dispatch(200).unwrap(),
        Committed {
            value: (vec![], 1),
            next_due_ns: None
        }
    );
    let external = queue
        .create(Create {
            timing: Timing::Once(250),
            task: Task::Deliver(DeliveryPayload {
                destination: manager(),
                message: "exact immutable payload".into(),
            }),
        })
        .unwrap();
    assert_eq!(external.next_due_ns, Some(250));
    let Committed {
        value: (deliveries, _),
        next_due_ns: due,
    } = queue.dispatch(250).unwrap();
    assert_eq!(due, None);
    assert_eq!(
        queue
            .finish(&deliveries[0], 251, Outcome::Uncertain)
            .unwrap()
            .next_due_ns,
        None
    );
    // Another message changes the queue before the old delivery is reconciled.
    queue.create(increment(260)).unwrap();
    let reply = queue.finish(&deliveries[0], 252, Outcome::Success).unwrap();
    assert_eq!(reply.next_due_ns, Some(260));
    assert_eq!(
        queue.inspect(external.value).unwrap().status,
        Status::Completed
    );
    drop(queue);
    let (queue, due) = Queue::open(memory, None).unwrap();
    assert_eq!(due, reply.next_due_ns);
    assert_eq!(queue.summary().unwrap().0, reply.next_due_ns);
}

#[test]
fn receipt_extraction_and_duplicate_acknowledgement_leave_storage_unchanged() {
    let memory = CountedMemory {
        backing: VectorMemory::default(),
        writes: std::rc::Rc::new(std::cell::Cell::new(0)),
    };
    let mut queue = Queue::open(
        memory.clone(),
        Some(Init {
            manager: manager(),
            delivery_sender: Some(manager()),
        }),
    )
    .unwrap()
    .0;
    let receipts: Vec<_> = [10, 20, 30]
        .into_iter()
        .map(|job| Delivery {
            job,
            occurrence: 0,
            sequence: 1,
            payload: DeliveryPayload {
                destination: manager(),
                message: format!("payload-{job}"),
            },
        })
        .collect();
    for receipt in &receipts {
        assert_eq!(queue.receive(manager(), receipt.clone()).unwrap(), *receipt);
    }
    let before = queue.cell.bytes().to_vec();
    memory.writes.set(0);
    for receipt in receipts.iter().rev() {
        assert_eq!(
            queue.receipt(manager(), receipt).unwrap(),
            Some(receipt.clone())
        );
        assert_eq!(queue.receive(manager(), receipt.clone()).unwrap(), *receipt);
    }
    let mut mismatched = receipts[1].clone();
    mismatched.sequence += 1;
    assert_eq!(
        queue.receive(manager(), mismatched.clone()),
        Err(Error::ReceiptMismatch)
    );
    // Queries return evidence by logical key; the sender checks its full envelope.
    assert_eq!(
        queue.receipt(manager(), &mismatched).unwrap(),
        Some(receipts[1].clone())
    );
    let mut absent = receipts[0].clone();
    absent.job = 99;
    assert_eq!(queue.receipt(manager(), &absent).unwrap(), None);
    assert_eq!(
        queue.receipt(Principal::anonymous(), &absent),
        Err(Error::Unauthorized)
    );
    assert_eq!(memory.writes.get(), 0);
    assert_eq!(queue.cell.bytes(), before);
    drop(queue);
    let queue = Queue::open(memory, None).unwrap().0;
    assert_eq!(queue.cell.bytes(), before);
    assert_eq!(queue.summary().unwrap().2, 3);
}

#[test]
fn oversized_retained_cell_is_refused_without_replacing_its_payload() {
    let backing = VectorMemory::default();
    let queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
    // Only this corruption fixture bypasses the consumer's bounded writer.
    let memory = queue.cell.into_memory();
    let mut raw = Cell::init(memory, Vec::<u8>::new());
    raw.set(vec![b'x'; MAX_BYTES + 1]);
    let memory = raw.into_memory();
    let mut before = vec![0; ic_memory::STABLE_CELL_HEADER_SIZE + MAX_BYTES + 1];
    memory.read(0, &mut before);
    for initial in [None, Some(config())] {
        assert!(matches!(
            Queue::open(backing.clone(), initial),
            Err(Error::InvalidStorage)
        ));
        let mut after = vec![0; before.len()];
        memory.read(0, &mut after);
        // Bootstrap may update allocation metadata. The refused cell's header
        // and payload must retain their exact bytes across both entry paths.
        assert_eq!(after, before);
    }
}
