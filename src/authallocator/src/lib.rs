#![feature(core_intrinsics,lang_items,start,default_alloc_error_handler,custom_test_frameworks)]

#![no_std]
#![no_main]

#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(feature = "panic_handler")]
mod lib_no_std;

#[cfg(feature = "panic_handler")]
pub use lib_no_std::*;

#[cfg(not(feature = "panic_handler"))]
mod lib_std;

#[cfg(not(feature = "panic_handler"))]
pub use lib_std::*;