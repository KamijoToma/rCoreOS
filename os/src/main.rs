#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

mod lang_items;
mod sbi;
#[macro_use]
mod console;
use sbi::{*};
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello World from wCore OS");
    shutdown(false);
}

// 将 bss 段的内容全部置零
fn clear_bss() {
    extern "C" {
        fn sbss(); // linker.ld 中设置的标记
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe {
            (a as *mut u8).write_volatile(0)
        }
    })
}
