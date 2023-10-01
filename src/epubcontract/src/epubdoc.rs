
use crate::epubarchive::EPubArchive;
use crate::api::ApiResponse;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use std::vec::Vec;
use std::string::String;
use std::collections::BTreeMap;
use std::boxed::Box;

use authcomp::{AuthT,Computation,AuthType,ProofStream};
use authparser::Document;

use anyhow::{anyhow,Result,Error};

use json_minimal::*;

enum EpubError {
    Generic(String)
}


#[cfg(test)]
use std::format;

pub type SpineType = Vec<String>;
pub type ResourcesType = BTreeMap<String,(String,String)>;
pub type MetadataType = BTreeMap<String,Vec<String>>;
pub type AEPubDoc<C> = AuthT<EPubDoc<C>,C>;

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq)]
pub struct EPubDoc<C> 
    where 
            C:Computation,
            C:AuthType<EPubArchive>,
            C:AuthType<SpineType>,
            C:AuthType<ResourcesType>,
            C:AuthType<MetadataType>

{

    archive: AuthT<EPubArchive,C>,

    /// epub spine ids
    spine: AuthT<Vec<String>,C>,

    /// resource id -> (path, mime)
    resources: AuthT<BTreeMap<String, (String, String)>,C>,

    /// table of content, list of `NavPoint` in the toc.ncx
    //pub toc: Vec<NavPoint>,

    metadata: AuthT<BTreeMap<String, Vec<String>>,C>
}

impl<C> EPubDoc<C> 
    where 
        C:Computation,
        C:AuthType<EPubArchive>,
        C:AuthType<SpineType>,
        C:AuthType<ResourcesType>,
        C:AuthType<MetadataType>
{


    pub fn new (source: &[u8]) -> Result<Self> {

        let archive = EPubArchive::new(source)?;
        let (spine,resources,metatdata) = EPubDoc::<C>::fill(&archive)?;

        Ok (
            EPubDoc {
                archive: C::auth(archive),
                spine : C::auth(spine),
                resources : C::auth(resources),
                metadata : C::auth(metatdata),
        })

    }

    //API

    //Get Manifest

    //Private 
    
    fn fill (archive: &EPubArchive) -> Result<(SpineType,ResourcesType,MetadataType)>{        

        let container_file_content = archive.get_container_file().ok_or_else(|| anyhow!("No container file"))?;

        let document = Document::new_from_xml(container_file_content);

        let root_file = document.find_element("rootfile")?;

        let package_file = root_file.attr("full-path").ok_or_else(|| anyhow!("No Full path attribute"))?;

        let package_file_vec = package_file.as_bytes().to_vec();
        let package_file_content = archive.get(&package_file_vec).ok_or_else(|| anyhow!("No content"))?;


        let package_document = Document::new_from_xml(package_file_content);

        let spine = EPubDoc::<C>::new_spine(&package_document)?;
        let resources = EPubDoc::<C>::new_resources(&package_document)?;

        Ok((spine,resources,MetadataType::new()))

    }
    

    fn new_spine(package_document: &Document) -> Result<SpineType> {
        let mut result = SpineType::new();

        let spine = package_document.find("spine")?;
        for itemref in spine.with_name("itemref").into_iter() {
            let element = itemref.as_element().ok_or_else(|| anyhow!("No element"))?;
            result.push(String::from(element.attr("idref").ok_or_else(|| anyhow!("No idref attribute"))?));
        }

        Ok(result)
    }

    fn new_resources(package_document: &Document) -> Result<ResourcesType> {
        let mut result = ResourcesType::new();

        let resources = package_document.find("manifest")?;
        for item in resources.with_name("item").into_iter() {
            let element = item.as_element().ok_or_else(|| anyhow!("No element"))?;

            let id = String::from(element.attr("id").ok_or_else(|| anyhow!("No id attribute"))?);
            let href = String::from(element.attr("href").ok_or_else(|| anyhow!("No href attribute"))?);
            let media_type = String::from(element.attr("media-type").ok_or_else(|| anyhow!("No media-type attribute"))?);

            result.insert(id, (href,media_type));
        }

        Ok(result)
 
    }  
 
    pub fn manifest(&self,proof_stream : Option<&ProofStream>) -> C 
        where   
            C:Computation<T=ApiResponse>
    {
        

        let mut json = Json::new();

        json.add(EPubDoc::<C>::context());


        let mut result = C::new(proof_stream);
        result.put(ApiResponse::String(json.print()));
        result
    }

    fn context() -> Json {
        Json::OBJECT {
            name: String::from("@context"),

            value: Box::new(
                Json::STRING( String::from("https://readium.org/webpub-manifest/context.jsonld") )
            )
        }
    }

    
}

