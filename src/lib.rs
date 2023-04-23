#![warn(unused_crate_dependencies, unused_extern_crates)]
#![warn(unreachable_pub)]

mod proof_system;
mod pwg;
mod smart_contract;

#[derive(Default)]
pub struct ArkworksGroth16;

impl acvm::Backend for ArkworksGroth16 {}
