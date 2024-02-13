use self::{
    fs::sys_write,
    process::{sys_exit, sys_yield},
    time::{syscall_get_time, TimeVal},
    wcore::syscall_get_task_info,
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

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_TASKINFO => syscall_get_task_info(),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => syscall_get_time(args[0] as *mut TimeVal, args[1] as usize),
        call_id => panic!("Unsupported syscall {}", call_id),
    }
}
