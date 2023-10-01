use anyhow::{anyhow,Result,Error};
use alloc::vec::Vec;
use alloc::vec;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::format;
use core::convert::TryInto;

use lazy_static::lazy_static;

use authcomp::{Computation,HashType,Prover,Verifier};
use authcomp::ProofStream;

use spin::Mutex;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;


use crate::alloc::string::ToString;
use crate::api::{Api,ApiResponse,ApiError};
use authselect::SimplifiedLocator;

use wapc_guest as guest;
use guest::prelude::*;

use corepack;

/*
lazy_static! {

    static ref API_PROVER : Mutex<Api<Prover<ApiResponse>>> = Mutex::new(Api::<Prover<ApiResponse>>::new());
  
}
*/
  

#[no_mangle]
pub extern "C" fn wapc_init() {

  register_function("prover::register", prover_register);

  register_function("prover::manifest", prover_manifest);
  register_function("prover::resource", prover_resource);
  register_function("prover::search", prover_search);
  register_function("prover::locate", prover_locate);
  register_function("prover::metadata", prover_metadata);
  register_function("prover::cover", prover_cover);

  register_function("verifier::manifest", verifier_manifest);
  register_function("verifier::resource", verifier_resource);
  register_function("verifier::search", verifier_search);
  register_function("verifier::locate", verifier_locate);
  register_function("verifier::metadata", verifier_metadata);
  register_function("verifier::cover", verifier_cover);



  #[cfg(test)]
  register_function("test::main",test);
}

#[cfg(test)]
fn test(msg: &[u8]) -> CallResult {
    crate::test_all();
    corepack::to_bytes(Vec::<u8>::new()).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(format!("Unable to serialize id {:?}",e)))))
}

#[derive(Serialize,Deserialize)]
struct RegisterRequest {
    epubsource: Vec<u8>
}

fn prover_register( msg: &[u8]) -> CallResult {

    let epubsource : &[u8]= corepack::from_bytes(msg).map_err(|e| guest::errors::new(guest::errors::ErrorKind::BadDispatch(format!("Unable to get epub source {:?}",e))))?;

    let id = Api::<Prover<ApiResponse,ApiError>>::register("prover",epubsource).map_err(|e| guest::errors::new(guest::errors::ErrorKind::BadDispatch(format!("Unable to register {:?}",e))))?;
        
    corepack::to_bytes(id.data).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(format!("Unable to serialize id {:?}",e)))))
    
    
}


fn prover_manifest(msg: &[u8]) -> CallResult {

    let id : HashType = corepack::from_bytes(msg).map_err(|e| guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string())))?;

    let rcomputation = Api::<Prover<ApiResponse,ApiError>>::api_manifest_prover("prover",&id,None);

    match rcomputation {
        Ok(mut comp) => {

            corepack::to_bytes(comp).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
        },
        Err(e) => corepack::to_bytes(e.to_string()).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
    }
       

}

fn verifier_manifest(msg: &[u8]) -> CallResult {

    let (id , proofs) : (HashType,ProofStream) = corepack::from_bytes(msg).map_err(|e| guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string())))?;

    let rcomputation = Api::<Verifier<ApiResponse,ApiError>>::api_manifest_verifier("verifier",&id,Some(proofs));

    match rcomputation {
        Ok(comp) => {
            
            corepack::to_bytes(comp).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
        },
        Err(e) => corepack::to_bytes(e.to_string()).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
    }
}   

    fn prover_resource(msg: &[u8]) -> CallResult {

            let (id,resource_name) : (HashType,Vec<u8>) = corepack::from_bytes(msg).map_err(|e| guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string())))?;
        
            let rcomputation = Api::<Prover<ApiResponse,ApiError>>::api_resource_prover("prover",&id,resource_name);
        
            match rcomputation {
                Ok(comp) => {
        
                    corepack::to_bytes(comp).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
                },
                Err(e) => corepack::to_bytes(e.to_string()).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
            }
            
        
    }    

    fn verifier_resource(msg: &[u8]) -> CallResult {

        let (id,resource_name,proofs) : (HashType,Vec<u8>,ProofStream) = corepack::from_bytes(msg).map_err(|e| guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string())))?;
    
        let rcomputation = Api::<Verifier<ApiResponse,ApiError>>::api_resource_verifier("verifier",&id,resource_name,proofs);
    
        match rcomputation {
            Ok(comp) => {
    
                corepack::to_bytes(comp).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
            },
            Err(e) => corepack::to_bytes(e.to_string()).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
        }
    }

    fn prover_search(msg: &[u8]) -> CallResult {

            let (id,resource_name,selector) : (HashType,Vec<u8>,Vec<u8>) = corepack::from_bytes(msg).map_err(|e| guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string())))?;
        
            let rcomputation = Api::<Prover<ApiResponse,ApiError>>::api_search_prover("prover",&id,resource_name,selector);
        
            match rcomputation {
                Ok(comp) => {
        
                    corepack::to_bytes(comp).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
                },
                Err(e) => corepack::to_bytes(e.to_string()).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
            }
            
        
    }            
    
    fn verifier_search(msg: &[u8]) -> CallResult {

        let (id,resource_name,selector,proofs) : (HashType,Vec<u8>,Vec<u8>,ProofStream) = corepack::from_bytes(msg).map_err(|e| guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string())))?;
    
        let rcomputation = Api::<Verifier<ApiResponse,ApiError>>::api_search_verifier("verifier",&id,resource_name,selector,proofs);
    
        match rcomputation {
            Ok(comp) => {
    
                corepack::to_bytes(comp).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
            },
            Err(e) => corepack::to_bytes(e.to_string()).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
        }
    }

    fn prover_metadata(msg: &[u8]) -> CallResult {

        let id : HashType = corepack::from_bytes(msg).map_err(|e| guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string())))?;
    
        let rcomputation = Api::<Prover<ApiResponse,ApiError>>::api_metadata_prover("prover",&id,None);
    
        match rcomputation {
            Ok(mut comp) => {
    
                corepack::to_bytes(comp).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
            },
            Err(e) => corepack::to_bytes(e.to_string()).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
        }
           
    
    }

    fn verifier_metadata(msg: &[u8]) -> CallResult {

        let (id , proofs) : (HashType,ProofStream) = corepack::from_bytes(msg).map_err(|e| guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string())))?;
    
        let rcomputation = Api::<Verifier<ApiResponse,ApiError>>::api_metadata_verifier("verifier",&id,Some(proofs));
    
        match rcomputation {
            Ok(comp) => {
                
                corepack::to_bytes(comp).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
            },
            Err(e) => corepack::to_bytes(e.to_string()).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
        }
    }


    fn prover_cover(msg: &[u8]) -> CallResult {

        let id : HashType = corepack::from_bytes(msg).map_err(|e| guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string())))?;
    
        let rcomputation = Api::<Prover<ApiResponse,ApiError>>::api_cover_prover("prover",&id,None);
    
        match rcomputation {
            Ok(mut comp) => {
    
                corepack::to_bytes(comp).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
            },
            Err(e) => corepack::to_bytes(e.to_string()).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
        }
           
    
    }

    fn verifier_cover(msg: &[u8]) -> CallResult {

        let (id , proofs) : (HashType,ProofStream) = corepack::from_bytes(msg).map_err(|e| guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string())))?;
    
        let rcomputation = Api::<Verifier<ApiResponse,ApiError>>::api_cover_verifier("verifier",&id,Some(proofs));
    
        match rcomputation {
            Ok(comp) => {
                
                corepack::to_bytes(comp).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
            },
            Err(e) => corepack::to_bytes(e.to_string()).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
        }
    }

    fn prover_locate(msg: &[u8]) -> CallResult {

        let (id,locator) : (HashType,SimplifiedLocator) = corepack::from_bytes(msg).map_err(|e| guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string())))?;
    
        let rcomputation = Api::<Prover<ApiResponse,ApiError>>::api_locate_prover("prover",&id,locator);
    
        match rcomputation {
            Ok(comp) => {
    
                corepack::to_bytes(comp).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
            },
            Err(e) => corepack::to_bytes(e.to_string()).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
        }
        
    }            

    fn verifier_locate(msg: &[u8]) -> CallResult {

        let (id,locator,proofs) : (HashType,SimplifiedLocator,ProofStream) = corepack::from_bytes(msg).map_err(|e| guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string())))?;

        let rcomputation = Api::<Verifier<ApiResponse,ApiError>>::api_locate_verifier("verifier",&id,locator,proofs);

        match rcomputation {
            Ok(comp) => {

                corepack::to_bytes(comp).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
            },
            Err(e) => corepack::to_bytes(e.to_string()).map_err(|e| Box::new(guest::errors::new(guest::errors::ErrorKind::BadDispatch(e.to_string()))))
        }
    }