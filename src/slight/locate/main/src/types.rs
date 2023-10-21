
use serde::{Serialize,Deserialize};

pub const METADATA : &str = "locate";
pub const MESSAGES : &str = "messages";

pub const TOPIC_INPUTS :&str = "locate-inputs";
pub const TOPIC_OUTPUTS :&str = "locate-outputs";

#[derive(Debug,Serialize,Deserialize)]
pub enum Request {
    Locate(String,String,String,String,String), //key, HREF, MediaType, from, to
    LocateWith(String,String,String,String,String,String) //With storage
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Response {
    Locate(String,Vec<u8>) //String(for proper mediatypes) and proofs
    //Locate(Vec<u8>,Vec<u8>) //For image querys or other media types. TO DO
}