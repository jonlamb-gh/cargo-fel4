#![no_std]
#![cfg_attr(feature = "alloc", feature(alloc))]

#[cfg(all(feature = "alloc"))]
#[macro_use]
extern crate alloc;

extern crate sel4_sys;
extern crate sel4twinkle_alloc;

#[cfg(all(feature = "test"))]
#[macro_use]
extern crate proptest;

#[cfg(feature = "test")]
pub mod fel4_test;

#[cfg(feature = "KernelPrinting")]
use sel4_sys::DebugOutHandle;
use sel4_sys::{seL4_CPtr, seL4_Word};
use sel4twinkle_alloc::Allocator;

macro_rules! debug_print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        DebugOutHandle.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

macro_rules! debug_println {
    ($fmt:expr) => (debug_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (debug_print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn init(_allocator: &mut Allocator, _global_fault_ep_cap: seL4_CPtr) {
    // be careful, this is called from the root task
    debug_println!("\nhello from a feL4 app init!\n");
}

pub fn handle_fault(badge: seL4_Word) {
    debug_println!("\n!!! Fault from badge 0x{:X}\n", badge);
}
