#![no_std]
#![no_main]

use ulib::shutdown;

#[macro_use]
extern crate ulib;

#[no_mangle]
fn main() -> i32 {
    println!("Shutdown machine");
    shutdown();
    println!("Shutdown commited");
    0
}
