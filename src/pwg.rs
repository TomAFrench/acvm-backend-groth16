use acvm::acir::{circuit::opcodes::BlackBoxFuncCall, native_types::Witness};
use acvm::{FieldElement, OpcodeResolution};
use acvm::{OpcodeResolutionError, PartialWitnessGenerator};
use std::collections::BTreeMap;

use crate::ArkworksGroth16;

impl PartialWitnessGenerator for ArkworksGroth16 {
    fn solve_black_box_function_call(
        &self,
        _initial_witness: &mut BTreeMap<Witness, FieldElement>,
        _func_call: &BlackBoxFuncCall,
    ) -> Result<OpcodeResolution, OpcodeResolutionError> {
        unreachable!("Groth16 does not support black box functions. This code is only reachable if executing ACIR for a different backend");
    }
}
