use std::vec::Vec;
use std::string::String;
use std::borrow::{Cow,ToOwned};
use std::string::ToString;

use strong_xml::XmlRead;

#[derive(XmlRead,PartialEq,Debug,Default)]
#[xml(tag = "ncx")]
pub struct Ncx<'a> {

    #[xml(child = "navMap")]
    pub map : NavMap<'a>,

    #[xml(child = "pageList")]
    pub page_list : Option<PageList<'a>>

}

#[derive(XmlRead,PartialEq,Debug,Default)]
#[xml(tag = "navMap")]
pub struct NavMap<'a> {
    #[xml(child = "navPoint")]
    pub points: Vec<NavPoint<'a>>,
}

#[derive(XmlRead,PartialEq,Debug,Default)]
#[xml(tag = "navPoint")]
pub struct NavPoint<'a> {
    #[xml(attr = "id")]
    pub id: Cow<'a, str>,

    #[xml(attr = "playOrder")]
    pub play_order: Option<u32>,
    

    #[xml(child = "navLabel")]
    pub nav_label: NavLabel<'a>, //Vec??

    #[xml(child = "content")]
    pub content: Content<'a>,

    #[xml(child = "navPoint")]
    pub nav_points: Vec<NavPoint<'a>>

}

#[derive(XmlRead,PartialEq,Debug,Default)]
#[xml(tag = "navLabel")]
pub struct NavLabel<'a> {
    #[xml(flatten_text = "text")]
    pub text: Cow<'a,str>,
}

#[derive(XmlRead,PartialEq,Debug,Default)]
#[xml(tag = "content")]
pub struct Content<'a> {
    #[xml(attr = "src")]
    pub src: Cow<'a, str>,
}

#[derive(XmlRead,PartialEq,Debug,Default)]
#[xml(tag = "pageList")]
pub struct PageList<'a> {
    #[xml(attr = "id")]
    pub id: Option<Cow<'a, str>>,

    #[xml(attr = "class")]
    pub class: Option<Cow<'a, str>>,

    #[xml(child = "pageTarget")]
    pub page_targets: Vec<PageTarget<'a>>

}


#[derive(XmlRead,PartialEq,Debug,Default)]
#[xml(tag = "pageTarget")]
pub struct PageTarget<'a> {
    #[xml(attr = "id")]
    pub id: Option<Cow<'a, str>>,

    #[xml(attr = "playOrder")]
    pub play_order: Option<u32>,
    
    #[xml(attr = "value")]
    pub value: Option<Cow<'a, str>>,    

    #[xml(attr = "type")]
    pub typ: Cow<'a, str>,

    #[xml(child = "navLabel")]
    pub nav_label: NavLabel<'a>, //Vec??

    #[xml(child = "content")]
    pub content: Content<'a>,

}