//! Bounded, unpublished Jobs consumer used for canister qualification.
//! Storage, authorization, payloads and lifecycle belong to this application.

pub mod api;
#[cfg(target_arch = "wasm32")]
mod canister;
#[cfg(any(test, target_arch = "wasm32"))]
mod queue;
