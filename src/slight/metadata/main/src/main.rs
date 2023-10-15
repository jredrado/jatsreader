use anyhow::Result;

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

use epubcontract::{Api, ApiError, ApiResponse, EPubParser, Publication};

fn main() -> Result<()> {

    println!("Main");
    let configs = Configs::open("config-store")?;
    println!("Config");
    let instance = String::from_utf8(configs.get(&"INSTANCE")?)?;
    println!("Instance {}", &instance);
    let storage_instance = String::from_utf8(configs.get(&"STORAGEINSTANCE")?)?;
    println!("Storage {}",storage_instance);

    let inputs = Sub::open(MESSAGES)?;
    let outputs = Pub::open(MESSAGES)?;

    println!("Messages");

    let inputs_token = inputs.subscribe(&format!("{}-{}",&instance,TOPIC_INPUTS))?;

    let storage_client = storage::StorageClient::new(&storage_instance)?;

    loop {

        let raw_input_message = inputs.receive(&inputs_token)?;
        let (client_id,request) : ([u8;4],Request) = rmp_serde::from_read(raw_input_message.as_slice())?;

        let output = match request {
            Request::Metadata(key) => {
                let source = storage_client.get(key)?;
    
                let authpub : AuthTProver<Publication<Prover<ApiResponse, ApiError>>> = authcomp::from_bytes(&source)
                                                                                            .expect("Unable to decode");

                let comp = Api::<Prover<ApiResponse, ApiError>>::metadata(
                    &authpub,
                    None,
                ).expect("Unable to get metadata");
        
                let result = Computation::get(&comp);
        
                let proofs = authcomp::to_vec(Computation::get_proofs(&comp));
                
                match result {
                    Some(ApiResponse::String(data)) => {
    
                        Response::Metadata(data.to_owned(),proofs)
                    }
                    None => { anyhow::bail!("Metadata Unexpected result: {:?}", result) }
                    _ => { anyhow::bail!("Metadata _ Unexpected result: {:?}", result)  }
                }
            }

            Request::MetadataWith(key, storage) => {
                let storage_client_with = storage::StorageClient::new(&storage)?;
                let source = storage_client_with.get(key)?;

                let authpub : AuthTProver<Publication<Prover<ApiResponse, ApiError>>> = authcomp::from_bytes(&source)
                                                                                            .expect("Unable to decode");

                let comp = Api::<Prover<ApiResponse, ApiError>>::metadata(
                    &authpub,
                    None,
                ).expect("Unable to get metadata");
        
                let result = Computation::get(&comp);
        
                let proofs = authcomp::to_vec(Computation::get_proofs(&comp));
                
                match result {
                    Some(ApiResponse::String(data)) => {
    
                        Response::Metadata(data.to_owned(),proofs)
                    }
                    None => { anyhow::bail!("Metadata Unexpected result: {:?}", result) }
                    _ => { anyhow::bail!("Metadata _ Unexpected result: {:?}", result)  }
                }

            }
        };

        let raw_output = rmp_serde::to_vec(&output)?;

        outputs.publish(&raw_output, &format!("{:?}-{}",client_id,TOPIC_OUTPUTS))?;

    }

    Ok(())
}
