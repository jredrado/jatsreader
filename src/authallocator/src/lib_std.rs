#![feature(core_intrinsics,lang_items,start,default_alloc_error_handler,custom_test_frameworks)]


#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
extern crate wee_alloc;


use alloc::string::String;
//use wapc_guest::prelude::*;


pub fn print (s: &String ) -> CallResult {
    /*
    use corepack;

    use alloc::format;
    use alloc::boxed::Box;

    let msg = corepack::to_bytes(s).map_err(|e| Box::new(wapc_guest::errors::new(wapc_guest::errors::ErrorKind::BadDispatch(format!("Unable to serialize {:?}",e)))))?;

    host_call("wapc", "env", "print", &msg)
    */
    todo!()
}

pub fn print_str (s: &str ) -> CallResult {
    /*
    use corepack;

    use alloc::format;
    use alloc::boxed::Box;

    let msg = corepack::to_bytes(s).map_err(|e| Box::new(wapc_guest::errors::new(wapc_guest::errors::ErrorKind::BadDispatch(format!("Unable to serialize {:?}",e)))))?;

    host_call("wapc", "env", "print", &msg)
    */
    todo!()
}

// Use `wee_alloc` as the global allocator.

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Need to provide a tiny `panic` implementation for `#![no_std]`.
// This translates into an `unreachable` instruction that will
// raise a `trap` the WebAssembly execution if we panic at runtime.










/*
#[alloc_error_handler]
fn error_handler(_: core::alloc::Layout) -> ! {
        //::core::intrinsics::abort();
        loop {}
}
*/

pub fn test_runner(tests: &[&dyn Fn()]) {

        for test in tests {
            test();
        }

}

#[cfg(test)]
#[start]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    unsafe {
    ::core::hint::unreachable_unchecked();
    }
    loop {} //::core::intrinsics::abort();
}


use alloc::alloc::{alloc as global_alloc, dealloc as global_dealloc, Layout};

#[no_mangle]
pub unsafe fn alloc(size: usize) -> *mut u8 {
    let align = core::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(size, align);
    global_alloc(layout)
}

#[no_mangle]
pub unsafe fn dealloc(ptr: *mut u8, size: usize) {
    let align = core::mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(size, align);
    global_dealloc(ptr, layout);
}

#[cfg(test)]
mod tests {
    
    #[test_case]
    fn it_works() {
        assert_eq!(2 + 2, 5);
    }
}