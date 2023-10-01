

use serde::ser::Serializer;
use serde::de::Deserializer;

use crate::{Serialize,Deserialize,DeserializeOwned,DecodeOwned};
use minicbor::{Encode, Decode};

use minicbor::Encoder;
use minicbor::encode::Write;
use minicbor::encode::Error;
use minicbor::Decoder;

use core::fmt::Debug;
use std::rc::Rc;
use std::vec::Vec;
use core::convert::TryInto;

use core::cmp::Ordering;
use core::cmp::Ord;

use crate::computation::*;
use crate::hashvisitor::HashVisitor;

use core::cell::{Cell,RefCell};



//use authallocator::{print,print_str};

use crate::globals;
use crate::ToJSON;

#[derive(Debug,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub struct Verifier<A,E,P=crate::computation::CorePackProjection> {
    #[n(0)] result: Result<A,E>,
    #[n(1)] proofs : ProofStream,
    #[n(2)] p : core::marker::PhantomData<P>
}

impl<A: Default,E,P> Default for Verifier<A,E,P> {
    fn default() -> Self {
        Verifier::<A,E,P> {
            proofs: ProofStream::default(),
            result: Ok(A::default()),
            p: core::marker::PhantomData::default()
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct AuthTVerifier<A> 
    
{
    digest: HashType,
    //dummy: PhantomData<A>
    value: Rc<RefCell<A>>, //Only to return the value in unauth
    has_value: Cell<bool>
}

impl<A> AuthTVerifier<A> {


}

impl<A: Encode> Encode for AuthTVerifier<A> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        self.digest.data.encode(e)
    }
}

impl<'b, A: Decode<'b> + Default> Decode<'b> for AuthTVerifier<A> {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, minicbor::decode::Error> {
        d.bytes().map(| h| {
            AuthTVerifier::<A>{ digest: SHashType {data: h.try_into().unwrap() },value: Rc::new(RefCell::new(A::default())),has_value:Cell::new(false)}
        })
    }
}

impl<A:Serialize + Clone + Default + Debug + PartialEq+Encode+DecodeOwned + DeserializeOwned> AuthTypeReq for AuthTVerifier<A> {}

impl<A:Default+Serialize+Encode> Default for AuthTVerifier<A> {
    fn default() -> Self {
        AuthTVerifier::<A> {
            digest: hash::<Vec<u8>>(&shallow(&A::default())),
            value: Rc::new(RefCell::<A>::default()),
            has_value: Cell::new(false)
        }
    }
}

impl<A:Default> From<&HashType> for AuthTVerifier<A> {
    fn from(h:&HashType) -> Self {
        AuthTVerifier::<A> {
            digest: h.clone(),
            value: Rc::new(RefCell::default()),
            has_value: Cell::new(false)
        }
    }
}

impl<A:Default> From<HashType> for AuthTVerifier<A> {
    fn from(h:HashType) -> Self {
        AuthTVerifier::<A> {
            digest: h,
            value: Rc::new(RefCell::default()),
            has_value: Cell::new(false)
        }
    }
}


//The shallow projection of Auth type is the hash
impl<A: Serialize> Serialize for AuthTVerifier<A> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.digest.data)
    }
}

impl<'de,A:Default> Deserialize<'de> for AuthTVerifier<A> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let h = deserializer.deserialize_bytes(HashVisitor)?;

        Ok(AuthTVerifier::<A>{ digest: h,value: Rc::new(RefCell::new(A::default())),has_value:Cell::new(false)})
          
    }
}


impl <A: Eq> Eq for AuthTVerifier<A> {
}

impl <A: PartialOrd + Ord> PartialOrd for AuthTVerifier<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <A: PartialEq + Eq + Ord> Ord for AuthTVerifier<A>  {
    fn cmp(&self, other: &Self) -> Ordering{
        self.digest.cmp(&other.digest)
    }
}

impl<A:Default+Encode+DecodeOwned,E,P:Projection> Verifier<A,E,P> {

    fn new_auth_t<T:Serialize+Clone+Default+Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON>(value: T) -> <Verifier<A,E,P> as AuthType<T>>::AuthT {
        AuthTVerifier::<T> {
            digest: <Self as Computation>::hash(&<Self as Computation>::shallow(&value)),
            value: Rc::new(RefCell::new(T::default())),
            has_value : Cell::new (false)
            
        }
    }

    fn get_digest<'a,T:Serialize+Clone+Default+Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON>(a:&'a <Verifier<A,E,P> as AuthType<T>>::AuthT) -> &'a HashType
            
    {
            &a.digest
    }

    fn get_value<T:Serialize+Clone+Default+Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON>(a:& <Verifier<A,E,P> as AuthType<T>>::AuthT) -> Rc<RefCell<T>>
            
    {
            Rc::clone(&a.value)
    }

    fn get_authtype<'a,T:Serialize+Clone+Default+Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON>(a:&'a <Verifier<A,E,P> as AuthType<T>>::AuthT) -> &'a AuthTVerifier<T>
            
    {
            a
    }

    fn get_authtype_mut <'a,T:Serialize+Clone+Default + Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON>(a:&'a mut <Verifier<A,E,P> as AuthType<T>>::AuthT) -> &'a mut AuthTVerifier<T>
            
    {
            a
    }

    /*
    fn get_digest_container<'a,T:Serialize+Clone+Default+Debug+PartialEq+Encode+DecodeOwned>(a:&'a <Verifier<A,E> as AuthContainer<T>>::AuthT) -> &'a HashType
            
    {
            &a.digest
    }

    fn get_value_container<T:Serialize+Clone+Default+Debug+PartialEq+Encode+DecodeOwned>(a:& <Verifier<A,E> as AuthContainer<T>>::AuthT) -> Rc<RefCell<Vec<<Self as AuthType<T>>::AuthT>>>
            
    {
            Rc::clone(&a.value)
    }

    fn get_authtype_container<'a,T:Serialize+Clone+Default + Debug+PartialEq+Encode+DecodeOwned>(a:&'a <Verifier<A,E> as AuthContainer<T>>::AuthT) 
    -> &'a AuthTVerifier<Vec<<Self as AuthType<T>>::AuthT>>
    
    {
        a
    }
    */


}

impl<A:Default+Encode+DecodeOwned,T:Serialize+Clone + Default+Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON,E,P> AuthType<T> for Verifier<A,E,P> 
{
    type AuthT= AuthTVerifier<T>;

    /*
    fn new_auth_type() -> Self::AuthT {
        AuthTVerifier::<T> {
            digest: <Self as Computation>::hash(&<Self as Computation>::shallow(&HashType::default())),
            value: Rc::new(RefCell::<T>::default()),
            has_value : Cell::new(false)
        }
    }
    

    fn signature (a:&<Self as AuthType<T>>::AuthT) -> &HashType {
        &Verifier::<T,E>::get_authtype( a ).digest
    }
    */
}

/*
impl<A:Default,T:Serialize+Clone + Default + Debug +PartialEq> AuthContainer<T> for Verifier<A> 
{
    type AuthT= AuthTVerifier<Vec<AuthTVerifier<T>>>;
}
*/


/*
impl<A:Default+Encode+DecodeOwned,T:Serialize+Clone + Default + Debug +PartialEq+Encode+DecodeOwned,E> AuthContainer<T> for Verifier<A,E> {
    type AuthT= AuthTVerifier<Vec< <Self as AuthType<T>>::AuthT>>;
}
*/

impl<T:Default+Encode+DecodeOwned,E,P:Projection> Computation for Verifier<T,E,P> {

    type T = T;
    type E = E;
    type P = P;

    //Constructor
    fn new (prfs:Option<&ProofStream>) -> Self {
        if let Some(p) = prfs {
            Verifier::<T,E,P> {
                proofs: p.clone(),
                result: Ok(T::default()),
                p: core::marker::PhantomData::default()
            }
        } else {
            Verifier::<T,E,P> {
                proofs: ProofStream::default(),
                result: Ok(T::default()),
                p: core::marker::PhantomData::default()
            }
        }
    }

    fn get_proofs(&self) -> &ProofStream {
        &self.proofs
    }

    fn transfer_proofs(self) -> ProofStream {
        self.proofs
    }
    
    //Monadic computation
    fn pure(&mut self,r:T) -> Self where T: Serialize {

        let h = <Self as Computation>::hash(&<Self as Computation>::shallow(&r));

        match self.proofs.pop_front() {
            Some (x) => {
                assert_eq!(h , <Self as Computation>::hash::<ProofType>(x.as_ref() ));
                
            }
            None => panic!("Not proof")
        }

        Verifier::<T,E,P> {
            proofs: ProofStream::default(),
            result: Ok(r),
            p: core::marker::PhantomData::default()
        }
    }

    //self is modified instead of return a new computation 
    //mergind the proofs and a new result
    fn bind <F: FnOnce(&T) -> Self>(&mut self, f: F) {
        
        if let Ok(result) = &self.result {
            let mut p = f(&result);

            self.result = p.result;
            self.proofs.append(&mut p.proofs);
        }

    }

    //State computation
    fn get(&self) -> Option<&T> {
        match self.result {
            Ok(ref t) => Some(t),
            Err (_) => None
        }
    }

    fn put(&mut self, value:T) {
        self.result = Ok(value);
    }

    fn put_err(&mut self, e : Self::E) {
        self.result = Err(e);
    }
    
    fn transfer<O,U>(&mut self,value: Self::T,from_comp: O)    
        where O:Computation<T=U>
        {
            self.result = Ok(value);
            self.proofs = from_comp.transfer_proofs();
        }

    //Auth computation
    fn auth<A> (value: A) -> <Self as AuthType<A>>::AuthT  
            where   A: Serialize + Clone+Default+Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON,
                    Self:AuthType<A>
    {
            Self::new_auth_t(value)
    }

    fn unauth<A> (& mut self,a: & <Self as AuthType<A>>::AuthT) -> Rc<RefCell<A>>
                where   A: Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
                        Self:AuthType<A>
    {
        let h = Self::get_digest(a);
        
        match self.proofs.pop_front() {
            Some (x) => {
                //print(&format!("Unauth H: {:?} Proof: {:?}",h,x));
                assert_eq!(h , &<Self as Computation>::hash::<ProofType>(x.as_ref()));

                let av = Self::get_authtype(a);
                if av.has_value.get() {
                    Self::get_value(a)
                } else {

                    //let v = serde_json::from_str::<A>(&x).unwrap();
                    //let v = postcard::from_bytes(&x).unwrap();
                    let v = Self::unshallow(&x).unwrap();
                    let ra = Self::get_value(a);
                
                    ra.replace(v);
                    av.has_value.set(true);
                    ra
                }
            }
            None => {
                        //print_str("Not proof");
                        panic!("Not proof")
                    }
        }
        


    }

   

    fn unauth_mut<A> (& mut self,a: & mut <Self as AuthType<A>>::AuthT) -> Rc<RefCell<A>>
                where   A: Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned+ToJSON,
                        Self:AuthType<A>
    {
        self.unauth(a)
    }

    fn unauth2<A> (a: & <Self as AuthType<A>>::AuthT) -> Rc<RefCell<A>>
    where   A: Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned+ToJSON,
            Self:AuthType<A>
    {
        let h = Self::get_digest(a);

        let p = globals::get::<RefCell<ProofStream>>();

        if let Some(proofs) = p {
            match proofs.borrow_mut().pop_front() {
                Some (x) => {
                    //print(&format!("Unauth H: {:?} Proof: {:?}",h,x));
                    assert_eq!(h , &<Self as Computation>::hash::<ProofType>(x.as_ref()));

                    let av = Self::get_authtype(a);
                    if av.has_value.get() {
                        Self::get_value(a)
                    } else {

                        //let v = serde_json::from_str::<A>(&x).unwrap();
                        //let v = postcard::from_bytes(&x).unwrap();
                        let v = Self::unshallow(&x).unwrap();
                        let ra = Self::get_value(a);
                    
                        ra.replace(v);
                        av.has_value.set(true);
                        ra
                    }
                }
                None => {
                            //print_str("Not proof");
                            panic!("Not proof")
                        }
            }
        }else {
            panic!("Not proof variable")
        }

    }



    fn unauth_mut2<A> (a: & mut <Self as AuthType<A>>::AuthT) -> Rc<RefCell<A>>
        where   A: Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned+ToJSON,
                Self:AuthType<A>
    {
        Self::unauth2(a)
    }

    fn run_with_proofs<F> (prfs: ProofStream,f: F,) -> Self
    where   
        F: FnOnce() -> Result<T,E>
    {
        let p = globals::get::<RefCell<ProofStream>>();
        if let Some(proofs) = p {
            proofs.replace(prfs);
            Verifier::<T,E,P> {
                result : f(),
                proofs : proofs.borrow().clone(),
                p: core::marker::PhantomData::default()
            }
        }else {
            Verifier::<T,E,P>::default()
        }
    }

    fn run<F> (f: F) -> Self
    where   
        F: FnOnce() -> Result<Self::T,Self::E>{
            Self::run_with_proofs(ProofStream::new(),f)
    }

    /*
    fn unauth2_container<A> (a: &<Self as AuthContainer<A>>::AuthT) ->Rc<RefCell<Vec<<Self as AuthType<A>>::AuthT>>>
    where   A: Serialize + DeserializeOwned+ Clone+Default+Debug+PartialEq+Encode+DecodeOwned,
            Self:AuthContainer<A> {
                let h = Self::get_digest_container(a);

                let p = globals::get::<RefCell<ProofStream>>();
        
                if let Some(proofs) = p {
                    match proofs.borrow_mut().pop_front() {
                        Some (x) => {
                            print(&format!("Unauth H: {:?} Proof: {:?}",h,x));
                            assert_eq!(h , &<Self as Computation>::hash::<ProofType>(x.as_ref()));
        
                            let av = Self::get_authtype_container(a);
                            if av.has_value.get() {
                                Self::get_value_container(a)
                            } else {
        
                                //let v = serde_json::from_str::<A>(&x).unwrap();
                                //let v = postcard::from_bytes(&x).unwrap();
                                let v = Self::unshallow(&x).unwrap();
                                let ra = Self::get_value_container(a);
                            
                                ra.replace(v);
                                av.has_value.set(true);
                                ra
                            }
                        }
                        None => {
                                    print_str("Not proof");
                                    panic!("Not proof")
                                }
                    }
                }else {
                    panic!("Not proof variable")
                }
        

    }

    fn unauth_mut2_container<A> (a: & mut <Self as AuthContainer<A>>::AuthT) -> Rc<RefCell<Vec<<Self as AuthType<A>>::AuthT>>>
    where   A: Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned,
            Self:AuthContainer<A>{

            Self::unauth2_container(a)
    }    
    */

    fn auth_update<A> (a: &mut <Self as AuthType<A>>::AuthT)
                where   A: Serialize + DeserializeOwned+ Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
                        Self:AuthType<A>
    {
        let av = Self::get_authtype_mut(a);
        let value = &*av.value.borrow();

        av.digest = <Self as Computation>::hash(&<Self as Computation>::shallow(value))

    }    

    fn signature<'a,A: 'a> (a:&'a <Self as AuthType<A>>::AuthT) -> &HashType 
        where   A: Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
                Self:AuthType<A>
    {
        &Self::get_authtype( a ).digest
    }
}

use core::ops::{Deref,DerefMut};

impl<T,E,P> Deref for Verifier<T,E,P> {
    type Target = Result<T,E>;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

impl<T,E,P> DerefMut for Verifier<T,E,P> {

    fn deref_mut(&mut self) -> &mut Self::Target { 
        &mut self.result
    }

}


impl<A> UnAuth<A> for AuthTVerifier<A> 
    where A: Serialize + DeserializeOwned + Clone + Default + Debug + PartialEq + Encode + DecodeOwned
           
{

    fn unauth(&self) -> Rc<RefCell<A>>
    {

        let p = globals::get::<RefCell<ProofStream>>();

        if let Some(proofs) = p {
            match proofs.borrow_mut().pop_front() {
                Some (x) => {
                    //print(&format!("Unauth H: {:?} Proof: {:?} Hash: {:?}",self.digest,x,hash::<ProofType>(x.as_ref())));
                    assert_eq!(&self.digest , &hash::<ProofType>(x.as_ref()));

                    if self.has_value.get() {
                        Rc::clone(&self.value)
                    } else {

                        let v = unshallow(&x).unwrap();
    
                        self.value.replace(v);
                        self.has_value.set(true);

                        Rc::clone(&self.value)
                    }
                }
                None => {
                            //print_str("Not proof");
                            panic!("Not proof")
                        }
            }
        }else {
            panic!("Not proof variable")
        }
    }
}

impl<A> UnAuthMut<A> for AuthTVerifier<A> 
    //where A: Serialize + DeserializeOwned + Clone + Default + Debug + PartialEq + Encode + DecodeOwned
    where Self: UnAuth<A>
{
    fn unauth_mut(&mut self) ->  Rc<RefCell<A>>
    {

      self.unauth()

    }
}

impl<A> AuthUpdate for AuthTVerifier<A> 
    where A: Serialize + DeserializeOwned + Clone + Default + Debug + PartialEq + Encode + DecodeOwned
{
    fn update(&mut self) 
    {

        let value = &*self.value.borrow();
        self.digest = hash::<Vec<u8>>(&shallow(value))
    }
}


use nanoserde::SerJsonState;

impl<A:ToJSON> ToJSON for AuthTVerifier<A> 
    where A : Serialize + DeserializeOwned + Clone + Default + Debug + PartialEq + Encode + DecodeOwned
{

    fn ser_json(&self, d: usize, s: &mut SerJsonState) {
        //self.digest.data.ser_json(d,s)
        let r = self.unauth();
        //authallocator::print(&alloc::format!("AuthTVerifier ser_json {:?}",&r));
        (&*r.borrow()).ser_json(d,s);
    }
}