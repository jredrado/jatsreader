use anyhow::Result;

use messaging::*;
wit_bindgen_rust::import!("../../wit/messaging.wit");
wit_error_rs::impl_error!(messaging::MessagingError);

mod types;
pub use types::*;

use getrandom::getrandom;

pub struct ResolverClient {
    inputs : messaging::Pub,
    outputs : messaging::Sub,
    outputs_token: String,
    instance : String,
    client_id : [u8;4]
}

impl ResolverClient {

    pub fn new(instance: &str) -> Result<Self> {
        let inputs = Pub::open(MESSAGES)?;
        let outputs = Sub::open(MESSAGES)?;
        
        let mut client_id :[u8;4]=[0,0,0,0];
        getrandom(&mut client_id).map_err ( |e| anyhow::Error::msg(e.to_string()))?;

        let outputs_token = outputs.subscribe(&format!("{:?}-{}",client_id,TOPIC_OUTPUTS))?;

        Ok(ResolverClient{
            inputs,
            outputs,
            outputs_token,
            instance : String::from(instance),
            client_id
        })
    }

    
    pub fn get_streamers(&self,id:String) -> Result<Vec<StreamerInfo>> {

        let request = Request::GetStreamers(id);
        let raw_request = rmp_serde::to_vec(&(self.client_id,request))?;

        self.inputs.publish(&raw_request, &format!("{}-{}",&self.instance,TOPIC_INPUTS))?;

        let raw_response = self.outputs.receive(&self.outputs_token)?;

        let response : Response = rmp_serde::from_read(raw_response.as_slice())?;

        match response {
            Response::StreamerLists(l) => Ok(l),
            _ => anyhow::bail!("Resolve lib Unexpected response: {:?}", response)
        }
        
    }

    pub fn add_content(&self,streamer_id:StreamerInfo,cids:Vec<String>) -> Result<()> {

        let request = Request::AddContent(streamer_id,cids);
        let raw_request = rmp_serde::to_vec(&(self.client_id,request))?;

        self.inputs.publish(&raw_request, &format!("{}-{}",&self.instance,TOPIC_INPUTS))?;

        let raw_response = self.outputs.receive(&self.outputs_token)?;

        let response : Response = rmp_serde::from_read(raw_response.as_slice())?;

        match response {
            Response::Ok=> Ok(()),
            _ => anyhow::bail!("Resolve lib Unexpected response: {:?}", response)
        }
        
    }

    pub fn remove_content(&self,streamer_id:String,cids:Vec<String>) -> Result<()> {

        let request = Request::RemoveContent(streamer_id,cids);
        let raw_request = rmp_serde::to_vec(&(self.client_id,request))?;

        self.inputs.publish(&raw_request, &format!("{}-{}",&self.instance,TOPIC_INPUTS))?;

        let raw_response = self.outputs.receive(&self.outputs_token)?;

        let response : Response = rmp_serde::from_read(raw_response.as_slice())?;

        match response {
            Response::Ok=> Ok(()),
            _ => anyhow::bail!("Resolve lib Unexpected response: {:?}", response)
        }
        
    }    

}