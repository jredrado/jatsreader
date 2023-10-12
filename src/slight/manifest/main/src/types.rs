
use serde::{Serialize,Deserialize};

pub const COMPONENT : &str = "manifest";
pub const MESSAGES : &str = "messages";

pub const TOPIC_INPUTS :&str = "manifest-inputs";
pub const TOPIC_OUTPUTS :&str = "manifest-outputs";

#[derive(Debug,Serialize,Deserialize)]
pub enum Request {
    Manifest(String),
    ManifestWith(String,String) //ID, storage service
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Response {
    Manifest(String,String,Vec<u8>),
}