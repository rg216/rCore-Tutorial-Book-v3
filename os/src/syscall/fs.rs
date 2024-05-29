use crate::batch::*;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            if sys_write_check(buf, len) {
                let slice = unsafe { core::slice::from_raw_parts(buf, len) };
                let str = core::str::from_utf8(slice).unwrap();
                print!("{}", str);
                len as isize
            } else {
                -1
            }
        }
        _ => {
            println!("Unsupported fd in sys_write!");
            -1 as isize
        }
    }
}
