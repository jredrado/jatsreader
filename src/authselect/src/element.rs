
use authcomp::{AuthT,UnAuth,Computation,AuthType};
use authdoc::{Node,QualName,NodeString};
use authparser::AuthDocument;

use authcomp::{Encode,Decode,Serialize,Deserialize,DeserializeOwned,DecodeOwned};
use nanoserde::ToJSON;

use core::cell::RefCell;
use std::rc::Rc;
use std::string::String;
use std::vec::Vec;

use indextree::{NodeId,Arena,NodeEdge};

use simplecss::*;

use crate::cfi::{CFIComponent,CFIComponentList};
use crate::types::*;

use std::format;

use crate::Range;
use crate::DOMRange;

use splitty::*;

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode,ToJSON)]
pub struct ElementRef<C>
    where   
            C:Computation,
            C:AuthType<Node>,
            C:AuthType<Arena<<C as AuthType<Node>>::AuthT>>
{
    #[n(0)] pub id: Option<NodeId>,
    #[n(1)] pub doc: Rc<AuthDocument<C>>,
}


impl<C> ElementRef<C> 
    where   
        C:Computation,
        C:AuthType<Node>,
        C:AuthType<Arena<<C as AuthType<Node>>::AuthT>>
{

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
                            if selectors.matches(&element_ref) {
                                return Some(Rc::clone(&node))
                            }
                        }
                    }
                    None
                }).collect()
        }

        Vec::new()
    }

    pub fn select_fmt(&self, selector_str: &str) -> Vec<String>  {
        let selectors = Selector::parse(selector_str).unwrap();


        let arena_ref = (*self.doc).unauth();
        let arena = &arena_ref.borrow();        

        if let Some(node_id) = self.id {
            return
                node_id.descendants(arena)
                    .filter (  |node_id| {
                            if let Some(arena_node) = arena.get(*node_id) {
                                let auth_node = arena_node.get();
                                let node = auth_node.unauth();
                                if (*node).borrow().is_element() {
                                    let element_ref = ElementRef::<C> {
                                        id: Some(*node_id),
                                        doc: Rc::clone(&self.doc)
                                    };
                                    return selectors.matches(&element_ref)
                                }
                            }
                            false
                        })
                    .map ( | id | {
                        
                        id.traverse(arena).fold(String::from(""), |result, node_edge|{
                            let fmt_string = match node_edge {
                                NodeEdge::Start(node_id) => {
                                        if let Some(arena_node) = arena.get(node_id) {
                                            let auth_node = arena_node.get();
                                            let node = auth_node.unauth();
                                            
                                            std::format!("{:+}",*node.borrow())
                                        }else {String::from("")}
                                }
                                NodeEdge::End(node_id) => {
                                        if let Some(arena_node) = arena.get(node_id) {
                                            let auth_node = arena_node.get();
                                            let node = auth_node.unauth();
                                            
                                            std::format!("{:-}",*node.borrow())
                                        }else {String::from("")}
                                }
                            };

                            result + &fmt_string
                        })
                
                    }).collect()
        }

        Vec::new()
    }    

    pub fn select_ids(&self, selector_str: &str) -> Vec<NodeId>  {
        let selectors = Selector::parse(selector_str).unwrap();

        let arena_ref = (*self.doc).unauth();
        let arena = &arena_ref.borrow();

        if let Some(node_id) = self.id {
            return
                node_id.descendants(arena).filter (  |node_id| {
                    if let Some(arena_node) = arena.get(*node_id) {
                        let auth_node = arena_node.get();
                        let node = auth_node.unauth();
                        if (*node).borrow().is_element() {
                            let element_ref = ElementRef::<C> {
                                id: Some(*node_id),
                                doc: Rc::clone(&self.doc)
                            };
                            return selectors.matches(&element_ref)
                        }
                    }
                    false
                }).collect()
        }

        Vec::new()
    }    

    fn fmt_node<A:core::fmt::Display,T:UnAuth<A>> (node_id: NodeId, arena: &Arena<T>) -> String {
        let mut result = String::new();

        for node_edge in node_id.traverse(arena) {
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

        result
    }

    pub fn select_fmt_alt(&self, selector_str: &str) -> Vec<String>  {
        let selectors = Selector::parse(selector_str).unwrap();


        let arena_ref = (*self.doc).unauth();
        let arena = &arena_ref.borrow();        

        let mut result = Vec::new();

        if let Some(node_id) = self.id {
            
            for node_id in node_id.descendants(arena) {
                if let Some(arena_node) = arena.get(node_id) {
                    let auth_node = arena_node.get();
                    let node = auth_node.unauth();
                    if (*node).borrow().is_element() {
                        let element_ref = ElementRef::<C> {
                            id: Some(node_id),
                            doc: Rc::clone(&self.doc)
                        };
                        if selectors.matches(&element_ref) {
                            result.push( Self::fmt_node(node_id,arena) )
                        }
                    }
                }
            }
        }  
    
        result
    }
 
    pub fn select_with_path(&self, selector_str: &str) -> Vec<String>  {

        let mut selector_list = split_unquoted_char(selector_str, ' ')
            .unwrap_quotes(true);
        
        let arena_ref = (*self.doc).unauth();
        let arena = &arena_ref.borrow();        

        let mut root_nodes = Vec::new();
        if let Some(id) = self.id {
            root_nodes.insert(0,id);
        }

        for selector_str in selector_list {
            let selectors = Selector::parse(selector_str).unwrap();

            let mut result_nodeset = Vec::new();

            for node_id in root_nodes {
                    
                for node_id in node_id.children(arena) {
                        if let Some(arena_node) = arena.get(node_id) {
                            let auth_node = arena_node.get();
                            let node = auth_node.unauth();
                            if (*node).borrow().is_element() {
                                let element_ref = ElementRef::<C> {
                                    id: Some(node_id),
                                    doc: Rc::clone(&self.doc)
                                };
                                if selectors.matches(&element_ref) {
                                    result_nodeset.push(node_id);
                                }
                            }
                        }
                }
                
            }

            root_nodes = result_nodeset;
        }

        
        let mut result = Vec::new();

        for node_id in root_nodes {
            result.push( Self::fmt_node(node_id,arena))
        }

        result
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
                            if (*node).borrow().is_element() {
                                let element_ref = ElementRef::<C> {
                                    id: Some(node_id),
                                    doc: Rc::clone(&self.doc)
                                };
                                if selectors.matches(&element_ref) {
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

    pub fn select_cfi_node_id(&self, cfi_components: &CFIComponentList) -> Option<NodeId>  {

        let arena_ref = (*self.doc).unauth();
        let arena = &arena_ref.borrow();        

        if let Some(node_id) = self.id {

            let mut current_node = node_id;
            //print(&format!("Root: {:?}",current_node));

            for cfi_component in cfi_components {
                let (even,index) = if cfi_component.node_index % 2 == 0  {(true,(cfi_component.node_index / 2)-1)}else {(false,cfi_component.node_index / 2)}; //odd -> text children

                //print(&format!("Even,index: {:?},{:?},{:?}",even,index,cfi_component.node_index));

                let result_node = if even {
                            current_node
                                .children(&arena)
                                .filter (  |node_id| {
                                    if let Some(arena_node) = arena.get(*node_id) {
                                        let auth_node = arena_node.get();
                                        let node = auth_node.unauth();
                                        return (*node).borrow().is_element();
                                    }
                                    false
                                }).for_each(|node_id| {
                                    if let Some(arena_node) = arena.get(node_id) {
                                        let auth_node = arena_node.get();
                                        let node = auth_node.unauth();
                                        //print(&format!("Element: {:?}",node));
                                    }
                                    
                                });
                            
                            current_node
                                .children(&arena)
                                .filter (  |node_id| {
                                    if let Some(arena_node) = arena.get(*node_id) {
                                        let auth_node = arena_node.get();
                                        let node = auth_node.unauth();
                                        return (*node).borrow().is_element();
                                    }
                                    false
                                })
                                .nth(index)
                        }else {
                            current_node
                            .children(&arena)
                            .filter (  |node_id| {
                                if let Some(arena_node) = arena.get(*node_id) {
                                    let auth_node = arena_node.get();
                                    let node = auth_node.unauth();
                                    return (*node).borrow().is_text();
                                }
                                false
                            })
                            .for_each(|node_id| {
                                if let Some(arena_node) = arena.get(node_id) {
                                    let auth_node = arena_node.get();
                                    let node = auth_node.unauth();
                                    //print(&format!("Text: {:?}",node));
                                }
                                
                            });
                        
                            current_node
                                .children(&arena)
                                .filter (  |node_id| {
                                    if let Some(arena_node) = arena.get(*node_id) {
                                        let auth_node = arena_node.get();
                                        let node = auth_node.unauth();
                                        return (*node).borrow().is_text();
                                    }
                                    false
                                })
                                .nth(index)
                        };

                //print(&format!("Result node {:?}",result_node));

                if result_node.is_some() {
                    current_node = result_node.unwrap();
                }else {return None}
            }

            return Some(current_node)
        }

        return None;

    }
    pub fn select_cfi(&self, cfi_components: &CFIComponentList) -> Option<Rc<RefCell<Node>>>  {

        let arena_ref = (*self.doc).unauth();
        let arena = &arena_ref.borrow();        

        if let Some(node_id) = self.id {

            let mut current_node = node_id;
            //print(&format!("Root: {:?}",current_node));

            for cfi_component in cfi_components {
                let (even,index) = if cfi_component.node_index % 2 == 0  {(true,(cfi_component.node_index / 2)-1)}else {(false,cfi_component.node_index / 2)}; //odd -> text children

                //print(&format!("Even,index: {:?},{:?},{:?}",even,index,cfi_component.node_index));

                let result_node = if even {
                            current_node
                                .children(&arena)
                                .filter (  |node_id| {
                                    if let Some(arena_node) = arena.get(*node_id) {
                                        let auth_node = arena_node.get();
                                        let node = auth_node.unauth();
                                        return (*node).borrow().is_element();
                                    }
                                    false
                                }).for_each(|node_id| {
                                    if let Some(arena_node) = arena.get(node_id) {
                                        let auth_node = arena_node.get();
                                        let node = auth_node.unauth();
                                        //print(&format!("Element: {:?}",node));
                                    }
                                    
                                });
                            
                            current_node
                                .children(&arena)
                                .filter (  |node_id| {
                                    if let Some(arena_node) = arena.get(*node_id) {
                                        let auth_node = arena_node.get();
                                        let node = auth_node.unauth();
                                        return (*node).borrow().is_element();
                                    }
                                    false
                                })
                                .nth(index)
                        }else {
                            current_node
                            .children(&arena)
                            .filter (  |node_id| {
                                if let Some(arena_node) = arena.get(*node_id) {
                                    let auth_node = arena_node.get();
                                    let node = auth_node.unauth();
                                    return (*node).borrow().is_text();
                                }
                                false
                            })
                            .for_each(|node_id| {
                                if let Some(arena_node) = arena.get(node_id) {
                                    let auth_node = arena_node.get();
                                    let node = auth_node.unauth();
                                    //print(&format!("Text: {:?}",node));
                                }
                                
                            });
                        
                            current_node
                                .children(&arena)
                                .filter (  |node_id| {
                                    if let Some(arena_node) = arena.get(*node_id) {
                                        let auth_node = arena_node.get();
                                        let node = auth_node.unauth();
                                        return (*node).borrow().is_text();
                                    }
                                    false
                                })
                                .nth(index)
                        };

                //print(&format!("Result node {:?}",result_node));

                if result_node.is_some() {
                    current_node = result_node.unwrap();
                }else {return None}
            }

            if let Some(arena_node) = arena.get(current_node) {
                let auth_node = arena_node.get();
                let node = auth_node.unauth();
                return Some(Rc::clone(&node));
            }
        }

        return None;

    }    

    pub fn select_cfi_range(&self, from:&CFIComponentList, to:&CFIComponentList) -> Vec<Rc<RefCell<Node>>> {

        let mut result = Vec::new();

        if let  Some(from_id) = self.select_cfi_node_id(from) {
            if let Some(to_id) = self.select_cfi_node_id(to) {
                if let Some(root) = self.id {
                    let arena_ref = (*self.doc).unauth();
                    let arena = &arena_ref.borrow();     

                    let range = Range::new(arena,root,from_id,to_id);

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
                                    /*if let Some(arena_node) = arena.get(node_id) {
                                        let auth_node = arena_node.get();
                                        let node = auth_node.unauth();
                                        
                                        result.push_str(&std::format!("{:-}",*node.borrow()));
                                    }*/
                            }
                        };
                    }

        }}}

        return result;
    }

    pub fn select_cfi_fragment_fmt (&self, cfi_fragment: &Fragment) -> String  {



        println!("select_cfi_frament_fmt");

        let arena_ref = (*self.doc).unauth();
        let arena = &arena_ref.borrow();        

        macro_rules! printN {
            ($i:ident) => {  
                            if let Some(n_id) = $i {
                                if let Some(arena_node) = arena.get(n_id) {
                                    let auth_node = arena_node.get();
                                    let node = auth_node.unauth();

                                    println!("{:+}",*node.borrow());  
                                }
                            }
                        };
        }

        let mut result = String::new();

        if let Some(node_id) = self.id {

            if let Some(root_arena_node) = arena.get(node_id){
                if let Some(current_node) = root_arena_node.first_child(){
            
                    println!("Root: {:?}",current_node);
                    

                    let mut root_node = self.select_steps(current_node,&cfi_fragment.path.local_path.steps);

                    println!("Root result: {:?}",root_node);
                    printN!(root_node);

                    let (start_node,end_node) = 
                        if let (Some(rnode),Some(range)) = (root_node,&cfi_fragment.range) {
                            (   
                                self.select_steps(rnode,&range.start.steps), 
                                self.select_steps(rnode,&range.end.steps)
                            )
                        } else { (root_node,root_node) };


                    //Traverse and format the nodes
                    if let (Some(root_node_f),Some(start_node_f),Some(end_node_f)) = (root_node,start_node,end_node){

                        println!("Range");

                        let range = Range::new(arena,root_node_f,start_node_f,end_node_f);

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
                        }
                }
            }
        }

        return result

    }

    pub fn select_steps (&self,root_node:NodeId, steps : &Vec<Step>) -> Option<NodeId> {
        let arena_ref = (*self.doc).unauth();
        let arena = &arena_ref.borrow();      

        macro_rules! printN {
            ($i:ident) => {  
                            if let Some(n_id) = $i {
                                if let Some(arena_node) = arena.get(n_id) {
                                    let auth_node = arena_node.get();
                                    let node = auth_node.unauth();

                                    println!("{:+}",*node.borrow());  
                                }
                            }
                        };
        }

        let mut current_node = root_node;
        let mut result_node = None;

        for step in steps {
            let node_index :usize = step.integer.parse().ok()?;

            let (even,index) = if node_index % 2 == 0  {(true,(node_index / 2)-1)}else {(false,node_index / 2)}; //odd -> text children

            println!("Even,index: {:?},{:?},{:?}",even,index,node_index);

            result_node = if even {
                        current_node    
                        .children(&arena)
                        .filter (  |node_id| {
                                if let Some(arena_node) = arena.get(*node_id) {
                                    let auth_node = arena_node.get();
                                    let node = auth_node.unauth();
                                    return (*node).borrow().is_element();
                                }
                                false
                            })
                        .nth(index)
            } else {
                        current_node
                        .children(&arena)
                        .filter (  |node_id| {
                                if let Some(arena_node) = arena.get(*node_id) {
                                    let auth_node = arena_node.get();
                                    let node = auth_node.unauth();
                                    return (*node).borrow().is_text();
                                }
                                false
                            })
                        .nth(index)
            };

            println!("Result node {:?}",result_node);
            printN!(result_node);

            //Update the current node
            if result_node.is_some() {
                current_node = result_node.unwrap();
            }else {return None}

        }

        return result_node   
        
    }
}


impl<C> simplecss::Element for ElementRef<C> 
    where   
        C:Computation,
        C:AuthType<Node>,
        C:AuthType<Arena<<C as AuthType<Node>>::AuthT>>
{

    fn parent_element(&self) -> Option<Self> {

        if let Some(node_id) = self.id {
            
            let arena_ref = (*self.doc).unauth();
            let arena = arena_ref.borrow();
            let node_opt = arena.get(node_id);

            if let Some(node) = node_opt {
                if let Some(parent_id) = node.parent() {
                    return Some(
                                ElementRef {
                                    id: Some(parent_id),
                                    doc: Rc::clone(&self.doc)
                                }
                            )
                }
            }
        }

        None
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        if let Some(node_id) = self.id {
            let arena_ref = (*self.doc).unauth();
            let arena = &arena_ref.borrow();

            //Avoid the node itself
            let mut iter = node_id.preceding_siblings(arena).skip(1);

            iter.find_map ( |sibling| {
                        if let Some(arena_node) = arena.get(sibling) {
                            let auth_node = arena_node.get();
                            let node = auth_node.unauth();
                            if (*node).borrow().is_element() {
                                    return Some(ElementRef {
                                                id: Some(sibling),
                                                doc: Rc::clone(&self.doc)
                                            })
                            }
                        }
                        None
            })
        }else {None}
        
    }

    fn has_local_name(&self, local_name: &str) -> bool {
        if let Some(node_id) = self.id {
            
            let arena_ref = (*self.doc).unauth();
            let arena = arena_ref.borrow();
            let node_opt = arena.get(node_id);

            if let Some(arena_node) = node_opt {
                let auth_node = arena_node.get();
                let node = auth_node.unauth();
                let n = (*node).borrow();
                
                if let Some(element) =n.as_element() {
                    return element.name() == local_name
                }
                
            }
        }

        false
    }

    fn attribute_matches(&self, local_name: &str, operator: AttributeOperator) -> bool {
        if let Some(node_id) = self.id {
            
            let arena_ref = (*self.doc).unauth();
            let arena = arena_ref.borrow();
            let node_opt = arena.get(node_id);

            if let Some(arena_node) = node_opt {
                let auth_node = arena_node.get();
                let node = auth_node.unauth();
                let n = (*node).borrow();
                if let Some(element) =n.as_element() {
                    let qn = QualName::new(None,NodeString::from(""),NodeString::from(local_name));
                    if let Some(value) = element.attrs.get(&qn) {
                        return operator.matches(value)
                    }
                }
            }
        }

        false
    }

    fn pseudo_class_matches(&self, class: PseudoClass) -> bool {
        match class {
            PseudoClass::FirstChild => self.prev_sibling_element().is_none(),
            _ => false,
        }
    }
}
