#![no_std]
#![no_main]

use ulib::{get_time, TimeVal};

#[macro_use]
extern crate ulib;

#[no_mangle]
fn main() -> i32 {
    println!("Hello world!");
    let mut f = TimeVal::new();
    get_time(&mut f);
    println!("[sec: {}] [usec: {}]", f.sec, f.usec);
    0
}
