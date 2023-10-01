
use anyhow::Result;

use messaging::*;
wit_bindgen_rust::import!("../../wit/messaging.wit");
wit_error_rs::impl_error!(messaging::MessagingError);

mod types;
use types::*;

use getrandom::getrandom;

pub struct StorageClient {
    inputs : messaging::Pub,
    outputs : messaging::Sub,
    outputs_token : String,
    instance : String,
    client_id : [u8;4]
}

impl StorageClient {

    pub fn new(instance: &str) -> Result<Self> {
        let inputs = Pub::open(MESSAGES)?;
        let outputs = Sub::open(MESSAGES)?;
        
        
        let mut client_id :[u8;4]=[0,0,0,0];
        getrandom(&mut client_id).map_err ( |e| anyhow::Error::msg(e.to_string()))?;

        let outputs_token = outputs.subscribe(&format!("{:?}-{}",client_id,TOPIC_OUTPUTS))?;

        Ok(StorageClient{
            inputs,
            outputs,
            outputs_token,
            instance : String::from(instance),
            client_id
        })
    }

    pub fn get(&self,key: String) -> Result<Vec<u8>> {

        let request = Request::Get(key);
        let raw_request = rmp_serde::to_vec(&(self.client_id,request))?;

        self.inputs.publish(&raw_request, &format!("{}-{}",&self.instance,TOPIC_INPUTS))?;
        let raw_response = self.outputs.receive(&self.outputs_token)?;

        let response : Response = rmp_serde::from_read(raw_response.as_slice())?;

        match response {
            Response::Get(value) => Ok(value),
            _ => anyhow::bail!("Storage Lib Get Unexpected response: {:?}", response)
        }

    }

    pub fn put(&self,key: String,value:Vec<u8>) -> Result<()> {
        let request = Request::Put(key,value);
        let raw_request = rmp_serde::to_vec(&(self.client_id,request))?;

        self.inputs.publish(&raw_request, &format!("{}-{}",&self.instance,TOPIC_INPUTS))?;
        let raw_response = self.outputs.receive(&self.outputs_token)?;

        let response : Response = rmp_serde::from_read(raw_response.as_slice())?;

        match response {
            Response::Put => Ok(()),
            _ => anyhow::bail!("Storage Lib Put Unexpected response: {:?}", response)
        }
    }
}