#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use core::arch::global_asm;

mod lang_items;
mod log;
mod sbi;
#[macro_use]
mod console;
mod config;
mod loader;
mod mm;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;
use ::log::info;
use sbi::*;

use crate::task::run_first_task;
extern crate alloc;
#[macro_use]
extern crate bitflags;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

extern "C" {
    fn sbss(); // linker.ld 中设置的标记
    fn ebss();
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    log::init().expect("Error init log module.");
    mm::init();
    mm::memory_set::remap_test();
    info!("Hello World from wCore OS");
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    run_first_task();
}

// 将 bss 段的内容全部置零
fn clear_bss() {
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0)
    }
}
