
use serde::{Serialize,Deserialize};

pub const COMPONENT : &str = "resourceverifier";
pub const MESSAGES : &str = "messages";

pub const TOPIC_INPUTS :&str = "resourceverifier-inputs";
pub const TOPIC_OUTPUTS :&str = "resourceverifier-outputs";

#[derive(Debug,Serialize,Deserialize)]
pub enum Request {
    ResourceVerifier(String,Vec<u8>), //Id,Path
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Response {
    ResourceVerifier(Option<String>,Vec<u8>), //Content-type, Content
}