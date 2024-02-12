use log::info;

use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};

pub fn sys_exit(xstate: i32) -> ! {
    info!("[kernel] Application exited with code {}", xstate);
    exit_current_and_run_next();
    unreachable!("sys_exit never reached.")
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}
