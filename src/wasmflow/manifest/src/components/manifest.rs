
pub use crate::components::generated::manifest::*;

use authcomp::{Computation,Prover,HashType,AuthTProver};
use epubcontract::{EPubParser,Publication,Api,ApiError,ApiResponse};

use crate::error::Error;

#[async_trait::async_trait]
impl wasmflow_sdk::v1::ephemeral::BatchedComponent for Component {
    async fn job(
        inputs: Self::Inputs,
        outputs: Self::Outputs,
        
        config: Option<Self::Config>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let authpub : AuthTProver<Publication<Prover<ApiResponse, ApiError>>> = authcomp::from_bytes(&inputs.source)
      .expect("Unable to decode");

    let comp = Api::<Prover<ApiResponse,ApiError>>::manifest(&authpub, None).expect("Unable to get manifest");

    let result = Computation::get(&comp);

    let proofs = authcomp::to_vec(Computation::get_proofs(&comp));
    
    match result {

          Some(ApiResponse::String(response)) => {

                outputs.contenttype.done(String::from("application/webpub+json"))?;
                //outputs.data.done(response.to_string())?;
                outputs.proofs.done(proofs)?;

          }
          None => { return Err( Box::new(Error::Other) ); }
          _ => { return Err( Box::new(Error::Other) );  }
    }

    Ok(())
  }
}
