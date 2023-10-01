
use std::string::String;
use authcomp::{Encode,Decode,DecodeOwned};
use authcomp::{Serialize,Deserialize};
use authcomp::ToJSON;

#[derive(Debug,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub enum Error{
    #[n(0)] NotFound (#[n(0)] String),
    #[n(1)] Unknown
}


impl Default for Error {

    fn default() -> Self {
        Error::Unknown
    }
}


impl authcomp::ToJSON for Error {

    fn ser_json (&self, d: usize, s: &mut authcomp::JSONState) {
        s.st_pre();
    

        s.st_post(d);
    }
}