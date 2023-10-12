
use serde::{Serialize,Deserialize};

pub const COMPONENT : &str = "manifestverifier";
pub const MESSAGES : &str = "messages";

pub const TOPIC_INPUTS :&str = "manifestverifier-inputs";
pub const TOPIC_OUTPUTS :&str = "manifestverifier-outputs";

#[derive(Debug,Serialize,Deserialize)]
pub enum Request {
    ManifestVerifier(String), //ID
    ManifestVerifierWith(String,String,String), //ID, Manifest provider, Storage provider
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Response {
    ManifestVerifier(String,String), //Content-type, manifest JSON
}