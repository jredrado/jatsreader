use std::vec::Vec;
use std::string::String;
use std::collections::btree_map::BTreeMap;
use std::boxed::Box;
use core::any::Any;

#[derive(Default,Debug)]
pub struct Metadata {

    pub rdftype: String,
    pub title: MultiLanguage,
    pub identifier: String,
    pub author: Vec<Contributor>,
    pub translator: Vec<Contributor>,
    pub editor: Vec<Contributor>,
    pub artist: Vec<Contributor>,
    pub illustrator: Vec<Contributor>,
    pub letterer: Vec<Contributor>,
    pub penciler: Vec<Contributor>,
    pub colorist: Vec<Contributor>,
    pub inker: Vec<Contributor>,
    pub narrator: Vec<Contributor>,
    pub contributor: Vec<Contributor>,
    pub publisher: Vec<Contributor>,
    pub imprint: Vec<Contributor>,
    pub language: Vec<String>,
    pub modified: String, //TO FIX
    pub publication_date: String, //TO FIX
    pub description: String,
    pub direction: String,
    pub presentation: Properties,
    pub source: String,
    pub epub_type: Vec<String>,
    pub rights: String,
    pub subject: Vec<Subject>,
    pub belongs_to: BelongsTo,
    pub duration: u32,

    pub other_metadata: Vec<Meta>
    
}

#[derive(Debug)]
pub struct Meta {
    property: String,
    value: Box<dyn Any>,
    children: Vec<Meta>
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            property: String::default(),
            children: Vec::default(),
            value: Box::new(false)
        }
    }
}

#[derive(Debug,Default)]
pub struct Contributor {
    name: MultiLanguage,
    sort_as: String,
    identifier: String,
    role: String
}


#[derive(Default,Debug)]
pub struct Properties {
    contains: Vec<String>,
    layout: String,
    mediaoverlay: String,
    orientation: String,
    overflow: String,
    page: String,
    spread: String,
    //encrypted
}

#[derive(Debug,Default)]
pub struct Subject {
    name: String,
    sort_as: String,
    scheme: String,
    code: String
}

#[derive(Debug)]
pub enum MultiLanguage {
    SingleString(String),
    MultiString(BTreeMap<String,String>)
}

impl Default for MultiLanguage {
    fn default() -> Self {
        MultiLanguage::SingleString(String::from(""))
    }
}



#[derive(Debug,Default)]
pub struct BelongsTo {
    series: Vec<Collection>,
    collection: Vec<Collection>
}

#[derive(Debug,Default)]
pub struct Collection {
    name: String,
    sort_as: String,
    identifier: String,
    position: f32
}
