#![no_std]
#![no_main]

use ulib::{get_taskinfo, println};

extern crate ulib;

#[no_mangle]
fn main() -> i32 {
    println!("This task is {}", get_taskinfo());
    0
}
