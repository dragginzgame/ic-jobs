use candid::CandidType;
use serde::de::DeserializeOwned;

use crate::api::Error;

// Decode only. Callers own exact evidence comparison and uncertain-effect policy.
pub(crate) fn decode_reply<T, E>(response: Result<impl AsRef<[u8]>, E>) -> Option<T>
where
    T: CandidType + DeserializeOwned,
{
    candid::decode_one::<Result<T, Error>>(response.ok()?.as_ref())
        .ok()?
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::Delivery;
    use candid::Principal;

    #[test]
    fn reply_decoding_preserves_evidence_and_distinguishes_absent_receipts() {
        let receipt = Delivery {
            job: 1,
            occurrence: 0,
            sequence: 2,
            destination: Principal::anonymous(),
            message: "exact evidence".into(),
        };
        let bytes = candid::encode_one(Ok::<_, Error>(&receipt)).unwrap();
        assert_eq!(
            decode_reply::<Delivery, ()>(Ok(bytes)),
            Some(receipt.clone())
        );
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
