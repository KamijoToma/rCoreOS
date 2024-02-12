
// fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize;

// fn sys_exit(exit_code: usize) -> !;

use core::arch::asm;

use crate::TimeVal;
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        // 进行系统调用，使用ecall指令
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_TASKINFO: usize = 255;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(state: i32) -> isize {
    syscall(SYSCALL_EXIT, [state as usize, 0, 0])
}

pub fn sys_get_task_info() -> isize {
    syscall(SYSCALL_TASKINFO, [0; 3])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0; 3])
}

pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    syscall(SYSCALL_GET_TIME, [ts as usize, _tz, 0])
}