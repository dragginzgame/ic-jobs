use super::*;
use crate::api::{Delivery, DeliveryPayload};
use ic_platform::{Principal, candid::contract::encode_one};
use ic_wire::native;

fn delivery() -> Delivery {
    Delivery {
        job: u64::MAX,
        occurrence: u64::MAX,
        sequence: u64::MAX,
        payload: DeliveryPayload {
            destination: Principal::self_authenticating(b"receiver"),
            message: "x".repeat(256),
        },
    }
}

#[test]
fn requests_preserve_the_envelope_and_reject_other_protocols() {
    let delivery = delivery();
    for (method, other) in [(RECEIVE_ID, RECEIPT_ID), (RECEIPT_ID, RECEIVE_ID)] {
        let bytes = native::encode_request(method, &delivery, REQUEST_LIMITS).unwrap();
        assert_eq!(
            native::decode_request::<Delivery>(method, &bytes, REQUEST_LIMITS).unwrap(),
            delivery
        );
        assert_eq!(
            native::decode_request::<Delivery>(other, &bytes, REQUEST_LIMITS),
            Err(ic_wire::Error::Identity)
        );
        let response = native::encode_response(method, &delivery, REQUEST_LIMITS).unwrap();
        assert_eq!(
            native::decode_request::<Delivery>(method, &response, REQUEST_LIMITS),
            Err(ic_wire::Error::Identity)
        );
        for end in 0..bytes.len() {
            assert!(
                native::decode_request::<Delivery>(method, &bytes[..end], REQUEST_LIMITS).is_err()
            );
        }
        let mut trailing = bytes.clone();
        trailing.push(0);
        assert_eq!(
            native::decode_request::<Delivery>(method, &trailing, REQUEST_LIMITS),
            Err(ic_wire::Error::Trailing)
        );
        let candid = encode_one(&delivery).unwrap();
        assert_eq!(
            native::decode_request::<Delivery>(method, &candid, REQUEST_LIMITS),
            Err(ic_wire::Error::Identity)
        );
        // A redundant zero group decodes to job 1 but is not canonical Postcard.
        let small = Delivery {
            job: 1,
            ..delivery.clone()
        };
        let mut noncanonical = native::encode_request(method, &small, REQUEST_LIMITS).unwrap();
        noncanonical.splice(11..12, [0x81, 0]);
        assert_eq!(
            native::decode_request::<Delivery>(method, &noncanonical, REQUEST_LIMITS),
            Err(ic_wire::Error::Noncanonical)
        );
    }
}

#[test]
fn replies_preserve_exact_evidence_and_distinguish_absence_from_failure() {
    let delivery = delivery();
    let bytes =
        native::encode_response(RECEIVE_ID, &Ok::<_, Error>(&delivery), RESPONSE_LIMITS).unwrap();
    assert_eq!(
        decode_reply::<Delivery, ()>(RECEIVE_ID, Ok(&bytes)),
        Some(delivery.clone())
    );
    assert_eq!(decode_reply::<Delivery, ()>(RECEIPT_ID, Ok(&bytes)), None);
    for end in 0..bytes.len() {
        assert_eq!(
            decode_reply::<Delivery, ()>(RECEIVE_ID, Ok(&bytes[..end])),
            None
        );
    }
    let request = native::encode_request(RECEIVE_ID, &delivery, REQUEST_LIMITS).unwrap();
    assert_eq!(decode_reply::<Delivery, ()>(RECEIVE_ID, Ok(request)), None);
    let mut trailing = bytes;
    trailing.push(0);
    assert_eq!(decode_reply::<Delivery, ()>(RECEIVE_ID, Ok(trailing)), None);

    for value in [Some(delivery.clone()), None] {
        let bytes =
            native::encode_response(RECEIPT_ID, &Ok::<_, Error>(&value), RESPONSE_LIMITS).unwrap();
        assert_eq!(
            decode_reply::<Option<Delivery>, ()>(RECEIPT_ID, Ok(bytes)),
            Some(value)
        );
    }
    for error in [Error::Unauthorized, Error::Memory("refused".into())] {
        let bytes = native::encode_response(
            RECEIPT_ID,
            &Err::<Option<Delivery>, _>(error),
            RESPONSE_LIMITS,
        )
        .unwrap();
        assert_eq!(
            decode_reply::<Option<Delivery>, ()>(RECEIPT_ID, Ok(bytes)),
            None
        );
    }
    // Both the previous nested contract and the older flat envelope are refused.
    let candid = encode_one(Ok::<_, Error>(&delivery)).unwrap();
    assert_eq!(decode_reply::<Delivery, ()>(RECEIVE_ID, Ok(candid)), None);
    assert_eq!(
        decode_reply::<Delivery, ()>(
            RECEIVE_ID,
            Ok(include_bytes!(
                "../reply/fixtures/delivery-reply-015.didbin"
            ))
        ),
        None
    );
    assert_eq!(
        decode_reply::<Delivery, ()>(RECEIVE_ID, Err::<Vec<u8>, _>(())),
        None
    );
    assert_eq!(
        decode_reply::<Delivery, ()>(RECEIVE_ID, Ok(b"malformed")),
        None
    );
}

#[test]
fn limits_bound_frames_allocations_and_traversal_before_result_admission() {
    let mut delivery = delivery();
    let bytes = native::encode_request(RECEIVE_ID, &delivery, REQUEST_LIMITS).unwrap();
    for limits in [
        Limits {
            allocation: 0,
            ..REQUEST_LIMITS
        },
        Limits {
            nodes: 1,
            ..REQUEST_LIMITS
        },
        Limits {
            depth: 1,
            ..REQUEST_LIMITS
        },
    ] {
        assert_eq!(
            native::decode_request::<Delivery>(RECEIVE_ID, &bytes, limits),
            Err(ic_wire::Error::Limit)
        );
    }
    delivery.payload.message = "x".repeat(REQUEST_LIMITS.bytes);
    assert_eq!(
        native::encode_request(RECEIVE_ID, &delivery, REQUEST_LIMITS),
        Err(ic_wire::Error::Limit)
    );
    let bytes = native::encode_request(RECEIVE_ID, &delivery, RESPONSE_LIMITS).unwrap();
    assert_eq!(
        native::decode_request::<Delivery>(RECEIVE_ID, &bytes, REQUEST_LIMITS),
        Err(ic_wire::Error::Limit)
    );
    delivery.payload.message = "x".repeat(RESPONSE_LIMITS.bytes);
    let bytes = native::encode_response(
        RECEIPT_ID,
        &Ok::<_, Error>(Some(delivery)),
        Limits {
            bytes: 16 * 1024,
            allocation: 256 * 1024,
            ..RESPONSE_LIMITS
        },
    )
    .unwrap();
    assert!(bytes.len() > RESPONSE_LIMITS.bytes);
    assert_eq!(
        decode_reply::<Option<Delivery>, ()>(RECEIPT_ID, Ok(bytes)),
        None
    );
}
