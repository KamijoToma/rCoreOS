#![no_std]
#![no_main]

use ulib::{get_time, println, yield_};
#[no_mangle]
fn main() -> i32 {
    println!("Sleeping 1s...");
    let ms = get_time();
    let wait_for = ms + 1_000;
    while get_time() < wait_for
    {
        yield_();
    }
    println!("OK!");
    0
}
