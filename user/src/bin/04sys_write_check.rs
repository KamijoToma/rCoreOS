#![no_std]
#![no_main]

use ulib::{println, write};

extern crate ulib;

#[no_mangle]
fn main() -> i32 {
    println!("Hello");
    unsafe {
        assert_eq!(
            write(1, core::slice::from_raw_parts(core::ptr::NonNull::dangling().as_ptr(), 10)),
            -1
        );
        assert_ne!(write(1, "Test output".as_bytes()), -1);
    }
    0
}
