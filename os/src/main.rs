#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

mod lang_items;
mod log;
mod sbi;
#[macro_use]
mod console;
mod batch;
mod sync;
mod trap;
mod syscall;
use ::log::info;
use sbi::*;
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
    info!("Hello World from wCore OS");
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    batch::init();
    trap::init();
    batch::run_next_app();
}

// 将 bss 段的内容全部置零
fn clear_bss() {
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
