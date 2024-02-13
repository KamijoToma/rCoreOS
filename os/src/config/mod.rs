pub(crate) const USER_STACK_SIZE: usize = 4096 * 2;
pub(crate) const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub(crate) const MAX_APP_NUM: usize = 16;
pub(crate) const APP_BASE_ADDRESS: usize = 0x80400000;
pub(crate) const APP_SIZE_LIMIT: usize = 0x20000;
pub const CLOCK_FREQ: usize = 12500000; // QEMU
pub const KERNEL_HEAP_SIZE: usize = 4096 * 4;