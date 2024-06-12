//! File and filesystem-related syscalls

use crate::batch::sys_write_check;

const FD_STDOUT: usize = 1;



/// write buf of length `len` to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            match sys_write_check(slice) {
                None => {
                    println!("Illegal address in sys_write!");
                    -1 as isize
                },
                Some(i_len) => {
                    let str = core::str::from_utf8(slice).unwrap();
                    print!("{}", str);
                    i_len
                }
            }
        }
        _ => {
            println!("Unsupported fd in sys_write!");
            -1 as isize
        }
    }
}