
use authcomp::{AuthType,AuthT,AuthTypeReq};

use std::vec::Vec;
use std::string::String;
use std::rc::Rc;
use core::cell::RefCell;
use authselect_html5ever::ElementRef;

pub type AString<C> = AuthT<String,C>;
pub type AFloat<C> = AuthT<f32,C>;

pub type AVec<C,T> = AuthT<Vec<Rc<RefCell<AuthT<T,C>>>>,C>;
pub type AVecString<C> = AVec<C,String>;

pub type AResources<C> = AuthT<BTreeMap<AuthT<Vec<u8>,C>,(AuthT<Vec<u8>,C>,Option<ElementRef<C>>)>,C>;

use core::any::Any;
use core::any::TypeId;
use std::boxed::Box;
use std::collections::BTreeMap;

use core::fmt::Debug;
use authcomp::{Encode,Decode,Serialize,Deserialize,DecodeOwned,DeserializeOwned};


use typed_key::Key;

use core::iter::Iterator;

#[derive(Debug,Default)]
pub struct HashMap {
    data: BTreeMap<&'static str, Box<dyn Any>>
}

impl HashMap {
    pub fn new() -> HashMap {
        HashMap { data: BTreeMap::new() }
    }

    pub fn insert<T: Any>(&mut self, key: Key<T>, value: T) 
        where T: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned
    {
        self.data.insert(key.name(), Box::new(value));
    }

    pub fn get<T: Any>(&self, key: Key<T>) -> Option<&T> 
        where T: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned
    {
        //use core::any::type_name_of_val;
        use std::format;

        let value = self.data.get(key.name())?;
        
        //print(&format!("{:?}, u8: {:?}",type_name_of_val(&value),value.is::<String>()));
        let value = value.downcast_ref().expect("Corrupted HashMap");
        Some(value)
    }

    fn inner_value<T: Any> (value : &Box<dyn Any>) -> &T
        where T: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned
    {
        let value = value.downcast_ref().expect("Corrupted HashMap");
        value
    }
}


//Serialize,Deserialize,Clone,PartialEq,Encode,Decode
/*
use minicbor::encode::Write;
use minicbor::Encoder;
use minicbor::encode::Error;

impl Encode for HashMap
{
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> 
    {
        //e.map(as_u64(self.data.len()))?;
        for (k, v) in &self.data {
            k.encode(e)?;
            let value = HashMap::inner_value(v);
            //v.encode(e)?;
        }
        Ok(())
    }


}
*/


//https://github.com/BartMassey/typedata-rs/blob/master/src/stackque.rs

/// Wrapper type to help with type safety vs incorrect usage.
/// 

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub struct StackQue<T>( #[n(0)] T);

/// Type of empty `StackQue`. Pushing onto this creates a
/// new `StackQue`. Popping from a `StackQue` with one
/// element from either end returns this as the queue.

#[derive(Debug,Default,Serialize,Deserialize,Clone,PartialEq,Encode,Decode)]
pub struct EmptySQ;

/// Trait for pushing onto a `StackQue`: provides the
/// `push()` operation. From a queue's point of view,
/// `push()` is "`push_back()`".
pub trait Push<E, I> 
    where   
        E: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned,
        I: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned
{
    fn push(self, insert: E) -> I;
}

// Case: Push onto empty stack.
impl<E> Push<E, StackQue<(E, EmptySQ)>> for EmptySQ 
    where E: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned
{
    fn push(self, insert: E) -> StackQue<(E, EmptySQ)> {
        StackQue((insert, EmptySQ))
    }
}

// Case: Push onto non-empty stack.
impl<E, I> Push<E, StackQue<(E, I)>> for StackQue<I> 
    where
        E: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned,
        I: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned
{
    fn push(self, insert: E) -> StackQue<(E, I)> {
        StackQue((insert, self.0))
    }
}

/// Trait for popping a `StackQue`: provides the
/// `pop()` operation. From a queue's point of view,
/// `pop()` is "`pop_back()`".
pub trait Pop<E, R> 
    where   
        E: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned,
        R: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned
{
    fn pop(self) -> (E, R);
}

// Case: Pop last element.
impl<E> Pop<E, EmptySQ> for StackQue<(E, EmptySQ)> 
    where
        E: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned
{
    fn pop(self) -> (E, EmptySQ) {
        (self.0 .0, EmptySQ)
    }
}

// Case: Pop non-last element.
impl<E, E1, R1> Pop<E, StackQue<(E1, R1)>> for StackQue<(E, (E1, R1))> 
    where
        E: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned,
        E1: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned,
        R1: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned
{
    fn pop(self) -> (E, StackQue<(E1, R1)>) {
        (self.0 .0, StackQue(self.0 .1))
    }
}

/// Trait for popping the front element from a `StackQue`
/// viewed as a queue: provides the `pop_front()` operation.
pub trait PopFront<E, R> 
    where   
        E: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned,
        R: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned
{
    type Popped: Debug+Default+Serialize+DeserializeOwned+Clone+PartialEq+Encode+DecodeOwned;

    fn pop_front(self) -> (E, Self::Popped);
}

impl<E> PopFront<E, EmptySQ> for StackQue<(E, EmptySQ)> 
    where 
        E: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned
{
    type Popped = EmptySQ;

    fn pop_front(self) -> (E, Self::Popped) {
        (self.0 .0, EmptySQ)
    }
}

impl<E, E1, E2, R, R2> PopFront<E, R> for StackQue<(E1, (E2, R2))>
    where (E1, (E2, R2)): PopFront<E, R>,
        E: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned,
        E1: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned,
        E2: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned,
        R2: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned,
        R: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned
{
    type Popped = StackQue<<(E1, (E2, R2)) as PopFront<E, R>>::Popped>;

    fn pop_front(self) -> (E, Self::Popped) {
        let (e, q) = self.0.pop_front();
        (e, StackQue(q))
    }
}

impl<E> PopFront<E, EmptySQ> for (E, EmptySQ) 
    where E: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned
{
    type Popped = EmptySQ;

    fn pop_front(self) -> (E, Self::Popped) {
        (self.0, EmptySQ)
    }
}

impl<E, E1, E2, R, R2> PopFront<E, R> for (E1, (E2, R2))
    where (E2, R2): PopFront<E, R>,
        E: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned,
        E1: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned,
        E2: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned,
        R: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned,
        R2: Debug + Default + Serialize + DeserializeOwned + Clone + PartialEq + Encode + DecodeOwned
{
    type Popped = (E1, <(E2, R2) as PopFront<E, R>>::Popped);

    fn pop_front(self) -> (E, Self::Popped) {
        let (e, q) = self.1.pop_front();
        (e, (self.0, q))
    }
}


/// The end of a heterogeneous list.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Nil;

/// Main buildling block of a heterogeneous list.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Cons<H, T> {
    /// Value of this element of the list.
    pub head: H,
    /// Remaining elements of the list.
    pub tail: T,
}

impl<Head, Tail> Cons<Head, Tail> {
    /// Returns a iterator over this cons-list.
    pub fn iter<'a>(&'a self) -> ConsIterator<'a, Self> {
        ConsIterator::new(self)
    }
}

pub fn cons<H, T>(head: H, tail: T) -> Cons<H, T> {
    Cons { head, tail }
}

impl Default for Nil {
    fn default() -> Nil {
        Nil
    }
}

impl Nil {
   
    /// Returns an empty [ConsIterator](iter/struct.ConsIterator.html).
    ///
    /// See [ConsIterator](iter/struct.ConsIterator.html) for an example.
    pub fn iter<'a>(&'a self) -> ConsIterator<'a, Self> {
        ConsIterator::new(self)
    }

}

#[derive(Debug)]
pub struct ConsIterator<'a, List, A = Nil> {
    list: &'a List,
    adapter: A,
}

impl<'a, List> ConsIterator<'a, List> {
    /// Creates a new `ConsIterator` over an `Cons`-list
    pub fn new(list: &'a List) -> Self {
        ConsIterator { list, adapter: Nil }
    }
}
impl<'a, List, A> ConsIterator<'a, List, A> {
    /// Creates a new `ConsIterator` over an `Cons`-list with an adapter (see
    /// [Adapter](trait.Adapter.html)).
    pub fn with_adapter(list: &'a List, adapter: A) -> Self {
        ConsIterator { list, adapter }
    }
}

impl<'a, H, T, A> ConsIterator<'a, Cons<H, T>, A>
where
    A: Adapter<&'a H>,
{
    /// Returns the next value (if exists) along with a new iterator advanced to the next element of
    /// the list.
    pub fn next(mut self) -> (<A as Adapter<&'a H>>::Output, ConsIterator<'a, T, A>) {
        (
            self.adapter.adapt(&self.list.head),
            ConsIterator::with_adapter(&self.list.tail, self.adapter),
        )
    }
    /// Creates an iterator which call a [MapFunc](trait.MapFunc.html) on each element.
    ///
    /// See [MapAdapter](struct.MapAdapter.html) for more information.
    pub fn map<F>(self, f: F) -> ConsIterator<'a, Cons<H, T>, Cons<MapAdapter<F>, A>>
    where
        F: MapFunc<<A as Adapter<&'a H>>::Output>,
    {
        ConsIterator::with_adapter(
            self.list,
            Cons {
                head: MapAdapter { f },
                tail: self.adapter,
            },
        )
    }
}


/// An iterator component that transforms an input.
pub trait Adapter<T> {
    /// Transformation output type
    type Output;
    /// Transforms the input and returns its output
    fn adapt(&mut self, input: T) -> Self::Output;
}

impl<T> Adapter<T> for Nil {
    type Output = T;
    fn adapt(&mut self, input: T) -> Self::Output {
        input
    }
}

impl<T, Head, Tail> Adapter<T> for Cons<Head, Tail>
where
    Tail: Adapter<T>,
    Head: Adapter<<Tail as Adapter<T>>::Output>,
{
    type Output = <Head as Adapter<<Tail as Adapter<T>>::Output>>::Output;

    fn adapt(&mut self, input: T) -> Self::Output {
        self.head.adapt(self.tail.adapt(input))
    }
}


#[derive(Debug)]
pub struct MapAdapter<F> {
    f: F,
}

impl<F, T> Adapter<T> for MapAdapter<F>
where
    F: MapFunc<T>,
{
    type Output = <F as MapFunc<T>>::Output;
    fn adapt(&mut self, input: T) -> Self::Output {
        self.f.call(input)
    }
}

/// Function for use in mapping over heterogeneous lists.
///
/// This trait must be implemented for all types contained in the list.
pub trait MapFunc<T> {
    /// Output of mapped function
    type Output;
    /// Evaluate this function on the input
    fn call(&mut self, item: T) -> Self::Output;
}

pub trait Len {
    /// The length of this list
    const LEN: usize;

    /// Returns the length of this list
    fn len(&self) -> usize {
        Self::LEN
    }
}

impl Len for Nil {
    const LEN: usize = 0;
}
impl<H, T> Len for Cons<H, T>
where
    T: Len,
{
    const LEN: usize = 1 + <T as Len>::LEN;
}



/*
pub trait Key {
    type Value: AuthTypeReq;
}

#[derive(Debug,Default)]
pub struct Container {
    map: BTreeMap<TypeId, Box<dyn Any>>,
}

impl Container {
    pub fn new() -> Container {
        Container {
            map: BTreeMap::new(),
        }
    }

    pub fn set<K: Key+ 'static>(&mut self, value: K::Value) {
        let v = self
            .map
            .entry(TypeId::of::<K>())
            .or_insert_with(|| Box::new(Vec::<K::Value>::with_capacity(1)));
        let v: &mut Vec<K::Value> = v.downcast_mut().unwrap();
        v.push(value);
    }

    pub fn get<K: Key+ 'static>(&self) -> Option<&[K::Value]> {
        match self.map.get(&TypeId::of::<K>()) {
            None => None,
            Some(boxed_value) => Some(&boxed_value.downcast_ref::<Vec<K::Value>>().unwrap()),
        }
    }
}
*/

/*
use minicbor::encode::Write;
use minicbor::Encoder;
use minicbor::encode::Error;

impl Encode for Container
{
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), Error<W::Error>> 
    {
        //e.map(as_u64(self.data.len()))?;
        for (k, vector) in &self.map {
            //k.encode(e)?;
            for v in vector {
                v.encode(e)?;
            }
            
        }
        Ok(())
    }


}
*/