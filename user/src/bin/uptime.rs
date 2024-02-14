#![no_std]
#![no_main]

use ulib::{get_time, println, TimeVal};

extern crate ulib;

#[no_mangle]
fn main() -> i32 {
    let mut f = TimeVal::new();
    get_time(&mut f);
    println!(
        "uptime: {}s {}ms {}us",
        f.usec / 1_000_000,
        (f.usec / 1000) % 1000,
        (f.usec % 1_000_000) % 1000
    );
    0
}
