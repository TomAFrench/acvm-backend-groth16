use acvm::{acir::circuit::Circuit, async_trait, CommonReferenceString};

use crate::{ArkworksGroth16, BackendError};

// TODO(#185): Ensure CRS download works in JS
#[async_trait(?Send)]
impl CommonReferenceString for ArkworksGroth16 {
    type Error = BackendError;

    async fn generate_common_reference_string(
        &self,
        _circuit: &Circuit,
    ) -> Result<Vec<u8>, Self::Error> {
        // Note: This is insecure and will result in keys for which it is possible
        // to generate invalid proofs which the verifier will be accept as valid.
        //
        // If you are using Groth16 in production then you must generate proving and
        // verification keys through a proper trusted setup.
        let rng_seed: u64 = 1234;

        Ok(rng_seed.to_be_bytes().to_vec())
    }

    async fn update_common_reference_string(
        &self,
        common_reference_string: Vec<u8>,
        _circuit: &Circuit,
    ) -> Result<Vec<u8>, Self::Error> {
        Ok(common_reference_string)
    }
}
