

use serde::{Deserialize, Serialize};
use authcomp::{AuthT,UnAuth,Computation,AuthType,ProofStream,Encode,Decode};
use authparser::AuthDocument;

use nanoserde::ToJSON;

use std::cell::RefCell;
use std::rc::Rc;
use std::num::NonZeroUsize;
use std::iter::Map;
use std::fmt::Debug;

use selectors::attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint};
use selectors::matching;
use selectors::{Element, OpaqueElement};

use indextree::{NodeId,Arena,Descendants,NodeEdge};

use crate::selector::Simple;
use crate::selector::{PseudoElement,NonTSPseudoClass};

use html5ever::{LocalName, Namespace};

use authdoc::Node;

use crate::Selector;
use crate::Range;
use crate::DOMRange;

use splitty::*;

#[derive(Debug,Serialize,Deserialize,Clone,PartialEq,Encode,Decode,ToJSON)]
pub struct ElementRef<C>
    where   
            C:AuthType<Node>,
            C:Computation,
            C:Debug,
            C:Clone,
            C:AuthType<Arena<<C as AuthType<Node>>::AuthT>>
{
    #[n(0)] pub id: Option<NodeId>,
    #[n(1)] pub doc: Rc<AuthDocument<C>>,
    //#[n(1)] pub doc: Rc<RefCell<AuthT<Arena <AuthT<Node,C>>,C>>>,
    //pub computation : Rc<RefCell<C>>
}


impl<'a,C> ElementRef<C> 
where 
    C:AuthType<Node>,
    C:Computation,
    C:Debug,
    C:Clone,
    C:AuthType<Arena<<C as AuthType<Node>>::AuthT>>
{

    pub fn new (id: Option<NodeId>, 
                doc : AuthT<Arena <AuthT<Node,C>>,C>, 
                /*computation: C */) -> Self {
            ElementRef {
                id,
                doc: Rc::new(doc),
                //computation: Rc::new(RefCell::new(computation))
            }
    }

    fn value (&self) -> Option<Node> {
        match self.id{
            Some(id)=> {
                    let m = &*self.doc;   
                    //let b = self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m);
                    let b = m.unauth();
                    let doc = &*b.borrow();
                    match doc.get(id) {
                            Some (node) =>  {                                                
                                        //let e = unauth_node!(self,&node.get());
                                        //let eb= self.computation.borrow_mut().unauth::<Node>(&node.get());
                                        let eb = (&node.get()).unauth();
                                        let e = &*eb.borrow();
                                        //TO FIX Review this to avoid cloning
                                        Some(e.clone())
                                    }
                            None=> None
                    }
                }
            None => None
        }
    }

    pub fn select(&self, selector_str: &str) -> Vec<Rc<RefCell<Node>>>  {
        let selectors = Selector::parse(selector_str).unwrap();

        let arena_ref = (*self.doc).unauth();
        let arena = &arena_ref.borrow();

        if let Some(node_id) = self.id {
            return
                node_id.descendants(arena).filter_map (  |node_id| {
                    if let Some(arena_node) = arena.get(node_id) {
                        let auth_node = arena_node.get();
                        let node = auth_node.unauth();
                        if (*node).borrow().is_element() {
                            let element_ref = ElementRef::<C> {
                                id: Some(node_id),
                                doc: Rc::clone(&self.doc)
                            };
                            if selectors.matches_with_scope(&element_ref,None) {
                                return Some(Rc::clone(&node))
                            }
                        }
                    }
                    None
                }).collect()
        }

        Vec::new()
    }

    pub fn select_ids_with_path(&self, selector_str: &str) -> Vec<NodeId>  {

        let mut selector_list = split_unquoted_char(selector_str, ' ')
            .unwrap_quotes(true);
        
        let arena_ref = (*self.doc).unauth();
        let arena = &arena_ref.borrow();        

        let mut root_nodes = Vec::new();
        if let Some(id) = self.id {
            root_nodes.insert(0,id);
        }

        for selector_str in selector_list {

            println!("Selector {}",&selector_str);
            
            let selectors = Selector::parse(selector_str).unwrap();

            let mut result_nodeset = Vec::new();

            for node_id in root_nodes {
                    
                for node_id in node_id.children(arena) {
                        if let Some(arena_node) = arena.get(node_id) {
                            let auth_node = arena_node.get();
                            let node = auth_node.unauth();
                            let n = (*node).borrow();
                            if n.is_element() {

                                println!("Node {:?}",&n);

                                let element_ref = ElementRef::<C> {
                                    id: Some(node_id),
                                    doc: Rc::clone(&self.doc)
                                };
                                if selectors.matches_with_scope(&element_ref,None) {

                                    println!("Match {:?} {:?}",&selector_str,&n);

                                    result_nodeset.push(node_id);
                                }
                            }
                        }
                }
                
            }

            root_nodes = result_nodeset;
        }

        root_nodes
    }

    pub fn select_nodes_with_range(&self, range:&DOMRange) -> Vec<Rc<RefCell<Node>>> {

        let mut result = Vec::new();

        println!("Select nodes with range");

        if let  Some(from_id) = self.select_ids_with_path(&range.start.css_selector).get(0) {

            println!("From id {:?}",&from_id);

            if let Some(to_id) = self.select_ids_with_path(&range.end.css_selector).get(0) {

                println!("From id {:?}",&to_id);

                if let Some(root) = self.id {
                    let arena_ref = (*self.doc).unauth();
                    let arena = &arena_ref.borrow();     

                    let range = Range::new(arena,root,*from_id,*to_id);

                    for node_edge in range {
                        match node_edge {
                            NodeEdge::Start(node_id) => {
                                    
                                    if let Some(arena_node) = arena.get(node_id) {
                                        let auth_node = arena_node.get();
                                        let node = auth_node.unauth();
                                                                                
                                        result.push(Rc::clone(&node));
                                    }
                                    
                            }
                            NodeEdge::End(node_id) => {
       
                            }
                        };
                    }

        }}}

        return result;
    }

    pub fn select_nodes_with_range_fmt(&self, range:&DOMRange) -> String {

        let mut result = String::new();

        println!("Select nodes with range fmt {:?}",&range);

        if let  Some(from_id) = self.select_ids_with_path(&range.start.css_selector).get(0) {
            println!("From id {:?}",&from_id);

            if let Some(to_id) = self.select_ids_with_path(&range.end.css_selector).get(0) {
                println!("To id {:?}",&to_id);

                if let Some(root) = self.id {
                    let arena_ref = (*self.doc).unauth();
                    let arena = &arena_ref.borrow();     

                    let range = Range::new(arena,root,*from_id,*to_id);

                    for node_edge in range {
                        match node_edge {
                            NodeEdge::Start(node_id) => {
                                    
                                    if let Some(arena_node) = arena.get(node_id) {
                                        let auth_node = arena_node.get();
                                        let node = auth_node.unauth();

                                        result.push_str(&std::format!("{:+}",*node.borrow()));                                                                       
                                    }
                                    
                            }
                            NodeEdge::End(node_id) => {
                                    if let Some(arena_node) = arena.get(node_id) {
                                        let auth_node = arena_node.get();
                                        let node = auth_node.unauth();
                                        
                                        result.push_str(&std::format!("{:-}",*node.borrow()));
                                    }
                            }
                        };
                    }

        }}}

        return result;
    }

    /*
    pub fn get_computation(&self) -> Rc<RefCell<C>> {
            self.computation.clone()
    }
    */

    /*
    pub fn descendants (&self) -> Option<Map< Descendants<'_, <C as AuthType<Node>>::AuthT>,Box<dyn Fn(NodeId)->ElementRef<C>>>>{
        let node_to_elementref = Box::new( |node:NodeId| -> ElementRef<C> {
            ElementRef {
                        id:Some(node),
                        doc: self.doc.clone(),
                        computation: self.computation.clone()
                    }
        });

        match self.id{
            Some(id)=> {
                let m = &*self.doc.borrow();                    
                let doc = &*self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m).borrow();
                let doc_clone = self.doc.clone();
                let comp_clone = self.computation.clone();
                Some (id.descendants(doc).map ( Box::new(|node| {
                    ElementRef {
                        id:Some(node),
                        doc: doc_clone,
                        computation: comp_clone
                    }
                })))
            }
            None => None
        }
    } */  
}

impl<'a,C> Default for ElementRef<C> 
    where
        C:AuthType<Node>,
        C:Computation,
        C:Debug,
        C:Clone,
        C:AuthType<Arena<<C as AuthType<Node>>::AuthT>>
{
    fn default() -> Self { 
        ElementRef::<C> {
            id: None,
            doc: Rc::new(<C as AuthType<Arena<<C as AuthType<Node>>::AuthT>>>::AuthT::default()),
            //computation: Rc::new(RefCell::new(C::new(None)))
        }
    }
}


impl<C> Element for ElementRef<C>
    where   
            C:AuthType<Node>,
            C:Computation,
            //C:Computation<Node>,
            C:AuthType<Arena<<C as AuthType<Node>>::AuthT>>,
            C:Debug,
            C:Clone

{
    type Impl = Simple;

    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(&self.id)
    }

    fn parent_element(&self) -> Option<Self> {
        match self.id {
            Some(id) => {
                            let m = &*self.doc;            
                            //let db = self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m);  
                            let db = m.unauth();
                            let doc = &*db.borrow();
                            match doc.get(id) {
                            Some (node) =>  { 
                                                match node.parent() {
                                                Some(parentId)  =>   {
                                                                            Some ( ElementRef { 
                                                                                        id:Some(parentId),
                                                                                        doc: self.doc.clone(),
                                                                                        //computation: self.computation.clone()
                                                                                        }
                                                                                )
                                                                        }
                                                None => None
                                                }
                                            }

                            None => None
                    }}
            None => None
        }
    }

    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }

    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    fn is_pseudo_element(&self) -> bool {
        false
    }

    fn is_part(&self, _name: &LocalName) -> bool {
        false
    }

    fn is_same_type(&self, other: &Self) -> bool {
        match (self.id,other.id) {
            (Some(id),Some(id_other)) => {
                    let m = &*self.doc;                    
                    //let db = self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m);
                    let db = m.unauth();
                    let doc = &*db.borrow();
                    match (doc.get(id),doc.get(id_other)) {
                            (Some (node),Some(node_other)) =>  {
                                                
                                        //let e = unauth_node!(self,&node.get());
                                        //let eb = self.computation.borrow_mut().unauth::<Node>(&node.get());
                                        let eb = (&node.get()).unauth();
                                        let e = &*eb.borrow();
                                        //let e_other = unauth_node!(self,&node_other.get());
                                        //let eob = self.computation.borrow_mut().unauth::<Node>(&node_other.get());
                                        let eob = (&node_other.get()).unauth();
                                        let e_other = &*eob.borrow();
                                        e == e_other
                                    }
                            (_,_) => false
                    }}
            (_,_) => false
        }
    }

    fn exported_part(&self, _: &LocalName) -> Option<LocalName> {
        None
    }

    fn imported_part(&self, _: &LocalName) -> Option<LocalName> {
        None
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        if let Some(id) = self.id {
            //let doc = &*self.doc.borrow();
            let m = &*self.doc;                    
            //let db = self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m);
            let db = m.unauth();
            let doc = &*db.borrow();
            //Avoid the node itself
            let mut iter = id.preceding_siblings(doc).skip(1);
            
            iter.find( |sibling| {
                                    if let Some(node) = doc.get(*sibling) {
                                        //let ae = unauth_node!(self,&node.get());
                                        //let aeb = self.computation.borrow_mut().unauth::<Node>(&node.get());
                                        let aeb = (&node.get()).unauth();
                                        let ae = &*aeb.borrow();
                                        //Fix is_element
                                        //Rewrite htmlelement as Node enumeration
                                        true
                                    }else {false}
                                 }
            ).map(|nodeid| {
                ElementRef::<C> {
                    id: Some(nodeid),
                    doc: self.doc.clone(),
                    //computation: self.computation.clone()
                }
            })
        }else {None}

    }

    fn next_sibling_element(&self) -> Option<Self> {
        if let Some(id) = self.id {
            //let doc = &*self.doc.borrow();
            let m = &*self.doc;                    
            //let db = self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m);
            let db = m.unauth();
            let doc = &*db.borrow();

            let mut iter = id.following_siblings(doc).skip(1);
            iter.find( |sibling| {
                                    if let Some(node) = doc.get(*sibling) {
                                        //let ae = unauth_node!(self,&node.get());
                                        //let aeb = self.computation.borrow_mut().unauth::<Node>(&node.get());
                                        let aeb = (&node.get()).unauth();
                                        let ae = &*aeb.borrow();
                                        //Fix is_element
                                        //Rewrite htmlelement as Node enumeration
                                        true
                                    }else {false}
                                 }
            ).map(|nodeid| {
                ElementRef::<C> {
                    id: Some(nodeid),
                    doc: self.doc.clone(),
                    //computation: self.computation.clone()
                }
            })
        }else {None}

    }

    fn is_html_element_in_html_document(&self) -> bool {
        // FIXME: Is there more to this?
        //self.value().name.ns == ns!(html)
        false
    }

    fn has_local_name(&self, name: &LocalName) -> bool {
        match self.id{
            Some(id)=> {
                    let m = &*self.doc;                    
                    //let db = self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m);
                    let db = m.unauth();
                    let doc = &*db.borrow();
                    match doc.get(id) {
                            Some (node) =>  {                                                
                                        //let e = unauth_node!(self,&node.get());
                                        //let eb = self.computation.borrow_mut().unauth::<Node>(&node.get());
                                        let eb = (&node.get()).unauth();
                                        let e = &*eb.borrow();
                                        if e.is_element() {
                                            //println!("Name checking {:?} == {:?}",e.as_element().unwrap().name(), name);
                                            e.as_element().unwrap().name() == name
                                        }else {false}
                                        
                                    }
                            None=> false
                    }}
            None => false
        }
    }

    fn has_namespace(&self, namespace: &Namespace) -> bool {
        //&self.value().name.ns == namespace
        false
    }

    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&Namespace>,
        local_name: &LocalName,
        operation: &AttrSelectorOperation<&String>,
    ) -> bool {
        if let Some(node) = self.value(){
            if let Some(e) =  node.as_element(){
                e.attrs.iter().any(|(key, value)| {
                    !matches!(*ns, NamespaceConstraint::Specific(url) if *url != *key.ns)
                        && *local_name == *key.local
                        && operation.eval_str(value)
                })
            }else {false}
        }else {false}
    }

    fn match_non_ts_pseudo_class<F>(
        &self,
        _pc: &NonTSPseudoClass,
        _context: &mut matching::MatchingContext<Self::Impl>,
        _flags_setter: &mut F,
    ) -> bool {
        false
    }

    fn match_pseudo_element(
        &self,
        _pe: &PseudoElement,
        _context: &mut matching::MatchingContext<Self::Impl>,
    ) -> bool {
        false
    }

    fn is_link(&self) -> bool {
        if let Some(node) = self.value(){
            if let Some(e) =  node.as_element(){
                    e.name() == "link"
            }else {false}
        }else {false}
    }

    fn is_html_slot_element(&self) -> bool {
        true
    }

    fn has_id(&self, id: &LocalName, case_sensitivity: CaseSensitivity) -> bool {
        if let Some(node) = self.value(){
            if let Some(e) =  node.as_element(){
                match e.id {
                    Some(ref val) => case_sensitivity.eq(id.as_bytes(), val.as_bytes()),
                    None => false,
                }
            }else {false}
        }else {false}
    }

    fn has_class(&self, name: &LocalName, case_sensitivity: CaseSensitivity) -> bool {
        if let Some(node) = self.value(){
            if let Some(e) =  node.as_element(){
                e.has_class(name, case_sensitivity)
            }else {false}
        }else {false}
    }

    fn is_empty(&self) -> bool {

        let m = &*self.doc;                    
        //let db = self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m);
        let db = m.unauth();
        let doc = &*db.borrow();

        let element_or_text = | id |  {
                        
                        match doc.get(id) {
                                Some (node) =>  {                                                
                                            //let e = unauth_node!(self,&node.get());
                                            //let eb = self.computation.borrow_mut().unauth::<Node>(&node.get());
                                            let eb = (&node.get()).unauth();
                                            let e = &*eb.borrow();
                                            e.is_element() || e.is_text()
                                        }
                                None=> false
                        }
    
        };

        match self.id{
            Some(id)=> {id.children(doc).into_iter().any (element_or_text)}
          
            None => false
        }

    }

    fn is_root(&self) -> bool {
        
        match self.id {
            Some(id) => {
                let m = &*self.doc;                    
                //let db = self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m);
                let db = m.unauth();
                let doc = &*db.borrow();
                match doc.get(id) {
                 Some (node) =>  { 
                        match node.parent() {
                            Some(parentId)  =>  {
                                                if let Some(parent) = doc.get(parentId){
                                                    //let e = unauth_node!(self,&parent.get());
                                                    //let eb = self.computation.borrow_mut().unauth::<Node>(&parent.get());
                                                    let eb = (&parent.get()).unauth();
                                                    let e = &*eb.borrow();
                                                    e.is_document()
                                                }else {false}
                                                
                                            }      
                                                
                            None => false
                            }
            
                }
                None => false
            }}
            None => false
            }
    }
    
}