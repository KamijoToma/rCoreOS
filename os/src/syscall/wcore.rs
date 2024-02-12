use crate::batch;

pub fn syscall_get_task_info() -> isize {
    batch::get_current_app() as isize
}