use acvm::pwg::OpcodeResolutionError;
use acvm::{BlackBoxFunctionSolver, FieldElement};

use crate::ArkworksGroth16;

impl BlackBoxFunctionSolver for ArkworksGroth16 {
    fn schnorr_verify(
        &self,
        _public_key_x: &FieldElement,
        _public_key_y: &FieldElement,
        _signature: &[u8],
        _message: &[u8],
    ) -> Result<bool, OpcodeResolutionError> {
        unreachable!("Groth16 does not support black box functions. This code is only reachable if executing ACIR for a different backend");
    }

    fn pedersen(
        &self,
        _inputs: &[FieldElement],
        _domain_separator: u32,
    ) -> Result<(FieldElement, FieldElement), OpcodeResolutionError> {
        unreachable!("Groth16 does not support black box functions. This code is only reachable if executing ACIR for a different backend");
    }

    fn fixed_base_scalar_mul(
        &self,
        _input: &FieldElement,
    ) -> Result<(FieldElement, FieldElement), OpcodeResolutionError> {
        unreachable!("Groth16 does not support black box functions. This code is only reachable if executing ACIR for a different backend");
    }
}
