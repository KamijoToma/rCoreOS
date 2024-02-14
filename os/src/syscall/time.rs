

use crate::{
    mm::page_table::{translate_memcopy},
    task::processor::current_user_token,
    timer::{self, MICRO_PER_SEC},
};

#[repr(C)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn syscall_get_time(ts: usize, _tz: usize) -> isize {
    let usec = timer::get_time_us();
    let time_val = TimeVal {
        sec: usec / MICRO_PER_SEC,
        usec,
    };
    if translate_memcopy(current_user_token(), ts as *const u8, &time_val)
        .is_err() {
        -1
    }else {
        0
    }
}
