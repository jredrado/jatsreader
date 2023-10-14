
use serde::{Serialize,Deserialize};

pub const STORAGE : &str = "storage";
pub const MESSAGES : &str = "messages";

pub const TOPIC_INPUTS :&str = "storage-inputs";
pub const TOPIC_OUTPUTS :&str = "storage-outputs";

#[derive(Debug,Serialize,Deserialize)]
pub enum Request {
    Get(String),
    Put(String,Vec<u8>),
    List
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Response {
    Get(Vec<u8>),
    Put,
    List(Vec<String>)
}