

use alloc::sync::Arc;

use lazy_static::lazy_static;
use log::{info};

use crate::loader::{get_app_data_by_name};
use crate::shutdown;
use crate::task::context::TaskContext;



use self::manager::add_task;
use self::processor::{schedule, take_current_task};

use self::tasks::TaskControlBlock;
use self::tasks::TaskStatus;

pub mod context;
pub mod manager;
pub mod pid;
pub mod processor;
pub mod switch;
pub mod tasks;




pub fn suspend_current_and_run_next() {
    let task = take_current_task().unwrap();
    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    add_task(task);
    schedule(task_cx_ptr);
}

pub fn exit_current_and_run_next(exit_code: i32) {
    let task = take_current_task().unwrap();
    let pid = task.getpid();
    if pid == 0 {
        info!("[kernel] Idle process exit with exit_code {}", exit_code);
        shutdown(exit_code != 0);
    }
    let mut inner = task.inner_exclusive_access();
    inner.task_status = TaskStatus::Zombie;
    inner.exit_code = exit_code;
    // 退出进程的孤儿进程设置为init进程的子进程
    {
        let mut initproc_inner = INIT_PROC.inner_exclusive_access();
        for child in inner.children.iter() {
            child.inner_exclusive_access().parent = Some(Arc::downgrade(&INIT_PROC));
            initproc_inner.children.push(child.clone());
        }
    }
    inner.children.clear();
    inner.memory_set.recycle_data_pages();
    drop(inner);
    drop(task);
    let mut _unused = TaskContext::zero_init();
    schedule(&mut _unused as *mut _);
}

lazy_static! {
    pub static ref INIT_PROC: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
        get_app_data_by_name("winit").unwrap()
    ));
}

pub fn add_initproc() {
    add_task(INIT_PROC.clone());
}
