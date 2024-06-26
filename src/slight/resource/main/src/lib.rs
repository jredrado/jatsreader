use anyhow::Result;

use messaging::*;
wit_bindgen_rust::import!("../../wit/messaging.wit");
wit_error_rs::impl_error!(messaging::MessagingError);

mod types;
use types::*;

use getrandom::getrandom;

pub struct ResourceClient {
    inputs : messaging::Pub,
    outputs : messaging::Sub,
    outputs_token: String,
    instance : String,
    client_id : [u8;4]
}

impl ResourceClient {

    pub fn new(instance: &str) -> Result<Self> {
        let inputs = Pub::open(MESSAGES)?;
        let outputs = Sub::open(MESSAGES)?;
        
        let mut client_id :[u8;4]=[0,0,0,0];
        getrandom(&mut client_id).map_err ( |e| anyhow::Error::msg(e.to_string()))?;

        let outputs_token = outputs.subscribe(&format!("{:?}-{}",client_id,TOPIC_OUTPUTS))?;

        Ok(ResourceClient{
            inputs,
            outputs,
            outputs_token,
            instance : String::from(instance),
            client_id
        })
    }

    pub fn resource(&self,id:String,path:Vec<u8>) -> Result<(Option<String>,Vec<u8>,Vec<u8>)> {

        let request = Request::Resource(id,path);
        let raw_request = rmp_serde::to_vec(&(self.client_id,request))?;

        self.inputs.publish(&raw_request, &format!("{}-{}",&self.instance,TOPIC_INPUTS))?;

        let raw_response = self.outputs.receive(&self.outputs_token)?;

        let response : Response = rmp_serde::from_read(raw_response.as_slice())?;

        match response {
            Response::Resource(contenttype,data,proofs) => Ok((contenttype,data,proofs)),
            _ => anyhow::bail!("Resource lib Unexpected response: {:?}", response)
        }

    }

    pub fn resource_with(&self,id:String,path:Vec<u8>,storage: String) -> Result<(Option<String>,Vec<u8>,Vec<u8>)> {

        let request = Request::ResourceWith(id,path,storage);
        let raw_request = rmp_serde::to_vec(&(self.client_id,request))?;

        self.inputs.publish(&raw_request, &format!("{}-{}",&self.instance,TOPIC_INPUTS))?;

        let raw_response = self.outputs.receive(&self.outputs_token)?;

        let response : Response = rmp_serde::from_read(raw_response.as_slice())?;

        match response {
            Response::Resource(contenttype,data,proofs) => Ok((contenttype,data,proofs)),
            _ => anyhow::bail!("Resource lib Unexpected response: {:?}", response)
        }

    }

}