use std::vec::Vec;
use std::string::String;
use std::borrow::{Cow,ToOwned};
use std::string::ToString;

use strong_xml::XmlRead;

use serde::{Serialize,Deserialize};
use serde::de::DeserializeOwned;

#[derive(XmlRead,PartialEq,Debug,Clone,Default,Serialize,Deserialize)]
#[xml(tag = "container")]
pub struct Container<'a> {

    #[xml(child = "rootfiles")]
    pub rootfiles : RootFiles<'a>,
}

#[derive(XmlRead,PartialEq,Debug,Clone,Default,Serialize,Deserialize)]
#[xml(tag = "rootfiles")]
pub struct RootFiles<'a> {
    #[xml(child = "rootfile")]
    pub rootfile : RootFile<'a>
}

#[derive(XmlRead,PartialEq,Debug,Clone,Default,Serialize,Deserialize)]
#[xml(tag = "rootfile")]
pub struct RootFile<'a> {

    #[xml(attr = "full-path")]
    pub full_path: Cow<'a, str>,

    #[xml(attr = "media-type")]
    pub media_type: Cow<'a, str>,

    #[xml(attr = "version")]
    pub version: Option<Cow<'a, str>>,

}
