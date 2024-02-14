use alloc::{sync::Arc, task};
use lazy_static::lazy_static;
use log::debug;

use crate::{sync::up::UPSafeCell, trap::context::TrapContext};

use super::{context::TaskContext, manager::fetch_task, switch::__switch, tasks::{TaskControlBlock, TaskStatus}};

type OATaskControlBlock = Option<Arc<TaskControlBlock>>;
pub struct Processor {
    current: OATaskControlBlock,
    idle_task_cx: TaskContext,
}

impl Processor {
    pub fn new() -> Self {
        Self { current: None, idle_task_cx: TaskContext::zero_init() }
    }

    pub fn take_current(&mut self) -> OATaskControlBlock {
        self.current.take()
    }

    pub fn current(&self) -> OATaskControlBlock {
        self.current.as_ref().map(|task| Arc::clone(task))
    }

    pub fn get_idle_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cx as *mut _
    }
}

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = unsafe {
        UPSafeCell::new(Processor::new())
    };
}

pub fn take_current_task() -> OATaskControlBlock {
    PROCESSOR.exclusive_access().take_current()
}

pub fn current_task() -> OATaskControlBlock {
    PROCESSOR.exclusive_access().current()
}

pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    let token = task.inner_exclusive_access().get_user_token();
    token
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task().unwrap().inner_exclusive_access().get_trap_cx()
}

pub fn run_tasks() {
    loop {
        let mut processor = PROCESSOR.exclusive_access();
        if let Some(task) = fetch_task() {
            let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
            let mut task_inner = task.inner_exclusive_access();
            let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
            task_inner.task_status = TaskStatus::Running;
            drop(task_inner);
            processor.current = Some(task);
            drop(processor);
            unsafe {
                __switch(
                    idle_task_cx_ptr, 
                next_task_cx_ptr);
            }
        }
    }
}

pub fn schedule(switched_task_cx_ptr: *mut TaskContext) {
    let mut processor = PROCESSOR.exclusive_access();
    let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
    drop(processor);
    unsafe {
        __switch(
            switched_task_cx_ptr, 
            idle_task_cx_ptr);
    }
}