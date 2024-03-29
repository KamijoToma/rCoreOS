#![no_std]
#![no_main]

use ulib::yield_;

#[macro_use]
extern crate ulib;

#[no_mangle]
fn main() -> i32 {
    println!("Yield test program 1");
    for i in 0..5 {
        println!("Yield[1] cycle {}", i);
        yield_();
    }
    0
}
