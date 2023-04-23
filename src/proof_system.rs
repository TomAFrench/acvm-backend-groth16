use std::collections::BTreeMap;

use acvm::acir::{circuit::Circuit, native_types::Witness, BlackBoxFunc};
use acvm::FieldElement;
use acvm::{Language, ProofSystemCompiler};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_snark::SNARK;
use ark_std::rand::{rngs::StdRng, thread_rng, SeedableRng};

use ark_groth16::{PreparedVerifyingKey, Proof, ProvingKey};

use arkworks_backend::{from_fe, Curve, CurveAcir};

use crate::ArkworksGroth16;

type Groth16 = ark_groth16::Groth16<Curve>;

impl ProofSystemCompiler for ArkworksGroth16 {
    fn np_language(&self) -> Language {
        Language::R1CS
    }

    fn get_exact_circuit_size(&self, circuit: &Circuit) -> u32 {
        arkworks_backend::compute_num_constraints(circuit)
    }

    fn black_box_function_supported(&self, _opcode: &BlackBoxFunc) -> bool {
        // R1CS only supports Arithmetic opcodes.
        false
    }

    fn preprocess(&self, circuit: &Circuit) -> (Vec<u8>, Vec<u8>) {
        // Note: This is insecure and will result in keys for which it is possible
        // to generate invalid proofs which the verifier will be accept as valid.
        //
        // If you are using Groth16 in production then you must generate proving and
        // verification keys through a proper trusted setup.
        let mut rng = StdRng::seed_from_u64(1234);

        let (pk, vk) = Groth16::circuit_specific_setup(CurveAcir::from(circuit), &mut rng).unwrap();
        let vk = PreparedVerifyingKey::from(vk);

        let mut pk_bytes = Vec::new();
        pk.serialize_compressed(&mut pk_bytes)
            .expect("proving key should be serializable");
        let mut vk_bytes = Vec::new();
        vk.serialize_compressed(&mut vk_bytes)
            .expect("verification key should be serializable");

        (pk_bytes, vk_bytes)
    }

    fn prove_with_pk(
        &self,
        circuit: &Circuit,
        witness_values: BTreeMap<Witness, FieldElement>,
        proving_key: &[u8],
    ) -> Vec<u8> {
        let pk = ProvingKey::deserialize_compressed(proving_key)
            .expect("Could not deserialize proving key");

        // TODO: This relies on seeded entropy from `OsRng`. Is this secure enough?
        let mut rng = thread_rng();
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
        bytes
    }

    fn verify_with_vk(
        &self,
        proof: &[u8],
        public_inputs: BTreeMap<Witness, FieldElement>,
        _circuit: &Circuit,
        verification_key: &[u8],
    ) -> bool {
        let proof = Proof::deserialize_compressed(proof).expect("Could not deserialize proof");
        let vk = PreparedVerifyingKey::deserialize_compressed(verification_key)
            .expect("Could not deserialize verification key");

        let flattened_public_inputs: Vec<_> = public_inputs.into_values().map(from_fe).collect();

        Groth16::verify_proof(&vk, &proof, &flattened_public_inputs).unwrap()
    }
}
