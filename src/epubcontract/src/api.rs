

use authcomp::{Computation,AuthType,AuthT,AuthContainer,ProofStream, NoProofs, AuthTNoProofs};
use authcomp::{HashType,Prover,AuthTProver,Verifier,AuthTVerifier};
use authcomp::{Encode,Decode,Serialize,Deserialize,DeserializeOwned,DecodeOwned};
use authcomp::UnAuth;
use authcomp::ToJSON;


use authdoc::Node;
use indextree::Arena;
use nanoserde::SerJson;

use std::string::String;
use std::vec::Vec;
use std::boxed::Box;
use core::cell::RefCell;
use std::rc::Rc;
use std::format;

use std::collections::BTreeMap;
use core::fmt::Debug;
use core::str;
use std::borrow::ToOwned;

//use crate::epubdoc::*;
use crate::epubarchive::EPubArchive;
use crate::epubparser::EPubParser;

use crate::authpub::publication::*;
use crate::authpub::metadata::*;
use crate::authpub::mediaoverlay::*;

use is_type::Is;

//use wapc_guest as guest;
//use guest::prelude::*;

use spin::Mutex;

use anyhow::{anyhow,Result,Error};

use core::marker::PhantomData;

pub type ID = HashType;
//pub type EPUB<C> = AuthT<EPubDoc<C>,C>;

//use authselect::{ElementRef,DOMRange,DOMIndex,SimplifiedLocator};
//use authselect_html5ever::{ElementRef,DOMRange,DOMIndex,SimplifiedLocator};
use authselect::*;

use logos::Logos;

#[derive(Debug,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub enum ApiResponse {
    #[n(0)] Void,
    #[n(1)] String( #[n(0)] String),
    #[n(2)] Vec ( #[n(0)] Vec<u8> ),
    #[n(3)] VecAndString ( #[n(0)] Vec<u8> , #[n(1)] Option<String> ),
    #[n(4)] VecOfStrings ( #[n(0)] Vec<String> )
}

#[derive(Debug,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub enum ApiError {
    #[n(0)] String( #[n(0)] String),
}

impl Default for ApiError {
    fn default() -> Self {
        ApiError::String(String::from(""))
    }
}

impl Default for ApiResponse {
    fn default() -> Self {
        ApiResponse::Void
    }
}

impl From<()> for ApiResponse {
    fn from(response: ()) -> Self {
        ApiResponse::Void
    }
}

impl From<String> for ApiResponse {
    fn from(response: String) -> Self {
        ApiResponse::String(response)
    }
}

impl From<&String> for ApiResponse {
    fn from(response: &String) -> Self {
        ApiResponse::String(response.to_owned())
    }
}



#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq)]
pub struct Api<C>
    where
        C:Computation<T=ApiResponse,E=ApiError>,
        C: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,

        C:AuthType<Publication<C>>,
        C:AuthType<String>,

        C:AuthType<Metadata<C>>,
        C:AuthType<Link<C>>,
        C:AuthType<PublicationCollection<C>>,
        C:AuthType<Contributor<C>>,
        C:AuthType<MediaOverlayNode<C>>,
        C:AuthType<u32>,
        C:AuthType<f32>,
        C:AuthType<bool>,
        C:AuthType<Meta<C>>,

        C:AuthType<MultiLanguage<C>>,

        C:AuthType<BelongsTo<C>>,
        C:AuthType<Collection<C>>,
        C:AuthType<Subject<C>>,
        C:AuthType<Properties<C>>,
        C:AuthType<Contributor<C>>,            

        C:AuthType<BTreeMap<AuthT<String,C>, AuthT<String,C>>>,			

        C:AuthType<Vec<AuthT<Meta<C>,C>>>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        C:AuthType<Vec<AuthT<Link<C>,C>>>,
        C:AuthType<Vec<AuthT<PublicationCollection<C>,C>>>,
        C:AuthType<Vec<MediaOverlayNode<C>>>,

        C:AuthType<Vec<AuthT<Collection<C>,C>>>,
        //C:AuthType<Vec<AuthT<Subject<C>,C>>>,
        //C:AuthType<Vec<AuthT<Contributor<C>,C>>>,   
        
        C:AuthContainer<MediaOverlayNode<C>>,
        C:AuthContainer<Meta<C>>,
        C:AuthContainer<PublicationCollection<C>>,
        C:AuthContainer<Link<C>>,
        C:Encode,
        C:DecodeOwned,

        //C:AuthType<Internal<C>>,
        //C:AuthType<Vec<AuthT<Internal<C>,C>>>,

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

    phantom: PhantomData<C>
}


impl<C> Api<C>
    where
        C:Computation<T=ApiResponse,E=ApiError>,
        C: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq,

        C:AuthType<Publication<C>>,
        C:AuthType<String>,

        C:AuthType<Metadata<C>>,
        C:AuthType<Link<C>>,
        C:AuthType<PublicationCollection<C>>,
        C:AuthType<Contributor<C>>,
        C:AuthType<MediaOverlayNode<C>>,
        C:AuthType<u32>,
        C:AuthType<f32>,
        C:AuthType<bool>,
        C:AuthType<Meta<C>>,

        C:AuthType<MultiLanguage<C>>,

        C:AuthType<BelongsTo<C>>,
        C:AuthType<Collection<C>>,
        C:AuthType<Subject<C>>,
        C:AuthType<Properties<C>>,
        C:AuthType<Contributor<C>>,            

        C:AuthType<BTreeMap<AuthT<String,C>, AuthT<String,C>>>,			

        C:AuthType<Vec<AuthT<Meta<C>,C>>>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        C:AuthType<Vec<AuthT<Link<C>,C>>>,
        C:AuthType<Vec<AuthT<PublicationCollection<C>,C>>>,
        C:AuthType<Vec<MediaOverlayNode<C>>>,

        C:AuthType<Vec<AuthT<Collection<C>,C>>>,
        //C:AuthType<Vec<AuthT<Subject<C>,C>>>,
        //C:AuthType<Vec<AuthT<Contributor<C>,C>>>,      

        C:AuthContainer<MediaOverlayNode<C>>,
        C:AuthContainer<Meta<C>>,
        C:AuthContainer<PublicationCollection<C>>,
        C:AuthContainer<Link<C>>,
        C:Encode,
        C:DecodeOwned,

        //C:AuthType<Internal<C>>,
        //C:AuthType<Vec<AuthT<Internal<C>,C>>>,

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

    pub fn new () -> Api<C> {
        Default::default()
    }
  
    
    pub fn register (_binding: &str, epubsource: &[u8]) -> Result<(ID,AuthT<Publication<C>,C>)>             
        where 
                <C as AuthType<String>>::AuthT: Ord, 
                <C as AuthType<Vec<u8>>>::AuthT: Ord,
                BTreeMap<<C as AuthType<Vec<u8>>>::AuthT, (<C as AuthType<Vec<u8>>>::AuthT, std::option::Option<ElementRef<C>>)>: SerJson,
                BTreeMap<<C as AuthType<std::string::String>>::AuthT, <C as AuthType<std::string::String>>::AuthT> : SerJson
    {

        let publication_with_internals = EPubParser::<C>::parse (epubsource).map_err(|e| anyhow!(format!("Unable to parse epub {:?}",e)) )?;

        let authepub = C::auth(publication_with_internals.publication);

        //authallocator::print(&alloc::format!("Authenticated Publication {:?}",&authepub));

        let id = C::signature::<Publication::<C>>(&authepub);
        let r = id.to_owned();

        /*
        let params = (id.data,authcomp::to_vec(&authepub));
        let msg :Vec<u8> = corepack::to_bytes::<([u8;32],Vec<u8>)>(params).map_err(|e| anyhow!(format!("Unable to serialize params {:?}",e)) )?;

        let _res = host_call(binding, "storage", "insert", &msg).map_err(|e| anyhow!(format!("Unable to host call database insert {:?}",e)))?;
        */

        Ok((r,authepub.to_owned()))

    }

    pub fn api_manifest_verifier (doc: &ID,proof_stream : Option<ProofStream>) -> Result<C>
    where
        AuthTVerifier<Publication<C>> : Is< Type=<C as AuthType<Publication<C>>>::AuthT>
         
    {
        let aepub_verifier = AuthTVerifier::<Publication<C>>::from(doc);
        let aepub : &<C as AuthType<Publication<C>>>::AuthT = aepub_verifier.into_ref();

        Api::<C>::manifest(aepub, proof_stream)
        
                
    }
    
    /*
    pub fn api_manifest_prover (binding: &str,doc: &ID,proof_stream : Option<ProofStream>) -> Result<C>
    {

        let id_msg = corepack::to_bytes(doc.data).map_err(|e| anyhow!(format!("Unable to decode message {:?}",e)))?;

        let get_msg = host_call(binding, "storage", "get", &id_msg).map_err(|e| anyhow!(format!("Unable to host call storage get {:?}",e)))?;

        //let aepub_bytes : Vec<u8> = corepack::from_bytes(&get_msg).map_err(|e| anyhow!(format!("Unable to decode get result {:?} {:?}",e, &get_msg)))?;

        let (_,aepub_bytes) : ([u8;32],Vec<u8>) = corepack::from_bytes(&get_msg).map_err(|e| anyhow!(format!("Unable to decode get result {:?} {:?}",e, &get_msg)))?;

        //Unshallow
        //let aepub : AuthT<Publication<C>,C> = C::unshallow(&aepub_bytes).map_err(|e| anyhow!(format!("Unable to decode deserealize auth type {:?}",e)))?;

        let aepub : AuthT<Publication<C>,C> = authcomp::from_bytes(&aepub_bytes).map_err(|e| anyhow!(format!("Unable to decode deserealize auth type {:?}",e)))?;

        Api::manifest(&aepub, proof_stream)
        
                
    }

    pub fn api_manifest_verifier (binding: &str,doc: &ID,proof_stream : Option<ProofStream>) -> Result<C>
        where
            AuthTVerifier<Publication<C>> : Is< Type=<C as AuthType<Publication<C>>>::AuthT>
             
    {
        let aepub_verifier = AuthTVerifier::<Publication<C>>::from(doc);
        let aepub : &<C as AuthType<Publication<C>>>::AuthT = aepub_verifier.into_ref();

        Api::<C>::manifest(aepub, proof_stream)
        
                
    }


    //Pure API
    //No storage
    */
    pub fn manifest (doc: & AuthT<Publication<C>,C>,proof_stream : Option<ProofStream>) -> Result<C>


    {

        use json_minimal::*;

        let real_work = || {
                                    
            let json_document = doc.serialize_json();

            let json_result = Json::parse(json_document.as_bytes());

            match json_result {
                Ok(json) => Ok(ApiResponse::String(json.print())),
                Err ((_,s)) => Err(ApiError::String(String::from(s)))
            }
            
        };

        let comp = match proof_stream {
                Some(p) => C::run_with_proofs (p,real_work),
                None => C::run ( real_work),

        };
       
        Ok(comp)
    }
    
    pub fn metadata (doc: & AuthT<Publication<C>,C>,proof_stream : Option<ProofStream>) -> Result<C>


    {

        use json_minimal::*;

        let real_work = || {
                              
            
            let unauth_doc_ref = doc.unauth();
            let unauth_doc = &*unauth_doc_ref.borrow();

            let json_document = unauth_doc.metadata.serialize_json();

            let json_result = Json::parse(json_document.as_bytes());

            match json_result {
                Ok(json) => Ok(ApiResponse::String(json.print())),
                Err ((_,s)) => Err(ApiError::String(String::from(s)))
            }
            
        };

        let comp = match proof_stream {
                Some(p) => C::run_with_proofs (p,real_work),
                None => C::run ( real_work),

        };
       
        Ok(comp)
    }

    /*
    pub fn api_metadata_prover (binding: &str,doc: &ID,proof_stream : Option<ProofStream>) -> Result<C>
    {

        let id_msg = corepack::to_bytes(doc.data).map_err(|e| anyhow!(format!("Unable to decode message {:?}",e)))?;

        let get_msg = host_call(binding, "storage", "get", &id_msg).map_err(|e| anyhow!(format!("Unable to host call storage get {:?}",e)))?;

        //let aepub_bytes : Vec<u8> = corepack::from_bytes(&get_msg).map_err(|e| anyhow!(format!("Unable to decode get result {:?} {:?}",e, &get_msg)))?;

        let (_,aepub_bytes) : ([u8;32],Vec<u8>) = corepack::from_bytes(&get_msg).map_err(|e| anyhow!(format!("Unable to decode get result {:?} {:?}",e, &get_msg)))?;

        //Unshallow
        //let aepub : AuthT<Publication<C>,C> = C::unshallow(&aepub_bytes).map_err(|e| anyhow!(format!("Unable to decode deserealize auth type {:?}",e)))?;

        let aepub : AuthT<Publication<C>,C> = authcomp::from_bytes(&aepub_bytes).map_err(|e| anyhow!(format!("Unable to decode deserealize auth type {:?}",e)))?;

        Api::metadata(&aepub, proof_stream)
        
                
    }
    */
    pub fn api_metadata_verifier (doc: &ID,proof_stream : ProofStream) -> Result<C>
    where
        AuthTVerifier<Publication<C>> : Is< Type=<C as AuthType<Publication<C>>>::AuthT>
         
    {
        let aepub_verifier = AuthTVerifier::<Publication<C>>::from(doc);
        let aepub : &<C as AuthType<Publication<C>>>::AuthT = aepub_verifier.into_ref();

        Api::<C>::metadata(aepub, Some(proof_stream))
        
                
    }

    /*
    pub fn cover (doc: & AuthT<Publication<C>,C>,proof_stream : Option<ProofStream>) -> Result<C>


    {

        use json_minimal::*;

        let real_work = || {
                              
            
            let unauth_doc_ref = doc.unauth();
            let unauth_doc = &*unauth_doc_ref.borrow();

            let link = unauth_doc.get_cover().map_err(|e| ApiError::String(format!("Unable to get cover {:?}",e)) )?;
            
            let json_document = &*link.borrow().serialize_json();

            let json_result = Json::parse(json_document.as_bytes());

            match json_result {
                Ok(json) => Ok(ApiResponse::String(json.print())),
                Err ((_,s)) => Err(ApiError::String(String::from(s)))
            }
            
        };

        let comp = match proof_stream {
                Some(p) => C::run_with_proofs (p,real_work),
                None => C::run ( real_work),

        };
       
        Ok(comp)
    }

    pub fn api_cover_prover (binding: &str,doc: &ID,proof_stream : Option<ProofStream>) -> Result<C>
    {

        let id_msg = corepack::to_bytes(doc.data).map_err(|e| anyhow!(format!("Unable to decode message {:?}",e)))?;

        let get_msg = host_call(binding, "storage", "get", &id_msg).map_err(|e| anyhow!(format!("Unable to host call storage get {:?}",e)))?;

        //let aepub_bytes : Vec<u8> = corepack::from_bytes(&get_msg).map_err(|e| anyhow!(format!("Unable to decode get result {:?} {:?}",e, &get_msg)))?;

        let (_,aepub_bytes) : ([u8;32],Vec<u8>) = corepack::from_bytes(&get_msg).map_err(|e| anyhow!(format!("Unable to decode get result {:?} {:?}",e, &get_msg)))?;

        //Unshallow
        //let aepub : AuthT<Publication<C>,C> = C::unshallow(&aepub_bytes).map_err(|e| anyhow!(format!("Unable to decode deserealize auth type {:?}",e)))?;

        let aepub : AuthT<Publication<C>,C> = authcomp::from_bytes(&aepub_bytes).map_err(|e| anyhow!(format!("Unable to decode deserealize auth type {:?}",e)))?;

        Api::cover(&aepub, proof_stream)
        
                
    }

    pub fn api_cover_verifier (binding: &str,doc: &ID,proof_stream : Option<ProofStream>) -> Result<C>
    where
        AuthTVerifier<Publication<C>> : Is< Type=<C as AuthType<Publication<C>>>::AuthT>
         
    {
        let aepub_verifier = AuthTVerifier::<Publication<C>>::from(doc);
        let aepub : &<C as AuthType<Publication<C>>>::AuthT = aepub_verifier.into_ref();

        Api::<C>::cover(aepub, proof_stream)
        
                
    }
    */
    fn find_mime_type (publication: &Publication<C>, resource_name: &Vec<u8>) -> Option<String> {
        
        let predicate = | link_v : &Rc<RefCell<AuthT<Link<C>,C>>> | {
            let link_auth = &*link_v.borrow();
            let link_ref = link_auth.unauth();
            let link = &*link_ref.borrow();

            let href_ref = link.href.unauth();
            let href = &*href_ref.borrow();

            if let Ok(str) =  std::str::from_utf8(resource_name) {
                if href == str {
                    if let Some(mime_type_auth) = &link.type_link {
                        let mime_type_ref = mime_type_auth.unauth();
                        let mime_type = &*mime_type_ref.borrow();
                        return Some(mime_type.to_owned());
                    }
                }
            }
             
            None
        };

        let reading_order_ref = publication.reading_order.unauth();
        let reading_order = &*reading_order_ref.borrow();

        let type_link = reading_order.iter().find_map( predicate );

        if let Some(_) = &type_link {
                return type_link;
        }else {
            let resources_ref = publication.resources.unauth();
            let resources= &*resources_ref.borrow();

            let type_link_2 = resources.iter().find_map( predicate );

            return type_link_2; //Some or None
        }
    }
    

    pub fn resource( doc: & AuthT<Publication<C>,C>,resource_name: Vec<u8>, proof_stream : Option<ProofStream> ) -> Result<C> 
        where <C as AuthType<Vec<u8>>>::AuthT: Ord

    {

        let real_work = || { 
            let unauth_doc_ref = doc.unauth();
            let unauth_doc = &*unauth_doc_ref.borrow();

            let resources_ref = unauth_doc.raw_resources.unauth();
            let resources = &*resources_ref.borrow();

            let mime_type = Self::find_mime_type(unauth_doc,&resource_name);
            

            if let Some(result) = resources.get(&C::auth(resource_name)){
                
                let unauth_result_ref = result.0.unauth();
                let unauth_result = &*unauth_result_ref.borrow();
                
                Ok(ApiResponse::VecAndString(unauth_result.to_vec(),mime_type))
            }else {
                
                Err(ApiError::String(String::from("")))
            }

            
        };

        let comp = match proof_stream {
            Some(p) => C::run_with_proofs (p,real_work),
            None => C::run (real_work),

        };
   
        Ok(comp)
    }

    /*
    pub fn api_resource_prover (binding: &str,doc: &ID,resource_name: Vec<u8>) -> Result<C>
        where <C as AuthType<Vec<u8>>>::AuthT: Ord
    {

        let id_msg = corepack::to_bytes(doc.data).map_err(|e| anyhow!(format!("Unable to decode message {:?}",e)))?;

        let get_msg = host_call(binding, "storage", "get", &id_msg).map_err(|e| anyhow!(format!("Unable to host call storage get {:?}",e)))?;

        let (_,aepub_bytes) : ([u8;32],Vec<u8>) = corepack::from_bytes(&get_msg).map_err(|e| anyhow!(format!("Unable to decode get result {:?} {:?}",e, &get_msg)))?;

        let aepub : AuthT<Publication<C>,C> = authcomp::from_bytes(&aepub_bytes).map_err(|e| anyhow!(format!("Unable to decode deserealize auth type {:?}",e)))?;

        Api::resource(&aepub, resource_name,None)
        
                
    }
    */
    pub fn api_resource_verifier (doc: &ID,resource_name: Vec<u8>,proofs: ProofStream) -> Result<C>
        where
            AuthTVerifier<Publication<C>> : Is< Type=<C as AuthType<Publication<C>>>::AuthT>,
            <C as AuthType<Vec<u8>>>::AuthT: Ord
    {

        let aepub_verifier = AuthTVerifier::<Publication<C>>::from(doc);
        let aepub : &<C as AuthType<Publication<C>>>::AuthT = aepub_verifier.into_ref();

        Api::resource(aepub, resource_name,Some(proofs))
        
                
    }
    /*
    pub fn search( doc: & AuthT<Publication<C>,C>,resource_name: Vec<u8>, selector:Vec<u8>, proof_stream : Option<ProofStream> ) -> Result<C> 
            where <C as AuthType<Vec<u8>>>::AuthT: Ord
    {

        let real_work = || {
            let unauth_doc_ref = doc.unauth();
            let unauth_doc = &*unauth_doc_ref.borrow();
            
            let raw_resources_ref = unauth_doc.raw_resources.unauth();
            let raw_resources = (&*raw_resources_ref.borrow());

            if let Some(result) = raw_resources.get(&C::auth(resource_name)){
                if let Some(element_ref) = &result.1 {
                    if let Ok(str_selector) = str::from_utf8(&selector){
                        let result = element_ref.select_fmt(str_selector);
                        return Ok(ApiResponse::VecOfStrings(result));
                    }
                }
            }

            Err(ApiError::String(String::from("No results")))
        };

        let comp = match proof_stream {
            Some(p) => C::run_with_proofs (p,real_work),
            None => C::run (real_work),

        };
   
        Ok(comp)
    }

    pub fn api_search_prover (binding: &str,doc: &ID,resource_name: Vec<u8>,selector:Vec<u8>) -> Result<C>
        where <C as AuthType<Vec<u8>>>::AuthT: Ord
    {

        let id_msg = corepack::to_bytes(doc.data).map_err(|e| anyhow!(format!("Unable to decode message {:?}",e)))?;

        let get_msg = host_call(binding, "storage", "get", &id_msg).map_err(|e| anyhow!(format!("Unable to host call storage get {:?}",e)))?;

        let (_,aepub_bytes) : ([u8;32],Vec<u8>) = corepack::from_bytes(&get_msg).map_err(|e| anyhow!(format!("Unable to decode get result {:?} {:?}",e, &get_msg)))?;

        let aepub : AuthT<Publication<C>,C> = authcomp::from_bytes(&aepub_bytes).map_err(|e| anyhow!(format!("Unable to decode deserealize auth type {:?}",e)))?;

        Api::search(&aepub, resource_name,selector,None)
        
                
    }

    pub fn api_search_verifier (binding: &str,doc: &ID,resource_name: Vec<u8>,selector:Vec<u8>,proofs: ProofStream) -> Result<C>
        where
            AuthTVerifier<Publication<C>> : Is< Type=<C as AuthType<Publication<C>>>::AuthT>,
            <C as AuthType<Vec<u8>>>::AuthT: Ord
    {

        let aepub_verifier = AuthTVerifier::<Publication<C>>::from(doc);
        let aepub : &<C as AuthType<Publication<C>>>::AuthT = aepub_verifier.into_ref();

        Api::search(aepub, resource_name,selector,Some(proofs))
        
                
    }
    */

    pub fn locate( doc: & AuthT<Publication<C>,C>,locator: SimplifiedLocator, proof_stream : Option<ProofStream> ) -> Result<C> 
            where <C as AuthType<Vec<u8>>>::AuthT: Ord
    {

        let real_work = || {
            let unauth_doc_ref = doc.unauth();
            let unauth_doc = &*unauth_doc_ref.borrow();
            
            let raw_resources_ref = unauth_doc.raw_resources.unauth();
            let raw_resources = (&*raw_resources_ref.borrow());

            println!("Locate {:?}",&locator);

            if locator.media_type == "text/html" || locator.media_type == "text/xml" {
                if let Some(result) = raw_resources.get(&C::auth(locator.href.as_bytes().to_owned())){

                    if let Some(element_ref) = &result.1 {
                        let range = DOMRange {
                            start: DOMIndex {
                                css_selector: locator.from_css_selector,
                                text_node_index:None,
                                offset:None
                            },
                            end: DOMIndex {
                                css_selector: locator.to_css_selector,
                                text_node_index:None,
                                offset:None
                            }
                        };

                        let result = element_ref.select_nodes_with_range_fmt(&range);
                        return Ok(ApiResponse::String(result));
                        
                    }
                }
            }

            Err(ApiError::String(String::from("No results")))
        };

        let comp = match proof_stream {
            Some(p) => C::run_with_proofs (p,real_work),
            None => C::run (real_work),

        };
   
        Ok(comp)
    }
    /*
    pub fn api_locate_prover (binding: &str,doc: &ID,locator: SimplifiedLocator) -> Result<C>
        where <C as AuthType<Vec<u8>>>::AuthT: Ord
    {

        let id_msg = corepack::to_bytes(doc.data).map_err(|e| anyhow!(format!("Unable to decode message {:?}",e)))?;

        let get_msg = host_call(binding, "storage", "get", &id_msg).map_err(|e| anyhow!(format!("Unable to host call storage get {:?}",e)))?;

        let (_,aepub_bytes) : ([u8;32],Vec<u8>) = corepack::from_bytes(&get_msg).map_err(|e| anyhow!(format!("Unable to decode get result {:?} {:?}",e, &get_msg)))?;

        let aepub : AuthT<Publication<C>,C> = authcomp::from_bytes(&aepub_bytes).map_err(|e| anyhow!(format!("Unable to decode deserealize auth type {:?}",e)))?;

        Api::locate(&aepub, locator,None)
        
                
    }
    */
    pub fn api_locate_verifier (doc: &ID, locator: SimplifiedLocator,proofs: ProofStream) -> Result<C>
        where
            AuthTVerifier<Publication<C>> : Is< Type=<C as AuthType<Publication<C>>>::AuthT>,
            <C as AuthType<Vec<u8>>>::AuthT: Ord
    {

        let aepub_verifier = AuthTVerifier::<Publication<C>>::from(doc);
        let aepub : &<C as AuthType<Publication<C>>>::AuthT = aepub_verifier.into_ref();

        Api::locate(aepub, locator,Some(proofs))
        
                
    }
    
    pub fn locate_with_cfi( doc: & AuthT<Publication<C>,C>,locator: SimplifiedLocatorCFI, proof_stream : Option<ProofStream> ) -> Result<C> 
    where <C as AuthType<Vec<u8>>>::AuthT: Ord
        {

            let real_work = || {
                let unauth_doc_ref = doc.unauth();
                let unauth_doc = &*unauth_doc_ref.borrow();
                
                let raw_resources_ref = unauth_doc.raw_resources.unauth();
                let raw_resources = (&*raw_resources_ref.borrow());

                println!("Locate {:?}",&locator);

                if locator.media_type == "text/html" || locator.media_type == "text/xml" {
                    if let Some(result) = raw_resources.get(&C::auth(locator.href.as_bytes().to_owned())){

                        if let Some(element_ref) = &result.1 {
                            let mut lexer = authselect::Token::lexer(&locator.cfi);

                            let p = FragmentParser::new();
                            let r = p.parse(&locator.cfi,lexer).map_err(|e| ApiError::String(String::from("No results")) ) ?;

                            let result = element_ref.select_cfi_fragment_fmt(&r);
                            return Ok(ApiResponse::String(result));
                        }

                    }
                }

                Err(ApiError::String(String::from("No results")))
            };

            let comp = match proof_stream {
                Some(p) => C::run_with_proofs (p,real_work),
                None => C::run (real_work),

            };

            Ok(comp)
        }    
    
}


//Prover implementation

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<MediaOverlayNode<Prover<ApiResponse,E>>> for Prover<ApiResponse,E> 

{
    type AuthT= AuthTProver<Vec< Rc<RefCell< AuthTProver<MediaOverlayNode<Prover<ApiResponse,E>>> >> >>;

}


impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<Meta<Prover<ApiResponse,E>> > for Prover<ApiResponse,E> 
{
    type AuthT= AuthTProver<Vec< Rc<RefCell< AuthTProver<Meta<Prover<ApiResponse,E>>>>>>>;

    
}

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<PublicationCollection<Prover<ApiResponse,E>>> for Prover<ApiResponse,E> 
{
    type AuthT= AuthTProver<Vec< Rc<RefCell< AuthTProver<PublicationCollection<Prover<ApiResponse,E>>>>>>>;

    
}

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned>  AuthContainer<Link<Prover<ApiResponse,E>>> for Prover<ApiResponse,E> 
{
    type AuthT= AuthTProver<Vec<Rc<RefCell< AuthTProver<Link<Prover<ApiResponse,E>>>>>>>;

    
}

//NoProofs implementation

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<MediaOverlayNode<NoProofs<ApiResponse,E>>> for NoProofs<ApiResponse,E> 

{
    type AuthT= AuthTNoProofs<Vec< Rc<RefCell< AuthTNoProofs<MediaOverlayNode<NoProofs<ApiResponse,E>>> >> >>;

}


impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<Meta<NoProofs<ApiResponse,E>> > for NoProofs<ApiResponse,E> 
{
    type AuthT= AuthTNoProofs<Vec< Rc<RefCell< AuthTNoProofs<Meta<NoProofs<ApiResponse,E>>>>>>>;

    
}

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<PublicationCollection<NoProofs<ApiResponse,E>>> for NoProofs<ApiResponse,E> 
{
    type AuthT= AuthTNoProofs<Vec< Rc<RefCell< AuthTNoProofs<PublicationCollection<NoProofs<ApiResponse,E>>>>>>>;

    
}

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned>  AuthContainer<Link<NoProofs<ApiResponse,E>>> for NoProofs<ApiResponse,E> 
{
    type AuthT= AuthTNoProofs<Vec<Rc<RefCell< AuthTNoProofs<Link<NoProofs<ApiResponse,E>>>>>>>;

    
}

//Verifier

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<MediaOverlayNode<Verifier<ApiResponse,E>>> for Verifier<ApiResponse,E> 
{
    type AuthT= AuthTVerifier<Vec<Rc<RefCell< AuthTVerifier<MediaOverlayNode<Verifier<ApiResponse,E>>>>>>>;

    
}

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<Meta<Verifier<ApiResponse,E>> > for Verifier<ApiResponse,E> 
{
    type AuthT= AuthTVerifier<Vec<Rc<RefCell< AuthTVerifier<Meta<Verifier<ApiResponse,E>>>>>>>;

    
}

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<PublicationCollection<Verifier<ApiResponse,E>>> for Verifier<ApiResponse,E> 
{
    type AuthT= AuthTVerifier<Vec< Rc<RefCell< AuthTVerifier<PublicationCollection<Verifier<ApiResponse,E>>>>>>>;

    
}

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned>  AuthContainer<Link<Verifier<ApiResponse,E>>> for Verifier<ApiResponse,E> 
{
    type AuthT= AuthTVerifier<Vec<Rc<RefCell< AuthTVerifier<Link<Verifier<ApiResponse,E>>>>>>>;

    
}


//Prover

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<MediaOverlayNode<Prover<(),E>>> for Prover<(),E> 
{
    type AuthT= AuthTProver<Vec<Rc<RefCell< AuthTProver<MediaOverlayNode<Prover<(),E>>>>>>>;

    
}

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<Meta<Prover<(),E>>> for Prover<(),E> 
{
    type AuthT= AuthTProver<Vec<Rc<RefCell< AuthTProver<Meta<Prover<(),E>>>>>>>;

    
}

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<PublicationCollection<Prover<(),E>> > for Prover<(),E> 
{
    type AuthT= AuthTProver<Vec<Rc<RefCell< AuthTProver<PublicationCollection<Prover<(),E>>>>>>>;

    
}

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned>  AuthContainer<Link<Prover<(),E>> > for Prover<(),E> 
{
    type AuthT= AuthTProver<Vec<Rc<RefCell< AuthTProver<Link<Prover<(),E>>>>>>>;

    
}

//NoProofs

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<MediaOverlayNode<NoProofs<(),E>>> for NoProofs<(),E> 
{
    type AuthT= AuthTNoProofs<Vec<Rc<RefCell< AuthTNoProofs<MediaOverlayNode<NoProofs<(),E>>>>>>>;

    
}

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<Meta<NoProofs<(),E>>> for NoProofs<(),E> 
{
    type AuthT= AuthTNoProofs<Vec<Rc<RefCell< AuthTNoProofs<Meta<NoProofs<(),E>>>>>>>;

    
}

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<PublicationCollection<NoProofs<(),E>> > for NoProofs<(),E> 
{
    type AuthT= AuthTNoProofs<Vec<Rc<RefCell< AuthTNoProofs<PublicationCollection<NoProofs<(),E>>>>>>>;

    
}

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned>  AuthContainer<Link<NoProofs<(),E>> > for NoProofs<(),E> 
{
    type AuthT= AuthTNoProofs<Vec<Rc<RefCell< AuthTNoProofs<Link<NoProofs<(),E>>>>>>>;

    
}


//Verifier

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<MediaOverlayNode<Verifier<(),E>>> for Verifier<(),E> 
{
    type AuthT= AuthTVerifier<Vec<Rc<RefCell< AuthTVerifier<MediaOverlayNode<Verifier<(),E>>>>>>>;

    
}

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<Meta<Verifier<(),E>> > for Verifier<(),E> 
{
    type AuthT= AuthTVerifier<Vec<Rc<RefCell< AuthTVerifier<Meta<Verifier<(),E>>>>>>>;

    
}

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned> AuthContainer<PublicationCollection<Verifier<(),E>>> for Verifier<(),E> 
{
    type AuthT= AuthTVerifier<Vec<Rc<RefCell< AuthTVerifier<PublicationCollection<Verifier<(),E>>>>>>>;

    
}

impl<E: DecodeOwned + Encode + Debug + Clone +PartialEq + Serialize + DeserializeOwned>  AuthContainer<Link<Verifier<(),E>>> for Verifier<(),E> 
{
    type AuthT= AuthTVerifier<Vec<Rc<RefCell< AuthTVerifier<Link<Verifier<(),E>>>>>>>;

    
}
