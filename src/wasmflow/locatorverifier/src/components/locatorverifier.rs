pub use crate::components::generated::locatorverifier::*;

use authcomp::{Computation, HashType, ProofStream, Verifier};
use authselect::SimplifiedLocator;
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

        let locator = SimplifiedLocator {
            href: inputs.href,
            media_type: inputs.mediatype,
            from_css_selector: inputs.from,
            to_css_selector: inputs.to,
        };

        let rcomputation =
            Api::<Verifier<ApiResponse, ApiError>>::api_locate_verifier(&s, locator, proofs);

        match rcomputation {
            Ok(comp) => {
                if let Some(ApiResponse::String(data)) = comp.get() {
                    outputs.output.done(data.to_owned())?;
                }
            }
            Err(e) => return Err(Box::new(Error::NotVerified)),
        }

        Ok(())
    }
}
