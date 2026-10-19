//! Explicit live PocketIC tests. Ordinary CI compiles these without launching a server.

use ic_platform::{
    Principal,
    candid::{
        DecodeError, DecodeLimits,
        contract::{
            ArgumentEncoder, CandidType, encode_args, encode_one, serde::de::DeserializeOwned,
        },
        decode_args,
    },
};
use ic_testkit::pic::{
    CandidCallError, CanisterDiagnosticsRequest, CanisterInstallError, CanisterInstallExt,
    CanisterLogRenderLimits, ErrorCode, InstallSpec, PocketIc, PocketIcBuilder,
    PocketIcDiagnosticsExt, PocketIcInstance, PocketIcStartupConfig, RejectResponse, RetryPolicy,
    advance_and_tick, query_candid_args, tick_until, update_candid_args,
};
use ic_wire::native;
use jobs_test_consumer::{
    api::{Create, Delivery, DeliveryPayload, Error, Init, Status, Summary, Task, Timing, View},
    delivery_wire::{RECEIPT_ID, RECEIVE_ID, REQUEST_LIMITS, RESPONSE_LIMITS},
};
use std::{convert::Infallible, path::PathBuf, task::Poll, time::Duration};

// Replies contain at most eight views with 256-byte messages. Keep wire/header
// and decoder work finite without changing PocketIC transport or execution.
const REPLY_LIMITS: DecodeLimits = DecodeLimits {
    reply_bytes: 64 * 1024,
    decoding_quota: 1_000_000,
    skipping_quota: 10_000,
    type_entries: 128,
    header_bytes: 8 * 1024,
};

struct Fixture {
    pic: PocketIcInstance<PocketIc>,
    wasm: Vec<u8>,
    manager: Principal,
    sender: Principal,
    receiver: Principal,
}

impl Fixture {
    fn new() -> Self {
        let path = PathBuf::from(
            std::env::var_os("IC_JOBS_CONSUMER_WASM")
                .expect("build-consumer must select IC_JOBS_CONSUMER_WASM"),
        );
        let wasm = std::fs::read(path).expect("read selected consumer Wasm");
        let config = PocketIcStartupConfig::from_env(Duration::from_secs(30))
            .expect("use ic-testkit-server run after explicit setup/check");
        let pic = config
            .build_instance(PocketIcBuilder::new().with_application_subnet())
            .expect("bounded PocketIC startup");
        let manager = Principal::self_authenticating(b"jobs-manager");
        let install = |delivery_sender| {
            pic.instance
                .try_create_and_install(InstallSpec {
                    wasm: wasm.clone(),
                    init_bytes: encode_one(Init {
                        manager,
                        delivery_sender,
                    })
                    .unwrap(),
                    cycles: 10_000_000_000_000,
                    install_sender: None,
                    label: Some("jobs-test-consumer".into()),
                })
                .unwrap_or_else(|error| {
                    if let CanisterInstallError::InstallCode { canister_id, .. } = &error {
                        // Diagnostics are subordinate to the original install failure,
                        // including a failed stderr write or collection panic.
                        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            let report = pic.instance.collect_canister_diagnostics(
                                CanisterDiagnosticsRequest {
                                    canister_id: *canister_id,
                                    status_sender: Principal::anonymous(),
                                    log_sender: Principal::anonymous(),
                                    log_limits: CanisterLogRenderLimits::default(),
                                },
                            );
                            eprintln!("install Jobs consumer: {report}");
                        }));
                    }
                    panic!("install Jobs consumer: {error:?}");
                })
        };
        let sender = install(None);
        let receiver = install(Some(sender));
        Self {
            pic,
            wasm,
            manager,
            sender,
            receiver,
        }
    }

    fn update<T: CandidType + DeserializeOwned, A: ArgumentEncoder>(
        &self,
        method: &str,
        args: A,
    ) -> Result<T, Error> {
        self.update_as(self.manager, method, args)
            .expect("PocketIC Candid call")
    }
    fn update_as<T: CandidType + DeserializeOwned, A: ArgumentEncoder>(
        &self,
        caller: Principal,
        method: &str,
        args: A,
    ) -> Result<T, CandidCallError> {
        update_candid_args::<(T,), _>(
            &self.pic.instance,
            self.sender,
            caller,
            method,
            args,
            REPLY_LIMITS,
        )
        .map(|(value,)| value)
    }
    fn query<T: CandidType + DeserializeOwned, A: ArgumentEncoder>(
        &self,
        id: Principal,
        method: &str,
        args: A,
    ) -> Result<T, Error> {
        query_candid_args::<(Result<T, Error>,), _>(
            &self.pic.instance,
            id,
            self.manager,
            method,
            args,
            REPLY_LIMITS,
        )
        .expect("PocketIC Candid call")
        .0
    }
    fn view(&self, id: u64) -> View {
        self.query(self.sender, "inspect", (id,)).unwrap()
    }
    fn summary(&self, id: Principal) -> Summary {
        self.query(id, "summary", ()).unwrap()
    }
    fn upgrade(&self, id: Principal) -> Result<(), RejectResponse> {
        self.pic
            .instance
            .upgrade_canister(id, self.wasm.clone(), encode_args(()).unwrap(), None)
    }
    fn until(&self, predicate: impl Fn() -> bool) {
        tick_until(&self.pic.instance, 30, Duration::ZERO, |_| {
            if predicate() {
                Poll::Ready(Ok::<_, Infallible>(()))
            } else {
                Poll::Pending
            }
        })
        .expect("bounded canister progress did not complete");
    }
}

fn assert_canister_trap(error: CandidCallError) {
    assert!(
        matches!(
            &error,
            CandidCallError::CanisterReject { response, .. }
                if response.error_code == ErrorCode::CanisterCalledTrap
        ),
        "expected canister trap: {error:?}"
    );
}

fn increment(at: u64, amount: u64) -> Create {
    Create {
        timing: Timing::Once(at),
        task: Task::Increment(amount),
    }
}

#[test]
fn reply_limits_admit_the_largest_consumer_page() {
    assert!(
        decode_args::<(Result<Vec<View>, Error>,)>(
            include_bytes!("fixtures/pending-page-0162.didbin"),
            REPLY_LIMITS,
        )
        .is_err()
    );
    let views = (0..8)
        .map(|id| View {
            id,
            task: Task::Deliver(DeliveryPayload {
                destination: Principal::self_authenticating(b"receiver"),
                message: "x".repeat(256),
            }),
            status: Status::Pending { due_ns: u64::MAX },
            sequence: u64::MAX,
            occurrence: u64::MAX,
        })
        .collect::<Vec<_>>();
    let bytes = encode_one(Ok::<_, Error>(&views)).unwrap();
    let (decoded,): (Result<Vec<View>, Error>,) = decode_args(&bytes, REPLY_LIMITS).unwrap();
    assert_eq!(decoded.unwrap(), views);
    // The tuple is the wire reply arity; the application Result stays inside it.
    let bytes = encode_one(Err::<Vec<View>, _>(Error::Unauthorized)).unwrap();
    let (decoded,): (Result<Vec<View>, Error>,) = decode_args(&bytes, REPLY_LIMITS).unwrap();
    assert_eq!(decoded, Err(Error::Unauthorized));
    let bytes = encode_args((Ok::<_, Error>(&views), 7_u64)).unwrap();
    let (decoded,): (Result<Vec<View>, Error>,) = decode_args(&bytes, REPLY_LIMITS).unwrap();
    assert_eq!(decoded.unwrap(), views);
    let oversized = vec![0; REPLY_LIMITS.reply_bytes + 1];
    assert!(matches!(
        decode_args::<((),)>(&oversized, REPLY_LIMITS),
        Err(DecodeError::ReplyTooLarge { .. })
    ));
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn delivery_wire_endpoints_refuse_candid_and_preserve_authorized_receipts() {
    let fixture = Fixture::new();
    let delivery = Delivery {
        job: 99,
        occurrence: 0,
        sequence: 1,
        payload: DeliveryPayload {
            destination: fixture.receiver,
            message: "x".repeat(256),
        },
    };
    for (method, id, other) in [
        ("receive_delivery", RECEIVE_ID, RECEIPT_ID),
        ("delivery_receipt", RECEIPT_ID, RECEIVE_ID),
    ] {
        for bytes in [
            encode_one(&delivery).unwrap(),
            native::encode_request(other, &delivery, REQUEST_LIMITS).unwrap(),
            native::encode_response(id, &delivery, REQUEST_LIMITS).unwrap(),
        ] {
            let error = fixture
                .pic
                .instance
                .update_call(fixture.receiver, fixture.sender, method, bytes)
                .unwrap_err();
            assert_eq!(error.error_code, ErrorCode::CanisterCalledTrap);
            assert_eq!(fixture.summary(fixture.receiver).receipts, 0);
        }
        let error = fixture
            .pic
            .instance
            .update_call(
                fixture.receiver,
                fixture.sender,
                method,
                vec![0; REQUEST_LIMITS.bytes + 1],
            )
            .unwrap_err();
        assert_eq!(error.error_code, ErrorCode::CanisterRejectedMessage);
        assert_eq!(fixture.summary(fixture.receiver).receipts, 0);
    }
    let request = native::encode_request(RECEIVE_ID, &delivery, REQUEST_LIMITS).unwrap();
    let bytes = fixture
        .pic
        .instance
        .update_call(
            fixture.receiver,
            Principal::anonymous(),
            "receive_delivery",
            request.clone(),
        )
        .unwrap();
    assert_eq!(
        native::decode_response::<Result<Delivery, Error>>(RECEIVE_ID, &bytes, RESPONSE_LIMITS)
            .unwrap(),
        Err(Error::Unauthorized)
    );
    assert_eq!(fixture.summary(fixture.receiver).receipts, 0);
    for _ in 0..2 {
        let bytes = fixture
            .pic
            .instance
            .update_call(
                fixture.receiver,
                fixture.sender,
                "receive_delivery",
                request.clone(),
            )
            .unwrap();
        assert_eq!(
            native::decode_response::<Result<Delivery, Error>>(RECEIVE_ID, &bytes, RESPONSE_LIMITS)
                .unwrap(),
            Ok(delivery.clone())
        );
        assert_eq!(fixture.summary(fixture.receiver).receipts, 1);
    }
    fixture.upgrade(fixture.receiver).unwrap();
    let request = native::encode_request(RECEIPT_ID, &delivery, REQUEST_LIMITS).unwrap();
    let bytes = fixture
        .pic
        .instance
        .update_call(
            fixture.receiver,
            fixture.sender,
            "delivery_receipt",
            request,
        )
        .unwrap();
    assert_eq!(
        native::decode_response::<Result<Option<Delivery>, Error>>(
            RECEIPT_ID,
            &bytes,
            RESPONSE_LIMITS
        )
        .unwrap(),
        Ok(Some(delivery))
    );
    assert_eq!(fixture.summary(fixture.receiver).receipts, 1);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn management_upgrade_and_real_watchdog_delivery() {
    let fixture = Fixture::new();
    let now = fixture.pic.instance.get_time().as_nanos_since_unix_epoch();
    let unauthorized: Result<u64, Error> = fixture
        .update_as(
            Principal::anonymous(),
            "create",
            (increment(now + 20_000_000_000, 5),),
        )
        .expect("PocketIC Candid call");
    assert_eq!(unauthorized, Err(Error::Unauthorized));
    let first: u64 = fixture
        .update("create", (increment(now + 20_000_000_000, 5),))
        .unwrap();
    let second: u64 = fixture
        .update("create", (increment(now + 30_000_000_000, 7),))
        .unwrap();
    let before = fixture.view(second);
    assert!(fixture.summary(fixture.sender).armed);
    fixture.upgrade(fixture.sender).unwrap();
    assert_eq!(fixture.view(second), before);
    assert_eq!(
        fixture.summary(fixture.sender).next_due_ns,
        Some(now + 20_000_000_000)
    );
    fixture.update::<(), _>("cancel", (first,)).unwrap();
    assert_eq!(
        fixture.summary(fixture.sender).next_due_ns,
        Some(now + 30_000_000_000)
    );
    let page: Vec<View> = fixture
        .query(fixture.sender, "list", (Some(first), 1_u8))
        .unwrap();
    assert_eq!(page, vec![before]);
    fixture.pic.instance.advance_time(Duration::from_secs(40));
    fixture.until(|| fixture.view(second).status == Status::Completed);
    assert_eq!(fixture.view(first).status, Status::Cancelled);
    assert_eq!(fixture.summary(fixture.sender).counter, 7);
    assert!(!fixture.summary(fixture.sender).armed);
    let third: u64 = fixture
        .update(
            "create",
            (Create {
                timing: Timing::Once(
                    fixture.pic.instance.get_time().as_nanos_since_unix_epoch() + 10_000_000_000,
                ),
                task: Task::Deliver(DeliveryPayload {
                    destination: fixture.receiver,
                    message: "timer delivery".into(),
                }),
            },),
        )
        .unwrap();
    fixture.upgrade(fixture.sender).unwrap();
    fixture.pic.instance.advance_time(Duration::from_secs(15));
    fixture.until(|| fixture.view(third).status == Status::Completed);
    assert_eq!(fixture.summary(fixture.receiver).receipts, 1);
    fixture.upgrade(fixture.sender).unwrap();
    fixture.upgrade(fixture.receiver).unwrap();
    advance_and_tick(&fixture.pic.instance, Duration::ZERO, 3);
    assert_eq!(fixture.summary(fixture.receiver).receipts, 1);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn synchronous_trap_rolls_back_storage_identity_and_timer_changes() {
    let fixture = Fixture::new();
    let id: u64 = fixture
        .update(
            "create",
            (increment(
                fixture.pic.instance.get_time().as_nanos_since_unix_epoch() + 60_000_000_000,
                3,
            ),),
        )
        .unwrap();
    let before = fixture.summary(fixture.sender);
    let failed = fixture.update_as::<(), _>(
        fixture.manager,
        "fixture_trap_after_create",
        (increment(
            fixture.pic.instance.get_time().as_nanos_since_unix_epoch() + 30_000_000_000,
            9,
        ),),
    );
    assert_canister_trap(failed.unwrap_err());
    assert_eq!(fixture.summary(fixture.sender), before);
    let page: Vec<View> = fixture
        .query(fixture.sender, "list", (None::<u64>, 8_u8))
        .unwrap();
    assert_eq!(page.len(), 1);
    fixture.upgrade(fixture.sender).unwrap();
    assert_eq!(fixture.summary(fixture.sender), before);
    let next: u64 = fixture
        .update(
            "create",
            (increment(
                fixture.pic.instance.get_time().as_nanos_since_unix_epoch() + 120_000_000_000,
                4,
            ),),
        )
        .unwrap();
    assert_eq!(next, id + 1);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn watchdog_batch_trap_rolls_back_then_recovers_without_repeating_local_effects() {
    let fixture = Fixture::new();
    let due = fixture.pic.instance.get_time().as_nanos_since_unix_epoch() + 10_000_000_000;
    let healthy: u64 = fixture.update("create", (increment(due, 1),)).unwrap();
    let failed: u64 = fixture
        .update("create", (increment(due, u64::MAX),))
        .unwrap();
    let later_due = due + 30_000_000_000;
    let later: u64 = fixture
        .update("create", (increment(later_due, 4),))
        .unwrap();
    let healthy_before = fixture.view(healthy);
    let failed_before = fixture.view(failed);
    fixture
        .update::<(), _>("fixture_trap_watchdog_batch", (true,))
        .unwrap();
    fixture.pic.instance.advance_time(Duration::from_secs(10));
    // Observe a real trap, not merely a dispatched callback awaiting execution.
    let marker = b"fixture: watchdog trap after stable batch write";
    fixture.until(|| {
        fixture
            .pic
            .instance
            .fetch_canister_logs(fixture.sender, Principal::anonymous())
            .unwrap()
            .iter()
            .any(|record| {
                record
                    .content
                    .windows(marker.len())
                    .any(|part| part == marker)
            })
    });
    assert_eq!(fixture.view(healthy), healthy_before);
    assert_eq!(fixture.view(failed), failed_before);
    let rolled_back = fixture.summary(fixture.sender);
    assert_eq!(rolled_back.counter, 0);
    assert_eq!(rolled_back.next_due_ns, Some(due));
    assert!(rolled_back.armed);

    // This control does not rearm the timer: recovery must use its survivor.
    fixture
        .update::<(), _>("fixture_trap_watchdog_batch", (false,))
        .unwrap();
    fixture.pic.instance.advance_time(Duration::from_secs(2));
    fixture.until(|| fixture.view(healthy).status == Status::Completed);
    assert_eq!(fixture.view(failed).status, Status::Failed);
    assert_eq!(fixture.view(healthy).sequence, 1);
    assert_eq!(fixture.view(failed).sequence, 1);
    let recovered = fixture.summary(fixture.sender);
    assert_eq!(recovered.counter, 1);
    assert_eq!(recovered.next_due_ns, Some(later_due));
    assert!(recovered.armed);
    assert_eq!(
        fixture.view(later).status,
        Status::Pending { due_ns: later_due }
    );

    fixture.pic.instance.advance_time(Duration::from_secs(40));
    fixture.until(|| fixture.view(later).status == Status::Completed);
    assert_eq!(fixture.summary(fixture.sender).counter, 5);
    assert!(!fixture.summary(fixture.sender).armed);
    fixture.upgrade(fixture.sender).unwrap();
    advance_and_tick(&fixture.pic.instance, Duration::from_secs(3), 12);
    assert_eq!(fixture.summary(fixture.sender).counter, 5);
    assert_eq!(fixture.view(failed).status, Status::Failed);
    assert!(!fixture.summary(fixture.sender).armed);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn lost_result_keeps_intent_blocked_and_receipt_resolution_does_not_redeliver() {
    let fixture = Fixture::new();
    let input = Create {
        timing: Timing::Once(0),
        task: Task::Deliver(DeliveryPayload {
            destination: fixture.receiver,
            message: "retained external effect".into(),
        }),
    };
    let failed = fixture.update_as::<Result<(), Error>, _>(
        fixture.manager,
        "fixture_dispatch_trap_result",
        (input,),
    );
    assert_canister_trap(failed.unwrap_err());
    let page: Vec<View> = fixture
        .query(fixture.sender, "list", (None::<u64>, 8_u8))
        .unwrap();
    let id = page[0].id;
    assert_eq!(page[0].status, Status::Running);
    assert_eq!(fixture.summary(fixture.receiver).receipts, 1);
    fixture.upgrade(fixture.sender).unwrap();
    fixture.upgrade(fixture.receiver).unwrap();
    fixture.update::<(), _>("dispatch_due", ()).unwrap();
    assert_eq!(fixture.view(id).status, Status::Running);
    fixture
        .update::<(), _>("fixture_mark_uncertain", (id,))
        .unwrap();
    fixture.upgrade(fixture.sender).unwrap();
    assert_eq!(fixture.view(id).status, Status::Uncertain);
    assert!(
        fixture
            .update::<bool, _>("reconcile_delivery", (id,))
            .unwrap()
    );
    assert_eq!(fixture.view(id).status, Status::Completed);
    fixture.update::<(), _>("dispatch_due", ()).unwrap();
    assert_eq!(fixture.summary(fixture.receiver).receipts, 1);
    let absent: u64 = fixture
        .update(
            "fixture_create_blocked",
            (Create {
                timing: Timing::Once(0),
                task: Task::Deliver(DeliveryPayload {
                    destination: fixture.receiver,
                    message: "never sent".into(),
                }),
            },),
        )
        .unwrap();
    fixture.upgrade(fixture.sender).unwrap();
    assert!(
        !fixture
            .update::<bool, _>("reconcile_delivery", (absent,))
            .unwrap()
    );
    assert_eq!(fixture.view(absent).status, Status::Running);
    assert!(fixture.update::<(), _>("cancel", (absent,)).is_err());
    for state in [Status::Running, Status::Uncertain] {
        if state == Status::Uncertain {
            fixture
                .update::<(), _>("fixture_mark_uncertain", (absent,))
                .unwrap();
        }
        let before = fixture.view(absent);
        assert_eq!(before.status, state);
        for method in ["pause", "resume"] {
            assert!(fixture.update::<(), _>(method, (absent,)).is_err());
        }
        assert!(
            fixture
                .update::<(), _>(
                    "reschedule",
                    (
                        absent,
                        Timing::Once(fixture.pic.instance.get_time().as_nanos_since_unix_epoch())
                    )
                )
                .is_err()
        );
        assert_eq!(fixture.view(absent), before);
    }
    assert_eq!(fixture.summary(fixture.receiver).receipts, 1);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn invalid_restore_refuses_upgrade_and_preserves_the_controller_snapshot() {
    let fixture = Fixture::new();
    let id: u64 = fixture
        .update(
            "create",
            (increment(
                fixture.pic.instance.get_time().as_nanos_since_unix_epoch() + 3_600_000_000_000,
                1,
            ),),
        )
        .unwrap();
    // The controller's independent snapshot remains the explicit recovery owner.
    fixture
        .pic
        .instance
        .stop_canister(fixture.sender, None)
        .unwrap();
    let backup = fixture
        .pic
        .instance
        .take_canister_snapshot(fixture.sender, None, None)
        .unwrap();
    fixture
        .pic
        .instance
        .start_canister(fixture.sender, None)
        .unwrap();
    fixture
        .update::<(), _>("fixture_corrupt_record", (id,))
        .unwrap();
    assert!(matches!(
        fixture
            .query::<View, _>(fixture.sender, "inspect", (id,))
            .unwrap_err(),
        Error::InvalidStorage
    ));
    let rejected = fixture.upgrade(fixture.sender).unwrap_err();
    assert_eq!(rejected.error_code, ErrorCode::CanisterCalledTrap);
    fixture
        .pic
        .instance
        .stop_canister(fixture.sender, None)
        .unwrap();
    fixture
        .pic
        .instance
        .load_canister_snapshot(fixture.sender, None, backup.id)
        .unwrap();
    fixture
        .pic
        .instance
        .start_canister(fixture.sender, None)
        .unwrap();
    // Snapshot loading consumes installation resources too. Testkit owns the
    // bounded rate-limit retry; every other rejection must fail immediately.
    // The job's one-hour deadline exceeds the maximum thirty-minute cooldown.
    fixture
        .pic
        .instance
        .retry_install_code(
            RetryPolicy {
                max_attempts: std::num::NonZeroUsize::new(4).unwrap(),
                cooldown: Duration::from_secs(600),
            },
            || fixture.upgrade(fixture.sender),
        )
        .unwrap();
    assert!(matches!(fixture.view(id).status, Status::Pending { .. }));
    assert_eq!(fixture.summary(fixture.sender).counter, 0);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn overdue_fixed_rate_work_reconstructs_catch_up_and_skip_policies() {
    let fixture = Fixture::new();
    let first_ns = fixture.pic.instance.get_time().as_nanos_since_unix_epoch() + 10_000_000_000;
    let mut ids = vec![];
    for catch_up in [false, true] {
        let id: u64 = fixture
            .update(
                "create",
                (Create {
                    timing: Timing::FixedRate {
                        first_ns,
                        every_ns: 1_000_000_000,
                        catch_up,
                    },
                    task: Task::Increment(1),
                },),
            )
            .unwrap();
        ids.push(id);
        fixture.update::<(), _>("pause", (id,)).unwrap();
    }
    fixture.upgrade(fixture.sender).unwrap();
    fixture.pic.instance.advance_time(Duration::from_secs(13));
    for id in &ids {
        assert_eq!(fixture.view(*id).status, Status::Paused);
        assert_eq!(fixture.view(*id).occurrence, 0);
    }
    assert_eq!(fixture.summary(fixture.sender).counter, 0);
    assert!(!fixture.summary(fixture.sender).armed);
    fixture.upgrade(fixture.sender).unwrap();
    for id in &ids {
        fixture.update::<(), _>("resume", (*id,)).unwrap();
    }
    fixture.until(|| fixture.view(ids[0]).occurrence == 1 && fixture.view(ids[1]).occurrence >= 1);
    let Status::Pending { due_ns } = fixture.view(ids[0]).status else {
        panic!("overdue occurrence must remain pending");
    };
    assert!(due_ns > fixture.pic.instance.get_time().as_nanos_since_unix_epoch());
    for id in ids {
        fixture.update::<(), _>("cancel", (id,)).unwrap();
    }
    assert!(!fixture.summary(fixture.sender).armed);
    let before = fixture.summary(fixture.sender).counter;
    advance_and_tick(&fixture.pic.instance, Duration::from_secs(30), 3);
    assert_eq!(fixture.summary(fixture.sender).counter, before);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn durable_management_reconstructs_wakeups_and_traps_roll_back_changes() {
    let fixture = Fixture::new();
    let id: u64 = fixture
        .update(
            "create",
            (increment(
                fixture.pic.instance.get_time().as_nanos_since_unix_epoch() + 30_000_000_000,
                5,
            ),),
        )
        .unwrap();
    for method in ["pause", "resume"] {
        let denied: Result<(), Error> = fixture
            .update_as(Principal::anonymous(), method, (id,))
            .expect("PocketIC Candid call");
        assert_eq!(denied, Err(Error::Unauthorized));
    }
    let denied: Result<(), Error> = fixture
        .update_as(Principal::anonymous(), "reschedule", (id, Timing::Once(0)))
        .expect("PocketIC Candid call");
    assert_eq!(denied, Err(Error::Unauthorized));
    fixture.update::<(), _>("pause", (id,)).unwrap();
    assert_eq!(fixture.view(id).status, Status::Paused);
    assert_eq!(fixture.summary(fixture.sender).next_due_ns, None);
    assert!(!fixture.summary(fixture.sender).armed);
    fixture.upgrade(fixture.sender).unwrap();
    advance_and_tick(&fixture.pic.instance, Duration::from_secs(40), 3);
    assert_eq!(fixture.summary(fixture.sender).counter, 0);
    let later = fixture.pic.instance.get_time().as_nanos_since_unix_epoch() + 100_000_000_000;
    fixture
        .update::<(), _>("reschedule", (id, Timing::Once(later)))
        .unwrap();
    let paused = fixture.view(id);
    assert_eq!(paused.status, Status::Paused);
    assert_eq!((paused.sequence, paused.occurrence), (0, 0));
    fixture.upgrade(fixture.sender).unwrap();
    assert_eq!(fixture.view(id), paused);
    fixture.update::<(), _>("resume", (id,)).unwrap();
    assert_eq!(fixture.summary(fixture.sender).next_due_ns, Some(later));
    assert!(fixture.summary(fixture.sender).armed);
    let due = fixture.pic.instance.get_time().as_nanos_since_unix_epoch() + 15_000_000_000;
    fixture
        .update::<(), _>("reschedule", (id, Timing::Once(due)))
        .unwrap();
    let before = fixture.view(id);
    let summary = fixture.summary(fixture.sender);
    let failed = fixture.update_as::<(), _>(
        fixture.manager,
        "fixture_trap_after_reschedule",
        (
            id,
            Timing::Once(
                fixture.pic.instance.get_time().as_nanos_since_unix_epoch() + 5_000_000_000,
            ),
        ),
    );
    assert_canister_trap(failed.unwrap_err());
    assert_eq!(fixture.view(id), before);
    assert_eq!(fixture.summary(fixture.sender), summary);
    let failed = fixture.update_as::<(), _>(fixture.manager, "fixture_trap_after_pause", (id,));
    assert_canister_trap(failed.unwrap_err());
    assert_eq!(fixture.view(id), before);
    assert_eq!(fixture.summary(fixture.sender), summary);
    fixture.upgrade(fixture.sender).unwrap();
    fixture.pic.instance.advance_time(Duration::from_secs(20));
    fixture.until(|| fixture.view(id).status == Status::Completed);
    assert_eq!(fixture.summary(fixture.sender).counter, 5);
    assert_eq!(fixture.view(id).sequence, 1);
    assert!(!fixture.summary(fixture.sender).armed);
}
