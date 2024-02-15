use log::info;
use sbi_rt::{system_reset, NoReason, Shutdown};

use crate::task::processor::current_task;

pub fn syscall_get_task_info() -> isize {
    if let Some(task) = current_task() {
        task.pid.0 as isize
    } else {
        -1
    }
}

pub fn syscall_reboot(cmd: isize) -> isize {
    if cmd == 1 {
        // shutdown
        info!("[kernel] system shutdown!");
        system_reset(Shutdown, NoReason);
    }
    -1
}