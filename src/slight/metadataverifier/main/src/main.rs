use anyhow::{anyhow,Result};

mod types;
use types::*;

use messaging::*;
wit_bindgen_rust::import!("../../wit/messaging.wit");
wit_error_rs::impl_error!(messaging::MessagingError);

use configs::*;
wit_bindgen_rust::import!("../../wit/configs.wit");
wit_error_rs::impl_error!(ConfigsError);

use authcomp::{Computation, HashType};
use authcomp::{AuthTNoProofs, NoProofs};
use authcomp::{AuthTProver, Prover};
use authcomp::{AuthTVerifier, Verifier};
use authcomp::ProofStream;

use epubcontract::{Api, ApiError, ApiResponse, EPubParser, Publication};
use tracing::{debug,info};


fn main() -> Result<()> {

    let configs = Configs::open("config-store")?;
    let instance = String::from_utf8(configs.get(&"INSTANCE")?)?;
    let metadata_instance = String::from_utf8(configs.get(&"METADATAINSTANCE")?)?;

    let inputs = Sub::open(MESSAGES)?;
    let outputs = Pub::open(MESSAGES)?;

    let inputs_token = inputs.subscribe(&format!("{}-{}",&instance,TOPIC_INPUTS))?;

    let metadata_client = metadata::MetadataClient::new(&metadata_instance)?;

    loop {

        let raw_input_message = inputs.receive(&inputs_token)?;
        
        let (client_id,request) : ([u8;4],Request) = rmp_serde::from_read(raw_input_message.as_slice())?;

        let output = match request {
            Request::MetadataVerifier(key) => {

                let metadata = metadata_client.metadata(key.clone())?;

                let proofs: ProofStream =authcomp::from_bytes(&metadata.1).expect("Unable to get proofs");
    
                let id = hex::decode(key).map_err(|e| anyhow!(e.to_string()))?;

                let s = HashType {
                    data: id.try_into().expect("Unable to get id"),
                };

                let rcomputation = Api::<Verifier<ApiResponse, ApiError>>::api_metadata_verifier(
                    &s,
                    proofs,
                );
        
                match rcomputation {
                    Ok(comp) => {
                        if let Some(ApiResponse::String(sdata)) = comp.get() {
                            Response::MetadataVerifier(sdata.to_owned())
                        }else {
                            anyhow::bail!("Metadata _ Unexpected result: {:?}", comp) 
                        }
                    }                    
                    _ => { anyhow::bail!("Metadata _ Unexpected result: {:?}", rcomputation)  }
                }
                
            }

            Request::MetadataVerifierWith(key,metadata, storage) => {

                let metadata_client_with = metadata::MetadataClient::new(&metadata)?;
                let metadata = metadata_client_with.metadata_with(key.clone(),storage)?;

                let proofs: ProofStream =authcomp::from_bytes(&metadata.1).expect("Unable to get proofs");
    
                let id = hex::decode(key).map_err(|e| anyhow!(e.to_string()))?;

                let s = HashType {
                    data: id.try_into().expect("Unable to get id"),
                };

                let rcomputation = Api::<Verifier<ApiResponse, ApiError>>::api_metadata_verifier(
                    &s,
                    proofs,
                );
        
                match rcomputation {
                    Ok(comp) => {
                        if let Some(ApiResponse::String(sdata)) = comp.get() {
                            Response::MetadataVerifier(sdata.to_owned())
                        }else {
                            anyhow::bail!("Metadata _ Unexpected result: {:?}", comp) 
                        }
                    }                    
                    _ => { anyhow::bail!("Metadata _ Unexpected result: {:?}", rcomputation)  }
                }

            }

             
        };

        let raw_output = rmp_serde::to_vec(&output)?;

        outputs.publish(&raw_output, &format!("{:?}-{}",client_id,TOPIC_OUTPUTS))?;

    }

    Ok(())
}
