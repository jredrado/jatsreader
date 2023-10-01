use anyhow::Result;

mod types;
use types::*;

use keyvalue::*;
wit_bindgen_rust::import!("../../wit/keyvalue.wit");
wit_error_rs::impl_error!(KeyvalueError);

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
use tracing::{debug,info};



fn main() -> Result<()> {

    let NO_KEY = String::from("failed to get key");

    let configs = Configs::open("config-store")?;
    let instance = String::from_utf8(configs.get(&"INSTANCE")?)?;
    let storage_instance = String::from_utf8(configs.get(&"DHTINSTANCE")?)?;

    let inputs = Sub::open(MESSAGES)?;
    let outputs = Pub::open(MESSAGES)?;

    let inputs_token = inputs.subscribe(&format!("{}-{}",&instance,TOPIC_INPUTS))?;

    let dht_client = Keyvalue::open(&instance)?;

    loop {

        let raw_input_message = inputs.receive(&inputs_token)?;
        
        let (client_id,request) : ([u8;4],Request) = rmp_serde::from_read(raw_input_message.as_slice())?;

        let output = match request {

            Request::GetStreamers(cid) => {

                match dht_client.get(&cid) {
                    Ok(raw_streamers) => {
                        let streamers: Vec<StreamerInfo> = rmp_serde::from_read(raw_streamers.as_slice())?;
                        Response::StreamerLists(streamers)
                    }
                    Err(KeyvalueError::UnexpectedError(NO_KEY)) => {
                        Response::Error(String::from("CID not found"))
                    }
                    Err(keyvalue::KeyvalueError::KeyNotFound(_)) => {
                        Response::Error(String::from("CID not found"))
                    }
                    Err(e) => {
                        Response::Error(format!("Resolve unexpected result to get: {:?}",e))
                    }                    
                    _ => {
                        Response::Error(format!("Resolve unexpected result to get: {:?}",&cid))
                    }
                }
            },
            Request::AddContent(streamer_info,cids) => {
                for cid in cids {
                    //Retrieve the peers
                    match dht_client.get(&cid) {
                        Ok(raw_streamers) => {
                            let mut streamers: Vec<StreamerInfo> = rmp_serde::from_read(raw_streamers.as_slice())?;
                            streamers.push(streamer_info.clone());

                            let new_streamers = rmp_serde::to_vec(&streamers)?;
                            dht_client.set(&cid,&new_streamers);
                        }
                        Err(KeyvalueError::UnexpectedError(NO_KEY)) => {
                            //If not found
                            let raw_streamers = vec![streamer_info.to_owned()];
                            let streamers = rmp_serde::to_vec(&raw_streamers)?;
                            dht_client.set(&cid,&streamers);
                        }
                        Err(keyvalue::KeyvalueError::KeyNotFound(_)) => {
                            //If not found
                            let raw_streamers = vec![streamer_info.clone()];
                            let streamers = rmp_serde::to_vec(&raw_streamers)?;
                            dht_client.set(&cid,&streamers);
                        }
                        Err(e) => {
                            anyhow::bail!("Resolve unexpected result to get: {:?}",e);
                        }                            
                        _ => {
                            anyhow::bail!("Resolve unexpected result to get: {:?}",&cid);
                        }
                    }
                    
                }
                Response::Ok
            },
            Request::RemoveContent(streamer_id,cids) => {
                for cid in cids {
                    //Retrieve the peers
                    match dht_client.get(&cid) {
                        Ok(raw_streamers) => {
                            let mut streamers: Vec<String> = rmp_serde::from_read(raw_streamers.as_slice())?;

                            streamers.retain(|x| *x != streamer_id);

                            let new_streamers = rmp_serde::to_vec(&streamers)?;
                            dht_client.set(&cid,&new_streamers);
                        }
                        Err(keyvalue::KeyvalueError::KeyNotFound(_)) => {
                
                        }
                        _ => {
                            anyhow::bail!("Resolve unexpected result to get: {:?}",&cid);
                        }
                    }
                    
                }                
                Response::Ok
            }
            Request::RemoveStreamer(streamer_id) => {
                anyhow::bail!("Not implemented");
            }
        };

        let raw_output = rmp_serde::to_vec(&output)?;

        outputs.publish(&raw_output, &format!("{:?}-{}",client_id,TOPIC_OUTPUTS))?;

    }

    Ok(())
}
