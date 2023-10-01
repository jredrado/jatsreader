//---------------------------------------------

//extern crate authmacro;

//use authmacro::auth;

//---------------------------------------------


use serde::{Serialize,Deserialize};
use serde::de::DeserializeOwned;


//use serde_big_array::big_array;

//use postcard::{to_allocvec};

use minicbor::{Encode, Decode};

use core::ops::FnOnce;
use core::convert::TryInto;

use crate::error;

use crate::ToJSON;

use core::cell::{RefCell};
use std::rc::Rc;
use std::format;
use std::vec::Vec;
use std::string::String;



use sha3::{Digest,Sha3_256};

pub type ProofType = std::vec::Vec<u8>;
pub type ProofStream = std::collections::LinkedList<ProofType>;

pub trait DecodeOwned: for<'de> Decode<'de> {}
impl<T> DecodeOwned for T where T: for<'de> Decode<'de> {}

//pub type HashType = GenericArray<u8, <Sha3_256 as Digest>::OutputSize>;

//big_array! { BigArray; }

const SHA3_256_OUTPUT : usize = 32;//256bits -> 32 bytes

#[derive(Serialize, Deserialize,Debug,PartialEq,Clone,Eq,PartialOrd,Ord,Encode,Decode)]
pub struct SHashType {
        #[cbor(n(0), with = "minicbor::bytes")]
        pub data: [u8;SHA3_256_OUTPUT],  
}

pub type HashType = SHashType;

impl Default for SHashType {
        fn default() -> Self {
                SHashType {
                        data: [0;SHA3_256_OUTPUT]
                }
        }
}

impl AsRef<[u8]> for SHashType
{
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

pub trait  Projection {

    fn shallow<A> (value: &A) -> ProofType 
            where A: Encode + Serialize;

    fn unshallow<'a,A: DeserializeOwned+Decode<'a>> (value: &'a[u8]) -> Result<A,error::Error>;

}

#[derive(Serialize, Deserialize,Debug,PartialEq,Clone,Eq,PartialOrd,Ord,Encode,Decode)]
pub struct CorePackProjection {

}

impl  Projection for CorePackProjection {  
        
        fn shallow<A> (value: &A) -> ProofType 
                where A: Encode + Serialize{
                        shallow(value)
        }

        fn unshallow<'a,A: DeserializeOwned+Decode<'a>> (value: &'a[u8]) -> Result<A,error::Error>{
                unshallow(value)
        }
}


//-------------------------------------------




///  #Authenticated Computation
///
///  A computation which returns a value of type T
///  This computation is modeled as a state monand
/// 
/// 

pub trait Computation
        
{

    type T: Default;
    type E;
    type P: Projection ;

    /// Constructor
    fn new(proofs:Option<&ProofStream>) -> Self;

    /// Monadic computation
    /// 
    /// Wrap a value as the return value of the computation
    fn pure (&mut self, r:Self::T) -> Self where Self::T:Serialize;

    /// Combine the this computation which other one
    fn bind <F: FnOnce(&Self::T) -> Self>(&mut self, f: F);

    /// State computation
    fn get(&self) -> Option<&Self::T>;
    fn put(&mut self,value: Self::T);
    fn put_err(&mut self, e : Self::E);

    fn get_proofs(&self) -> &ProofStream;
    fn transfer_proofs(self) -> ProofStream;

    fn transfer<O,U>(&mut self,value: Self::T,from_comp: O)    
                where O:Computation<T=U>;

    /// Auth computation
    fn auth<A> (value: A) -> <Self as AuthType<A>>::AuthT 
            where   A:Serialize + Clone + Default+Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON,
                    Self:AuthType<A>;

    /*             
    fn unauth<'a,A> (&mut self,a: &'a<Self as AuthType<A>>::AuthT) -> &'a A
            where   A:Serialize + DeserializeOwned + Clone+Default,
                    Self:AuthType<A>;

    fn unauth_mut<'a,A> (&mut self,a: &'a mut <Self as AuthType<A>>::AuthT) -> &'a mut A
                where   A: Serialize + DeserializeOwned + Clone+Default,
                        Self:AuthType<A>;
    */


        fn unauth2<A> (a: &<Self as AuthType<A>>::AuthT) -> Rc<RefCell<A>>
                where   A:Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
                        Self:AuthType<A>;

        fn unauth_mut2<A> (a: & mut <Self as AuthType<A>>::AuthT) -> Rc<RefCell<A>>
                where   A: Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
                        Self:AuthType<A>;

        /*
        fn unauth2_container<A> (a: &<Self as AuthContainer<A>>::AuthT) -> Rc<RefCell<Vec<<Self as AuthType<A>>::AuthT>>>
                        where   A:Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned,
                                Self:AuthContainer<A>,
                                Self:AuthType<A>;
        
        fn unauth_mut2_container<A> (a: & mut <Self as AuthContainer<A>>::AuthT) -> Rc<RefCell<Vec<<Self as AuthType<A>>::AuthT>>>
                        where   A: Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned,
                                Self:AuthContainer<A>,
                                Self:AuthType<A>;
        */

        fn unauth<A> (& mut self,a: &<Self as AuthType<A>>::AuthT) -> Rc<RefCell<A>>
          where   A:Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
            Self:AuthType<A>;

        fn unauth_mut<A> (& mut self,a: & mut <Self as AuthType<A>>::AuthT) -> Rc<RefCell<A>>
            where   A: Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
                    Self:AuthType<A>;

        fn auth_update<A> (a: &mut <Self as AuthType<A>>::AuthT)
                where   A: Serialize + DeserializeOwned+ Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
                            Self:AuthType<A>;


        fn run<F> (f: F) -> Self
                            where   
                                F: FnOnce() -> Result<Self::T,Self::E>;
        fn run_with_proofs<F> (prfs: ProofStream,f: F,) -> Self
                                where   
                                    F: FnOnce() -> Result<Self::T,Self::E>;

   /*
    fn unauth<'a,A> (&mut self,a: &'a<Self as AuthType<A>>::AuthT) -> &'a RefCell<A>
    where   A:Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq,
            Self:AuthType<A>;

    
    fn unauth_mut<'a,A> (&mut self,a: &'a mut <Self as AuthType<A>>::AuthT) -> &'a RefCell<A>
        where   A: Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq,
                Self:AuthType<A>;
    */

    fn signature<'a, A: 'a> (a:&'a <Self as AuthType<A>>::AuthT) -> &HashType
        where   A: Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
                Self:AuthType<A>;

    #[inline]
    fn shallow<A> (value: &A) -> ProofType 
            where A: Encode + Serialize             
    {
        Self::P::shallow(value)
    }

    #[inline]
    fn unshallow<'a,A: DeserializeOwned+Decode<'a>> (value: &'a[u8]) -> Result<A,error::Error> {
        Self::P::unshallow(value)
    }
    
    #[inline]
    fn hash<A: AsRef<[u8]>>(input: &A) -> HashType {
       hash(input)
    }

}


pub fn shallow<A> (value: &A) -> ProofType 
        where A:Serialize             
{


        /* 
        let mut buff = alloc::vec::Vec::with_capacity(1024);
        
        let result = serde_json_core::to_slice(value,&mut buff);

        println!("shallow {:?} {:?} {:?}",buff,buff.len(),result);
        buff
        */
        
        //corepack::to_bytes(value).unwrap()
        /*
        use minicbor::Encoder;

        let mut e = Encoder::new(Vec::new());
        value.encode(&mut e);
        e.into_inner()
        */

        corepack::to_bytes(value).unwrap()

        //Lenient JSON support recursive structures
        //serde_json_lenient::to_vec(value).unwrap()
        
}

pub fn unshallow<'a,A: DeserializeOwned> (value: &'a[u8]) -> Result<A,error::Error> {

        corepack::from_bytes(value).map_err(|e| error::Error::Shallow(String::from(format!("{:?}",e))))
        //minicbor::decode(value).map_err(|e| error::Error::Shallow(String::from(format!("{:?}",e))))
        //println!("unshallow {:?}",value);
        //serde_json_core::from_slice(value).map(|r| r.0).map_err(|e| error::Error::Shallow(String::from(format!("{:?}",e))))

        //serde_json_lenient::from_slice(value).map_err(|e| error::Error::Shallow(String::from(format!("{:?}",e))))


}

pub fn hash<A: AsRef<[u8]>>(input: &A) -> HashType {
        let mut hasher = Sha3_256::default();
        hasher.update(input);

        let msg = format!("slice with incorrect size {:?}",<Sha3_256 as Digest>::output_size());

        SHashType {
        data: hasher.finalize().as_slice().try_into().expect(&msg)
        }
}




//TODO Handle errors. Result<Vec<u8>>
pub fn to_vec <A> (value: &A) -> Vec<u8>
        where A: Encode                
{
        
        use minicbor::Encoder;

        let mut e = Encoder::new(Vec::new());
        let _ = value.encode(&mut e);

        e.into_inner()

                
}

pub fn from_bytes<'a,A: Decode<'a>> (value: &'a[u8]) -> Result<A,error::Error> {

        minicbor::decode(value).map_err(|e| error::Error::Shallow(String::from(format!("{:?}",e))))
        
}





/// 
/// #AuthT
/// AuthT<A>
/// Represents the computation's authenticated type
/// 
pub type AuthT<A,C> = <C as AuthType<A>>::AuthT;


pub trait AuthTypeReq : Serialize + DeserializeOwned + Clone + Default + Debug + PartialEq + Encode + DecodeOwned {

} 

/// 
/// #Authenticated type
/// 
/// AuthType represents the computation's authenticated type
/// This is a trick because stable Rust does not support generalized associtated types
/// 

use core::fmt::Debug;

pub trait AuthType<A> {
    type AuthT: AuthTypeReq + UnAuth<A> + UnAuthMut<A> + AuthUpdate + ToJSON;
    //fn new_auth_type() -> Self::AuthT;
    //fn signature (&self) -> &HashType;

}

pub trait AuthContainer<A> {
        //type AuthT: AuthType<IntoIterator<Item=AuthType<A,AuthT=<C as AuthType<A>>::AuthT>,IntoIter=Iterator<Item=AuthType<A,AuthT=<C as AuthType<A>>::AuthT>>>,AuthT=<C as AuthType<A>>::AuthT> ;
        //type AuthT: AuthType<Vec<AuthType<A,AuthT=<C as AuthType<A>>::AuthT>,AuthT=<C as AuthType<A>>::AuthT> ;
        type AuthT: AuthTypeReq + ToJSON;

        //fn into<'a,T,U:AuthType<T>>(from:&'a Self::AuthT) -> &'a <U as AuthType<T>>::AuthT;

        /*
        fn into<'a,T,U>(from:&'a Self::AuthT) -> &'a <U as AuthType<T>>::AuthT
        where   
                U:AuthType<T,AuthT=Self::AuthT>
                {
                        from
                }
        */
                
}

pub type AuthTContainer<A,C> = <C as AuthContainer<A>>::AuthT;


pub trait UnAuth<A> {
        fn unauth(&self)-> Rc<RefCell<A>>;  
}
    
pub trait UnAuthMut<A> {
        fn unauth_mut(&mut self)-> Rc<RefCell<A>>;
}

pub trait AuthUpdate {
        fn update(&mut self);
}

impl<T:Default,A:Default> UnAuthMut<A> for Option<T> 
        where T: UnAuth<A>
{

        fn unauth_mut(&mut self)-> Rc<RefCell<A>>{
                match self {
                        Some(ref value) => value.unauth(),
                        None => {
                                self.replace(T::default());
                                self.unauth_mut()
                        }
                }
        }


}


impl<T> AuthUpdate for Option<T> 
        where T: AuthUpdate {
                fn update(&mut self){   
                        match self {
                                Some(ref mut value) => value.update(),
                                None => {} //TO FIX
                        }
                }

}