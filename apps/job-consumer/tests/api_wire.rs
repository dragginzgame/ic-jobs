//! Frozen pre-Platform API bytes protect the consumer wire boundary.
use ic_platform::{Principal, candid::contract::encode_args};
use jobs_test_consumer::api::{
    Create, Delivery, DeliveryPayload, Error, Init, Status, Summary, Task, Timing, View,
};

#[test]
fn external_contract_matches_pre_platform_wire() {
    let manager = Principal::self_authenticating(b"jobs-manager");
    let receiver = Principal::self_authenticating(b"receiver");
    let payload = DeliveryPayload {
        destination: receiver,
        message: "delivery".into(),
    };
    let bytes = encode_args((
        Init {
            manager,
            delivery_sender: Some(receiver),
        },
        vec![
            Create {
                timing: Timing::Once(0),
                task: Task::Increment(1),
            },
            Create {
                timing: Timing::FixedRate {
                    first_ns: 100,
                    every_ns: 7,
                    catch_up: true,
                },
                task: Task::Deliver(payload.clone()),
            },
            Create {
                timing: Timing::AfterCompletion {
                    first_ns: u64::MAX,
                    every_ns: 1,
                },
                task: Task::Increment(0),
            },
        ],
        [
            Status::Pending { due_ns: u64::MAX },
            Status::Paused,
            Status::Running,
            Status::Uncertain,
            Status::Completed,
            Status::Failed,
            Status::Cancelled,
        ]
        .into_iter()
        .map(|status| View {
            id: u64::MAX,
            task: Task::Increment(2),
            status,
            sequence: 3,
            occurrence: 4,
        })
        .collect::<Vec<_>>(),
        Delivery {
            job: 1,
            occurrence: 2,
            sequence: 3,
            payload,
        },
        Summary {
            next_due_ns: Some(100),
            counter: 3,
            receipts: 1,
            armed: true,
        },
        vec![
            Error::Unauthorized,
            Error::UnknownJob,
            Error::Capacity,
            Error::InvalidInput,
            Error::InvalidStorage,
            Error::Job("job".into()),
            Error::Memory("memory".into()),
            Error::ReceiptMismatch,
        ],
    ))
    .unwrap();
    assert_eq!(bytes, include_bytes!("fixtures/api-pre-platform.didbin"));
}
