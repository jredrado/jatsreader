pub use crate::components::generated::resource::*;

use authcomp::{AuthTProver, Computation, HashType, Prover};
use epubcontract::{Api, ApiError, ApiResponse, EPubParser, Publication};

use crate::error::Error;

#[async_trait::async_trait]
impl wasmflow_sdk::v1::ephemeral::BatchedComponent for Component {
    async fn job(
        inputs: Self::Inputs,
        outputs: Self::Outputs,

        config: Option<Self::Config>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let authpub: AuthTProver<Publication<Prover<ApiResponse, ApiError>>> =
            authcomp::from_bytes(&inputs.source).map_err(|e| Error::DecodeError)?;

        let comp = Api::<Prover<ApiResponse, ApiError>>::resource(
            &authpub,
            inputs.path.into_bytes(),
            None,
        )
        .map_err(|e| Error::NotFound)?;

        let result = Computation::get(&comp);

        let proofs = authcomp::to_vec(Computation::get_proofs(&comp));
        outputs.proofs.done(proofs)?;

        /*
        match result {
            Some(ApiResponse::VecAndString(_, contenttype)) => {
                if let Some(ct) = contenttype {
                    outputs.contenttype.done(ct.to_owned())?;
                }

                outputs.proofs.done(proofs)?;
            }
            None => {
                return Err(Box::new(Error::ResponseError));
            }
            _ => {
                return Err(Box::new(Error::Other));
            }
        }
        */

        Ok(())
    }
}
