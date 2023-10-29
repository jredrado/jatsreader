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
use authselect_html5ever::SimplifiedLocator;

use epubcontract::{Api, ApiError, ApiResponse, EPubParser, Publication};
use tracing::{debug,info};


fn main() -> Result<()> {



    let configs = Configs::open("config-store")?;

    let instance = String::from_utf8(configs.get(&"INSTANCE")?)?;
 
    let locate_instance = String::from_utf8(configs.get(&"LOCATEINSTANCE")?)?;


    let inputs = Sub::open(MESSAGES)?;
    let outputs = Pub::open(MESSAGES)?;

    let inputs_token = inputs.subscribe(&format!("{}-{}",&instance,TOPIC_INPUTS))?;

    let locate_client = locate::LocateClient::new(&locate_instance)?;

    loop {

        let raw_input_message = inputs.receive(&inputs_token)?;
        
        let (client_id,request) : ([u8;4],Request) = rmp_serde::from_read(raw_input_message.as_slice())?;

        let output = match request {
            Request::LocateVerifier(key,href,mediatype,from,to) => {

                let locate = locate_client.locate(key.clone(),href.clone(),mediatype.clone(),from.clone(),to.clone())?;

                let proofs: ProofStream =authcomp::from_bytes(&locate.1).expect("Unable to get proofs");
    
                let id = hex::decode(key).map_err(|e| anyhow!(e.to_string()))?;

                let s = HashType {
                    data: id.try_into().expect("Unable to get id"),
                };

                let locator = SimplifiedLocator {
                    href: href,
                    media_type: mediatype,
                    from_css_selector: from,
                    to_css_selector: to,
                };

                let rcomputation = Api::<Verifier<ApiResponse, ApiError>>::api_locate_verifier(
                    &s,
                    locator,
                    proofs,
                );
        
                match rcomputation {
                    Ok(comp) => {
                        if let Some(ApiResponse::String(sdata)) = comp.get() {
                            Response::LocateVerifier(sdata.to_owned())
                        }else {
                            anyhow::bail!("Locate _ Unexpected result: {:?}", comp) 
                        }
                    }                    
                    _ => { anyhow::bail!("Locate _ Unexpected result: {:?}", rcomputation)  }
                }
                
            }

            Request::LocateVerifierWith(key,href,mediatype,from,to,locate,storage) => {

                let locate_client_with = locate::LocateClient::new(&locate)?;
                let locate = locate_client_with.locate_with(key.clone(),href.clone(),mediatype.clone(),from.clone(),to.clone(),storage)?;

                let proofs: ProofStream =authcomp::from_bytes(&locate.1).expect("Unable to get proofs");
    
                let id = hex::decode(key).map_err(|e| anyhow!(e.to_string()))?;

                let s = HashType {
                    data: id.try_into().expect("Unable to get id"),
                };

                let locator = SimplifiedLocator {
                    href: href,
                    media_type: mediatype,
                    from_css_selector: from,
                    to_css_selector: to,
                };

                let rcomputation = Api::<Verifier<ApiResponse, ApiError>>::api_locate_verifier(
                    &s,
                    locator,
                    proofs,
                );
        
                match rcomputation {
                    Ok(comp) => {
                        if let Some(ApiResponse::String(sdata)) = comp.get() {
                            Response::LocateVerifier(sdata.to_owned())
                        }else {
                            anyhow::bail!("Locate _ Unexpected result: {:?}", comp) 
                        }
                    }                    
                    _ => { anyhow::bail!("Locate _ Unexpected result: {:?}", rcomputation)  }
                }


            }

             
        };

        let raw_output = rmp_serde::to_vec(&output)?;

        outputs.publish(&raw_output, &format!("{:?}-{}",client_id,TOPIC_OUTPUTS))?;

    }

    Ok(())
}
