use crate::task::get_current_task;

pub fn syscall_get_task_info() -> isize {
    get_current_task() as isize
}
