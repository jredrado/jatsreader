
use anyhow::Result;

use messaging::*;
wit_bindgen_rust::import!("../../wit/messaging.wit");
wit_error_rs::impl_error!(messaging::MessagingError);

mod types;
use types::*;

use getrandom::getrandom;

pub struct LocateClient {
    inputs : messaging::Pub,
    outputs : messaging::Sub,
    outputs_token : String,
    instance : String,
    client_id : [u8;4]
}

impl LocateClient {

    pub fn new(instance: &str) -> Result<Self> {
        let inputs = Pub::open(MESSAGES)?;
        let outputs = Sub::open(MESSAGES)?;
        
        
        let mut client_id :[u8;4]=[0,0,0,0];
        getrandom(&mut client_id).map_err ( |e| anyhow::Error::msg(e.to_string()))?;

        let outputs_token = outputs.subscribe(&format!("{:?}-{}",client_id,TOPIC_OUTPUTS))?;

        Ok(LocateClient{
            inputs,
            outputs,
            outputs_token,
            instance : String::from(instance),
            client_id
        })
    }

    pub fn locate(&self,key: String,href: String, mediatype:String, from:String,to:String) -> Result<(String,Vec<u8>)> {

        let request = Request::Locate(key,href,mediatype,from,to);
        let raw_request = rmp_serde::to_vec(&(self.client_id,request))?;

        self.inputs.publish(&raw_request, &format!("{}-{}",&self.instance,TOPIC_INPUTS))?;
        let raw_response = self.outputs.receive(&self.outputs_token)?;

        let response : Response = rmp_serde::from_read(raw_response.as_slice())?;

        match response {
            Response::Locate(value,proofs) => Ok((value,proofs)),
            _ => anyhow::bail!("Locate Lib Get Unexpected response: {:?}", response)
        }

    }

    pub fn locate_with(&self,key: String,href: String, mediatype:String, from:String,to:String, storage:String) -> Result<(String,Vec<u8>)> {

        let request = Request::LocateWith(key,href,mediatype,from,to,storage);
        let raw_request = rmp_serde::to_vec(&(self.client_id,request))?;

        self.inputs.publish(&raw_request, &format!("{}-{}",&self.instance,TOPIC_INPUTS))?;
        let raw_response = self.outputs.receive(&self.outputs_token)?;

        let response : Response = rmp_serde::from_read(raw_response.as_slice())?;

        match response {
            Response::Locate(value,proofs) => Ok((value,proofs)),
            _ => anyhow::bail!("Locate Lib Get Unexpected response: {:?}", response)
        }

    }    

    pub fn locate_with_cfi(&self,key: String,href: String, mediatype:String, cfi:String, storage:String) -> Result<(String,Vec<u8>)> {

        let request = Request::LocateWithCFI(key,href,mediatype,cfi,storage);
        let raw_request = rmp_serde::to_vec(&(self.client_id,request))?;

        self.inputs.publish(&raw_request, &format!("{}-{}",&self.instance,TOPIC_INPUTS))?;
        let raw_response = self.outputs.receive(&self.outputs_token)?;

        let response : Response = rmp_serde::from_read(raw_response.as_slice())?;

        match response {
            Response::Locate(value,proofs) => Ok((value,proofs)),
            _ => anyhow::bail!("Locate Lib CFI Unexpected response: {:?}", response)
        }

    }    
 
}