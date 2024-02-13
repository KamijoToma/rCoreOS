#![no_std]
#![no_main]

use ulib::{get_time, println, yield_, TimeVal};
#[no_mangle]
fn main() -> i32 {
    let mut timer = TimeVal::default();
    get_time(&mut timer);
    let wait_for = timer.usec + 1000_000;
    while {get_time(&mut timer); timer.usec} < wait_for {
        yield_();
    }
    println!("Test sleep OK!");
    0
}