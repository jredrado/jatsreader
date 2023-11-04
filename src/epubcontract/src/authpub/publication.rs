use std::vec::Vec;
use std::string::String;
use std::collections::BTreeMap;
use std::boxed::Box;
use core::any::Any;
use core::cell::RefCell;
use std::rc::Rc;
use std::format;

//use anyhow::{anyhow,Result,Error};

use indextree::Arena;

use is_type::Is;

use core::fmt::Debug;
use std::borrow::ToOwned;

use crate::authpub::metadata::*;
use crate::authpub::mediaoverlay::*;
use crate::authpub::error::Error;

use authcomp::{Computation,AuthType,AuthT,AuthContainer,AuthTContainer,ProofStream};
use authcomp::{Encode,Decode,Serialize,Deserialize,DeserializeOwned,DecodeOwned};
use authcomp::{UnAuth,UnAuthMut};

use authselect::ElementRef;
use authdoc::Node;

use nanoserde::ToJSON;

use crate::authpub::types::*;

use typed_key::Key;

#[derive(Debug,Default)]
pub struct PublicationWithInternal<C> 
    where     
        C:Computation + Clone + Debug,  
        C:AuthType<String>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        //C:AuthType<Vec<AuthT<Link<C>,C>>>,
        //C:AuthType<Vec<AuthT<PublicationCollection<C>,C>>>,
        C:AuthType<Vec<MediaOverlayNode<C>>>,
        C:AuthType<Metadata<C>>,
        C:AuthType<Link<C>>,
        C:AuthType<PublicationCollection<C>>,
        C:AuthType<Contributor<C>>,
        C:AuthType<MediaOverlayNode<C>>,
        C:AuthType<u32>,
        C:AuthType<f32>,
        C:AuthType<bool>,
        C:AuthType<Meta<C>>,
        C:AuthType<Vec<AuthT<Meta<C>,C>>>,
        C:AuthType<MultiLanguage<C>>,
        C:AuthType<BTreeMap<AuthT<String,C>, AuthT<String,C>>>,
        C:AuthType<BelongsTo<C>>,
        C:AuthType<Collection<C>>,
        C:AuthType<Subject<C>>,
        C:AuthType<Properties<C>>,
        C:AuthType<Contributor<C>>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        C:AuthType<Vec<AuthT<Collection<C>,C>>>,
        //C:AuthType<Vec<AuthT<Subject<C>,C>>>,
        //C:AuthType<Vec<AuthT<Contributor<C>,C>>>,        
        C:AuthContainer<MediaOverlayNode<C>>,
        C:AuthContainer<Meta<C>>,
        C:AuthContainer<PublicationCollection<C>>,
        C:AuthContainer<Link<C>>,

        C:AuthType<Vec<Rc<RefCell< AuthT<PublicationCollection<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Link<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Meta<C>,C>>>>>,

        C:AuthType<Vec<Rc<RefCell< AuthT<Contributor<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Subject<C>,C>>>>>,

        C:AuthType<Vec<Rc<RefCell<AuthT<String,C>>>>>,
        C:AuthType<Vec<u8>>,
        C:AuthType<BTreeMap<AuthT<Vec<u8>,C>, (AuthT<Vec<u8>,C>,Option<ElementRef<C>>)>>,
        C:AuthType<Node>,
        C:AuthType<Arena<AuthT<Node,C>>>
{
    pub publication: Publication<C>,
    pub internal: HashMap,
    pub publication_members : Vec<PublicationMembers<C>>
}

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode,ToJSON)]
pub struct Publication<C> 
    where   
        C:Computation + Clone + Debug,
        C:AuthType<String>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        //C:AuthType<Vec<AuthT<Link<C>,C>>>,
        //C:AuthType<Vec<AuthT<PublicationCollection<C>,C>>>,
        C:AuthType<Vec<MediaOverlayNode<C>>>,
        C:AuthType<Metadata<C>>,
        C:AuthType<Link<C>>,
        C:AuthType<PublicationCollection<C>>,
        C:AuthType<Contributor<C>>,
        C:AuthType<MediaOverlayNode<C>>,
        C:AuthType<u32>,
        C:AuthType<f32>,
        C:AuthType<bool>,
        C:AuthType<Meta<C>>,
        C:AuthType<Vec<AuthT<Meta<C>,C>>>,
        C:AuthType<MultiLanguage<C>>,
        C:AuthType<BTreeMap<AuthT<String,C>, AuthT<String,C>>>,
        C:AuthType<BelongsTo<C>>,
        C:AuthType<Collection<C>>,
        C:AuthType<Subject<C>>,
        C:AuthType<Properties<C>>,
        C:AuthType<Contributor<C>>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        C:AuthType<Vec<AuthT<Collection<C>,C>>>,
        //C:AuthType<Vec<AuthT<Subject<C>,C>>>,
        //C:AuthType<Vec<AuthT<Contributor<C>,C>>>,        
        C:AuthContainer<MediaOverlayNode<C>>,
        C:AuthContainer<Meta<C>>,
        C:AuthContainer<PublicationCollection<C>>,
        C:AuthContainer<Link<C>>,

        C:AuthType<Vec<Rc<RefCell< AuthT<PublicationCollection<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Link<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Meta<C>,C>>>>>,

        C:AuthType<Vec<Rc<RefCell< AuthT<Contributor<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Subject<C>,C>>>>>,

        C:AuthType<Vec<Rc<RefCell<AuthT<String,C>>>>>,
        C:AuthType<Vec<u8>>,
        C:AuthType<BTreeMap<AuthT<Vec<u8>,C>, (AuthT<Vec<u8>,C>,Option<ElementRef<C>>)>>,
        C:AuthType<Node>,
        C:AuthType<Arena<AuthT<Node,C>>>

{
    #[n(0)] #[nserde(rename = "@context")] pub context: AVecString<C>,
    #[n(1)] pub metadata: AMetadata<C>,

    #[n(2)] pub links: AVec<C,Link<C>>,
    #[n(3)] #[nserde(rename = "readingOrder")] pub reading_order: AVec<C,Link<C>>,
    #[n(4)] pub resources: AVec<C,Link<C>>,
    #[n(5)] pub toc: AVec<C,Link<C>>,
    #[n(6)] #[nserde(rename = "pageList")] pub page_list: AVec<C,Link<C>>,
    #[n(7)] pub landmarks: AVec<C,Link<C>>,

    #[n(8)] pub loi: Option<AVec<C,Link<C>>>,
    #[n(9)] pub loa: Option<AVec<C,Link<C>>>,
    #[n(10)] pub lov: Option<AVec<C,Link<C>>>,
    #[n(11)] pub lot: Option<AVec<C,Link<C>>>,

    #[n(12)] pub other_links: Option<AVec<C,Link<C>>>,
    #[n(13)] pub other_collections: Option<AVec<C,PublicationCollection<C>>>,

    #[n(14)]  #[nserde(skip)] pub raw_resources : AResources<C>,

    //#[n(14)] internal: AVec<C,Internal<C>>

}

pub type APublication<C> = AuthT<Publication<C>,C>;


#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode,ToJSON)]
pub struct Internal<C> 
    where
        C:Computation,
        C:AuthType<String>
{
    #[n(0)] name: AString<C>,
    #[n(1)] value: AString<C>
}

pub type AInternal<C> = AuthT<Internal<C>,C>;

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode,ToJSON)]
pub struct PublicationCollection<C> 
    where
        C:Computation,
        C:AuthType<String>,
        C:AuthType<Link<C>>,
        C:AuthType<Meta<C>>,
        //C:AuthType<PublicationCollection<C>>,
        //C:AuthType<Vec<AuthT<Link<C>,C>>>,
        C:AuthType<Vec<AuthT<Meta<C>,C>>>,
       // C:AuthType<Vec<AuthT<PublicationCollection<C>,C>>>,
        C:AuthType<Properties<C>>,
        C:AuthType<Vec<MediaOverlayNode<C>>>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        C:AuthType<u32>,
        C:AuthType<bool>,
        C:AuthType<MediaOverlayNode<C>>,
        C:AuthContainer<MediaOverlayNode<C>>,
        C:AuthContainer<Meta<C>>,
        C:AuthContainer<PublicationCollection<C>>,
        C:AuthContainer<Link<C>>,

        C:AuthType<Vec<Rc<RefCell< AuthT<Link<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Meta<C>,C>>>>>,

        C:AuthType<Vec<Rc<RefCell<AuthT<String,C>>>>>,
        
{
    #[n(0)] pub role: AString<C>,
    #[n(1)] pub metadata: AVec<C,Meta<C>>,
    #[n(2)] pub links: AVec<C,Link<C>>,
    #[n(3)] pub children: AuthTContainer<PublicationCollection<C>,C>
}

pub type APublicationCollection<C> = AuthT<PublicationCollection<C>,C>;

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode,ToJSON)]
pub struct Link<C> 
    where
        C:Computation,
        C:AuthType<String>,
        //C:AuthType<Link<C>>,
        C:AuthType<MediaOverlayNode<C>>,
        C:AuthType<Vec<MediaOverlayNode<C>>>,
        C:AuthType<Properties<C>>,
        C:AuthType<u32>,
        C:AuthType<bool>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        //C:AuthType<Vec<AuthT<Link<C>,C>>>,
        C:AuthContainer<MediaOverlayNode<C>>,
        C:AuthContainer<Link<C>>,
        C:AuthType<Vec<Rc<RefCell<AuthT<String,C>>>>>,
        


{
    #[n(0)] pub href : AString<C>,
    #[n(1)] #[nserde(rename = "type")] pub type_link : Option<AString<C>>,
    #[n(2)] pub rel : AVecString<C>,
    
    #[n(3)] pub height : Option<AuthT<u32,C>>,
    #[n(4)] pub width : Option<AuthT<u32,C>>,

    #[n(5)] pub title : Option<AString<C>>,
    #[n(6)] pub properties : Option<AProperties<C>>,

    #[n(7)] pub duration : Option<AString<C>>,
    #[n(8)] pub templated : Option<AuthT<bool,C>>,
    #[n(9)] pub children : Option<AuthTContainer<Link<C>,C>>,
    #[n(10)] pub bitrate : Option<AuthT<u32,C>>,
    #[n(11)] pub mediaoverlays: Option<AuthTContainer<MediaOverlayNode<C>,C>>

}

pub type ALink<C> = AuthT<Link<C>,C>;


#[derive(Debug)]
pub enum PublicationMembers<C>
where    
    C:Computation + Clone + Debug,
    C:AuthType<String>,
    C:AuthType<Vec<MediaOverlayNode<C>>>,
    C:AuthType<Metadata<C>>,
    C:AuthType<Link<C>>,
    C:AuthType<PublicationCollection<C>>,
    C:AuthType<Contributor<C>>,
    C:AuthType<MediaOverlayNode<C>>,
    C:AuthType<u32>,
    C:AuthType<f32>,
    C:AuthType<bool>,
    C:AuthType<Meta<C>>,
    C:AuthType<Vec<AuthT<Meta<C>,C>>>,
    C:AuthType<MultiLanguage<C>>,
    C:AuthType<BTreeMap<AuthT<String,C>, AuthT<String,C>>>,
    C:AuthType<BelongsTo<C>>,
    C:AuthType<Collection<C>>,
    C:AuthType<Subject<C>>,
    C:AuthType<Properties<C>>,
    C:AuthType<Contributor<C>>,
    //C:AuthType<Vec<AuthT<String,C>>>,
    C:AuthType<Vec<AuthT<Collection<C>,C>>>,
    //C:AuthType<Vec<AuthT<Subject<C>,C>>>,
    //C:AuthType<Vec<AuthT<Contributor<C>,C>>>,        
    C:AuthContainer<MediaOverlayNode<C>>,
    C:AuthContainer<Meta<C>>,
    C:AuthContainer<PublicationCollection<C>>,
    C:AuthContainer<Link<C>>,
    C:AuthType<Vec<Rc<RefCell< AuthT<PublicationCollection<C>,C>>>>>,
    C:AuthType<Vec<Rc<RefCell< AuthT<Link<C>,C>>>>>,
    C:AuthType<Vec<Rc<RefCell< AuthT<Meta<C>,C>>>>>,

    C:AuthType<Vec<Rc<RefCell< AuthT<Contributor<C>,C>>>>>,
    C:AuthType<Vec<Rc<RefCell< AuthT<Subject<C>,C>>>>>,

    C:AuthType<Vec<Rc<RefCell<AuthT<String,C>>>>>,
    C:AuthType<Vec<u8>>,
    C:AuthType<BTreeMap<AuthT<Vec<u8>,C>, (AuthT<Vec<u8>,C>,Option<ElementRef<C>>)>>,
    C:AuthType<Node>,
    C:AuthType<Arena<AuthT<Node,C>>>    

{
    Strings(Rc<RefCell<Vec<Rc<RefCell<AuthT<String,C>>>>>>),
    Metadata(Rc<RefCell<Metadata<C>>>),
    Links(Rc<RefCell<Vec<Rc<RefCell<AuthT<Link<C>,C>>>>>>)
}

impl<C> Publication<C> 
    where   
        C:Computation + Clone + Debug,
        C:AuthType<String>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        //C:AuthType<Vec<AuthT<Link<C>,C>>>,
        //C:AuthType<Vec<AuthT<PublicationCollection<C>,C>>>,
        C:AuthType<Vec<MediaOverlayNode<C>>>,
        C:AuthType<Metadata<C>>,
        C:AuthType<Link<C>>,
        C:AuthType<PublicationCollection<C>>,
        C:AuthType<Contributor<C>>,
        C:AuthType<MediaOverlayNode<C>>,
        C:AuthType<u32>,
        C:AuthType<f32>,
        C:AuthType<bool>,
        C:AuthType<Meta<C>>,
        C:AuthType<Vec<AuthT<Meta<C>,C>>>,
        C:AuthType<MultiLanguage<C>>,
        C:AuthType<BTreeMap<AuthT<String,C>, AuthT<String,C>>>,
        C:AuthType<BelongsTo<C>>,
        C:AuthType<Collection<C>>,
        C:AuthType<Subject<C>>,
        C:AuthType<Properties<C>>,
        C:AuthType<Contributor<C>>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        C:AuthType<Vec<AuthT<Collection<C>,C>>>,
        //C:AuthType<Vec<AuthT<Subject<C>,C>>>,
        //C:AuthType<Vec<AuthT<Contributor<C>,C>>>,        
        C:AuthContainer<MediaOverlayNode<C>>,
        C:AuthContainer<Meta<C>>,
        C:AuthContainer<PublicationCollection<C>>,
        C:AuthContainer<Link<C>>,
        C:DecodeOwned,
        C:Encode,

        C:AuthType<Vec<Rc<RefCell< AuthT<PublicationCollection<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Link<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Meta<C>,C>>>>>,

        C:AuthType<Vec<Rc<RefCell< AuthT<Contributor<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Subject<C>,C>>>>>,

        C:AuthType<Vec<Rc<RefCell<AuthT<String,C>>>>>,
        C:AuthType<Vec<u8>>,
        C:AuthType<BTreeMap<AuthT<Vec<u8>,C>, (AuthT<Vec<u8>,C>,Option<ElementRef<C>>)>>,
        C:AuthType<Node>,
        C:AuthType<Arena<AuthT<Node,C>>>,
{

    
    fn create_member_index (&self) -> Vec<PublicationMembers<C>> {
        let mut result = Vec::new();

        let context_ref = self.context.unauth();
        let context = &context_ref.borrow();
        if !context.is_empty(){
            result.push(PublicationMembers::Strings(Rc::clone(&context_ref)));
        }

        let metadata_ref = self.metadata.unauth();
        let metadata = &metadata_ref.borrow();
        
        result.push(PublicationMembers::Metadata(Rc::clone(&metadata_ref)));
        
        let links_ref = self.links.unauth();
        let links = &links_ref.borrow();

        result.push(PublicationMembers::Links(Rc::clone(&links_ref)));

        let reading_order_ref = self.reading_order.unauth();
        let reading_order = &reading_order_ref.borrow();

        result.push(PublicationMembers::Links(Rc::clone(&reading_order_ref)));

        let resources_ref = self.resources.unauth();
        let resources = &resources_ref.borrow();

        result.push(PublicationMembers::Links(Rc::clone(&resources_ref)));

        let toc_ref = self.toc.unauth();
        let toc = &toc_ref.borrow();

        result.push(PublicationMembers::Links(Rc::clone(&toc_ref)));

        let page_list_ref = self.page_list.unauth();
        let page_list = &page_list_ref.borrow();

        result.push(PublicationMembers::Links(Rc::clone(&page_list_ref)));

        let landmarks_ref = self.landmarks.unauth();
        let landmarks = &landmarks_ref.borrow();

        result.push(PublicationMembers::Links(Rc::clone(&landmarks_ref)));

        return result;
    }

    fn create_member_map (&self) -> BTreeMap<&str,PublicationMembers<C>> {
        let mut result = BTreeMap::new();

        let context_ref = self.context.unauth();
        let context = &context_ref.borrow();
        if !context.is_empty(){
            result.insert("@context",PublicationMembers::Strings(Rc::clone(&context_ref)));
        }

        let metadata_ref = self.metadata.unauth();
        let metadata = &metadata_ref.borrow();
        
        result.insert("metadata",PublicationMembers::Metadata(Rc::clone(&metadata_ref)));
        
        let links_ref = self.links.unauth();
        let links = &links_ref.borrow();

        result.insert("links",PublicationMembers::Links(Rc::clone(&links_ref)));

        let reading_order_ref = self.reading_order.unauth();
        let reading_order = &reading_order_ref.borrow();

        result.insert("readingOrder",PublicationMembers::Links(Rc::clone(&reading_order_ref)));

        let resources_ref = self.resources.unauth();
        let resources = &resources_ref.borrow();

        result.insert("resources",PublicationMembers::Links(Rc::clone(&resources_ref)));

        let toc_ref = self.toc.unauth();
        let toc = &toc_ref.borrow();

        result.insert("toc",PublicationMembers::Links(Rc::clone(&toc_ref)));

        let page_list_ref = self.page_list.unauth();
        let page_list = &page_list_ref.borrow();

        result.insert("pagelist",PublicationMembers::Links(Rc::clone(&page_list_ref)));

        let landmarks_ref = self.landmarks.unauth();
        let landmarks = &landmarks_ref.borrow();

        result.insert("landmarks",PublicationMembers::Links(Rc::clone(&landmarks_ref)));

        return result;
    }

    pub fn get_cover (&self) -> Result<Rc<RefCell<Link<C>>>,Error>
    where
        C: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,
    {
        self.search_link_by_rel2("cover")
    }
    

    /*

    pub fn get_cover<R> (&self,proof_stream : Option<&ProofStream>) -> R
        where
            C: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,
            R: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,
            R: Computation<T=Rc<RefCell<Link<C>>>,E=Error>    
    {
        self.search_link_by_rel("cover",proof_stream)
    }

    pub fn get_nav_doc<R> (&self,proof_stream : Option<&ProofStream>) -> R
        where
            C: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,
            R: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,
            R: Computation<T=Rc<RefCell<Link<C>>>,E=Error>    
    {
        self.search_link_by_rel("contents",proof_stream)
    }


    pub fn search_link_by_rel<R> (& self, rel: &str ,proof_stream : Option<&ProofStream>) -> R 
        where
            C: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,
            R: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,
            R: Computation<T=Rc<RefCell<Link<C>>>,E=Error>
    {

        let mut comp = C::new(proof_stream);
        let mut r = R::new(None);

        
        let unauth_resources_ref  = C::unauth::<Vec<AuthT<Link<C>,C>>>(&mut comp,&self.resources);
        let unauth_resources  = &*unauth_resources_ref.borrow();


        for auth_resource in unauth_resources {
            let unauth_resource_ref = C::unauth::<Link<C>>(&mut comp,auth_resource);
            let unauth_resource = &*unauth_resource_ref.borrow();
            
            let unauth_rel_ref = C::unauth::<Vec<AuthT<String,C>>>(&mut comp,&unauth_resource.rel);
            let unauth_rel = &*unauth_rel_ref.borrow();

            for res_rel in unauth_rel {
                let unauth_res_rel_ref = C::unauth::<String>(&mut comp,res_rel);
                let unauth_res_rel = &*unauth_res_rel_ref.borrow();
                if unauth_res_rel == rel {
                    r.transfer(Rc::clone(&unauth_resource_ref),comp);
                    return r
                }
            }
        }
        
        let unauth_reading_order_ref  = C::unauth::<Vec<AuthT<Link<C>,C>>>(&mut comp,&self.reading_order);
        let unauth_reading_order  = &*unauth_reading_order_ref.borrow();

        for auth_item in unauth_reading_order {
            let unauth_item_ref = C::unauth::<Link<C>>(&mut comp,auth_item);
            let unauth_item = &*unauth_item_ref.borrow();
            
            let unauth_rel_ref = C::unauth::<Vec<AuthT<String,C>>>(&mut comp,&unauth_item.rel);
            let unauth_rel = &*unauth_rel_ref.borrow();

            for item_rel in unauth_rel {
                let unauth_item_rel_ref = C::unauth::<String>(&mut comp,item_rel);
                let unauth_item_rel = &*unauth_item_rel_ref.borrow();
                if unauth_item_rel == rel {
                    r.transfer(Rc::clone(&unauth_item_ref),comp);
                    return r
                }
            }
        }        


        let unauth_links_ref  = C::unauth::<Vec<AuthT<Link<C>,C>>>(&mut comp,&self.links);
        let unauth_links  = &*unauth_links_ref.borrow();

        for auth_link in unauth_links {
            let unauth_link_ref = C::unauth::<Link<C>>(&mut comp,auth_link);
            let unauth_link = &*unauth_link_ref.borrow();
            
            let unauth_rel_ref = C::unauth::<Vec<AuthT<String,C>>>(&mut comp,&unauth_link.rel);
            let unauth_rel = &*unauth_rel_ref.borrow();

            for link_rel in unauth_rel {
                let unauth_link_rel_ref = C::unauth::<String>(&mut comp,link_rel);
                let unauth_link_rel = &*unauth_link_rel_ref.borrow();
                if unauth_link_rel == rel {
                    r.transfer(Rc::clone(&unauth_link_ref),comp);
                    return r
                }
            }
        }           

        //r.put( Err(anyhow!(format!("Can't find {:?} in publication",rel))) );        
        r.put_err ( Error::NotFound(format!("Can't find {:?} in publication",rel)));
        r
    }

    pub fn add_link<R> (&mut self, type_link: String, rel: &[String], url:String, templated: bool,proof_stream : Option<&ProofStream>) -> R
        where
            C: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,
            C: Computation,
            C: AuthType<Vec<<C as AuthType<String>>::AuthT>>,
            C: AuthType<String>,
            R: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,
            R: Computation<T=()>
    {

        let mut comp = C::new(proof_stream);
        let mut r = R::new(None);

        let mut link = Link::<C>::default();
        link.href = C::auth(url);
        link.type_link = C::auth(type_link);
        link.templated = C::auth(templated);

        if rel.len() > 0 {
            let r : Vec<AuthT<String,C>> = rel.iter().map( |s| C::auth(s.to_owned())).collect();
            link.rel = C::auth(r);
        }

        let links = C::unauth::<Vec<AuthT<Link<C>,C>>>(&mut comp,&self.links);
        (*links.borrow_mut()).push(C::auth(link));
        
        C::auth_update::<Vec<AuthT<Link<C>,C>>>(&mut self.links);

        r.transfer((),comp);
        r
        
    }
    */

    pub fn add_link2 (&mut self, type_link: String, rel: &[String], url:String, templated: bool)
        where
            C: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,
            C: Computation,
            C: AuthType<Vec<Rc<RefCell<AuthT<String,C>>>>>,
            C: AuthType<String>,
    
    {

        let mut link = Link::<C>::default();
        link.href = C::auth(url);
        link.type_link = Some(C::auth(type_link));
        link.templated = Some(C::auth(templated));

        if rel.len() > 0 {
            let r : Vec<Rc<RefCell<AuthT<String,C>>>> = rel.iter().map( |s| Rc::new(RefCell::new(C::auth(s.to_owned()))) ).collect();
            link.rel = C::auth(r);
        }

        let links = C::unauth2::<Vec<Rc<RefCell<AuthT<Link<C>,C>>>>>(&self.links);
        (*links.borrow_mut()).push(Rc::new(RefCell::new(C::auth(link))));
        
        C::auth_update::<Vec<Rc<RefCell<AuthT<Link<C>,C>>>>>(&mut self.links);
        
    }    

    pub fn search_link_by_rel2 (& self, rel: &str ) -> Result<Rc<RefCell<Link<C>>>,Error>
        where
            C: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,
    {

        
        let unauth_resources_ref  = C::unauth2::<Vec<Rc<RefCell<AuthT<Link<C>,C>>>>>(&self.resources);
        let unauth_resources  = &*unauth_resources_ref.borrow();


        for auth_resource in unauth_resources {
            let unauth_resource_ref = C::unauth2::<Link<C>>(&*auth_resource.borrow());
            let unauth_resource = &*unauth_resource_ref.borrow();
            
            let unauth_rel_ref = C::unauth2::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&unauth_resource.rel);
            let unauth_rel = &*unauth_rel_ref.borrow();

            for res_rel in unauth_rel {
                let unauth_res_rel_ref = C::unauth2::<String>(&*res_rel.borrow());
                let unauth_res_rel = &*unauth_res_rel_ref.borrow();
                if unauth_res_rel == rel {
                    return Ok(Rc::clone(&unauth_resource_ref));
                }
            }
        }
        
        let unauth_reading_order_ref  = C::unauth2::<Vec<Rc<RefCell<AuthT<Link<C>,C>>>>>(&self.reading_order);
        let unauth_reading_order  = &*unauth_reading_order_ref.borrow();

        for auth_item in unauth_reading_order {
            let unauth_item_ref = C::unauth2::<Link<C>>(&*auth_item.borrow());
            let unauth_item = &*unauth_item_ref.borrow();
            
            let unauth_rel_ref = C::unauth2::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&unauth_item.rel);
            let unauth_rel = &*unauth_rel_ref.borrow();

            for item_rel in unauth_rel {
                let unauth_item_rel_ref = C::unauth2::<String>(&*item_rel.borrow());
                let unauth_item_rel = &*unauth_item_rel_ref.borrow();
                if unauth_item_rel == rel {
                    return Ok(Rc::clone(&unauth_item_ref));
                }
            }
        }        


        let unauth_links_ref  = C::unauth2::<Vec<Rc<RefCell<AuthT<Link<C>,C>>>>>(&self.links);
        let unauth_links  = &*unauth_links_ref.borrow();

        for auth_link in unauth_links {
            let unauth_link_ref = C::unauth2::<Link<C>>(&*auth_link.borrow());
            let unauth_link = &*unauth_link_ref.borrow();
            
            let unauth_rel_ref = C::unauth2::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&unauth_link.rel);
            let unauth_rel = &*unauth_rel_ref.borrow();

            for link_rel in unauth_rel {
                let unauth_link_rel_ref = C::unauth2::<String>(&*link_rel.borrow());
                let unauth_link_rel = &*unauth_link_rel_ref.borrow();
                if unauth_link_rel == rel {
                    return Ok(Rc::clone(&unauth_link_ref));
                  
                }
            }
        }           
      
        Err( Error::NotFound(format!("Can't find {:?} in publication",rel)))
    }    

    pub fn find_all_mediaoverlay (&self) -> Vec<Rc<RefCell< AuthT<MediaOverlayNode<C>,C>>>>
        where
            C: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,
            C: AuthContainer<MediaOverlayNode<C>>,
            C: AuthType<MediaOverlayNode<C>>,
            C: AuthType<Vec<Rc<RefCell< AuthT<MediaOverlayNode<C>,C>>>>>,

            AuthTContainer<MediaOverlayNode<C>,C> : Is< Type= AuthT<Vec<Rc<RefCell< AuthT<MediaOverlayNode<C>,C> >>>,C> >
            
           
    {
        let mut overlay = Vec::new();

        let unauth_reading_order_ref  = C::unauth2::<Vec<Rc<RefCell<AuthT<Link<C>,C>>>>>(&self.reading_order);
        let unauth_reading_order  = &*unauth_reading_order_ref.borrow();

        for auth_link in unauth_reading_order {
            let unauth_link_ref = C::unauth2::<Link<C>>(&*auth_link.borrow());
            let unauth_link = &mut *unauth_link_ref.borrow_mut();

            if let Some(unauth_mediaoverlays_ref_value) = &unauth_link.mediaoverlays {
                let unauth_mediaoverlays_ref = C::unauth2::<Vec<Rc<RefCell< AuthT<MediaOverlayNode<C>,C>>>>>( unauth_mediaoverlays_ref_value.into_ref() );

                let unauth_mediaoverlays = &*unauth_mediaoverlays_ref.borrow();

                if unauth_mediaoverlays.len() > 0 {
                    for ov in unauth_mediaoverlays {                
                        overlay.push(Rc::clone(ov));
                    }
                }
            }
        }

        overlay
    }

    pub fn find_all_mediaoverlay_by_ref(&self, href: &str) -> Vec<Rc<RefCell< AuthT<MediaOverlayNode<C>,C>>>>
        where
            C: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,
            C: AuthContainer<MediaOverlayNode<C>>,
            C: AuthType<MediaOverlayNode<C>>,
            C: AuthType<Vec<Rc<RefCell< AuthT<MediaOverlayNode<C>,C>>>>>,
            AuthTContainer<MediaOverlayNode<C>,C> : Is< Type= AuthT<Vec<Rc<RefCell< AuthT<MediaOverlayNode<C>,C> >>>,C> > 
    {
        let mut overlay = Vec::new();

        let unauth_reading_order_ref  = C::unauth2::<Vec<Rc<RefCell<AuthT<Link<C>,C>>>>>(&self.reading_order);
        let unauth_reading_order  = &*unauth_reading_order_ref.borrow();

        for auth_link in unauth_reading_order {
            let unauth_link_ref = C::unauth2::<Link<C>>(&*auth_link.borrow());
            let unauth_link = &mut *unauth_link_ref.borrow_mut();

            let unauth_href_ref = C::unauth2::<String>(&unauth_link.href);
            let unauth_href = &*unauth_href_ref.borrow();

            if unauth_href.contains(href) {
                if let Some (unauth_mediaoverlays_ref_value) = &unauth_link.mediaoverlays {
                    let unauth_mediaoverlays_ref = C::unauth2::<Vec<Rc<RefCell< AuthT<MediaOverlayNode<C>,C>>>>>( unauth_mediaoverlays_ref_value.into_ref() );                
                    let unauth_mediaoverlays = &*unauth_mediaoverlays_ref.borrow();

                    if unauth_mediaoverlays.len() > 0 {
                        for ov in unauth_mediaoverlays {                
                            overlay.push(Rc::clone(ov));
                        }
                    }
                }
            }
        }

        overlay
    }

    
    pub fn get_prefetch_resources(&self) -> Vec<Rc<RefCell<Link<C>>>> 
        where
            C: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,
    {

        use std::vec;

        let media_types = vec!["text/css", "application/vnd.ms-opentype", "text/javascript"];

        let mut resources = Vec::new();

        let unauth_resources_ref  = C::unauth2::<Vec<Rc<RefCell<AuthT<Link<C>,C>>>>>(&self.resources);
        let unauth_resources  = &*unauth_resources_ref.borrow();


        for auth_resource in unauth_resources {
            let unauth_resource_ref = C::unauth2::<Link<C>>(&*auth_resource.borrow());
            let unauth_resource = &mut *unauth_resource_ref.borrow_mut();

            //let unauth_type_link = C::unauth2::<String>(&unauth_resource.type_link);
            let unauth_type_link = unauth_resource.type_link.unauth_mut();
            if media_types.iter().any( |s| s == &*unauth_type_link.borrow()) {
                    resources.push(Rc::clone(&unauth_resource_ref));
            }
        }

        resources
    }

    pub fn search_link_by_href (& self, href: &str ) -> Result<Rc<RefCell<Link<C>>>,Error>
    where
        C: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,
    {

        
        let unauth_resources_ref  = C::unauth2::<Vec<Rc<RefCell<AuthT<Link<C>,C>>>>>(&self.resources);
        let unauth_resources  = &*unauth_resources_ref.borrow();


        for auth_resource in unauth_resources {
            let unauth_resource_ref = C::unauth2::<Link<C>>(&*auth_resource.borrow());
            let unauth_resource = &*unauth_resource_ref.borrow();
            
            let unauth_href_ref = C::unauth2::<String>(&unauth_resource.href);
            let unauth_href = &*unauth_href_ref.borrow();

            if unauth_href == href {
                    return Ok(Rc::clone(&unauth_resource_ref))
            }
     
        }
        
        let unauth_reading_order_ref  = C::unauth2::<Vec<Rc<RefCell<AuthT<Link<C>,C>>>>>(&self.reading_order);
        let unauth_reading_order  = &*unauth_reading_order_ref.borrow();

        for auth_item in unauth_reading_order {
            let unauth_item_ref = C::unauth2::<Link<C>>(&*auth_item.borrow());
            let unauth_item = &*unauth_item_ref.borrow();
            
            let unauth_href_ref = C::unauth2::<String>(&unauth_item.href);
            let unauth_href = &*unauth_href_ref.borrow();

            if unauth_href == href {
                return Ok(Rc::clone(&unauth_item_ref))
            }
        }        


        let unauth_links_ref  = C::unauth2::<Vec<Rc<RefCell<AuthT<Link<C>,C>>>>>(&self.links);
        let unauth_links  = &*unauth_links_ref.borrow();

        for auth_link in unauth_links {
            let unauth_link_ref = C::unauth2::<Link<C>>(&*auth_link.borrow());
            let unauth_link = &*unauth_link_ref.borrow();
            
            let unauth_href_ref = C::unauth2::<String>(&unauth_link.href);
            let unauth_href = &*unauth_href_ref.borrow();

            if unauth_href == href {
                return Ok(Rc::clone(&unauth_link_ref))
            }
            
        }           
    
        Err( Error::NotFound(format!("Can't find {:?} in publication",href)))
    }

}


impl<C> core::ops::Index<usize> for PublicationWithInternal<C> 
    where       
        C:Computation + Clone + Debug,
        C:AuthType<String>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        //C:AuthType<Vec<AuthT<Link<C>,C>>>,
        //C:AuthType<Vec<AuthT<PublicationCollection<C>,C>>>,
        C:AuthType<Vec<MediaOverlayNode<C>>>,
        C:AuthType<Metadata<C>>,
        C:AuthType<Link<C>>,
        C:AuthType<PublicationCollection<C>>,
        C:AuthType<Contributor<C>>,
        C:AuthType<MediaOverlayNode<C>>,
        C:AuthType<u32>,
        C:AuthType<f32>,
        C:AuthType<bool>,
        C:AuthType<Meta<C>>,
        C:AuthType<Vec<AuthT<Meta<C>,C>>>,
        C:AuthType<MultiLanguage<C>>,
        C:AuthType<BTreeMap<AuthT<String,C>, AuthT<String,C>>>,
        C:AuthType<BelongsTo<C>>,
        C:AuthType<Collection<C>>,
        C:AuthType<Subject<C>>,
        C:AuthType<Properties<C>>,
        C:AuthType<Contributor<C>>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        C:AuthType<Vec<AuthT<Collection<C>,C>>>,
        //C:AuthType<Vec<AuthT<Subject<C>,C>>>,
        //C:AuthType<Vec<AuthT<Contributor<C>,C>>>,        
        C:AuthContainer<MediaOverlayNode<C>>,
        C:AuthContainer<Meta<C>>,
        C:AuthContainer<PublicationCollection<C>>,
        C:AuthContainer<Link<C>>,

        C:AuthType<Vec<Rc<RefCell< AuthT<PublicationCollection<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Link<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Meta<C>,C>>>>>,

        C:AuthType<Vec<Rc<RefCell< AuthT<Contributor<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Subject<C>,C>>>>>,

        C:AuthType<Vec<Rc<RefCell<AuthT<String,C>>>>>,
        C:AuthType<Vec<u8>>,
        C:AuthType<BTreeMap<AuthT<Vec<u8>,C>, (AuthT<Vec<u8>,C>,Option<ElementRef<C>>)>>,
        C:AuthType<Node>,
        C:AuthType<Arena<AuthT<Node,C>>>
{
    type Output = PublicationMembers<C>;

    fn index(&self, index: usize ) -> &Self::Output {
        &self.publication_members[index]
    }
}


impl <C> Link<C> 
    where
        C:Computation,
        C:AuthType<String>,
        //C:AuthType<Link<C>>,
        C:AuthType<MediaOverlayNode<C>>,
        C:AuthType<Vec<MediaOverlayNode<C>>>,
        C:AuthType<Properties<C>>,
        C:AuthType<u32>,
        C:AuthType<bool>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        //C:AuthType<Vec<AuthT<Link<C>,C>>>,
        C:AuthContainer<MediaOverlayNode<C>>,
        C:AuthContainer<Link<C>>,
        C:AuthType<Vec<Rc<RefCell<AuthT<String,C>>>>>,
        C:Computation
{

    pub fn add_rel (&mut self, s: &str) {

        let rel_ref = C::unauth2::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&self.rel);
        
        let rel_already_present = (&*rel_ref.borrow()).iter().any( |r| {
            let r_ref = &*r.borrow();
            let r_value = C::unauth2::<String>(r_ref);
            let b = &*r_value.borrow() == s; 
            b
        });
        
        if !rel_already_present {
            (&mut *rel_ref.borrow_mut()).push(Rc::new(RefCell::new(C::auth( String::from(s) ))));
            C::auth_update::<Vec<Rc<RefCell<AuthT<String,C>>>>>(&mut self.rel);
        }
        
    }

    pub fn add_href_absolute (&mut self, href: &str, basefile: &str) {
        use unix_path::Path;

        let href_path = Path::new(href);
        let basefile_path = Path::new(basefile);
        if let Some(dir) = basefile_path.parent(){
            let href_str = dir.join(href);
            if let Some(s) = href_str.to_str() {
                self.href = C::auth ( String::from (s));
            }
        }

    }
}

impl<C> PublicationWithInternal<C> 
    where       
        C:Computation + Clone + Debug,
        C:AuthType<String>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        //C:AuthType<Vec<AuthT<Link<C>,C>>>,
        //C:AuthType<Vec<AuthT<PublicationCollection<C>,C>>>,
        C:AuthType<Vec<MediaOverlayNode<C>>>,
        C:AuthType<Metadata<C>>,
        C:AuthType<Link<C>>,
        C:AuthType<PublicationCollection<C>>,
        C:AuthType<Contributor<C>>,
        C:AuthType<MediaOverlayNode<C>>,
        C:AuthType<u32>,
        C:AuthType<f32>,
        C:AuthType<bool>,
        C:AuthType<Meta<C>>,
        C:AuthType<Vec<AuthT<Meta<C>,C>>>,
        C:AuthType<MultiLanguage<C>>,
        C:AuthType<BTreeMap<AuthT<String,C>, AuthT<String,C>>>,
        C:AuthType<BelongsTo<C>>,
        C:AuthType<Collection<C>>,
        C:AuthType<Subject<C>>,
        C:AuthType<Properties<C>>,
        C:AuthType<Contributor<C>>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        C:AuthType<Vec<AuthT<Collection<C>,C>>>,
        //C:AuthType<Vec<AuthT<Subject<C>,C>>>,
        //C:AuthType<Vec<AuthT<Contributor<C>,C>>>,        
        C:AuthContainer<MediaOverlayNode<C>>,
        C:AuthContainer<Meta<C>>,
        C:AuthContainer<PublicationCollection<C>>,
        C:AuthContainer<Link<C>>,

        C:AuthType<Vec<Rc<RefCell< AuthT<PublicationCollection<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Link<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Meta<C>,C>>>>>,

        C:AuthType<Vec<Rc<RefCell< AuthT<Contributor<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Subject<C>,C>>>>>,

        C:AuthType<Vec<Rc<RefCell<AuthT<String,C>>>>>,
        C:AuthType<Vec<u8>>,
        C:AuthType<BTreeMap<AuthT<Vec<u8>,C>, (AuthT<Vec<u8>,C>,Option<ElementRef<C>>)>>,
        C:AuthType<Node>,
        C:AuthType<Arena<AuthT<Node,C>>>
{

    pub fn add_to_internal<T: Any>(&mut self, key: Key<T>, value: T) 
            where T: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned 
    {
        self.internal.insert(key,value);
    }

}