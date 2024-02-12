use log::info;

use crate::loader::run_next_app;

pub fn sys_exit(xstate: i32) -> ! {
    info!("[kernel] Application exited with code {}", xstate);
    run_next_app()
}