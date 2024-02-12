use riscv::register::time;

use crate::{config::CLOCK_FREQ, set_timer};

/// 获取当前time寄存器时间(tick)
pub fn get_time() -> usize {
    time::read()
}

const TICKS_PRE_SEC: usize = 100; // 10ms per tick

pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PRE_SEC);
}

pub const MICRO_PER_SEC: usize = 1_000_000;

pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}