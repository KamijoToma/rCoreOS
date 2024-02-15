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
const SYSCALL_FORK: usize = 220;
const SYSSCALL_WAITPID: usize = 260;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_READ: usize = 63;
const SYSCALL_REBOOT: usize = 520;

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(state: i32) -> isize {
    syscall(SYSCALL_EXIT, [state as usize, 0, 0])
}

pub fn sys_get_task_info() -> usize {
    syscall(SYSCALL_TASKINFO, [0; 3]) as usize
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0; 3])
}

pub fn sys_get_time(ts: &mut TimeVal, _tz: usize) -> isize {
    syscall(SYSCALL_GET_TIME, [ts as *mut _ as usize, _tz, 0])
}

pub fn sys_fork() -> isize {
    syscall(SYSCALL_FORK, [0; 3])
}

pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    syscall(SYSSCALL_WAITPID, [pid as usize, exit_code as usize, 0])
}

/// sys_exec system call
/// - path: the program you want to run
/// 
/// return value:
/// - 0 normal, actually never returns
/// - -1 app not found
/// - -2 illegal argument, actually should not happen
pub fn sys_exec(path: &str) -> isize {
    syscall(SYSCALL_EXEC, [path.as_ptr() as usize, 0, 0])
}

pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(
        SYSCALL_READ,
        [fd, buffer.as_mut_ptr() as usize, buffer.len()],
    )
}


pub fn sys_reboot(cmd: isize) -> isize {
    syscall(
        SYSCALL_REBOOT, [cmd as usize, 0, 0])
}