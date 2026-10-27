//! Native projection of the consumer-owned management contract.

use ic_testkit::pic::WireCallSpec;
use jobs_test_consumer::{
    api::Init,
    management_wire::{Method, REQUEST_LIMITS, RESPONSE_LIMITS},
};

pub(super) fn init_bytes(config: &Init) -> Result<Vec<u8>, ic_wire::Error> {
    ic_wire::native::encode_request(Method::Init as u64, config, REQUEST_LIMITS)
}

pub(super) const fn control(method: Method) -> WireCallSpec {
    WireCallSpec {
        method: method.name(),
        method_id: method as u64,
        request_limits: REQUEST_LIMITS,
        response_limits: RESPONSE_LIMITS,
    }
}
