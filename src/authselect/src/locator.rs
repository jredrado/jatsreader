use std::string::String;
use std::vec::Vec;
use authcomp::{Encode,Decode,Serialize,Deserialize,DeserializeOwned,DecodeOwned};

/*

{
    "href": "http://example.com/chapter1",
    "type": "text/html",
    "title": "Chapter 1",
    "locations": {
        "progression": 0.8,
        "fragments": [
            "#paragraph-id",
            "t=15"
        ],
        "cssSelector": "body.rootClass div:nth-child(2) p#paragraph-id",
        "partialCfi": "/4/2/8/6[paragraph-id]",
        "domRange": {
            "start": {
                "cssSelector": "body.rootClass div:nth-child(2) p#paragraph-id",
                "textNodeIndex": 0,
                "offset": 11
            },
            "end": {
                "cssSelector": "body.rootClass div:nth-child(2) p#paragraph-id",
                "textNodeIndex": 0,
                "offset": 22
            }
        }
    }
}

href	The URI of the resource that the Locator Object points to.	URI	Yes
type	The media type of the resource that the Locator Object points to.	Media Type	Yes
title	The title of the chapter or section which is more relevant in the context of this locator.	String	No
locations	One or more alternative expressions of the location.	Location Object	No
text	Textual context of the locator.	Text Object	No
*/

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub struct SimplifiedLocator {
    #[n(0)] pub href : String,
    #[n(1)] pub media_type : String,
    #[n(2)] pub from_css_selector : String,
    #[n(3)] pub to_css_selector : String,  
}

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub struct Locator {
    #[n(0)] pub href : String,
    #[n(1)] pub media_type : String,
    #[n(2)] pub title : Option<String>,
    #[n(3)] pub locations : Vec<Location>,
    #[n(4)] pub text : Option<TextContext>
}

impl Locator {

    fn new_with_range(href: String, media_type: String, range:DOMRange) -> Locator {

        let mut location = Location::default();
        location.dom_range = Some(range);

        let mut locator = Locator::default();
        locator.href = href;
        locator.media_type = media_type;
        locator.locations.insert(0,location);
        
        locator
    }
}

/*
fragments	Contains one or more fragment in the resource referenced by the Locator Object.	Array of strings	No
progression	Progression in the resource expressed as a percentage.	Float between 0 and 1	No
position	An index in the publication.	Integer where the value is > 1	No
totalProgression	Progression in the publication expressed as a percentage.	Float between 0 and 1	No
*/

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub struct Location {
    #[n(0)] pub fragments: Vec<String>,
    #[n(1)] pub progression: Option<f32>,
    #[n(2)] pub position: Option<u32>,
    #[n(3)] pub totalProgression: Option<u32>,
    #[n(4)] pub css_selector : Option<String>,
    #[n(5)] pub partial_cfi : Option<String>,
    #[n(6)] pub dom_range: Option<DOMRange>

}

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub struct DOMRange {
    #[n(0)] pub start: DOMIndex,
    #[n(1)] pub end: DOMIndex

}
#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub struct DOMIndex {
    #[n(0)] pub css_selector: String,
    #[n(1)] pub text_node_index: Option<u32>,
    #[n(2)] pub offset: Option<u32>
}

/*
after	The text after the locator.	String	No
before	The text before the locator.	String	No
highlight	The text at the locator.	String	No
*/

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub struct TextContext {
    #[n(0)] after: Option<String>,
    #[n(1)] before: Option<String>,
    #[n(2)] highlight: Option<String>
}