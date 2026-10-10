//! Explicit live PocketIC tests. Ordinary CI compiles these without launching a server.

use candid::{CandidType, Principal, utils::ArgumentEncoder};
use ic_testkit::pic::{
    CandidCallExt, CanisterInstallExt, ErrorCode, PocketIc, PocketIcBuilder, PocketIcBuilderExt,
    PocketIcStartupConfig, RetryPolicy,
};
use jobs_test_consumer::api::{Create, Error, Init, Status, Summary, Task, Timing, View};
use serde::de::DeserializeOwned;
use std::{path::PathBuf, time::Duration};

struct Fixture {
    pic: PocketIc,
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
        let pic = PocketIcBuilder::new()
            .with_application_subnet()
            .try_build(config)
            .expect("bounded PocketIC startup");
        let manager = Principal::self_authenticating(b"jobs-manager");
        let sender = pic.create_canister();
        let receiver = pic.create_canister();
        for (id, delivery_sender) in [(sender, None), (receiver, Some(sender))] {
            pic.add_cycles(id, 10_000_000_000_000);
            pic.install_canister(
                id,
                wasm.clone(),
                candid::encode_one(Init {
                    manager,
                    delivery_sender,
                })
                .unwrap(),
                None,
            );
        }
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
        self.pic
            .update_candid_as_or_panic(self.sender, self.manager, method, args)
    }
    fn query<T: CandidType + DeserializeOwned, A: ArgumentEncoder>(
        &self,
        id: Principal,
        method: &str,
        args: A,
    ) -> Result<T, Error> {
        self.pic
            .query_candid_as_or_panic(id, self.manager, method, args)
    }
    fn view(&self, id: u64) -> View {
        self.query(self.sender, "inspect", (id,)).unwrap()
    }
    fn summary(&self, id: Principal) -> Summary {
        self.query(id, "summary", ()).unwrap()
    }
    fn now(&self) -> u64 {
        self.pic.get_time().as_nanos_since_unix_epoch()
    }
    fn upgrade(&self, id: Principal) {
        self.pic
            .upgrade_canister(
                id,
                self.wasm.clone(),
                candid::encode_args(()).unwrap(),
                None,
            )
            .unwrap();
    }
    fn until(&self, predicate: impl Fn() -> bool) {
        for _ in 0..30 {
            if predicate() {
                return;
            }
            self.pic.tick();
        }
        assert!(predicate(), "bounded canister progress did not complete");
    }
}

fn increment(at: u64, amount: u64) -> Create {
    Create {
        timing: Timing::Once(at),
        task: Task::Increment(amount),
    }
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn management_upgrade_and_real_watchdog_delivery() {
    let fixture = Fixture::new();
    let now = fixture.now();
    let unauthorized: Result<u64, Error> = fixture.pic.update_candid_as_or_panic(
        fixture.sender,
        Principal::anonymous(),
        "create",
        (increment(now + 20_000_000_000, 5),),
    );
    assert_eq!(unauthorized, Err(Error::Unauthorized));
    let first: u64 = fixture
        .update("create", (increment(now + 20_000_000_000, 5),))
        .unwrap();
    let second: u64 = fixture
        .update("create", (increment(now + 30_000_000_000, 7),))
        .unwrap();
    let before = fixture.view(second);
    assert!(fixture.summary(fixture.sender).armed);
    fixture.upgrade(fixture.sender);
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
    fixture.pic.advance_time(Duration::from_secs(40));
    fixture.until(|| fixture.view(second).status == Status::Completed);
    assert_eq!(fixture.view(first).status, Status::Cancelled);
    assert_eq!(fixture.summary(fixture.sender).counter, 7);
    assert!(!fixture.summary(fixture.sender).armed);
    let third: u64 = fixture
        .update(
            "create",
            (Create {
                timing: Timing::Once(fixture.now() + 10_000_000_000),
                task: Task::Deliver {
                    destination: fixture.receiver,
                    message: "timer delivery".into(),
                },
            },),
        )
        .unwrap();
    fixture.upgrade(fixture.sender);
    fixture.pic.advance_time(Duration::from_secs(15));
    fixture.until(|| fixture.view(third).status == Status::Completed);
    assert_eq!(fixture.summary(fixture.receiver).receipts, 1);
    fixture.upgrade(fixture.sender);
    fixture.upgrade(fixture.receiver);
    for _ in 0..3 {
        fixture.pic.tick();
    }
    assert_eq!(fixture.summary(fixture.receiver).receipts, 1);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn synchronous_trap_rolls_back_storage_identity_and_timer_changes() {
    let fixture = Fixture::new();
    let id: u64 = fixture
        .update("create", (increment(fixture.now() + 60_000_000_000, 3),))
        .unwrap();
    let before = fixture.summary(fixture.sender);
    let failed = fixture.pic.update_candid_as::<(), _>(
        fixture.sender,
        fixture.manager,
        "fixture_trap_after_create",
        (increment(fixture.now() + 30_000_000_000, 9),),
    );
    assert_eq!(
        failed.unwrap_err().reject_response().unwrap().error_code,
        ErrorCode::CanisterCalledTrap
    );
    assert_eq!(fixture.summary(fixture.sender), before);
    let page: Vec<View> = fixture
        .query(fixture.sender, "list", (None::<u64>, 8_u8))
        .unwrap();
    assert_eq!(page.len(), 1);
    fixture.upgrade(fixture.sender);
    assert_eq!(fixture.summary(fixture.sender), before);
    let next: u64 = fixture
        .update("create", (increment(fixture.now() + 120_000_000_000, 4),))
        .unwrap();
    assert_eq!(next, id + 1);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn lost_result_keeps_intent_blocked_and_receipt_resolution_does_not_redeliver() {
    let fixture = Fixture::new();
    let input = Create {
        timing: Timing::Once(0),
        task: Task::Deliver {
            destination: fixture.receiver,
            message: "retained external effect".into(),
        },
    };
    let failed = fixture.pic.update_candid_as::<Result<(), Error>, _>(
        fixture.sender,
        fixture.manager,
        "fixture_dispatch_trap_result",
        (input,),
    );
    assert_eq!(
        failed.unwrap_err().reject_response().unwrap().error_code,
        ErrorCode::CanisterCalledTrap
    );
    let page: Vec<View> = fixture
        .query(fixture.sender, "list", (None::<u64>, 8_u8))
        .unwrap();
    let id = page[0].id;
    assert_eq!(page[0].status, Status::Running);
    assert_eq!(fixture.summary(fixture.receiver).receipts, 1);
    fixture.upgrade(fixture.sender);
    fixture.upgrade(fixture.receiver);
    fixture.update::<(), _>("dispatch_due", ()).unwrap();
    assert_eq!(fixture.view(id).status, Status::Running);
    fixture
        .update::<(), _>("fixture_mark_uncertain", (id,))
        .unwrap();
    fixture.upgrade(fixture.sender);
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
                task: Task::Deliver {
                    destination: fixture.receiver,
                    message: "never sent".into(),
                },
            },),
        )
        .unwrap();
    fixture.upgrade(fixture.sender);
    assert!(
        !fixture
            .update::<bool, _>("reconcile_delivery", (absent,))
            .unwrap()
    );
    assert_eq!(fixture.view(absent).status, Status::Running);
    assert!(fixture.update::<(), _>("cancel", (absent,)).is_err());
    assert_eq!(fixture.summary(fixture.receiver).receipts, 1);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn invalid_restore_refuses_upgrade_and_preserves_the_controller_snapshot() {
    let fixture = Fixture::new();
    let id: u64 = fixture
        .update("create", (increment(fixture.now() + 3_600_000_000_000, 1),))
        .unwrap();
    // The controller's independent snapshot remains the explicit recovery owner.
    fixture.pic.stop_canister(fixture.sender, None).unwrap();
    let backup = fixture
        .pic
        .take_canister_snapshot(fixture.sender, None, None)
        .unwrap();
    fixture.pic.start_canister(fixture.sender, None).unwrap();
    fixture
        .update::<(), _>("fixture_corrupt_record", (id,))
        .unwrap();
    assert!(matches!(
        fixture
            .query::<View, _>(fixture.sender, "inspect", (id,))
            .unwrap_err(),
        Error::Job(_)
    ));
    let rejected = fixture
        .pic
        .upgrade_canister(
            fixture.sender,
            fixture.wasm.clone(),
            candid::encode_args(()).unwrap(),
            None,
        )
        .unwrap_err();
    assert_eq!(rejected.error_code, ErrorCode::CanisterCalledTrap);
    fixture.pic.stop_canister(fixture.sender, None).unwrap();
    fixture
        .pic
        .load_canister_snapshot(fixture.sender, None, backup.id)
        .unwrap();
    fixture.pic.start_canister(fixture.sender, None).unwrap();
    // Snapshot loading consumes installation resources too. Testkit owns the
    // bounded rate-limit retry; every other rejection must fail immediately.
    // The job's one-hour deadline exceeds the maximum thirty-minute cooldown.
    fixture
        .pic
        .retry_install_code(
            RetryPolicy::try_new(4, Duration::from_secs(600)).unwrap(),
            || {
                fixture.pic.upgrade_canister(
                    fixture.sender,
                    fixture.wasm.clone(),
                    candid::encode_args(()).unwrap(),
                    None,
                )
            },
        )
        .unwrap();
    assert_eq!(fixture.view(id).status, Status::Pending);
    assert_eq!(fixture.summary(fixture.sender).counter, 0);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn overdue_fixed_rate_work_reconstructs_catch_up_and_skip_policies() {
    let fixture = Fixture::new();
    let first_ns = fixture.now() + 10_000_000_000;
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
    }
    fixture.upgrade(fixture.sender);
    fixture.pic.advance_time(Duration::from_secs(13));
    fixture.until(|| fixture.view(ids[0]).occurrence == 1 && fixture.view(ids[1]).occurrence >= 1);
    assert!(fixture.view(ids[0]).due_ns.unwrap() > fixture.now());
    for id in ids {
        fixture.update::<(), _>("cancel", (id,)).unwrap();
    }
    assert!(!fixture.summary(fixture.sender).armed);
    let before = fixture.summary(fixture.sender).counter;
    fixture.pic.advance_time(Duration::from_secs(30));
    for _ in 0..3 {
        fixture.pic.tick();
    }
    assert_eq!(fixture.summary(fixture.sender).counter, before);
}
