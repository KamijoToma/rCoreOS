use crate::timer::{self, MICRO_PER_SEC};

#[repr(C)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn syscall_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let usec = timer::get_time_us();
    unsafe {
        if let Some(t) = ts.as_mut() {
            t.usec = usec;
            t.sec = usec / MICRO_PER_SEC;
        }else{
            return -1
        }
    }
    0
}