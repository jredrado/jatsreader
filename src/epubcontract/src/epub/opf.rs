use std::vec::Vec;
use std::string::String;
use std::borrow::{Cow,ToOwned};

use strong_xml::XmlRead;

#[derive(XmlRead,PartialEq,Debug)]
#[xml(tag = "package")]
pub struct Opf<'a> {
    #[xml(child = "metadata")]
    pub metadata : Metadata<'a>,
    #[xml(child = "manifest")]
    pub manifest: Manifest<'a>,
    #[xml(child = "spine")]
    pub spine: Spine<'a>,
    #[xml(child = "guide")]
    pub guide: Option<Guide<'a>>,

    #[xml(attr = "dir")]
    pub dir: Option<Cow<'a, str>>,

    #[xml(attr = "version")]
    pub version: Option<Cow<'a, str>>,

    #[xml(attr = "unique-identifier")]
    pub unique_identifier: Cow<'a, str>,

}


#[derive(XmlRead,PartialEq,Debug)]
#[xml(tag = "metadata")]
pub struct Metadata<'a> {
    #[xml(child = "dc:title")]
    pub title : Vec<Title<'a>>,

    #[xml(flatten_text = "dc:language")]
    pub language: Vec<Cow<'a,str>>,
    
    #[xml(child = "dc:identifier")]
    pub identifier: Vec<Identifier<'a>>,
    
    #[xml(child = "dc:creator")]
    pub creator: Vec<Author<'a>>,

    #[xml(child = "dc:contributor")]
    pub contributor: Vec<Contributor<'a>>,

    #[xml(child = "dc:subject")]
    pub subject: Vec<Subject<'a>>,

    #[xml(child = "dc:date")]
    pub date: Vec<Date<'a>>,

    #[xml(child = "meta")]
    pub meta: Vec<Metafield<'a>>,

    #[xml(flatten_text = "dc:description")]
    pub description: Vec<Cow<'a,str>>,

    #[xml(flatten_text = "dc:publisher")]
    pub publisher: Vec<Cow<'a,str>>,

    #[xml(child = "type")]
    pub types: Vec<Typefield<'a>>,

    #[xml(flatten_text = "dc:format")]
    pub format: Vec<Cow<'a,str>>,

    #[xml(flatten_text = "dc:source")]
    pub source: Vec<Cow<'a,str>>,

    #[xml(flatten_text = "dc:relation")]
    pub relation: Vec<Cow<'a,str>>,

    #[xml(flatten_text = "dc:coverage")]
    pub coverage: Vec<Cow<'a,str>>,

    #[xml(flatten_text = "dc:rights")]
    pub rights: Vec<Cow<'a,str>>,

}


#[derive(XmlRead,PartialEq,Debug)]
#[xml(tag = "dc:subject")]
pub struct Subject<'a> {
    #[xml(text)]
    pub content: Cow<'a, str>,

    #[xml(attr = "term")]
    pub term: Option<Cow<'a, str>>,

    #[xml(attr = "authority")]
    pub authority: Option<Cow<'a, str>>,

    #[xml(attr = "lang")]
    pub lang: Option<Cow<'a, str>>,
}

#[derive(XmlRead,PartialEq,Debug)]
#[xml(tag = "dc:identifier")]
pub struct Identifier<'a> {
    #[xml(text)]
    pub content: Cow<'a, str>,
    #[xml(attr = "id")]
    pub id: Option<Cow<'a, str>>,
    #[xml(attr = "scheme")]
    pub scheme: Option<Cow<'a, str>>,
}

#[derive(XmlRead,PartialEq,Debug,Default)]
#[xml(tag = "dc:title")]
pub struct Title<'a> {
    #[xml(text)]
    pub content: Cow<'a, str>,
    #[xml(attr = "id")]
    pub id: Option<Cow<'a, str>>,
    #[xml(attr = "dir")]
    pub dir: Option<Cow<'a, str>>,        
    
    #[xml(attr = "lang")]
    pub lang: Option<Cow<'a, str>>,

    
}

pub trait AsAuthor<'a> {

    fn content(&'a self) -> &'a str;
    fn file_as(&'a self) -> Option<&'a str>;
    fn role(&'a self) -> Option<&'a str>;
    fn id(&'a self) -> Option<&'a str>;
}

#[derive(XmlRead,PartialEq,Debug)]
#[xml(tag = "dc:creator")]
pub struct Author<'a> {

    #[xml(text)]
    pub content: Cow<'a, str>,

    #[xml(attr = "file-as")]
    pub file_as: Option<Cow<'a, str>>,

    #[xml(attr = "role")]
    pub role: Option<Cow<'a, str>>,

    #[xml(attr = "id")]
    pub id: Option<Cow<'a, str>>,

}

impl<'a> AsAuthor<'a> for Author<'a> {

    fn content(&'a self) -> &'a str{
            self.content.as_ref()
    }
    fn file_as(&'a self) -> Option<&'a str>{
            self.file_as.as_ref().map ( |s| s.as_ref())
    }
    fn role(&'a self) -> Option<&'a str>{
            self.role.as_ref().map ( |s| s.as_ref())
    }
    fn id(&'a self) -> Option<&'a str>{
            self.id.as_ref().map ( |s| s.as_ref())
    }

}

#[derive(XmlRead,PartialEq,Debug)]
#[xml(tag = "dc:contributor")]
pub struct Contributor<'a> {
    #[xml(text)]
    pub content: Cow<'a, str>,

    #[xml(attr = "file-as")]
    pub file_as: Option<Cow<'a, str>>,

    #[xml(attr = "role")]
    pub role: Option<Cow<'a, str>>,

    #[xml(attr = "id")]
    pub id: Option<Cow<'a, str>>,
}

impl<'a> AsAuthor<'a> for Contributor<'a> {

    fn content(&'a self) -> &'a str{
            self.content.as_ref()
    }
    fn file_as(&'a self) -> Option<&'a str>{
            self.file_as.as_ref().map ( |s| s.as_ref())
    }
    fn role(&'a self) -> Option<&'a str>{
            self.role.as_ref().map ( |s| s.as_ref())
    }
    fn id(&'a self) -> Option<&'a str>{
            self.id.as_ref().map ( |s| s.as_ref())
    }

}

#[derive(XmlRead,PartialEq,Debug)]
#[xml(tag = "dc:date")]
pub struct Date<'a> {
    #[xml(text)]
    pub content: Cow<'a, str>,

    #[xml(attr = "event")]
    pub event: Option<Cow<'a, str>>,
}

#[derive(XmlRead,PartialEq,Debug)]
#[xml(tag = "meta")]
pub struct Metafield<'a> {
    #[xml(text)]
    pub content: Cow<'a, str>,
    #[xml(attr = "dir")]
    pub dir: Option<Cow<'a, str>>,
    #[xml(attr = "refines")]
    pub refines: Option<Cow<'a, str>>,
    #[xml(attr = "property")]
    pub property: Option<Cow<'a, str>>,
    #[xml(attr = "id")]
    pub id:  Option<Cow<'a, str>>,
    #[xml(attr = "lang")]
    pub lang: Option<Cow<'a, str>>,
    #[xml(attr = "scheme")]
    pub scheme: Option<Cow<'a, str>>,    

    //Legacy attributes 2.0
    
    #[xml(attr = "name")]
    pub name_legacy: Option<Cow<'a, str>>,   
    #[xml(attr = "content")]
    pub content_legacy: Option<Cow<'a, str>>,      
    
}

#[derive(XmlRead,PartialEq,Debug)]
#[xml(tag = "type")]
pub struct Typefield<'a> {
    #[xml(text)]
    pub content: Cow<'a, str>,
    #[xml(attr = "dir")]
    dir: Option<Cow<'a, str>>,
    #[xml(attr = "refines")]
    refines: Option<Cow<'a, str>>,
    #[xml(attr = "property")]
    property: Cow<'a, str>,
    #[xml(attr = "id")]
    id:  Option<Cow<'a, str>>,
    #[xml(attr = "lang")]
    lang: Option<Cow<'a, str>>,
    #[xml(attr = "scheme")]
    scheme: Option<Cow<'a, str>>,    
}


#[derive(XmlRead,PartialEq,Debug)]
#[xml(tag = "manifest")]
pub struct Manifest<'a> {
    #[xml(child = "item")]
    pub items: Vec<ManifestItem<'a>>,
}

#[derive(XmlRead,PartialEq,Debug)]
#[xml(tag = "item")]
pub struct ManifestItem<'a> {
    #[xml(attr = "id")]
    pub id:  Cow<'a, str>,

    #[xml(attr = "href")]
    pub href:  Cow<'a, str>,

    #[xml(attr = "media-type")]
    pub mediatype:  Cow<'a, str>,

    #[xml(attr = "fallback")]
    pub fallback: Option<Cow<'a, str>>,

    #[xml(attr = "properties")]
    pub properties: Option<Cow<'a, str>>,

    #[xml(attr = "media-overlay")]
    pub mediaoverlay: Option<Cow<'a, str>>
}

#[derive(XmlRead,PartialEq,Debug)]
#[xml(tag = "spine")]
pub struct Spine<'a> {
    #[xml(attr = "id")]
    pub id:  Option<Cow<'a, str>>,
    
    #[xml(attr = "toc")]
    pub toc:  Option<Cow<'a, str>>,

    #[xml(attr = "page-progression-direction")]
    pub page_progression: Option<Cow<'a, str>>,

    #[xml(child = "itemref")]
    pub items: Vec<SpineItem<'a>>
}

#[derive(XmlRead,PartialEq,Debug)]
#[xml(tag = "itemref")]
pub struct SpineItem<'a> {
    #[xml(attr = "id")]
    pub id:  Option<Cow<'a, str>>,
    
    #[xml(attr = "idref")]
    pub idref:  Cow<'a, str>,
    
    #[xml(attr = "properties")]
    pub properties:  Option<Cow<'a, str>>,
    
    #[xml(attr = "linear")]
    pub linear:  Option<Cow<'a, str>>,

}

#[derive(XmlRead,PartialEq,Debug)]
#[xml(tag = "guide")]
pub struct Guide<'a> {
    #[xml(child = "reference")]
    pub references: Vec<Reference<'a>>
}

#[derive(XmlRead,PartialEq,Debug)]
#[xml(tag = "reference")]
pub struct Reference<'a> {
    #[xml(attr = "href")]
    pub href:  Cow<'a, str>,

    #[xml(attr = "title")]
    pub title:  Option<Cow<'a, str>>,

    #[xml(attr = "type")]
    pub typ:  Cow<'a, str>,
}
