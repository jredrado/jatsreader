//! HTML nodes.


use std::collections::{BTreeMap as HashMap, BTreeSet as HashSet};
use std::collections::{btree_map as hash_map,btree_set as hash_set};

use std::fmt;
use std::vec::Vec;
use core::ops::Deref;

use crate::string::NodeString;

pub type Prefix = NodeString;
pub type LocalName = NodeString;
pub type Namespace = NodeString;
pub type Str = NodeString;

use serde::{Deserialize, Serialize};


use minicbor::{Encode,Decode};
use nanoserde::{ToJSON,SerJson};

use crate::format::PartialDisplay;

/// An HTML node.
/// 
/// 

#[derive(Serialize,Deserialize,Clone, PartialEq, Eq,Encode,Decode,ToJSON)]
pub enum Node {
    /// The document root.
    #[n(0)] Document,

    /// The fragment root.
    #[n(1)] Fragment,

    /// A doctype.
    #[n(2)] Doctype( #[n(0)] Doctype),

    /// A comment.
    #[n(3)] Comment( #[n(0)] Comment),

    /// Text.
    #[n(4)] Text(#[n(0)] Text),

    /// An element.
    #[n(5)] Element( #[n(0)] Element),

    /// A processing instruction.
    #[n(6)] ProcessingInstruction(#[n(0)] ProcessingInstruction),
}

impl Default for Node {
    fn default() -> Self { 
            Node::Comment (Comment {comment: Str::from("")})
    }
}

impl Node {
    /// Returns true if node is the document root.
    pub fn is_document(&self) -> bool {
        match *self {
            Node::Document => true,
            _ => false,
        }
    }

    /// Returns true if node is the fragment root.
    pub fn is_fragment(&self) -> bool {
        match *self {
            Node::Fragment => true,
            _ => false,
        }
    }

    /// Returns true if node is a doctype.
    pub fn is_doctype(&self) -> bool {
        match *self {
            Node::Doctype(_) => true,
            _ => false,
        }
    }

    /// Returns true if node is a comment.
    pub fn is_comment(&self) -> bool {
        match *self {
            Node::Comment(_) => true,
            _ => false,
        }
    }

    /// Returns true if node is text.
    pub fn is_text(&self) -> bool {
        match *self {
            Node::Text(_) => true,
            _ => false,
        }
    }

    /// Returns true if node is an element.
    pub fn is_element(&self) -> bool {
        match *self {
            Node::Element(_) => true,
            _ => false,
        }
    }

    /// Returns self as a doctype.
    pub fn as_doctype(&self) -> Option<&Doctype> {
        match *self {
            Node::Doctype(ref d) => Some(d),
            _ => None,
        }
    }

    /// Returns self as a comment.
    pub fn as_comment(&self) -> Option<&Comment> {
        match *self {
            Node::Comment(ref c) => Some(c),
            _ => None,
        }
    }

    /// Returns self as text.
    pub fn as_text(&self) -> Option<&Text> {
        match *self {
            Node::Text(ref t) => Some(t),
            _ => None,
        }
    }

    pub fn as_mutable_text(&mut self) -> Option<&mut Text> {
        match *self {
            Node::Text(ref mut t) => Some(t),
            _ => None,
        }
    }

    /// Returns self as an element.
    pub fn as_element(&self) -> Option<&Element> {
        match *self {
            Node::Element(ref e) => Some(e),
            _ => None,
        }
    }

    /// Returns self as an element.
    pub fn as_processing_instruction(&self) -> Option<&ProcessingInstruction> {
        match *self {
            Node::ProcessingInstruction(ref pi) => Some(pi),
            _ => None,
        }
    }
}

// Always use one line.
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Node::Document => write!(f, "Document"),
            Node::Fragment => write!(f, "Fragment"),
            Node::Doctype(ref d) => write!(f, "Doctype({:?})", d),
            Node::Comment(ref c) => write!(f, "Comment({:?})", c),
            Node::Text(ref t) => write!(f, "Text({:?})", t),
            Node::Element(ref e) => write!(f, "Element({:?})", e),
            Node::ProcessingInstruction(ref pi) => write!(f, "ProcessingInstruction({:?})", pi),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if f.sign_plus(){
            self.start_fmt(f)
        }
        else if f.sign_minus() {
            self.end_fmt(f)
        }else {Ok(())}
    }
}

impl crate::format::PartialDisplay for Node {
    fn start_fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Node::Document => Ok(()),
            Node::Fragment => Ok(()),
            Node::Doctype(ref d) => d.start_fmt(f),
            Node::Comment(ref c) => c.start_fmt(f),
            Node::Text(ref t) => t.start_fmt(f),
            Node::Element(ref e) => e.start_fmt(f),
            Node::ProcessingInstruction(ref pi) => pi.start_fmt(f),
        }
    }
    fn end_fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Node::Document => Ok(()),
            Node::Fragment => Ok(()),
            Node::Doctype(ref d) => d.end_fmt(f),
            Node::Comment(ref c) => c.end_fmt(f),
            Node::Text(ref t) => t.end_fmt(f),
            Node::Element(ref e) => e.end_fmt(f),
            Node::ProcessingInstruction(ref pi) => pi.end_fmt(f),
        }
    }    
}

/// A doctype.
#[derive(Serialize,Deserialize,Clone, PartialEq, Eq, Encode, Decode, ToJSON)]
pub struct Doctype {
    /// The doctype name.
    #[n(0)] pub name: Str,

    /// The doctype public ID.
    #[n(1)] pub public_id: Str,

    /// The doctype system ID.
    #[n(2)] pub system_id: Str,
}



impl Doctype {
    /// Returns the doctype name.
    pub fn name(&self) -> &str {
        self.name.deref()
    }

    /// Returns the doctype public ID.
    pub fn public_id(&self) -> &str {
        self.public_id.deref()
    }

    /// Returns the doctype system ID.
    pub fn system_id(&self) -> &str {
        self.system_id.deref()
    }
}

impl fmt::Debug for Doctype {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "<!DOCTYPE {} PUBLIC {:?} {:?}>",
            self.name(),
            self.public_id(),
            self.system_id()
        )
    }
}


impl crate::format::PartialDisplay for Doctype {
    fn start_fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "<!DOCTYPE {} PUBLIC {} {}>",
            self.name(),
            self.public_id(),
            self.system_id()
        )
    }
    fn end_fmt(&self, _f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        Ok(())
    }    
}

/// An HTML comment.
#[derive(Serialize,Deserialize,Clone, PartialEq, Eq,Encode,Decode, ToJSON)]
pub struct Comment {
    /// The comment text.
    #[n(0)] pub comment: Str,
}

impl Deref for Comment {
    type Target = str;

    fn deref(&self) -> &str {
        self.comment.deref()
    }
}

impl fmt::Debug for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<!-- {:?} -->", self.deref())
    }
}

impl crate::format::PartialDisplay for Comment {
    fn start_fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<!-- {} -->", self.deref())
    }

    fn end_fmt(&self, _f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        Ok(())
    }

}

/// HTML text.
#[derive(Serialize,Deserialize,Clone, PartialEq, Eq,Encode,Decode, ToJSON)]
pub struct Text {
    /// The text.
    #[n(0)] pub text: Str,
}

impl Deref for Text {
    type Target = str;

    fn deref(&self) -> &str {
        self.text.deref()
    }
}

impl fmt::Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.deref())
    }
}

impl crate::format::PartialDisplay for Text {
    fn start_fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.deref())
    }
    fn end_fmt(&self, _f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        Ok(())
    }    
}

#[derive(Hash,Serialize, Deserialize,Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Encode,Decode, ToJSON)]
pub struct QualName {
    #[n(0)] pub prefix: Option<Prefix>,
    #[n(1)] pub ns: Namespace,
    #[n(2)] pub local: LocalName,
}

impl QualName {

    pub fn new (prefix: Option<Prefix>, ns: Namespace, local: LocalName ) -> Self {
        QualName {
            prefix: prefix,
            ns: ns,
            local: local
        }
    }
}

type AttrMap = HashMap<QualName,Str>;

#[derive(Serialize,Deserialize,Clone, PartialEq, Eq,Debug,Encode,Decode,ToJSON)]
pub struct Attribute {
    #[n(0)] pub name: QualName,
    #[n(1)] pub value: Str,
}

/// An HTML element.
#[derive(Serialize,Deserialize,Clone, PartialEq, Eq,Encode, Decode, ToJSON)]
pub struct Element {
    /// The element name.

    #[n(0)] pub name: QualName,

    /// The element ID.
    #[n(1)] pub id: Option<LocalName>,

    /// The element classes.
    #[n(2)] pub classes: HashSet<LocalName>,

    /// The element attributes.
    #[n(3)] pub attrs: AttrMap,
}

impl Default for Element {
    fn default() -> Self { 
        Element {
            attrs: HashMap::new(),
            name: QualName::new(None, Namespace::from(""), LocalName::from("")),
            id: None,
            classes: HashSet::new()
        }
    }
}


impl Element {
    #[doc(hidden)]
    pub fn new(name: QualName, attrs: Vec<Attribute>) -> Self {
        let id = attrs
            .iter()
            .find(|a| a.name.local.deref() == "id")
            .map(|a| LocalName::from(a.value.deref()));

        let classes: HashSet<LocalName> = attrs
            .iter()
            .find(|a| a.name.local.deref() == "class")
            .map_or(HashSet::new(), |a| {
                a.value
                    .deref()
                    .split_whitespace()
                    .map(LocalName::from)
                    .collect()
            });

        Element {
            attrs: attrs.into_iter().map(|a| (a.name, a.value)).collect(),
            name,
            id,
            classes,
        }
    }

    /// Returns the element name.
    pub fn name(&self) -> &str {
        self.name.local.deref()
    }

    /// Returns the element ID.
    pub fn id(&self) -> Option<&str> {
        self.id.as_ref().map(Deref::deref)
    }

    /// Returns true if element has the class.
    pub fn has_class(&self, class: &str, case_sensitive: CaseSensitivity) -> bool {
        self.classes()
            .any(|c| case_sensitive.eq(c.as_bytes(), class.as_bytes()))
    }

    /// Returns an iterator over the element's classes.
    pub fn classes(&self) -> Classes {
        Classes {
            inner: self.classes.iter(),
        }
    }

    /// Returns the value of an attribute.
    pub fn attr(&self, attr: &str) -> Option<&str> {
        let qualname = QualName::new(None, Namespace::from(""), LocalName::from(attr));
        self.attrs.get(&qualname).map(|x| x.deref())
    }

    /// Returns an iterator over the element's attributes.
    pub fn attrs(&self) -> Attrs {
        Attrs {
            inner: self.attrs.iter(),
        }
    }
}

/// Iterator over classes.
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct Classes<'a> {
    inner: hash_set::Iter<'a, LocalName>,
}

impl<'a> Iterator for Classes<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.inner.next().map(Deref::deref)
    }
}

/// Iterator over attributes.
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct Attrs<'a> {
    inner: hash_map::Iter<'a, QualName, Str>,
}

impl<'a> Iterator for Attrs<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<(&'a str, &'a str)> {
        self.inner.next().map(|(k, v)| (k.local.deref(), v.deref()))
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{}", self.name())?;
        for (key, value) in self.attrs() {
            write!(f, " {}={:?}", key, value)?;
        }
        write!(f, ">")
    }
}

impl crate::format::PartialDisplay for Element {
    
    fn start_fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{}", self.name())?;
        for (key, value) in self.attrs() {
            write!(f, " {}={}", key, value)?;
        }
        write!(f, ">")
    }

    fn end_fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "</{}>", self.name())
    }
}

/// HTML Processing Instruction.
#[derive(Serialize,Deserialize,Clone, PartialEq, Eq,Encode,Decode, ToJSON)]
pub struct ProcessingInstruction {
    /// The PI target.
    #[n(0)] pub target: Str,
    /// The PI data.
    #[n(1)] pub data: Str,
}

impl Deref for ProcessingInstruction {
    type Target = str;

    fn deref(&self) -> &str {
        self.data.deref()
    }
}

impl fmt::Debug for ProcessingInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?} {:?}", self.target, self.data)
    }
}

impl crate::format::PartialDisplay for ProcessingInstruction {
    fn start_fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?} {}", self.target, self.data)
    }
    fn end_fmt(&self, _f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        Ok(())
    }    
}

// From https://docs.rs/selectors/0.22.0/src/selectors/attr.rs.html

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CaseSensitivity {
    CaseSensitive,
    AsciiCaseInsensitive,
}

impl CaseSensitivity {
    pub fn eq(self, a: &[u8], b: &[u8]) -> bool {
        match self {
            CaseSensitivity::CaseSensitive => a == b,
            CaseSensitivity::AsciiCaseInsensitive => a.eq_ignore_ascii_case(b),
        }
    }

    pub fn contains(self, haystack: &str, needle: &str) -> bool {
        match self {
            CaseSensitivity::CaseSensitive => haystack.contains(needle),
            CaseSensitivity::AsciiCaseInsensitive => {
                if let Some((&n_first_byte, n_rest)) = needle.as_bytes().split_first() {
                    haystack.bytes().enumerate().any(|(i, byte)| {
                        if !byte.eq_ignore_ascii_case(&n_first_byte) {
                            return false;
                        }
                        let after_this_byte = &haystack.as_bytes()[i + 1..];
                        match after_this_byte.get(..n_rest.len()) {
                            None => false,
                            Some(haystack_slice) => haystack_slice.eq_ignore_ascii_case(n_rest),
                        }
                    })
                } else {
                    // any_str.contains("") == true,
                    // though these cases should be handled with *NeverMatches and never go here.
                    true
                }
            },
        }
    }
}