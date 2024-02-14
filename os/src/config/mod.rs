pub(crate) const USER_STACK_SIZE: usize = 4096 * 2;
pub(crate) const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const CLOCK_FREQ: usize = 12500000; // QEMU
pub const KERNEL_HEAP_SIZE: usize = 0x40_0000;
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_BITS_SIZE: usize = 12;
pub const MEMORY_END: usize = 0x80800000;
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}
