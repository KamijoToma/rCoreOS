use core::{mem::size_of, slice::from_raw_parts};

use crate::{mm::page_table::translated_byte_buffer, task::current_user_token, timer::{self, MICRO_PER_SEC}};

#[repr(C)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn syscall_get_time(ts: usize, _tz: usize) -> isize {
    let usec = timer::get_time_us();
    let buffer = translated_byte_buffer(current_user_token(), ts as *const u8, size_of::<TimeVal>());
    let mut time_val = TimeVal {
        sec: usec / MICRO_PER_SEC,
        usec,
    };
    unsafe {
        let src = from_raw_parts(&time_val as *const TimeVal as *const u8, size_of::<TimeVal>());
        let mut start = 0usize;
        let mut end = 0usize;
        for dst in buffer {
            end += dst.len();
            dst.copy_from_slice(&src[start..end]);
            start = end;
        }
    }
    0
}
