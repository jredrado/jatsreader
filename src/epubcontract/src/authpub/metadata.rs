use std::vec::Vec;
use std::string::String;
use std::collections::BTreeMap;
use std::boxed::Box;
use core::any::Any;

use std::rc::Rc;
use core::cell::RefCell;

use authcomp::{Computation,AuthType,AuthTypeReq,AuthT,AuthContainer,AuthTContainer,ProofStream};
use authcomp::{Encode,Decode,Serialize,Deserialize,DeserializeOwned,DecodeOwned};

use nanoserde::ToJSON;

use crate::authpub::types::*;


#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode,ToJSON)]
pub struct Metadata<C> 
    where
        C:AuthType<String>,
        C:AuthType<Meta<C>>,
        C:AuthType<Vec<AuthT<Meta<C>,C>>>,
        C:AuthType<MultiLanguage<C>>,
        C:AuthType<BTreeMap<AuthT<String,C>, AuthT<String,C>>>,
        C:AuthType<BelongsTo<C>>,
        C:AuthType<Collection<C>>,
        C:AuthType<Subject<C>>,
        C:AuthType<Properties<C>>,
        C:AuthType<Contributor<C>>,
        C:AuthType<u32>,
        C:AuthType<f32>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        C:AuthType<Vec<AuthT<Collection<C>,C>>>,
        //C:AuthType<Vec<AuthT<Subject<C>,C>>>,
        //C:AuthType<Vec<AuthT<Contributor<C>,C>>>,
        C:AuthContainer<Meta<C>>,
        C:AuthType<Meta<C>>,

        C:AuthType<Vec<Rc<RefCell< AuthT<Meta<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Contributor<C>,C>>>>>,
        C:AuthType<Vec<Rc<RefCell< AuthT<Subject<C>,C>>>>>,

        C:AuthType<Vec<Rc<RefCell<AuthT<String,C>>>>>,

{

    #[n(0)]  #[nserde(rename = "@type")] pub rdftype: AString<C>,
    #[n(1)] pub title:  AMultiLanguage<C>,
    #[n(2)] pub identifier: Option<AString<C>>,
    #[n(4)] pub author: Option<AVec<C,Contributor<C>>>,
    #[n(5)] pub translator: Option<AVec<C,Contributor<C>>>,
    #[n(6)] pub editor: Option<AVec<C,Contributor<C>>>,
    #[n(7)] pub artist: Option<AVec<C,Contributor<C>>>,
    #[n(8)] pub illustrator: Option<AVec<C,Contributor<C>>>,
    #[n(9)] pub letterer: Option<AVec<C,Contributor<C>>>,
    #[n(10)] pub penciler: Option<AVec<C,Contributor<C>>>,
    #[n(11)] pub colorist: Option<AVec<C,Contributor<C>>>,
    #[n(12)] pub inker: Option<AVec<C,Contributor<C>>>,
    #[n(13)] pub narrator: Option<AVec<C,Contributor<C>>>,
    #[n(14)] pub contributor: Option<AVec<C,Contributor<C>>>,
    #[n(15)] pub publisher: Option<AVec<C,Contributor<C>>>,
    #[n(16)] pub imprint: Option<AVec<C,Contributor<C>>>,
    #[n(17)] pub language: Option<AVecString<C>>,
    #[n(18)] pub modified: Option<AString<C>>, //TO FIX
    #[n(19)] #[nserde(rename = "published")] pub publication_date: Option<AString<C>>, //TO FIX
    #[n(20)] pub description: Option<AString<C>>,
    #[n(21)] pub direction: Option<AString<C>>,
    #[n(22)] pub presentation: Option<AProperties<C>>,
    #[n(23)] pub source: Option<AString<C>>,
    #[n(24)] pub epub_type: Option<AVecString<C>>,
    #[n(25)] pub rights: Option<AString<C>>,
    #[n(26)] pub subject: Option<AVec<C,Subject<C>>>,
    #[n(28)] pub belongs_to: Option<ABelongsTo<C>>,
    #[n(29)] pub duration: Option<AuthT<u32,C>>,

    #[n(30)] pub other_metadata: Option<AVec<C,Meta<C>>>
    
}

pub type AMetadata<C> = AuthT<Metadata<C>,C>;

#[derive(Default,Debug,Serialize,Deserialize,Clone,PartialEq,Encode,Decode,ToJSON)]
pub struct Meta<C> 
    where
        C:AuthType<String>,
        //C:AuthType<Meta<C>>,
        //C:AuthType<Vec<AuthT<Meta<C>,C>>>,
        C:AuthContainer<Meta<C>>,
        
{
    #[n(0)] property: AString<C>,
    #[n(1)] value: AString<C>, //TO FIX
    #[n(2)] children: AuthTContainer<Meta<C>,C>
}

pub type AMeta<C> = AuthT<Meta<C>,C>;

/*
impl<C> Default for Meta<C> 
    where
        C:Computation,
        C:AuthType<String>,
        //C:AuthType<Meta<C>>,
        //C:AuthType<Vec<AuthT<Meta<C>,C>>>,
        C:AuthContainer<Meta<C>>

{
    fn default() -> Self {

        Meta {
            property: C::auth(String::default()),
            children: C::auth(Vec::default()),
            value: C::auth(String::default()),

        }
    }
}
*/


#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode,ToJSON)]
pub struct Contributor<C> 
    where   
        C:AuthType<String>,
        C:AuthType<MultiLanguage<C>>,
        C:AuthType<BTreeMap<AuthT<String,C>, AuthT<String,C>>>
{
    #[n(0)] pub name: AuthT<MultiLanguage<C>,C>,
    #[n(1)] pub sort_as: Option<AString<C>>,
    #[n(2)] pub identifier: Option<AString<C>>,
    #[n(3)] pub role: Option<AString<C>>
}

pub type AContributor<C> = AuthT<Contributor<C>,C>;

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode,ToJSON)]
pub struct Properties<C> 
    where   
        C:AuthType<String>,
        //C:AuthType<Vec<<C as AuthType<String>>::AuthT>>,
        C:AuthType<Vec<Rc<RefCell<AuthT<String,C>>>>>,
{
    #[n(0)] pub contains: Option<AVecString<C>>,
    #[n(1)] pub layout: Option<AString<C>>,
    #[n(2)] pub mediaoverlay: Option<AString<C>>,
    #[n(3)] pub orientation: Option<AString<C>>,
    #[n(4)] pub overflow: Option<AString<C>>,
    #[n(5)] pub page: Option<AString<C>>,
    #[n(6)] pub spread: Option<AString<C>>,
    //encrypted
}

pub type AProperties<C> = AuthT<Properties<C>,C>;

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode,ToJSON)]
pub struct Subject<C> 
    where
        C:AuthType<String>
{
    #[n(0)] pub name: AString<C>,
    #[n(1)] pub sort_as: Option<AString<C>>,
    #[n(2)] pub scheme: Option<AString<C>>,
    #[n(3)] pub code: Option<AString<C>>
}

pub type ASubject<C> = AuthT<Subject<C>,C>;

#[derive(Debug,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub enum MultiLanguage<C> 
    where   
        C:AuthType<String>,
        C:AuthType<BTreeMap<AuthT<String,C>, AuthT<String,C>>>
{
    #[n(0)] SingleString( #[n(0)] AString<C>),
    #[n(1)] MultiString( #[n(0)] AuthT<BTreeMap<AString<C>,AString<C>>,C>)
}

pub type AMultiLanguage<C> = AuthT<MultiLanguage<C>,C>;

impl<C> Default for MultiLanguage<C> 
    where   
        C:AuthType<String>,
        C:AuthType<BTreeMap<AuthT<String,C>, AuthT<String,C>>>,
        C:Computation
{
    fn default() -> Self {
        MultiLanguage::<C>::SingleString(C::auth(String::from("")))
    }
}

impl<C> authcomp::ToJSON for MultiLanguage<C> 
    where   
        C:AuthType<String>,
        C:AuthType<BTreeMap<AuthT<String,C>, AuthT<String,C>>>,
        C:Computation
{

    fn ser_json (&self, d: usize, s: &mut authcomp::JSONState) {

        match &self {
            MultiLanguage::SingleString(value) => value.ser_json(d,s),
            MultiLanguage::MultiString(value) => {
                s.st_pre();
                value.ser_json(d,s);
                s.st_post(d);
            }
        }
        
    

        
    }
}

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode,ToJSON)]
pub struct BelongsTo<C> 
    where   
        C:AuthType<String>,
        C:AuthType<f32>,
        C:AuthType<Collection<C>>,
        C:AuthType<Vec<<C as AuthType<Collection<C>>>::AuthT>>
{
    #[n(0)] series: AuthT<Vec<ACollection<C>>,C>,
    #[n(1)] collection: AuthT<Vec<ACollection<C>>,C>
}

pub type ABelongsTo<C> = AuthT<BelongsTo<C>,C>;

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode,ToJSON)]
pub struct Collection<C> 
    where
        C:AuthType<String>,
        C:AuthType<f32>
{
    #[n(0)] name: AString<C>,
    #[n(1)] sort_as: AString<C>,
    #[n(2)] identifier: AString<C>,
    #[n(3)] position: AFloat<C>
}

pub type ACollection<C> = AuthT<Collection<C>,C>;
