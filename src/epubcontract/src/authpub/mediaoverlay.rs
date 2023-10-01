
use std::vec::Vec;
use std::string::String;
use std::boxed::Box;

use std::rc::Rc;
use core::cell::RefCell;

use authcomp::{Computation,AuthType,AuthT,AuthContainer,AuthTContainer,ProofStream};
use authcomp::{Encode,Decode,Serialize,Deserialize,DeserializeOwned,DecodeOwned};
use nanoserde::ToJSON;

use crate::authpub::types::*;



#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode,ToJSON)]
pub struct MediaOverlayNode<C> 
    where   
        C:AuthType<String>,
        //C:AuthType<Vec<AuthT<String,C>>>,
        C:AuthContainer<MediaOverlayNode<C>>,
        //C:AuthType<Vec<AuthT<MediaOverlayNode<C>,C>>>
        C:AuthType<Vec<Rc<RefCell<AuthT<String,C>>>>>,
        
{
    #[n(0)] text: AString<C>,
    #[n(1)] audio: AString<C>,
    #[n(2)] role: AVecString<C>,
    #[n(3)] children: AuthTContainer<MediaOverlayNode<C>,C>
}

pub type AMediaOverlayMode<C> = AuthT<MediaOverlayNode<C>,C>;

