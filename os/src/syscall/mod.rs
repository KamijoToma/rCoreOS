use log::error;

use self::{
    fs::{sys_read, sys_write},
    process::{sys_exec, sys_exit, sys_fork, sys_waitpid, sys_yield},
    time::syscall_get_time,
    wcore::{syscall_get_task_info, syscall_reboot},
};

pub mod fs;
pub mod process;
pub mod time;
pub mod wcore;

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

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_TASKINFO => syscall_get_task_info(),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => syscall_get_time(args[0], args[1]),
        SYSCALL_FORK => sys_fork(),
        SYSCALL_EXEC => sys_exec(args[0] as *const u8),
        SYSCALL_READ => sys_read(args[0], args[1] as *const u8, args[2]),
        SYSSCALL_WAITPID => sys_waitpid(args[0] as isize, args[1] as *mut i32),
        SYSCALL_REBOOT => syscall_reboot(args[0] as isize),
        call_id => {
            error!("Unsupported syscall {}", call_id);
            -2
        },
    }
}
