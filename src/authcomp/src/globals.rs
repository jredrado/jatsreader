

use core::any::{Any,TypeId};
use core::cell::RefCell;
use std::collections::LinkedList;
use std::boxed::Box;

use core::{ops, marker};

/// Add `Sync` to an arbitrary type.
///
/// This primitive is used to get around the `Sync` requirement in `static`s (even thread local
/// ones! see rust-lang/rust#35035). Due to breaking invariants, creating a value of such type is
/// unsafe, and care must be taken upon usage.
///
/// In general, this should only be used when you know it won't be shared across threads (e.g. the
/// value is stored in a thread local variable).
pub struct Syncify<T>(T);

impl<T> Syncify<T> {
    /// Create a new `Syncify` wrapper.
    ///
    /// # Safety
    ///
    /// This is invariant-breaking and thus unsafe.
    const unsafe fn new(inner: T) -> Syncify<T> {
        Syncify(inner)
    }
}

impl<T> ops::Deref for Syncify<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

unsafe impl<T> marker::Sync for Syncify<T> {}

static COMPUTATIONS_STACK : Syncify<RefCell<LinkedList<(TypeId,&'static (dyn Any ))>>> = unsafe { Syncify::new(RefCell::new(LinkedList::new())) };


/*
pub fn push<C> ( computation: &'static C )
    where C: Computation + Any
{

    COMPUTATIONS_STACK.borrow_mut().push_front(computation);
}

pub fn pop<C> () -> Option<&'static C> 
    where C:Computation + Any 
{
    let ov = COMPUTATIONS_STACK.borrow_mut().pop_front();
    if let Some(v) = ov {
        let m = unsafe { v as *const dyn Any as *const C };
        
        unsafe { m.as_ref() }

    } else {
        None
    }
}
*/

pub fn get<T>() ->  Option<&'static T> 
where
    T: 'static +  Default,
{
    {
        let mut globals = COMPUTATIONS_STACK.borrow_mut();
        let id = TypeId::of::<T>();
        let p = globals.iter().find(|&r| r.0 == id);
        if let Some(v) = p {
            let m = v.1 as *const dyn Any as *const T ;
            return unsafe { m.as_ref() }
        }

        let v = Box::new(T::default());
        let handle = Box::leak(v);
        globals.push_front((id, handle));
    }
    get()
}

/*
pub fn get<C> () -> Option<&'static C> {
    COMPUTATIONS_STACK.borrow().front()
}
*/