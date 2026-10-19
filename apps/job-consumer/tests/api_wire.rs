//! Current management Wire contract and refusal of frozen pre-cut Candid bytes.
use ic_platform::Principal;
use ic_wire::native;
use jobs_test_consumer::{
    api::{Create, Init, Task, Timing},
    management_wire::{Method, REQUEST_LIMITS, RESPONSE_LIMITS},
};

#[test]
fn management_contract_admits_current_requests_and_refuses_retired_candid() {
    let init = Init {
        manager: Principal::anonymous(),
        delivery_sender: None,
    };
    let bytes = native::encode_request(Method::Init as u64, &init, REQUEST_LIMITS).unwrap();
    assert_eq!(
        native::decode_request::<Init>(Method::Init as u64, &bytes, REQUEST_LIMITS).unwrap(),
        init
    );
    assert!(native::decode_request::<Init>(Method::Create as u64, &bytes, REQUEST_LIMITS).is_err());
    assert!(
        native::decode_request::<Init>(
            Method::Init as u64,
            include_bytes!("fixtures/api-pre-platform.didbin"),
            REQUEST_LIMITS
        )
        .is_err()
    );

    let create = Create {
        timing: Timing::Once(100),
        task: Task::Increment(7),
    };
    // Platform decodes one argument directly; the native harness supplies its
    // singleton argument tuple. Both encode the same owned request shape.
    let bytes = native::encode_request(Method::Create as u64, &(create,), REQUEST_LIMITS).unwrap();
    let decoded: Create =
        native::decode_request(Method::Create as u64, &bytes, REQUEST_LIMITS).unwrap();
    assert!(matches!(decoded.timing, Timing::Once(100)));
    assert_eq!(decoded.task, Task::Increment(7));

    let bytes =
        native::encode_request(Method::List as u64, &(Some(4_u64), 8_u8), REQUEST_LIMITS).unwrap();
    assert_eq!(
        native::decode_request::<(Option<u64>, u8)>(Method::List as u64, &bytes, REQUEST_LIMITS)
            .unwrap(),
        (Some(4), 8)
    );
    let bytes = native::encode_request(Method::DispatchDue as u64, &(), REQUEST_LIMITS).unwrap();
    assert!(
        native::decode_request::<()>(Method::DispatchDue as u64, &bytes, REQUEST_LIMITS).is_ok()
    );
    let mut trailing = bytes;
    trailing.push(0);
    assert!(
        native::decode_request::<()>(Method::DispatchDue as u64, &trailing, REQUEST_LIMITS)
            .is_err()
    );
    let response =
        native::encode_response(Method::DispatchDue as u64, &(), RESPONSE_LIMITS).unwrap();
    assert!(
        native::decode_request::<()>(Method::DispatchDue as u64, &response, REQUEST_LIMITS)
            .is_err()
    );
}
