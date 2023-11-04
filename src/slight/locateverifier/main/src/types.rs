
use serde::{Serialize,Deserialize};

pub const COMPONENT : &str = "locateverifier";
pub const MESSAGES : &str = "messages";

pub const TOPIC_INPUTS :&str = "locateverifier-inputs";
pub const TOPIC_OUTPUTS :&str = "locateverifier-outputs";

#[derive(Debug,Serialize,Deserialize)]
pub enum Request {
    LocateVerifier(String,String,String,String,String), //key, HREF, MediaType, from, to
    LocateVerifierWith(String,String,String,String,String,String,String), //With locate, storage
    LocateVerifierWithCFI(String,String,String,String,String,String) //key, HREF, MediaType, cfi, With locate, storage
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Response {
    LocateVerifier(String),
}
