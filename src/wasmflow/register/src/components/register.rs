pub use crate::components::generated::register::*;

use authcomp::{Computation, HashType};
//use authcomp::{AuthTNoProofs, NoProofs};
use authcomp::{AuthTProver, Prover};

use epubcontract::{Api, ApiError, ApiResponse, EPubParser, Publication};
use tracing::{debug,info};

#[async_trait::async_trait]
impl wasmflow_sdk::v1::ephemeral::BatchedComponent for Component {
    async fn job(
        inputs: Self::Inputs,
        outputs: Self::Outputs,

        config: Option<Self::Config>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Register");


        let (id,authepub) = Api::<Prover<ApiResponse,ApiError>>::register("prover",&inputs.source).expect("Fail to register!");

        let authpub_bytes = authcomp::to_vec(&authepub);

        info!("Authpub bytes: {:?}", authpub_bytes.len());

        outputs.id.done(id.data.to_vec())?;
        outputs.value.done(authpub_bytes)?;

        Ok(())
    }
}
