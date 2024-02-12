use core::arch::global_asm;

use log::error;
use riscv::register::{
    scause,
    scause::{Exception, Trap},
    stval, stvec,
};

use crate::{syscall::syscall, task::exit_current_and_run_next};

use self::context::TrapContext;

pub mod context;

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __all_traps();
    }
    unsafe {
        stvec::write(__all_traps as usize, stvec::TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error!(
                "[kernel] PageFault in application, kernel killed it, pc = {:#x}",
                cx.sepc
            );
            exit_current_and_run_next()
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!(
                "[kernel] IllegalInstruction in application, kernel killed it, pc = {:#x}",
                cx.sepc
            );
            exit_current_and_run_next()
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}, pc = {:#x}!",
                scause.cause(),
                stval,
                cx.sepc
            );
        }
    }
    cx
}
