//! Native projection of the consumer-owned management contract.

use ic_platform::{pocket_ic::PocketIc, principal::ExternalPrincipal};
use ic_testkit::pic::{WireCallError, WireCallSpec, query_wire, update_wire};
use jobs_test_consumer::{
    api::Init,
    management_wire::{Method, REQUEST_LIMITS, RESPONSE_LIMITS},
};
use serde::{Serialize, de::DeserializeOwned};

// Transport and the management contract converge here. Each caller still owns
// application-error handling, test assertions and server/canister lifetimes.
/// Native calls against one installed qualification consumer.
pub struct Client<'a> {
    /// PocketIC instance owned by the caller.
    pub pic: &'a PocketIc,
    /// Installed qualification canister.
    pub canister: ExternalPrincipal,
    /// Identity used for the management call.
    pub caller: ExternalPrincipal,
}

impl Client<'_> {
    /// Query a consumer-owned management endpoint.
    pub fn query<T: DeserializeOwned>(
        &self,
        method: Method,
        args: &impl Serialize,
    ) -> Result<T, WireCallError> {
        query_wire(self.pic, self.canister, self.caller, control(method), args)
    }

    /// Update a consumer-owned management endpoint.
    pub fn update<T: DeserializeOwned>(
        &self,
        method: Method,
        args: &impl Serialize,
    ) -> Result<T, WireCallError> {
        update_wire(self.pic, self.canister, self.caller, control(method), args)
    }
}

/// Encode the consumer installation configuration.
pub fn init_bytes(config: &Init) -> Result<Vec<u8>, ic_wire::Error> {
    ic_wire::native::encode_request(Method::Init as u64, config, REQUEST_LIMITS)
}

/// Bounded Wire identity for one management endpoint.
pub const fn control(method: Method) -> WireCallSpec {
    WireCallSpec {
        method: method.name(),
        method_id: method as u64,
        request_limits: REQUEST_LIMITS,
        response_limits: RESPONSE_LIMITS,
    }
}
