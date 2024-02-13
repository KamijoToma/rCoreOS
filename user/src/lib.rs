#![feature(linkage)]
#![feature(panic_info_message)]
#![no_std]

pub mod console;
mod lang_item;
mod syscall;

use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

pub fn get_taskinfo() -> isize {
    sys_get_task_info()
}

pub fn yield_() -> isize {
    sys_yield()
}

pub fn get_time(ts: &mut TimeVal) -> isize {
    sys_get_time(&mut *ts, 0)
}

#[repr(C)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

impl TimeVal {
    pub fn new() -> Self {
        TimeVal { sec: 0, usec: 0 }
    }
}

impl Default for TimeVal {
    fn default() -> Self {
        Self::new()
    }
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    exit(main());
    unreachable!("unreachable after sys_exit!");
}

// Weak main
#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}
