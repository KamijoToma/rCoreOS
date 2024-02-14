use crate::{
    mm::page_table::{translated_byte_buffer, translated_byte_buffer_mut},
    task::{processor::current_user_token, suspend_current_and_run_next},
};

const FD_STDOUT: usize = 1;
const FD_STDIN: usize = 0;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            if let Some(buffers) = translated_byte_buffer(current_user_token(), buf, len) {
                for buffer in buffers {
                    print!("{}", core::str::from_utf8(buffer).unwrap());
                }
                len as isize
            } else {
                -1
            }
        }
        other_fd => panic!("Unsupported fd: {}", other_fd),
    }
}

#[allow(deprecated)]
pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDIN => {
            if len != 1 {
                return -1;
            }
            let mut c: usize;
            loop {
                c = sbi_rt::legacy::console_getchar();
                if c == 0 {
                    suspend_current_and_run_next();
                    continue;
                } else {
                    break;
                }
            }
            let ch = c as u8;
            if let Some(mut buffer_vec) = translated_byte_buffer_mut(current_user_token(), buf, 1) {
                unsafe {
                    buffer_vec[0].as_mut_ptr().write_volatile(ch);
                }
                0
            } else {
                -1
            }
        }
        _ => -1,
    }
}
