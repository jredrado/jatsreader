
//------------------------------------------------------------------------------
//-- NoProofs is a computation which returns a result of type a and a proof stream
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


#[derive(Debug,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub struct NoProofs<A,E> {
    #[n(0)] result: Result<A,E>,
}



impl<A: Default,E> Default for NoProofs<A,E> {
    fn default() -> Self {
        NoProofs::<A,E> {
            result: Ok(A::default()),

        }
    }
}


#[repr(C)]
#[derive(Debug,Clone,PartialEq)]
pub struct AuthTNoProofs<A> {
    value: Rc<RefCell<A>>,
    digest: HashType
}

impl<A:Default+Serialize+Encode> Default for AuthTNoProofs<A> {
    fn default() -> Self {
        AuthTNoProofs::<A> {
            digest: hash::<Vec<u8>>(&shallow(&A::default())),
            value: Rc::new(RefCell::<A>::default()),
        }
    }
}


impl<A> AuthTNoProofs<A> {

}

impl<A:Serialize + Clone + Default + Debug + PartialEq+Encode + DecodeOwned + DeserializeOwned> AuthTypeReq for AuthTNoProofs<A> {}


impl<A: Encode> Encode for AuthTNoProofs<A> {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> {

        e.bytes(&self.digest.data).map (|_| ())?;
        e.encode(&*(*self.value).borrow()).map(|_| ())
    }
}

impl<'b, A: Decode<'b> + Default> Decode<'b> for AuthTNoProofs<A> {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, minicbor::decode::Error> {
        
        
        
        let rdigest = d.bytes();
        match rdigest {
            Ok(digest) => {
                            let rvalue = A::decode(d);
                            match rvalue {
                                Ok(value) => Ok(AuthTNoProofs::<A>{ digest: SHashType {data: digest.try_into().unwrap() },value: Rc::new(RefCell::new(value))}),
                                Err(e) => Err(e)
                            }
            }
            Err(e) => Err(e)
        }
        
        
    }
}



//The shallow projection of Auth type is the hash
impl<A: Serialize> Serialize for AuthTNoProofs<A> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        
        serializer.serialize_bytes(&self.digest.data)

    }
}



impl<'de,A:Default> Deserialize<'de> for AuthTNoProofs<A> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let h = deserializer.deserialize_bytes(HashVisitor)?;

        Ok(AuthTNoProofs::<A>{ digest: h,value: Rc::new(RefCell::new(A::default()))})
          
    }
}


impl <A: Eq> Eq for AuthTNoProofs<A> {
}

impl <A: PartialOrd + Ord> PartialOrd for AuthTNoProofs<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <A: PartialEq + Eq + Ord> Ord for AuthTNoProofs<A>  {
    fn cmp(&self, other: &Self) -> Ordering{
        //We order with digest, it must be equal in NoProofs and verifier
        self.digest.cmp(&other.digest)
    }
}

impl<A:Default+Encode+DecodeOwned,E> NoProofs<A,E> {

    fn get_authtype<'a,T:Serialize+Clone+Default + Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON>(a:&'a <NoProofs<A,E> as AuthType<T>>::AuthT) -> &'a AuthTNoProofs<T>
            
    {
            a
    }

    fn get_authtype_mut <'a,T:Serialize+Clone+Default + Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON>(a:&'a mut <NoProofs<A,E> as AuthType<T>>::AuthT) -> &'a mut AuthTNoProofs<T>
            
    {
            a
    }

    
    fn get_authtype_container<'a,T:Serialize+Clone+Default + Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON>(a:&'a <NoProofs<A,E> as AuthType<T>>::AuthT) 
            -> &'a <NoProofs<A,E> as AuthType<T>>::AuthT
            
    {
            a
    }
    

    fn new_auth_t<T:Serialize+Clone+Default + Debug+PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON>(value: T) -> <NoProofs<A,E> as AuthType<T>>::AuthT {
        AuthTNoProofs::<T> {
            digest: <Self as Computation>::hash(&<Self as Computation>::shallow(&value)),
            value: Rc::new(RefCell::new(value))
        }
    }

}

impl<A:Default+Encode+DecodeOwned,T:Serialize+Clone + Default + Debug +PartialEq+Encode+DecodeOwned+DeserializeOwned + ToJSON,E> AuthType<T> for NoProofs<A,E> 
{
    type AuthT= AuthTNoProofs<T>;

}



impl<T:Default+Encode+DecodeOwned,E> Computation for NoProofs<T,E> {

    type T = T;
    type E = E;
    type P = CorePackProjection;
    //Constructor
    fn new (prfs:Option<&ProofStream>) -> Self {
        if let Some(_p) = prfs {
                NoProofs::<T,E> {
                    result: Ok(T::default()),
                }

        }else {
            NoProofs::<T,E> {
                result: Ok(T::default()),
            }
        }
    }


    //Monadic computation
    fn pure(&mut self,r:T) -> Self where T: Serialize {
        let projection = Self::shallow(&r);
        
        let mut l = LinkedList::new();
        l.push_back(projection);

        NoProofs::<T,E> {
            result: Ok(r),
        }
    }

    //self is modified instead of return a new computation 
    //mergind the proofs and a new result
    fn bind <F: FnOnce(&T) -> Self>(&mut self, f: F) {
        
        if let Ok(result) = &self.result {
            let p = f(&result);

            self.result = p.result;

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
    
    fn transfer<O,U>(&mut self,value: Self::T,_from_comp: O)    
            where O:Computation<T=U>
    {

        self.result = Ok(value);

    }
    fn get_proofs(&self) -> &ProofStream {
        todo!()
    }

    fn transfer_proofs(self) -> ProofStream {
        ProofStream::new()
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

        Rc::clone(&av.value)
    }

    
    fn unauth_mut<A> (&mut self,a: &mut <Self as AuthType<A>>::AuthT) -> Rc<RefCell<A>>
                where   A: Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
                        Self:AuthType<A>
    {
        let av = Self::get_authtype(a);

        Rc::clone(&av.value)
    }

    fn unauth2<A> (a: &<Self as AuthType<A>>::AuthT) -> Rc<RefCell<A>>
                where   A: Serialize + DeserializeOwned+ Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
                        Self:AuthType<A>
    {
        let av = Self::get_authtype(a);

        Rc::clone(&av.value)
    }

    
    fn unauth_mut2<A> (a: &mut <Self as AuthType<A>>::AuthT) -> Rc<RefCell<A>>
                where   A: Serialize + DeserializeOwned + Clone+Default+Debug+PartialEq+Encode+DecodeOwned + ToJSON,
                        Self:AuthType<A>
    {
        let av = Self::get_authtype(a);

        Rc::clone(&av.value)
    }

    fn run<F> (f: F) -> Self
    where   
        F: FnOnce() -> Result<T,E>
        {
            let p = globals::get::<RefCell<ProofStream>>();
            if let Some(proofs) = p {
                proofs.borrow_mut().clear();
                NoProofs::<T,E> {
                    result : f(),
                }
            }else {
                NoProofs::<T,E>::default()
            }
        }

    fn run_with_proofs<F> (_prfs: ProofStream,f: F,) -> Self
        where   
            F: FnOnce() -> Result<T,E>{

                Self::run(f)
    }


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

impl<T,E> Deref for NoProofs<T,E> {
    type Target = Result<T,E>;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

impl<T,E> DerefMut for NoProofs<T,E> {

    fn deref_mut(&mut self) -> &mut Self::Target { 
        &mut self.result
    }

}


impl<A> UnAuth<A> for AuthTNoProofs<A> 
    where A: Serialize + DeserializeOwned + Clone + Default + Debug + PartialEq + Encode + DecodeOwned
           
{

    fn unauth(&self) ->  Rc<RefCell<A>>
    {

        let _v = &*self.value.borrow();

        Rc::clone(&self.value)

    }
}

impl<A> UnAuthMut<A> for AuthTNoProofs<A> 
    where Self: UnAuth<A>
           
{

    fn unauth_mut(&mut self) ->  Rc<RefCell<A>>
    {

      self.unauth()

    }


}

impl<A> AuthUpdate for AuthTNoProofs<A> 
    where A: Serialize + DeserializeOwned + Clone + Default + Debug + PartialEq + Encode + DecodeOwned
    {
        fn update(&mut self) 
        {
    
            let value = &*self.value.borrow();
            self.digest = hash::<Vec<u8>>(&shallow(value))
        }
    }

use nanoserde::SerJsonState;

impl<A:ToJSON > ToJSON for AuthTNoProofs<A> 
    where A : Serialize + DeserializeOwned + Clone + Default + Debug + PartialEq + Encode + DecodeOwned
{

    fn ser_json(&self, d: usize, s: &mut SerJsonState) {
        let r = self.unauth();
       
        (&*r.borrow()).ser_json(d,s);
    }
}

impl<A,E> ToJSON for NoProofs<A,E> {

    fn ser_json(&self, _d: usize, _s: &mut SerJsonState) {
        //Ignore
    }
}