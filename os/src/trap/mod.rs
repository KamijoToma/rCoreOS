use core::arch::{asm, global_asm};

use log::{error, trace};
use riscv::register::{
    scause::{self, Exception, Trap},
    sie, stval, stvec,
};

use crate::{
    config::{TRAMPOLINE, TRAP_CONTEXT},
    syscall::syscall,
    task::{
        exit_current_and_run_next,
        processor::{current_trap_cx, current_user_token},
        suspend_current_and_run_next,
    },
    timer::set_next_trigger,
};

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

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, stvec::TrapMode::Direct);
    }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE, stvec::TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel!");
}

#[no_mangle]
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = current_user_token();
    extern "C" {
        fn __all_traps();
        fn __restore();
    }
    let restore_va = __restore as usize - __all_traps as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,
            in("a1") user_satp,
            options(noreturn)
        );
    }
}

#[no_mangle]
pub fn trap_handler(_cx: &mut TrapContext) -> ! {
    set_kernel_trap_entry();
    let cx = current_trap_cx();
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            let mut cx = current_trap_cx(); // due to fork
            cx.sepc += 4;
            let result = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
            cx = current_trap_cx();
            cx.x[10] = result as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error!(
                "[kernel] PageFault in application, kernel killed it, pc = {:#x}",
                cx.sepc
            );
            exit_current_and_run_next(-2)
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!(
                "[kernel] IllegalInstruction in application, kernel killed it, pc = {:#x}",
                cx.sepc
            );
            exit_current_and_run_next(-3)
        }
        Trap::Interrupt(scause::Interrupt::SupervisorTimer) => {
            set_next_trigger();
            trace!("Timer triggered.");
            suspend_current_and_run_next()
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
    trap_return();
}
