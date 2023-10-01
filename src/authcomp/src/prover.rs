
//------------------------------------------------------------------------------
//-- Prover is a computation which returns a result of type a and a proof stream
//------------------------------------------------------------------------------

use crate::{Serialize,Deserialize,DeserializeOwned,DecodeOwned};
use minicbor::{Encode, Decode};

use serde::ser::{Serializer};
use serde::de::Deserializer;

use minicbor::Encoder;
use minicbor::encode::Write;
use minicbor::encode::Error;
use minicbor::Decoder;

use core::cmp::Ordering;
use core::cmp::Ord;

use core::cell::{RefCell};

use std::rc::Rc;

use core::fmt::Debug;
use std::collections::LinkedList;
use std::vec::Vec;

use core::convert::TryInto;

use crate::computation::*;
use crate::hashvisitor::HashVisitor;

use crate::globals;

use crate::ToJSON;

/*
#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub struct Prover<A,E> {
    #[n(0)] result: A,
    #[n(1)] pub proofs: ProofStream
}*/

/*
#[derive(Debug,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub enum Prover<A,E> {
    #[n(0)] Ok { #[n(0)] result: A, #[n(1)] proofs: ProofStream},
    #[n(1)] Err( #[n(0)] E)
}
*/


#[derive(Debug,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub struct Prover<A,E,P=crate::computation::CorePackProjection> {
    #[n(0)] result: Result<A,E>,
    #[n(1)] pub proofs: ProofStream,
    #[n(2)] p : core::marker::PhantomData<P>

}



impl<A: Default,E,P> Default for Prover<A,E,P> {
    fn default() -> Self {
        Prover::<A,E,P> {
            result: Ok(A::default()),
            proofs: LinkedList::new(),
            p: core::marker::PhantomData::default()
        }
    }
}

/*
use serde::ser::Serializer;
impl<A: Serialize> Serialize for Prover<A> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_some(&self.result)
    }
}
*/

#[repr(C)]
#[derive(Debug,Clone,PartialEq)]
pub struct AuthTProver<A> {
    value: Rc<RefCell<A>>,
    digest: HashType
}

impl<A:Default+Serialize+Encode> Default for AuthTProver<A> {
    fn default() -> Self {
        AuthTProver::<A> {
            digest: hash::<Vec<u8>>(&shallow(&A::default())),
            value: Rc::new(RefCell::<A>::default()),
        }
    }
}


impl<A> AuthTProver<A> {

}

impl<A:Serialize + Clone + Default + Debug + PartialEq+Encode + DecodeOwned + DeserializeOwned> AuthTypeReq for AuthTProver<A> {}


impl<A: Encode> Encode for AuthTProver<A> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {
        //self.digest.data.encode(e)
        e.bytes(&self.digest.data).map (|_| ())?;
        e.encode(&*(*self.value).borrow()).map(|_| ())
    }
}

impl<'b, A: Decode<'b> + Default> Decode<'b> for AuthTProver<A> {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, minicbor::decode::Error> {
        
        /*d.bytes().map(| h| {
            AuthTProver::<A>{ digest: SHashType {data: h.try_into().unwrap() },value: Rc::new(RefCell::new(A::default()))}
        })*/
        
        
        let rdigest = d.bytes();
        match rdigest {
            Ok(digest) => {
                            let rvalue = A::decode(d);
                            match rvalue {
                                Ok(value) => Ok(AuthTProver::<A>{ digest: SHashType {data: digest.try_into().unwrap() },value: Rc::new(RefCell::new(value))}),
                                Err(e) => Err(e)
                            }
            }
            Err(e) => Err(e)
        }
        
        
    }
}



//The shallow projection of Auth type is the hash
impl<A: Serialize> Serialize for AuthTProver<A> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //serializer.serialize_some(&self.digest)
        serializer.serialize_bytes(&self.digest.data)
        /*
        let mut state = serializer.serialize_struct("AuthTProver", 2)?;
        state.serialize_field("digest", &self.digest.data)?;
        state.serialize_field("value", &*self.value)?;
        state.end()
        */
    }
}



impl<'de,A:Default> Deserialize<'de> for AuthTProver<A> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let h = deserializer.deserialize_bytes(HashVisitor)?;

        Ok(AuthTProver::<A>{ digest: h,value: Rc::new(RefCell::new(A::default()))})
          
    }
}


impl <A: Eq> Eq for AuthTProver<A> {
}

impl <A: PartialOrd + Ord> PartialOrd for AuthTProver<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <A: PartialEq + Eq + Ord> Ord for AuthTProver<A>  {
    fn cmp(&self, other: &Self) -> Ordering{
        //self.value.cmp(&other.value)
        //We order with digest, it must be equal in prover and verifier
        self.digest.cmp(&other.digest)
    }
}

impl<A:Default+Encode+DecodeOwned,E,P:Projection> Prover<A,E,P> {

    fn get_authtype<'a,T:Serialize+Clone+Default + Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON>(a:&'a <Prover<A,E,P> as AuthType<T>>::AuthT) -> &'a AuthTProver<T>
            
    {
            a
    }

    fn get_authtype_mut <'a,T:Serialize+Clone+Default + Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON>(a:&'a mut <Prover<A,E,P> as AuthType<T>>::AuthT) -> &'a mut AuthTProver<T>
            
    {
            a
    }

    
    fn get_authtype_container<'a,T:Serialize+Clone+Default + Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON>(a:&'a <Prover<A,E,P> as AuthType<T>>::AuthT) 
            -> &'a <Prover<A,E> as AuthType<T>>::AuthT
            
    {
            a
    }
    

    fn new_auth_t<T:Serialize+Clone+Default + Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON>(value: T) -> <Prover<A,E,P> as AuthType<T>>::AuthT {
        AuthTProver::<T> {
            digest: <Self as Computation>::hash(&<Self as Computation>::shallow(&value)),
            value: Rc::new(RefCell::new(value))
        }
    }

}

impl<A:Default+Encode+DecodeOwned,T:Serialize+Clone + Default + Debug +PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON,E,P:Projection> AuthType<T> for Prover<A,E,P> 
{
    type AuthT= AuthTProver<T>;

    /*
    fn new_auth_type() -> Self::AuthT {
        AuthTProver::<T> {
            digest: <Self as Computation>::hash(&<Self as Computation>::shallow(&HashType::default())),
            value: Rc::new(RefCell::<T>::default())
        }
    }

    
    
    fn signature (a:&<Self as AuthType<T>>::AuthT) -> &HashType {
        &Prover::<T,E>::get_authtype( a ).digest
    }*/
}

/*
impl<A:Default+Encode+DecodeOwned,T:Serialize+Clone + Default + Debug +PartialEq+Encode+DecodeOwned,E> AuthContainer<T> for Prover<A,E> {
    type AuthT= AuthTProver<Vec< <Self as AuthType<T>>::AuthT>>;
}
*/


/*
impl<A:Default,T:Serialize+Clone + Default + Debug +PartialEq> AuthContainer<T> for Prover<A> 
{
    type AuthT= AuthTProver<Vec<AuthTProver<T>>>;
}
*/

impl<T:Default+Encode+DecodeOwned,E,P:Projection> Computation for Prover<T,E,P> {

    type T = T;
    type E = E;
    type P = P;
    //Constructor
    fn new (prfs:Option<&ProofStream>) -> Self {
        if let Some(p) = prfs {
                Prover::<T,E,P> {
                    result: Ok(T::default()),
                    proofs: p.clone(),
                    p : core::marker::PhantomData::default()
                }

        }else {
            Prover::<T,E,P> {
                result: Ok(T::default()),
                proofs: LinkedList::new(),
                p : core::marker::PhantomData::default()
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
        let projection = Self::shallow(&r);
        
        let mut l = LinkedList::new();
        l.push_back(projection);

        Prover::<T,E,P> {
            result: Ok(r),
            proofs: l,
            p : core::marker::PhantomData::default()
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

    fn unauth<A> (&mut self,a: &<Self as AuthType<A>>::AuthT) -> Rc<RefCell<A>>
                where   A: Serialize + DeserializeOwned+ Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
                        Self:AuthType<A>
    {
        let av = Self::get_authtype(a);
        let v = &*av.value.borrow();

        let projection = Self::shallow(v);
        self.proofs.push_back(projection);

        Rc::clone(&av.value)
    }

    
    fn unauth_mut<A> (&mut self,a: &mut <Self as AuthType<A>>::AuthT) -> Rc<RefCell<A>>
                where   A: Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
                        Self:AuthType<A>
    {
        let av = Self::get_authtype(a);
        let v = &*av.value.borrow();

        let projection = Self::shallow(v);
        self.proofs.push_back(projection);

        Rc::clone(&av.value)
    }

    fn unauth2<A> (a: &<Self as AuthType<A>>::AuthT) -> Rc<RefCell<A>>
                where   A: Serialize + DeserializeOwned+ Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
                        Self:AuthType<A>
    {
        let av = Self::get_authtype(a);
        let v = &*av.value.borrow();

        let projection = Self::shallow(v);
        //self.proofs.push_back(projection);
        let p = globals::get::<RefCell<ProofStream>>();
        if let Some(proofs) = p {
            proofs.borrow_mut().push_back(projection);
        }

        Rc::clone(&av.value)
    }

    
    fn unauth_mut2<A> (a: &mut <Self as AuthType<A>>::AuthT) -> Rc<RefCell<A>>
                where   A: Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
                        Self:AuthType<A>
    {
        let av = Self::get_authtype(a);
        let v = &*av.value.borrow();

        let projection = Self::shallow(v);
        //self.proofs.push_back(projection);
        let p = globals::get::<RefCell<ProofStream>>();
        if let Some(proofs) = p {
            proofs.borrow_mut().push_back(projection);
        }

        Rc::clone(&av.value)
    }

    fn run<F> (f: F) -> Self
    where   
        F: FnOnce() -> Result<T,E>
        {
            let p = globals::get::<RefCell<ProofStream>>();
            if let Some(proofs) = p {
                proofs.borrow_mut().clear();
                Prover::<T,E,P> {
                    result : f(),
                    proofs : proofs.take(), //proofs.borrow().clone()
                    p : core::marker::PhantomData::default()
                }
            }else {
                Prover::<T,E,P>::default()
            }
        }

    fn run_with_proofs<F> (_prfs: ProofStream,f: F,) -> Self
        where   
            F: FnOnce() -> Result<T,E>{

                Self::run(f)
    }
            /*
    fn unauth2_container<A> (a: &<Self as AuthContainer<A>>::AuthT) ->Rc<RefCell<Vec<<Self as AuthType<A>>::AuthT>>>
                where   A: Serialize + DeserializeOwned+ Clone+Default+Debug+PartialEq+Encode+DecodeOwned,
                        Self:AuthContainer<A>
    {
        let av = Self::get_authtype_container(a);
        let v = &*av.value.borrow();

        let projection = Self::shallow(v);
        //self.proofs.push_back(projection);
        let p = globals::get::<RefCell<ProofStream>>();
        if let Some(proofs) = p {
            proofs.borrow_mut().push_back(projection);
        }

        Rc::clone(&av.value)
    }
    
    
    fn unauth_mut2_container<A> (a: &mut <Self as AuthContainer<A>>::AuthT) -> Rc<RefCell<Vec<<Self as AuthType<A>>::AuthT>>>
                where   A: Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
                        Self:AuthContainer<A>
    {
        let av = Self::get_authtype_container(a);
        let v = &*av.value.borrow();

        let projection = Self::shallow(v);
        //self.proofs.push_back(projection);
        let p = globals::get::<RefCell<ProofStream>>();
        if let Some(proofs) = p {
            proofs.borrow_mut().push_back(projection);
        }

        Rc::clone(&av.value)
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

impl<T,E,P> Deref for Prover<T,E,P> {
    type Target = Result<T,E>;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

impl<T,E,P> DerefMut for Prover<T,E,P> {

    fn deref_mut(&mut self) -> &mut Self::Target { 
        &mut self.result
    }

}


impl<A> UnAuth<A> for AuthTProver<A> 
    where A: Serialize + DeserializeOwned + Clone + Default + Debug + PartialEq + Encode + DecodeOwned
           
{

    fn unauth(&self) ->  Rc<RefCell<A>>
    {

        let v = &*self.value.borrow();

        let projection = shallow(v);
    
        let p = globals::get::<RefCell<ProofStream>>();
        if let Some(proofs) = p {
            //authallocator::print(&alloc::format!("Unauth: {:?} \n\t-> {:?} \n\t-> {:?}",&v,&projection,&hash::<Vec<u8>>(&projection)));
            proofs.borrow_mut().push_back(projection);
        }

        Rc::clone(&self.value)

    }
}

impl<A> UnAuthMut<A> for AuthTProver<A> 
    //where A: Serialize + DeserializeOwned + Clone + Default + Debug + PartialEq + Encode + DecodeOwned
    where Self: UnAuth<A>
           
{

    fn unauth_mut(&mut self) ->  Rc<RefCell<A>>
    {

      self.unauth()

    }


}

impl<A> AuthUpdate for AuthTProver<A> 
    where A: Serialize + DeserializeOwned + Clone + Default + Debug + PartialEq + Encode + DecodeOwned
    {
        fn update(&mut self) 
        {
    
            let value = &*self.value.borrow();
            self.digest = hash::<Vec<u8>>(&shallow(value))
        }
    }

use nanoserde::SerJsonState;

impl<A:ToJSON > ToJSON for AuthTProver<A> 
    where A : Serialize + DeserializeOwned + Clone + Default + Debug + PartialEq + Encode + DecodeOwned
{

    fn ser_json(&self, d: usize, s: &mut SerJsonState) {
        //(&*self.value.borrow()).ser_json(d,s)
        let r = self.unauth();
        //authallocator::print(&alloc::format!("AuthTProver ser_json {:?}",&r));
        (&*r.borrow()).ser_json(d,s);
    }
}

impl<A,E> ToJSON for Prover<A,E> {

    fn ser_json(&self, _d: usize, _s: &mut SerJsonState) {
        //Ignore
    }
}