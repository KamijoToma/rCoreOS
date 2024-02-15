#![no_std]
#![no_main]

use ulib::{get_time_us, println};

extern crate ulib;

#[no_mangle]
fn main() -> i32 {
    let usec = get_time_us();
    println!(
        "uptime: {}s {}ms {}us",
        usec / 1_000_000,
        (usec / 1000) % 1000,
        (usec % 1_000_000) % 1000
    );
    0
}
