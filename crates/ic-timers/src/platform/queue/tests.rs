use super::*;

#[test]
fn deadlines_reject_both_duration_and_addition_overflow() {
    use std::time::Duration;
    assert_eq!(deadline(0, Duration::ZERO), Ok(0));
    assert_eq!(deadline(u64::MAX, Duration::ZERO), Ok(u64::MAX));
    assert_eq!(
        deadline(u64::MAX, Duration::from_nanos(1)),
        Err(AdmissionError::DeadlineOverflow)
    );
    assert_eq!(
        deadline(0, Duration::from_secs(u64::MAX)),
        Err(AdmissionError::DeadlineOverflow)
    );
}

#[test]
fn admission_is_atomic_and_identities_never_wrap_or_recur() {
    let mut queue = Queue::<_, 1>::new();
    let first = queue.insert(10, "first").unwrap();
    assert_eq!(
        queue.insert(1, "rejected"),
        Err((AdmissionError::Capacity, "rejected"))
    );
    assert_eq!(queue.next(), Some((10, first)));
    assert_eq!(queue.remove(first), Some("first"));
    let second = queue.insert(1, "second").unwrap();
    assert_ne!(first, second);
    assert_eq!(queue.remove(first), None);
    assert_eq!(queue.remove(second), Some("second"));
    queue.last_id = u64::MAX;
    assert_eq!(
        queue.insert(0, "overflow"),
        Err((AdmissionError::IdentityExhausted, "overflow"))
    );
    assert_eq!(queue.next(), None);
}

#[test]
fn ordering_submission_retry_and_late_acceptance_share_one_task() {
    let mut queue = Queue::<_, 3>::new();
    let first = queue.insert(5, 1).unwrap();
    let second = queue.insert(5, 2).unwrap();
    let third = queue.insert(4, 3).unwrap();
    queue.submit(third);
    assert_eq!(queue.next(), Some((5, first)));
    queue.submit(first);
    assert_eq!(queue.next(), Some((5, second)));
    queue.retry(third, 6);
    assert_eq!(queue.remove(third), Some(3)); // late request wins once
    queue.retry(third, 7); // its transport response cannot resurrect it
    assert_eq!(queue.remove(third), None);
    assert_eq!(queue.remove(first), Some(1));
    assert_eq!(queue.remove(second), Some(2));
    assert_eq!(queue.next(), None);
}

#[test]
fn cancellation_churn_retains_no_deadline_records() {
    let mut queue = Queue::<_, 2>::new();
    let survivor = queue.insert(20, ()).unwrap();
    for _ in 0..10_000 {
        let id = queue.insert(u64::MAX, ()).unwrap();
        assert_eq!(queue.remove(id), Some(()));
        assert_eq!(queue.entries.len(), 1);
        assert_eq!(queue.next(), Some((20, survivor)));
    }
}

#[test]
fn transport_interleavings_release_each_capture_without_resurrection() {
    use std::rc::Rc;

    // Exhaust every five-event trace over two tasks: submit, retry, and terminal
    // removal (delivery or cancellation). Inspect ownership after every event,
    // including responses arriving after removal and repeated terminal events.
    for trace in 0_u32..6_u32.pow(5) {
        let captures = [Rc::new(()), Rc::new(())];
        let mut queue = Queue::<_, 2>::new();
        let ids = [
            queue.insert(2, Rc::clone(&captures[0])).unwrap(),
            queue.insert(1, Rc::clone(&captures[1])).unwrap(),
        ];
        let mut live = [true; 2];
        let mut submitted = [false; 2];
        let mut deadlines = [2, 1];
        let mut remaining = trace;
        for step in 0..5 {
            let event = remaining % 6;
            remaining /= 6;
            let index = usize::try_from(event % 2).unwrap();
            match event / 2 {
                0 => {
                    queue.submit(ids[index]);
                    submitted[index] = true;
                }
                1 => {
                    let deadline = 10 + step;
                    queue.retry(ids[index], deadline);
                    submitted[index] = false;
                    deadlines[index] = deadline;
                }
                _ => {
                    let removed = queue.remove(ids[index]);
                    assert_eq!(removed.is_some(), live[index], "trace {trace}, step {step}");
                    if let Some(capture) = removed {
                        assert!(Rc::ptr_eq(&capture, &captures[index]));
                    }
                    live[index] = false;
                }
            }
            let expected = (0..2)
                .filter(|&index| live[index] && !submitted[index])
                .map(|index| (deadlines[index], ids[index]))
                .min();
            assert_eq!(queue.next(), expected, "trace {trace}, step {step}");
            for index in 0..2 {
                assert_eq!(
                    Rc::strong_count(&captures[index]),
                    1 + usize::from(live[index])
                );
            }
        }
        drop(queue);
        assert!(
            captures
                .iter()
                .all(|capture| Rc::strong_count(capture) == 1)
        );
    }
}

#[test]
fn old_transport_events_cannot_claim_replacement_captures() {
    use std::rc::Rc;

    let mut queue = Queue::<_, 1>::new();
    let old_capture = Rc::new(());
    let old = queue.insert(0, Rc::clone(&old_capture)).unwrap();
    queue.submit(old);
    drop(queue.remove(old));
    assert_eq!(Rc::strong_count(&old_capture), 1);

    let replacement_capture = Rc::new(());
    let replacement = queue.insert(1, Rc::clone(&replacement_capture)).unwrap();
    for _ in 0..16 {
        queue.retry(old, 0);
        queue.submit(old);
        assert!(queue.remove(old).is_none());
        assert_eq!(queue.next(), Some((1, replacement)));
        assert_eq!(Rc::strong_count(&replacement_capture), 2);
    }
    assert!(Rc::ptr_eq(
        &queue.remove(replacement).unwrap(),
        &replacement_capture
    ));
    assert_eq!(Rc::strong_count(&replacement_capture), 1);
}
