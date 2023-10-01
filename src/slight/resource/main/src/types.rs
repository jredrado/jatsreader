
use serde::{Serialize,Deserialize};

pub const COMPONENT : &str = "resource";
pub const MESSAGES : &str = "messages";

pub const TOPIC_INPUTS :&str = "resource-inputs";
pub const TOPIC_OUTPUTS :&str = "resource-outputs";

#[derive(Debug,Serialize,Deserialize)]
pub enum Request {
    Resource(String,Vec<u8>), //Id,Path
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Response {
    Resource(Option<String>,Vec<u8>,Vec<u8>), //Content-type, Content, Proofs
}