use std::vec::Vec;
use std::string::String;
use std::collections::BTreeMap;
use std::boxed::Box;
use core::any::Any;

use std::format;

use anyhow::{anyhow,Result,Error};

use crate::publication::metadata::*;
use crate::publication::mediaoverlay::*;

#[derive(Default,Debug)]
pub struct Publication {
    context: Vec<String>,
    metadata: Metadata,

    links: Vec<Link>,
    reading_order: Vec<Link>,
    resources: Vec<Link>,
    toc: Vec<Link>,
    page_list: Vec<Link>,
    landmarks: Vec<Link>,

    loi: Vec<Link>,
    loa: Vec<Link>,
    lov: Vec<Link>,
    lot: Vec<Link>,

    other_links: Vec<Link>,
    other_collections: Vec<PublicationCollection>,

}

#[derive(Debug)]
pub struct Internal {
    name: String,
    value: Box<dyn Any>
}

impl Default for Internal {
    fn default() -> Self {
        Internal {
            name: String::from(""),
            value: Box::new(false)
        }
    }
}

#[derive(Default,Debug)]
pub struct PublicationCollection {
    pub role: String,
    pub metadata: Vec<Meta>,
    pub links: Vec<Link>,
    children: Vec<PublicationCollection>
}



#[derive(Default,Debug)]
pub struct Link {
    pub href : String,
    pub type_link : String,
    pub rel : Vec<String>,
    
    pub height : u32,
    pub width : u32,

    pub title : String,
    pub properties : Properties,

    pub duration : String,
    pub templated : bool,
    pub children : Vec<Link>,
    pub bitrate : u32,
    pub mediaoverlays: Vec<MediaOverlayNode>

}



impl Publication {


    fn get_cover (&self) -> Result<&Link> {
        self.search_link_by_rel("cover")
    }

    fn get_nav_doc (&self) -> Result<&Link> {
        self.search_link_by_rel("contents")
    }

    fn search_link_by_rel (&self, rel: &str ) -> Result<&Link> {

        for resource in &self.resources {
            for res_rel in &resource.rel {
                if res_rel == rel {
                    return Ok(resource)
                }
            }
        }

        for item in &self.reading_order {
            for item_rel in &item.rel {
                if item_rel == rel {
                    return Ok(item)
                }
            }
        }

        for link in &self.links {
            for link_rel in &link.rel {
                if link_rel == rel {
                    return Ok(link)
                }
            }
        }

        Err(anyhow!(format!("Can't find {:?} in publication",rel)))
    }


    fn add_link (&mut self, type_link: String, rel: Vec<String>, url:String, templated: bool) {

        let mut link = Link::default();
        link.href = url;
        link.type_link = type_link;
        link.templated = templated;

        if rel.len() > 0 {
            link.rel = rel;
        }

        self.links.push(link)
    }
    /*
        // FindAllMediaOverlay return all media overlay structure from struct
    func (publication *Publication) FindAllMediaOverlay() []MediaOverlayNode {
        var overlay []MediaOverlayNode

        for _, l := range publication.ReadingOrder {
            if len(l.MediaOverlays) > 0 {
                for _, ov := range l.MediaOverlays {
                    overlay = append(overlay, ov)
                }
            }
        }

        return overlay
    }
    */

    fn find_all_mediaoverlay (&self) -> Vec<&MediaOverlayNode> {
        let mut overlay = Vec::new();

        for link in &self.reading_order {
            if link.mediaoverlays.len() > 0 {
                for ov in &link.mediaoverlays {
                    overlay.push(ov)
                }
            }
        }

        overlay
    }

}





