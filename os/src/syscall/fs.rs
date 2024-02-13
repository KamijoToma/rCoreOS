use crate::{
    mm::page_table::translated_byte_buffer,
    task::current_user_token,
};

const FD_STDOUT: usize = 1;

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
