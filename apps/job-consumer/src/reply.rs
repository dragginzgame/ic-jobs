use candid::CandidType;
use ic_testkit_candid::{DecodeLimits, decode_one};
use serde::de::DeserializeOwned;

use crate::api::Error;

// Delivery replies contain one bounded envelope (at most 256 message bytes),
// optionally absent. These limits belong to this consumer, separately from the
// host harness's larger management-page allowance.
const REPLY_LIMITS: DecodeLimits = DecodeLimits {
    reply_bytes: 8 * 1024,
    decoding_quota: 100_000,
    skipping_quota: 10_000,
    type_entries: 64,
    header_bytes: 4 * 1024,
};

// Decode only. Callers own exact evidence comparison and uncertain-effect policy.
pub(crate) fn decode_reply<T, E>(response: Result<impl AsRef<[u8]>, E>) -> Option<T>
where
    T: CandidType + DeserializeOwned,
{
    decode_one::<Result<T, Error>>(response.ok()?.as_ref(), REPLY_LIMITS)
        .ok()?
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{Delivery, DeliveryPayload};
    use candid::Principal;

    #[test]
    fn reply_decoding_preserves_evidence_and_distinguishes_absent_receipts() {
        // This previously valid flat envelope grants no result disposition.
        assert_eq!(
            decode_reply::<Delivery, ()>(Ok(include_bytes!(
                "reply/fixtures/delivery-reply-015.didbin"
            ))),
            None
        );
        let receipt = Delivery {
            job: 1,
            occurrence: 0,
            sequence: 2,
            payload: DeliveryPayload {
                destination: Principal::anonymous(),
                message: "x".repeat(256),
            },
        };
        let bytes = candid::encode_one(Ok::<_, Error>(&receipt)).unwrap();
        assert_eq!(
            decode_reply::<Delivery, ()>(Ok(bytes)),
            Some(receipt.clone())
        );
        let oversized = Delivery {
            payload: DeliveryPayload {
                message: "x".repeat(REPLY_LIMITS.reply_bytes),
                ..receipt.payload.clone()
            },
            ..receipt.clone()
        };
        let bytes = candid::encode_one(Ok::<_, Error>(Some(oversized))).unwrap();
        assert!(bytes.len() > REPLY_LIMITS.reply_bytes);
        // An unreadable reply is not an authoritative absent-receipt response.
        assert_eq!(decode_reply::<Option<Delivery>, ()>(Ok(bytes)), None);
        let bytes = candid::encode_one(Ok::<_, Error>(Some(&receipt))).unwrap();
        assert_eq!(
            decode_reply::<Option<Delivery>, ()>(Ok(bytes)),
            Some(Some(receipt))
        );
        let bytes = candid::encode_one(Ok::<Option<Delivery>, Error>(None)).unwrap();
        assert_eq!(decode_reply::<Option<Delivery>, ()>(Ok(bytes)), Some(None));
        let bytes = candid::encode_one(Err::<Delivery, _>(Error::Unauthorized)).unwrap();
        assert_eq!(decode_reply::<Delivery, ()>(Ok(bytes)), None);
        assert_eq!(decode_reply::<Delivery, ()>(Err::<Vec<u8>, _>(())), None);
        assert_eq!(decode_reply::<Delivery, ()>(Ok(b"malformed")), None);
    }
}
