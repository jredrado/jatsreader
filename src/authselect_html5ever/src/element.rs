

use serde::{Deserialize, Serialize};
use authcomp::{AuthT,Computation,AuthType,ProofStream};

use std::cell::RefCell;
use std::rc::Rc;
use std::num::NonZeroUsize;
use std::iter::Map;

use selectors::attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint};
use selectors::matching;
use selectors::{Element, OpaqueElement};

use indextree::{NodeId,Arena,Descendants};

use crate::selector::Simple;
use crate::selector::{PseudoElement,NonTSPseudoClass};

use html5ever::{LocalName, Namespace};

use authdoc::Node;

#[derive(Debug,Clone)]
pub struct ElementRef<C>
    where   
            C:AuthType<Node>,
            C:Computation,
            C:AuthType<Arena<<C as AuthType<Node>>::AuthT>>
{
    pub id: Option<NodeId>,
    pub doc: Rc<RefCell<AuthT<Arena <AuthT<Node,C>>,C>>>,
    pub computation : Rc<RefCell<C>>
}


impl<'a,C> ElementRef<C> 
where 
    C:AuthType<Node>,
    C:Computation,
    C:AuthType<Arena<<C as AuthType<Node>>::AuthT>>
{

    pub fn new (id: Option<NodeId>, 
                doc : AuthT<Arena <AuthT<Node,C>>,C>, 
                computation: C ) -> Self {
            ElementRef {
                id,
                doc: Rc::new(RefCell::new(doc)),
                computation: Rc::new(RefCell::new(computation))
            }
    }

    fn value (&self) -> Option<Node> {
        match self.id{
            Some(id)=> {
                    let m = &*self.doc.borrow();   
                    let b = self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m);
                    let doc = &*b.borrow();
                    match doc.get(id) {
                            Some (node) =>  {                                                
                                        //let e = unauth_node!(self,&node.get());
                                        let eb= self.computation.borrow_mut().unauth::<Node>(&node.get());
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

    pub fn get_computation(&self) -> Rc<RefCell<C>> {
            self.computation.clone()
    }

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
        C:AuthType<Arena<<C as AuthType<Node>>::AuthT>>
{
    fn default() -> Self { 
        ElementRef::<C> {
            id: None,
            doc: Rc::new(RefCell::new(<C as AuthType<Arena<<C as AuthType<Node>>::AuthT>>>::AuthT::default())),
            computation: Rc::new(RefCell::new(C::new(None)))
        }
    }
}


impl<C> Element for ElementRef<C>
    where   
            C:AuthType<Node>,
            C:Computation,
            //C:Computation<Node>,
            C:AuthType<Arena<<C as AuthType<Node>>::AuthT>>,
            C:std::fmt::Debug,
            C:Clone

{
    type Impl = Simple;

    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(&self.id)
    }

    fn parent_element(&self) -> Option<Self> {
        match self.id {
            Some(id) => {
                            let m = &*self.doc.borrow();            
                            let db = self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m);        
                            let doc = &*db.borrow();
                            match doc.get(id) {
                            Some (node) =>  { 
                                                match node.parent() {
                                                Some(parentId)  =>   {
                                                                            Some ( ElementRef { 
                                                                                        id:Some(parentId),
                                                                                        doc: self.doc.clone(),
                                                                                        computation: self.computation.clone()
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
                    let m = &*self.doc.borrow();                    
                    let db = self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m);
                    let doc = &*db.borrow();
                    match (doc.get(id),doc.get(id_other)) {
                            (Some (node),Some(node_other)) =>  {
                                                
                                        //let e = unauth_node!(self,&node.get());
                                        let eb = self.computation.borrow_mut().unauth::<Node>(&node.get());
                                        let e = &*eb.borrow();
                                        //let e_other = unauth_node!(self,&node_other.get());
                                        let eob = self.computation.borrow_mut().unauth::<Node>(&node_other.get());
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
            let m = &*self.doc.borrow();                    
            let db = self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m);
            let doc = &*db.borrow();
            //Avoid the node itself
            let mut iter = id.preceding_siblings(doc).skip(1);
            
            iter.find( |sibling| {
                                    if let Some(node) = doc.get(*sibling) {
                                        //let ae = unauth_node!(self,&node.get());
                                        let aeb = self.computation.borrow_mut().unauth::<Node>(&node.get());
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
                    computation: self.computation.clone()
                }
            })
        }else {None}

    }

    fn next_sibling_element(&self) -> Option<Self> {
        if let Some(id) = self.id {
            //let doc = &*self.doc.borrow();
            let m = &*self.doc.borrow();                    
            let db = self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m);
            let doc = &*db.borrow();

            let mut iter = id.following_siblings(doc).skip(1);
            iter.find( |sibling| {
                                    if let Some(node) = doc.get(*sibling) {
                                        //let ae = unauth_node!(self,&node.get());
                                        let aeb = self.computation.borrow_mut().unauth::<Node>(&node.get());
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
                    computation: self.computation.clone()
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
                    let m = &*self.doc.borrow();                    
                    let db = self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m);
                    let doc = &*db.borrow();
                    match doc.get(id) {
                            Some (node) =>  {                                                
                                        //let e = unauth_node!(self,&node.get());
                                        let eb = self.computation.borrow_mut().unauth::<Node>(&node.get());
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

        let m = &*self.doc.borrow();                    
        let db = self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m);
        let doc = &*db.borrow();

        let element_or_text = | id |  {
                        
                        match doc.get(id) {
                                Some (node) =>  {                                                
                                            //let e = unauth_node!(self,&node.get());
                                            let eb = self.computation.borrow_mut().unauth::<Node>(&node.get());
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
                let m = &*self.doc.borrow();                    
                let db = self.computation.borrow_mut().unauth::<Arena<<C as AuthType<Node>>::AuthT>>(m);
                let doc = &*db.borrow();
                match doc.get(id) {
                 Some (node) =>  { 
                        match node.parent() {
                            Some(parentId)  =>  {
                                                if let Some(parent) = doc.get(parentId){
                                                    //let e = unauth_node!(self,&parent.get());
                                                    let eb = self.computation.borrow_mut().unauth::<Node>(&parent.get());
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