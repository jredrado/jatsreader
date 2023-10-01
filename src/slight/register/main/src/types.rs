
use serde::{Serialize,Deserialize};

pub const COMPONENT : &str = "register";
pub const MESSAGES : &str = "messages";

pub const TOPIC_INPUTS :&str = "register-inputs";
pub const TOPIC_OUTPUTS :&str = "register-outputs";

#[derive(Debug,Serialize,Deserialize)]
pub enum Request {
    RegisterEpub(Vec<u8>),
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Response {
    RegisterEpub(String),
}