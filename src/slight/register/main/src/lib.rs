use anyhow::Result;

use messaging::*;
wit_bindgen_rust::import!("../../wit/messaging.wit");
wit_error_rs::impl_error!(messaging::MessagingError);

mod types;
use types::*;

use getrandom::getrandom;

pub struct RegisterClient {
    inputs : messaging::Pub,
    outputs : messaging::Sub,
    outputs_token: String,
    instance : String,
    client_id : [u8;4]
}

impl RegisterClient {

    pub fn new(instance: &str) -> Result<Self> {
        let inputs = Pub::open(MESSAGES)?;
        let outputs = Sub::open(MESSAGES)?;
        
        let outputs_token = outputs.subscribe(TOPIC_OUTPUTS)?;

        let mut client_id :[u8;4]=[0,0,0,0];
        getrandom(&mut client_id).map_err ( |e| anyhow::Error::msg(e.to_string()))?;

        let outputs_token = outputs.subscribe(&format!("{:?}-{}",client_id,TOPIC_OUTPUTS))?;

        Ok(RegisterClient{
            inputs,
            outputs,
            outputs_token,
            instance : String::from(instance),
            client_id
        })
    }

    pub fn register(&self,epub:Vec<u8>) -> Result<String> {

        let request = Request::RegisterEpub(epub);
        let raw_request = rmp_serde::to_vec(&(self.client_id,request))?;

        self.inputs.publish(&raw_request, &format!("{}-{}",&self.instance,TOPIC_INPUTS))?;

        let raw_response = self.outputs.receive(&self.outputs_token)?;

        let response : Response = rmp_serde::from_read(raw_response.as_slice())?;

        match response {
            Response::RegisterEpub(id) => Ok(id),
            _ => anyhow::bail!("Register lib Unexpected response: {:?}", response)
        }

    }


}