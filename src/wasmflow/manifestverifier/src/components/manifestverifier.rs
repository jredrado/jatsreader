pub use crate::components::generated::manifestverifier::*;

use authcomp::{Computation, HashType, ProofStream, Verifier};
use epubcontract::{Api, ApiError, ApiResponse};

use crate::error::Error;

use std::convert::TryInto;

#[async_trait::async_trait]
impl wasmflow_sdk::v1::ephemeral::BatchedComponent for Component {
    async fn job(
        inputs: Self::Inputs,
        outputs: Self::Outputs,

        config: Option<Self::Config>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let proofs: ProofStream =
            authcomp::from_bytes(&inputs.proofs).expect("Unable to get proofs");

        let s = HashType {
            data: inputs.id.try_into().expect("Unable to get id"),
        };

        let rcomputation =
            Api::<Verifier<ApiResponse, ApiError>>::api_manifest_verifier(&s, Some(proofs));

        match rcomputation {
            Ok(comp) => {
                outputs.contenttype.done(inputs.contenttype)?;
                if let Some(ApiResponse::String(data_ref)) = comp.get() {
                    outputs.data.done(data_ref.to_owned())?;
                }
            }
            Err(e) => return Err(Box::new(Error::NotVerified)),
        }

        Ok(())
    }
}
