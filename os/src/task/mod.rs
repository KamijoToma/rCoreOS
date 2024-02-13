use lazy_static::lazy_static;
use log::{info, trace};

use crate::config::APP_SIZE_LIMIT;
use crate::loader::{get_base_i, init_app_cx};
use crate::shutdown;
use crate::task::context::TaskContext;
use crate::{config::MAX_APP_NUM, loader::get_num_app, sync::up::UPSafeCell};

use self::switch::__switch;
use self::tasks::TaskControlBlock;
use self::tasks::TaskStatus;

pub mod context;
pub mod switch;
pub mod tasks;

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: tasks::TaskStatus::UnInit,
        }; MAX_APP_NUM];
        for (i, item) in tasks.iter_mut().enumerate().take(num_app) {
            item.task_cx = TaskContext::goto_restore(init_app_cx(i));
            item.task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            },
        }
    };
}

impl TaskManager {
    fn mark_current_suspend(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &mut inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);
            trace!("[kernel] Switching task from {} to {}", current, next);
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            };
        } else {
            info!("[kernel] All applications completed!");
            shutdown(false);
        }
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    fn get_current_task(&self) -> usize {
        let inner = self.inner.exclusive_access();
        inner.current_task
    }

    fn get_current_task_memory_range(&self) -> [usize; 2] {
        let current = get_current_task();
        [get_base_i(current), get_base_i(current) + APP_SIZE_LIMIT]
    }

    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        trace!("Entering run first task sp={:#x}", task0.task_cx.sp);
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
        unreachable!("run_first_task never reach this");
    }
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspend()
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited()
}

fn run_next_task() {
    TASK_MANAGER.run_next_task()
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

pub fn get_current_task() -> usize {
    TASK_MANAGER.get_current_task()
}

pub fn get_current_task_memory_range() -> [usize; 2] {
    TASK_MANAGER.get_current_task_memory_range()
}

pub fn run_first_task() -> ! {
    TASK_MANAGER.run_first_task();
}
