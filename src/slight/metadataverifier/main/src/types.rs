
use serde::{Serialize,Deserialize};

pub const COMPONENT : &str = "metadataverifier";
pub const MESSAGES : &str = "messages";

pub const TOPIC_INPUTS :&str = "metadataverifier-inputs";
pub const TOPIC_OUTPUTS :&str = "metadataverifier-outputs";

#[derive(Debug,Serialize,Deserialize)]
pub enum Request {
    MetadataVerifier(String), //key
    MetadataVerifierWith(String,String,String), //key, metadata service, with storage
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Response {
    MetadataVerifier(String),
}
