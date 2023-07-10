use std::collections::BTreeMap;

use acvm::acir::circuit::{Circuit, Opcode};
use acvm::acir::native_types::{Witness, WitnessMap};
use acvm::{FieldElement, Language, ProofSystemCompiler};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_snark::SNARK;
use ark_std::rand::{rngs::StdRng, thread_rng, SeedableRng};

use ark_groth16::{PreparedVerifyingKey, Proof, ProvingKey};

use arkworks_backend::{from_fe, Curve, CurveAcir};

use crate::{ArkworksGroth16, BackendError};

type Groth16 = ark_groth16::Groth16<Curve>;

impl ProofSystemCompiler for ArkworksGroth16 {
    type Error = BackendError;

    fn np_language(&self) -> Language {
        Language::R1CS
    }

    fn get_exact_circuit_size(&self, circuit: &Circuit) -> Result<u32, BackendError> {
        Ok(arkworks_backend::compute_num_constraints(circuit))
    }

    fn supports_opcode(&self, opcode: &Opcode) -> bool {
        matches!(opcode, Opcode::Arithmetic(_))
    }

    fn preprocess(
        &self,
        common_reference_string: &[u8],
        circuit: &Circuit,
    ) -> Result<(Vec<u8>, Vec<u8>), BackendError> {
        // Note: This is insecure and will result in keys for which it is possible
        // to generate invalid proofs which the verifier will be accept as valid.
        //
        // If you are using Groth16 in production then you must generate proving and
        // verification keys through a proper trusted setup.
        let rng_seed: u64 = u64::from_be_bytes(common_reference_string.try_into().unwrap());
        let mut rng = StdRng::seed_from_u64(rng_seed);

        let (pk, vk) = Groth16::circuit_specific_setup(CurveAcir::from(circuit), &mut rng).unwrap();
        let vk = PreparedVerifyingKey::from(vk);

        let mut pk_bytes = Vec::new();
        pk.serialize_compressed(&mut pk_bytes)
            .expect("proving key should be serializable");
        let mut vk_bytes = Vec::new();
        vk.serialize_compressed(&mut vk_bytes)
            .expect("verification key should be serializable");

        Ok((pk_bytes, vk_bytes))
    }

    fn prove_with_pk(
        &self,
        _common_reference_string: &[u8],
        circuit: &Circuit,
        witness_values: WitnessMap,
        proving_key: &[u8],
        _is_recursive: bool,
    ) -> Result<Vec<u8>, Self::Error> {
        let pk = ProvingKey::deserialize_compressed(proving_key)
            .expect("Could not deserialize proving key");

        // TODO: This relies on seeded entropy from `OsRng`. Is this secure enough?
        let mut rng = thread_rng();

        let witness_values: BTreeMap<Witness, FieldElement> =
            BTreeMap::from_iter(witness_values.into_iter());
        let proof = Groth16::create_random_proof_with_reduction(
            CurveAcir::from((circuit, witness_values)),
            &pk,
            &mut rng,
        )
        .unwrap();

        let mut bytes = Vec::new();
        proof
            .serialize_compressed(&mut bytes)
            .expect("proof should be serializable");
        Ok(bytes)
    }

    fn verify_with_vk(
        &self,
        _common_reference_string: &[u8],
        proof: &[u8],
        public_inputs: WitnessMap,
        _circuit: &Circuit,
        verification_key: &[u8],
        _is_recursive: bool,
    ) -> Result<bool, Self::Error> {
        let proof = Proof::deserialize_compressed(proof).expect("Could not deserialize proof");
        let vk = PreparedVerifyingKey::deserialize_compressed(verification_key)
            .expect("Could not deserialize verification key");

        let flattened_public_inputs: Vec<_> = public_inputs
            .into_iter()
            .map(|(_, value)| from_fe(value))
            .collect();

        Ok(Groth16::verify_proof(&vk, &proof, &flattened_public_inputs).unwrap())
    }

    fn proof_as_fields(
        &self,
        _proof: &[u8],
        _public_inputs: WitnessMap,
    ) -> Result<Vec<FieldElement>, Self::Error> {
        todo!()
    }

    fn vk_as_fields(
        &self,
        _common_reference_string: &[u8],
        _verification_key: &[u8],
    ) -> Result<(Vec<FieldElement>, FieldElement), Self::Error> {
        todo!()
    }
}
