use crate::loader;

pub fn syscall_get_task_info() -> isize {
    loader::get_current_app() as isize
}