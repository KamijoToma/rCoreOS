#![feature(linkage)]
#![feature(panic_info_message)]
#![no_std]

mod syscall;
pub mod console;
mod lang_item;

use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf)}

pub fn exit(exit_code: i32) -> isize { sys_exit(exit_code)}

pub fn get_taskinfo() -> isize { sys_get_task_info() }

pub fn yield_() -> isize { sys_yield() }

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    unreachable!("unreachable after sys_exit!");
}

extern "C" {
    fn lib_sbss();
    fn lib_ebss();
}

fn clear_bss() {
    (lib_sbss as usize..lib_ebss as usize).for_each(|a| {
        unsafe{
            (a as *mut u8).write_volatile(0) 
        }
    })
}

// Weak main
#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}
