use self::{fs::sys_write, process::sys_exit, wcore::syscall_get_task_info};

pub mod fs;
pub mod process;
pub mod wcore;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_TASKINFO: usize = 255;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_TASKINFO => syscall_get_task_info(),
        call_id => panic!("Unsupported syscall {}", call_id),
    }
}