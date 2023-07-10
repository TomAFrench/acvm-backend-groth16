use acvm::SmartContract;

use crate::{ArkworksGroth16, BackendError};

impl SmartContract for ArkworksGroth16 {
    type Error = BackendError;

    fn eth_contract_from_vk(
        &self,
        _common_reference_string: &[u8],
        _verification_key: &[u8],
    ) -> Result<String, Self::Error> {
        todo!("acvm-backend-groth16 doesn't support generating smart contract verifiers yet.");
    }
}
