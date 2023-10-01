
//#![feature(type_alias_impl_trait)]

mod lexer;
mod html;
mod xml;
mod jats;

use crate::lexer::{Tokens,ModeAdapter,ModesType,Span,LexicalError};
use crate::xml::{xmlDocumentParser};

use authcomp::{AuthT,Computation,AuthType,ProofStream};
use authdoc::{Node,Element, QualName};

use anyhow::{anyhow,Result,Error};

use core::str;
use std::vec::Vec;

use indextree::{NodeId,Arena as Arena};

#[derive(Debug)]
pub struct XMLError {
    pub error: String,
}


#[derive(Default,Debug)]
pub struct Document {
    pub tree : Arena<Node>,
    pub root : Option<NodeId>
}

pub struct NodeSet<'a> {
    document: &'a Document,
    nodes : Vec<NodeId>
}


impl<'a> NodeSet<'a> {

    pub fn children (&self) -> Self {
        let mut nodeset = Vec::new();

        for n in &self.nodes {
            for r in n.children(&self.document.tree) {
                nodeset.push(r)
            }
        }

        NodeSet {
            document: self.document,
            nodes: nodeset
        }
    }

    pub fn descendants (&self) -> Self {
        let mut nodeset = Vec::new();

        for n in &self.nodes {
            for r in n.descendants(&self.document.tree) {
                nodeset.push(r)
            }
        }

        NodeSet {
            document: self.document,
            nodes: nodeset
        }
    }

    pub fn filter<P>(&self, mut predicate: P) -> Self
    where
        P: FnMut(&Node) -> bool, 
    
    {
        let mut nodeset = Vec::new();

        for n in &self.nodes {
                if let Some(r) = self.document.tree.get(*n) {
                            if predicate(r.get()) {
                                nodeset.push(*n);
                            }
                }
        }

        NodeSet {
            document: self.document,
            nodes: nodeset
        }
    }

    pub fn nav<'b,P,A>(&'b self, mut predicate: P, mut axis: A) -> Self
    where
        P: FnMut(&Node) -> bool, 
        A: FnMut(&'b NodeSet) -> NodeSet<'b>
    
    {
        let mut nodeset = Vec::new();

        for n in &axis(self).nodes {
                if let Some(r) = self.document.tree.get(*n) {
                            if predicate(r.get()) {
                                nodeset.push(*n);
                            }
                }
        }

        NodeSet {
            document: self.document,
            nodes: nodeset
        }
    }

    pub fn elements (&self) -> Self {
        self.filter (|n | n.is_element() )
    }

    pub fn with_name (&self, target: &str) -> Self {

        self.nav(|n | n.as_element().map_or(false,|e| e.name() == target) , |ns| ns.children() )
    }

}

use std::slice;
use core::iter::Map;

/*
impl<'a> IntoIterator for &'a NodeSet<'a> {
    type Item = &'a authdoc::Node;

    type IntoIter = impl Iterator<Item = &'a authdoc::Node> + 'a;

    fn into_iter(self) -> Self::IntoIter {
        (&self.nodes).into_iter().map(move |x| self.document.tree.get(*x).unwrap().get())
    }
}

*/
impl Document {

    pub fn new_from_xml(source: &[u8]) -> Self {

        let mut tree = Arena::<Node>::default();

        let root = read_unauth_xml(source, &mut tree);

        Document {
            tree,
            root
        }
    }

    pub fn find_element (&self, tag:&str) -> Result<&Element> {
        if let Some(r) = self.root {
            for n in r.descendants(&self.tree) {
                if let Some(e) = self.tree[n].get().as_element() {
                    if e.name() == tag {
                        return Ok(e)
                    }
                }
            }
        }
        Err(anyhow!(format!("Not found {:?}",tag)))
    }


    pub fn find (&self, tag:&str) -> Result<NodeSet> {
        if let Some(r) = self.root {
            for n in r.descendants(&self.tree) {
                if let Some(e) = self.tree[n].get().as_element() {
                    if e.name() == tag {
                        let mut nodes = Vec::new();
                        nodes.push(n);
                        return Ok(NodeSet {document: &self,nodes})
                    }
                }
            }
        }
        Err(anyhow!(format!("Not found {:?}",tag)))
    }    
}

fn read_unauth_xml(source: &[u8], document: &mut Arena<Node>) -> Option<NodeId> {

    let r = str::from_utf8(source);

    match r {
        Ok(s) =>  {
                let mut lexer = ModeAdapter::new();
                lexer.initialize(ModesType::Html,s);
            
                let mut root = None;
                let parser = xmlDocumentParser::new().parse::<Span<Tokens,LexicalError>,ModeAdapter>(s, document, &mut root, lexer);
            
                root
        }
        Err(_e) => None
    }

}    


use std::borrow::ToOwned;
use crate::html::{htmlDocumentParser};

pub type AuthDocument<C> = AuthT<Arena <AuthT<Node,C>>,C>;

pub fn read_auth_xml<C>(source: &[u8], auth_document: &mut AuthDocument<C>) -> Option<NodeId> 
where 
      C:AuthType<Node>,
      C:AuthType<Arena<<C as AuthType<Node>>::AuthT>>,
      C:Computation
{

    let r = str::from_utf8(source);

    match r {
        Ok(s) =>  {
                let mut lexer = ModeAdapter::new();
                lexer.initialize(ModesType::Html,s);

                let document = & mut Arena::<AuthT<Node,C>>::default();
            
                let mut root = None;
                let parser = htmlDocumentParser::new().parse::<C,Span<Tokens,LexicalError>,ModeAdapter>(s, document, &mut root, lexer);
                //authallocator::print(&alloc::format!("Parser result {:?}",parser));
                *auth_document = C::auth(document.to_owned());               

                root
        }
        Err(_e) => None
    }

}

use crate::jats::{jatsDocumentParser};

pub trait Filter<N> {

    /// The function which is used to filter something
    fn filter(&self, _: &N) -> bool;
}

pub fn read_auth_jats<C>(source: &[u8], auth_document: &mut AuthDocument<C>,filter:&dyn Filter<QualName>) -> Option<NodeId> 
where 
      C:AuthType<Node>,
      C:AuthType<Arena<<C as AuthType<Node>>::AuthT>>,
      C:Computation
{

    let r = str::from_utf8(source);

    match r {
        Ok(s) =>  {
                let mut lexer = ModeAdapter::new();
                lexer.initialize(ModesType::Html,s);

                let document = & mut Arena::<AuthT<Node,C>>::default();
            
                let mut root = None;
                let parser = jatsDocumentParser::new().parse::<C,Span<Tokens,LexicalError>,ModeAdapter>(s, document, &mut root, filter,lexer);
                //authallocator::print(&alloc::format!("Parser result {:?}",parser));
                *auth_document = C::auth(document.to_owned());               

                root
        }
        Err(_e) => None
    }

}

#[cfg(test)]
#[no_mangle]
pub extern "C"  fn test(){
    tests::all_tests();
}

#[cfg(test)]
mod tests {
    use alloc::format;
    use crate::alloc::borrow::ToOwned;
    

    pub fn all_tests() {
        //crate::tests::xml_basic();
        //crate::tests::basic();
        crate::tests::more_complex_html_test();
        crate::tests::jats_test();
        crate::tests::epub_parse_package_test();
    }
    
    fn html_basic() {

        use crate::lexer::{Tokens,ModeAdapter,ModesType,Span,LexicalError};
        use crate::html::{htmlDocumentParser};

        use authcomp::{Prover,AuthType,AuthTProver,Computation};
        use authdoc::Node;

        use indextree::{NodeId,Arena as Arena};

        let s = r#"<body hh="dddd" pp='dadf'>dadf<p />hola</body>"#;
        let mut lexer = ModeAdapter::new();
        lexer.initialize(ModesType::Html,s);

        let document = & mut Arena::<AuthTProver<Node>>::default();

        let mut root = None;
        let parser = htmlDocumentParser::new().parse::<Prover<(),()>,Span<Tokens,LexicalError>,ModeAdapter>(s, document, &mut root, lexer);

        let auth_document = Prover::<(),()>::auth(document.to_owned());

        println!("HTML Document {:?}", auth_document);
        println!("HTML Document signature {:?}", Prover::<(),()>::signature(&auth_document));
    }


    
    fn basic() {
        use crate::lexer::Tokens::*;
        use crate::lexer::*;
        use alloc::vec::Vec;

        use authdoc::Node;

        use indextree::{NodeId,Arena as Arena};
        use crate::xml::{xmlDocumentParser};


        let s = r#"<!DOCTYPE html>
                <html lang="en">
                    <head>
                        <meta charset="UTF-8"/>
                        <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                        <title>Document</title>
                        <style>
                            body {
                                background: black;
                            }
                    
                            h1 {
                                color: white;
                            }
                        </style>
                    </head>
                    <body>
                        <h1>Hello world</h1>
                        <!-- There should be more text here -->
                        <script>
                            const title = document.querySelector("h1")
                            title.innerText = "Hello from script"
                        </script>
                    </body>
                </html>        
            "#;
        let mut lexer = ModeAdapter::new();
        lexer.initialize(ModesType::Html,s);


        //let results: Vec<Span<Tokens,LexicalError>> = lexer.collect();
        
        //println!("Basic: {:?}",results);
        
        
        let document = & mut Arena::<Node>::default();

        let mut root = None;
        let parser = xmlDocumentParser::new().parse::<Span<Tokens,LexicalError>,ModeAdapter>(s, document, &mut root, lexer);

        println!("Basic {:?}", document);
        
        
    }

    
    fn xml_basic() {

        use alloc::vec::Vec;

        use crate::lexer::{Tokens,ModeAdapter,ModesType,Span,LexicalError};
        use crate::xml::{xmlDocumentParser};

        use authcomp::{Prover,AuthTProver,Computation};
        use authdoc::Node;

        use indextree::{NodeId,Arena as Arena};

        let s = r#"<?xml version='1.0' encoding='utf-8'?>
        <container xmlns="urn:oasis:names:tc:opendocument:xmlns:container" version="1.0">
          <rootfiles>
            <rootfile media-type="application/oebps-package+xml" full-path="EPUB/package.opf"/>
          </rootfiles>
        </container>"#;

        let mut lexer = ModeAdapter::new();
        lexer.initialize(ModesType::Html,s);

        let results: Vec<Span<Tokens,LexicalError>> = lexer.collect();
        
        println!("Lexer {:?}",results);

        /* 
        let document = & mut Arena::<Node>::default();

        let mut root = None;
        let parser = xmlDocumentParser::new().parse::<Span<Tokens,LexicalError>,ModeAdapter>(s, document, &mut root, lexer);

        println!("XML Document {:?}", document);
        */
    }    

    
    fn epub_parse_package_test () {

        use alloc::vec::Vec;
        use core::str::from_utf8;

        use crate::lexer::{Tokens,ModeAdapter,ModesType,Span,LexicalError};
        use crate::xml::{xmlDocumentParser};
        use authdoc::Node;

        use indextree::{NodeId,Arena as Arena};

        let doc = include_bytes!("../assets/childrens-literature/EPUB/package.opf");
        let s = from_utf8(doc).unwrap();

        let mut lexer = ModeAdapter::new();
        lexer.initialize(ModesType::Html,s);

        /*
        let results: Vec<Span<Tokens,LexicalError>> = lexer.collect();
        
        print(&format!("Package.opf: {:?}",results));
        */
        
        let document = & mut Arena::<Node>::default();

        let mut root = None;
        let parser = xmlDocumentParser::new().parse::<Span<Tokens,LexicalError>,ModeAdapter>(s, document, &mut root, lexer);

        println!("XML Document {:?}", document);
        
    }

    
    fn more_complex_html_test () {

        use alloc::vec::Vec;
        use core::str::from_utf8;

        use crate::lexer::{Tokens,ModeAdapter,ModesType,Span,LexicalError};
        use crate::xml::{xmlDocumentParser};
        use authdoc::Node;

        use indextree::{NodeId,Arena as Arena};

        println!("More complex test --");

        let doc = include_bytes!("../assets/childrens-literature/EPUB/s04.xhtml");
        let s = from_utf8(doc).unwrap();

        let mut lexer = ModeAdapter::new();
        lexer.initialize(ModesType::Html,s);

        
        /*
        let results: Vec<Span<Tokens,LexicalError>> = lexer.collect();
        
        print(&format!("More complex test: {:?}",results));
        */
        
        
        let document = & mut Arena::<Node>::default();

        let mut root = None;
        let parser = xmlDocumentParser::new().parse::<Span<Tokens,LexicalError>,ModeAdapter>(s, document, &mut root, lexer);

        println!("Parser result{:?}", parser);

        println!("Document {:?}", document);
        
    }

    fn jats_test () {

        use alloc::vec::Vec;
        use core::str::from_utf8;

        use crate::lexer::{Tokens,ModeAdapter,ModesType,Span,LexicalError};
        use crate::xml::{xmlDocumentParser};
        use authdoc::Node;

        use indextree::{NodeId,Arena as Arena};

        println!("More complex test --");

        let doc = include_bytes!("../../../pubmed/data/PMC9546530/pnas.202202536.nxml");
        let s = from_utf8(doc).unwrap();

        let mut lexer = ModeAdapter::new();
        lexer.initialize(ModesType::Html,s);

        
        /* 
        let results: Vec<Span<Tokens,LexicalError>> = lexer.collect();
        
        println!("More complex test: {:?}",results);
        */

         
        
        let document = & mut Arena::<Node>::default();

        let mut root = None;
        let parser = xmlDocumentParser::new().parse::<Span<Tokens,LexicalError>,ModeAdapter>(s, document, &mut root, lexer);

        println!("Parser result {:?}", parser);

        println!("Document {:?}", document);
        
    }

}
