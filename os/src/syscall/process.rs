use alloc::sync::Arc;
use log::{debug, info};

use crate::{
    loader::get_app_data_by_name,
    mm::page_table::{translate_memcopy, translate_str},
    task::{
        exit_current_and_run_next,
        manager::add_task,
        processor::{current_task, current_user_token},
        suspend_current_and_run_next,
    },
};

pub fn sys_exit(xstate: i32) -> ! {
    exit_current_and_run_next(xstate);
    unreachable!("sys_exit never reached.")
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_fork() -> isize {
    let current_task = current_task().unwrap();
    let new_task = current_task.fork();
    let new_pid = new_task.pid.0;
    let trap_cx = new_task.inner_exclusive_access().get_trap_cx();
    trap_cx.x[10] = 0;
    add_task(new_task);
    new_pid as isize
}

pub fn sys_exec(path: *const u8) -> isize {
    let token = current_user_token();
    if let Some(path) = translate_str(token, path) {
        if let Some(data) = get_app_data_by_name(path.as_str()) {
            let task = current_task().unwrap();
            task.exec(data);
            0 // Actually never return 0
        } else {
            -1
        }
    } else {
        -1
    }
}

pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let task = current_task().unwrap();

    let mut inner = task.inner_exclusive_access();
    if inner
        .children
        .iter()
        .find(|p| pid == -1 || pid as usize == p.getpid())
        .is_none()
    {
        return -1; // no zombie process found
    }
    let pair = inner.children.iter().enumerate().find(|(_, p)| {
        p.inner_exclusive_access().is_zombie() && (pid == -1 || pid as usize == p.getpid())
    });
    if let Some((idx, _)) = pair {
        let child = inner.children.remove(idx);
        assert_eq!(Arc::strong_count(&child), 1);
        let found_pid = child.getpid();
        let exit_code = child.inner_exclusive_access().exit_code;
        if translate_memcopy(
            inner.get_user_token(),
            exit_code_ptr as usize as *const u8,
            &exit_code,
        )
        .is_err()
        {
            return -1;
        }
        found_pid as isize
    } else {
        -2
    }
}
