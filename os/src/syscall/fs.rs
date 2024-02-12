const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let string = core::str::from_utf8(slice).unwrap();
            print!("{}", string);
            len as isize
        }
        other_fd => panic!("Unsupported fd: {}", other_fd)
    }
}