#![no_std]
#![no_main]

use ulib::{exec, fork, wait, yield_};

#[macro_use]
extern crate ulib;

#[no_mangle]
fn main() -> i32 {
    if fork() == 0 {
        // subprocess
        println!("winit");
        if exec("wsh\0") != 0 {
            // not exist
            println!("shell not exist");
            -1
        } else {
            unreachable!("shell process unreachable");
        }
    } else {
        // collect zombie subprocess
        loop {
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            if pid == -1 {
                // no more process waiting
                yield_();
                continue;
            }
        }
    }
}
