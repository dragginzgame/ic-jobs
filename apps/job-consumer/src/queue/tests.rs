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
    queue
        .create(Create {
            timing: Timing::Once(126),
            task: Task::Increment(u64::MAX),
        })
        .unwrap();
    let before = queue.cell.get().clone();
    assert_eq!(queue.dispatch(126), Err(Error::InvalidInput));
    assert_eq!(*queue.cell.get(), before);
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
