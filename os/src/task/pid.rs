use alloc::vec::Vec;
use lazy_static::lazy_static;

use crate::{
    config::{KERNEL_STACK_SIZE, PAGE_SIZE, TRAMPOLINE},
    mm::{address::VirtAddr, memory_set::MapPermission, KERNEL_SPACE},
    sync::up::UPSafeCell,
};

pub struct PidHandle(pub usize);

trait PidAllocate {
    fn new() -> Self;
    fn alloc(&mut self) -> PidHandle;
    fn dealloc(&mut self, pid: usize);
}

struct StackPidAllocator {
    current: usize,
    recycled: Vec<usize>,
}

impl PidAllocate for StackPidAllocator {
    fn new() -> Self {
        PidAllocator {
            current: 0,
            recycled: Vec::new(),
        }
    }

    fn alloc(&mut self) -> PidHandle {
        if let Some(pid) = self.recycled.pop() {
            PidHandle(pid)
        } else {
            self.current += 1;
            PidHandle(self.current - 1)
        }
    }

    fn dealloc(&mut self, pid: usize) {
        assert!(pid < self.current);
        assert!(!self.recycled.iter().any(|ppid| *ppid == pid));
        self.recycled.push(pid);
    }
}

type PidAllocator = StackPidAllocator;

lazy_static! {
    static ref PID_ALLOCATOR: UPSafeCell<PidAllocator> =
        unsafe { UPSafeCell::new(PidAllocator::new()) };
}

pub fn pid_alloc() -> PidHandle {
    PID_ALLOCATOR.exclusive_access().alloc()
}

impl Drop for PidHandle {
    fn drop(&mut self) {
        PID_ALLOCATOR.exclusive_access().dealloc(self.0);
    }
}

pub struct KernelStack {
    pid: usize,
}

pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}

impl KernelStack {
    pub fn new(pid_handle: &PidHandle) -> Self {
        let pid = pid_handle.0;
        let (kernel_stack_bottom, kernel_stack_top) = kernel_stack_position(pid);
        KERNEL_SPACE.exclusive_access().insert_framed_area(
            kernel_stack_bottom.into(),
            kernel_stack_top.into(),
            MapPermission::R | MapPermission::W,
        );
        KernelStack { pid: pid_handle.0 }
    }

    pub fn get_top(&self) -> usize {
        kernel_stack_position(self.pid).1
    }
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        let kernel_stack_bottom_va = VirtAddr::from(kernel_stack_position(self.pid).0);
        KERNEL_SPACE
            .exclusive_access()
            .remove_area_with_start_vpn(kernel_stack_bottom_va.into());
    }
}
