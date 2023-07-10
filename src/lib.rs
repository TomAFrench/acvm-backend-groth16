#![warn(unused_crate_dependencies, unused_extern_crates)]
#![warn(unreachable_pub)]

mod common_reference_string;
mod proof_system;
mod pwg;
mod smart_contract;

#[derive(Debug, thiserror::Error)]
#[error("Error in acvm-backend-groth16")]
pub struct BackendError;

#[derive(Default, Debug)]
pub struct ArkworksGroth16;

impl acvm::Backend for ArkworksGroth16 {}
