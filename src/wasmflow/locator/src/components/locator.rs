pub use crate::components::generated::locator::*;

use authcomp::{AuthTProver, Computation, HashType, Prover};
use authselect::SimplifiedLocator;
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

        let locator = SimplifiedLocator {
            href: inputs.href,
            media_type: inputs.mediatype,
            from_css_selector: inputs.from,
            to_css_selector: inputs.to,
        };

        let comp = Api::<Prover<ApiResponse, ApiError>>::locate(&authpub, locator, None)
            .map_err(|e| Error::NotFound)?;

        let proofs = authcomp::to_vec(Computation::get_proofs(&comp));

        outputs.proofs.done(proofs)?;

        Ok(())
    }
}
