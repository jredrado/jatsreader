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
    let resource_instance = String::from_utf8(configs.get(&"RESOURCEINSTANCE")?)?;

    let inputs = Sub::open(MESSAGES)?;
    let outputs = Pub::open(MESSAGES)?;

    let inputs_token = inputs.subscribe(&format!("{}-{}",&instance,TOPIC_INPUTS))?;

    let resource_client = resource::ResourceClient::new(&resource_instance)?;

    loop {

        let raw_input_message = inputs.receive(&inputs_token)?;
        
        let (client_id,request) : ([u8;4],Request) = rmp_serde::from_read(raw_input_message.as_slice())?;

        let output = match request {
            Request::ResourceVerifier(hex_id,path) => {

                let resource = resource_client.resource(hex_id.clone(),path.clone())?;

                let proofs: ProofStream =authcomp::from_bytes(&resource.2).expect("Unable to get proofs");
    
                let id = hex::decode(hex_id).map_err(|e| anyhow!(e.to_string()))?;

                let s = HashType {
                    data: id.try_into().expect("Unable to get id"),
                };

                let rcomputation = Api::<Verifier<ApiResponse, ApiError>>::api_resource_verifier(
                    &s,
                    path,
                    proofs,
                );
        
                match rcomputation {
                    Ok(comp) => {
                        if let Some(ApiResponse::VecAndString(sdata, contenttype)) = comp.get() {
                            Response::ResourceVerifier(contenttype.to_owned(),sdata.to_owned())
                        }else {
                            anyhow::bail!("Resource _ Unexpected result: {:?}", comp) 
                        }
                    }                    
                    _ => { anyhow::bail!("Resource _ Unexpected result: {:?}", rcomputation)  }
                }
                
            }

            Request::ResourceVerifierWith(hex_id,path,resource,storage) => {

                let resource_client_with = resource::ResourceClient::new(&resource)?;
                let resource = resource_client_with.resource_with(hex_id.clone(),path.clone(),storage)?;

                let proofs: ProofStream =authcomp::from_bytes(&resource.2).expect("Unable to get proofs");
    
                let id = hex::decode(hex_id).map_err(|e| anyhow!(e.to_string()))?;

                let s = HashType {
                    data: id.try_into().expect("Unable to get id"),
                };

                let rcomputation = Api::<Verifier<ApiResponse, ApiError>>::api_resource_verifier(
                    &s,
                    path,
                    proofs,
                );
        
                match rcomputation {
                    Ok(comp) => {
                        if let Some(ApiResponse::VecAndString(sdata, contenttype)) = comp.get() {
                            Response::ResourceVerifier(contenttype.to_owned(),sdata.to_owned())
                        }else {
                            anyhow::bail!("Resource _ Unexpected result: {:?}", comp) 
                        }
                    }                    
                    _ => { anyhow::bail!("Resource _ Unexpected result: {:?}", rcomputation)  }
                }
                
            }            
        };

        let raw_output = rmp_serde::to_vec(&output)?;

        outputs.publish(&raw_output, &format!("{:?}-{}",client_id,TOPIC_OUTPUTS))?;

    }

    Ok(())
}
