pub use crate::components::generated::resourceverifier::*;

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
        let s = HashType {
            data: inputs
                .id
                .try_into()
                .map_err(|e| Box::new(Error::NotVerified))?,
        };

        let proofs: ProofStream =
            authcomp::from_bytes(&inputs.proofs).map_err(|e| Box::new(Error::NotVerified))?;
        let rcomputation = Api::<Verifier<ApiResponse, ApiError>>::api_resource_verifier(
            &s,
            inputs.path.into_bytes(),
            proofs,
        );

        match rcomputation {
            Ok(comp) => {
                if let Some(ApiResponse::VecAndString(sdata, contenttype)) = comp.get() {
                    if let Some(ct) = contenttype {
                        outputs.contenttype.done(ct.to_owned())?;
                    }
                    outputs.data.done(sdata.to_owned())?;
                }
            }
            Err(e) => return Err(Box::new(Error::NotVerified)),
        }

        Ok(())
    }
}
