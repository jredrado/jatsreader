
use serde::{Serialize,Deserialize};

pub const COMPONENT : &str = "resolver";
pub const MESSAGES : &str = "messages";

pub const TOPIC_INPUTS :&str = "resolver-inputs";
pub const TOPIC_OUTPUTS :&str = "resolver-outputs";

#[derive(Debug,Serialize,Deserialize)]
pub enum Request {
    GetStreamers(String), // Retrieve the lists of streamers Ids for a content
    AddContent(StreamerInfo,Vec<String>), //Streamer Id, List of contents ids
    RemoveContent(String,Vec<String>), //Streamer Id, List of contents ids
    RemoveStreamer(String) //
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Response {
    StreamerLists(Vec<StreamerInfo>),
    Ok,
    Error(String)
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct StreamerInfo {
    pub id: String,
    pub endpoint: String
}