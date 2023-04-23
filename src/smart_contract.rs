use acvm::SmartContract;

use crate::ArkworksGroth16;

impl SmartContract for ArkworksGroth16 {
    fn eth_contract_from_vk(&self, _verification_key: &[u8]) -> String {
        todo!("acvm-backend-groth16 doesn't support generating smart contract verifiers yet.");
    }
}
