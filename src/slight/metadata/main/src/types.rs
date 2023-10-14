
use serde::{Serialize,Deserialize};

pub const METADATA : &str = "metadata";
pub const MESSAGES : &str = "messages";

pub const TOPIC_INPUTS :&str = "metadata-inputs";
pub const TOPIC_OUTPUTS :&str = "metadata-outputs";

#[derive(Debug,Serialize,Deserialize)]
pub enum Request {
    Metadata(String),
    MetadataWith(String,String) //With storage
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Response {
    Metadata(String,Vec<u8>) //Metadata and proofs
}