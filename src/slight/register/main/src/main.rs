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
//use authcomp::{AuthTNoProofs, NoProofs};
use authcomp::{AuthTProver, Prover};

use epubcontract::{Api, ApiError, ApiResponse, EPubParser, Publication};
use tracing::{debug,info};


fn main() -> Result<()> {

    let configs = Configs::open("config-store")?;
    let instance = String::from_utf8(configs.get(&"INSTANCE")?)?;
    let storage_instance = String::from_utf8(configs.get(&"STORAGEINSTANCE")?)?;
    let resolver_instance = String::from_utf8(configs.get(&"RESOLVER")?)?;
    let streamer_name = String::from_utf8(configs.get(&"STREAMER")?)?;
    let streamer_public_api = String::from_utf8(configs.get(&"STREAMER_API")?)?;

    let inputs = Sub::open(MESSAGES)?;
    let outputs = Pub::open(MESSAGES)?;

    let inputs_token = inputs.subscribe(&format!("{}-{}",&instance,TOPIC_INPUTS))?;

    let storage_client = storage::StorageClient::new(&storage_instance)?;
    let resolver_client = resolver::ResolverClient::new(&resolver_instance)?;

    loop {

        let raw_input_message = inputs.receive(&inputs_token)?;
        let (client_id,request) : ([u8;4],Request) = rmp_serde::from_read(raw_input_message.as_slice())?;

        let output = match request {
            Request::RegisterEpub(ref epub) => {

                let (id,authepub) = Api::<Prover<ApiResponse,ApiError>>::register("prover",epub).expect("Fail to register!");

                let authepub_bytes = authcomp::to_vec(&authepub);
        
                info!("Authpub bytes: {:?}", authepub_bytes.len());

                let hex_id = hex::encode(&id);
                storage_client.put(hex_id.clone(),authepub_bytes)?;

                let cids = vec![hex_id.clone()];

                //resolver_client.add_content(resolver::StreamerInfo{ id: streamer_name.clone(),endpoint: streamer_public_api.clone()},cids)?;

                resolver_client.add_content(resolver::StreamerInfo{ id: storage_instance.clone(),endpoint: storage_instance.clone()},cids)?;

                Response::RegisterEpub(hex_id)
            },
            Request::RegisterEpubWith(ref epub, ref storage) => {

                let (id,authepub) = Api::<Prover<ApiResponse,ApiError>>::register("prover",epub).expect("Fail to register!");

                let authepub_bytes = authcomp::to_vec(&authepub);
        
                info!("Authpub bytes: {:?}", authepub_bytes.len());

                let storage_client_with = storage::StorageClient::new(&storage)?;

                let hex_id = hex::encode(&id);
                storage_client_with.put(hex_id.clone(),authepub_bytes)?;

                let cids = vec![hex_id.clone()];

                resolver_client.add_content(resolver::StreamerInfo{ id: storage.clone(),endpoint: storage.clone()},cids)?;

                Response::RegisterEpub(hex_id)
            }            
            _ => { anyhow::bail!("Register Unexpected request: {:?}", request ) }
        };

        let raw_output = rmp_serde::to_vec(&output)?;

        outputs.publish(&raw_output, &format!("{:?}-{}",client_id,TOPIC_OUTPUTS))?;

    }

    Ok(())
}
