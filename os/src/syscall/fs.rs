use log::error;

use crate::loader;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            extern "C" {
                fn srodata();
            }
            let [app_start, app_end] = loader::get_current_app_memory_range();
            if (buf as usize) < (srodata as usize)  || (buf as usize) + len >= app_end {
                // TODO Fix memery protection
                error!("FS ERROR app [{:#x}, {:#x}), buf {:#x} len {:#x}", app_start, app_end, buf as usize, len);
                return -1
            }
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let string = core::str::from_utf8(slice).unwrap();
            print!("{}", string);
            len as isize
        }
        other_fd => panic!("Unsupported fd: {}", other_fd)
    }
}