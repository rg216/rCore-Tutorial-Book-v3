//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

#[allow(unused)]
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_TASK_INFO: usize = 410;

mod fs;
mod process;
mod task_info;

use lazy_static::*;
use log::info;
use fs::*;
use process::*;
use task_info::*;
use crate::task::TaskInfo;
use crate::config::MAX_SYSCALL_NUM;
use crate::sync::UPSafeCell;

#[derive(Clone, Copy)]
pub struct SyscallInfo {
    pub id: usize,
    pub times: usize
}

lazy_static! {
    static ref SYSCALL_STATS: UPSafeCell<[usize; MAX_SYSCALL_NUM]> =
        unsafe { UPSafeCell::new([0 as usize; MAX_SYSCALL_NUM]) };
}

pub fn print_syscall_stats() {
    let syscall_stats = SYSCALL_STATS.exclusive_access();
    info!("----------------syscall stats----------------");
    for i in 0..100 {
        if syscall_stats[i] != 0 {
            info!("syscall {} is called {} times", i, syscall_stats[i]);
        }
    }
    info!("--------------syscall stats end--------------");
}

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    // do some statiscal job here
    let mut syscall_stats = SYSCALL_STATS.exclusive_access();
    if syscall_id < 100 {
        syscall_stats[syscall_id] += 1;
    }
    drop(syscall_stats);
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_TASK_INFO => sys_task_info(args[0], args[1] as *mut TaskInfo),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}

