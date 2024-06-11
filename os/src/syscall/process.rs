use crate::mm::{VirtAddr, MapPermission};
use crate::mm::address::VPNRange;
use crate::mm::page_table::translated_byte_buffer;
use crate::task::{
    current_user_token, exit_current_and_run_next, suspend_current_and_run_next,
    get_current_task_page_table,
    create_new_map_area,
    unmap_consecutive_area
};
use crate::timer::get_time_us;
use crate::config::*;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let _us = get_time_us();
    // unsafe {
    //     *ts = TimeVal {
    //         sec: us / 1_000_000,
    //         usec: us % 1_000_000,
    //     };
    // }
    let dst_vec = translated_byte_buffer(current_user_token(), _ts as *const u8, core::mem::size_of::<TimeVal>());
    let ref time_val = TimeVal {
        sec: _us / 1_000_000,
        usec: _us % 1_000_000,
    };
    let src_ptr = time_val as *const TimeVal as *const TimeVal;
    for (idx, dst) in dst_vec.into_iter().enumerate() {
        let unit_len = dst.len();
        unsafe {
            dst.copy_from_slice(core::slice::from_raw_parts(
                src_ptr.wrapping_byte_add(idx * unit_len) as *const u8,
                unit_len)
            );
        }
    }
    0
}

/// port: page permission [2:0] X|W|R
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    if start % PAGE_SIZE != 0 /* start need to be page aligned */ || 
        port & !0x7 != 0 /* other bits of port needs to be zero */ ||
        port & 0x7 ==0 /* No permission set, meaningless */ ||
        start >= MAXVA /* mapping range should be an legal address */ {
        return -1;
    }

    // check the range [start, start + len)
    let start_vpn = VirtAddr::from(start).floor();
    let end_vpn = VirtAddr::from(start + len).ceil();
    let vpns = VPNRange::new(start_vpn, end_vpn);
    for vpn in vpns {
       if let Some(pte) = get_current_task_page_table(vpn) {
            // we find a pte that has been mapped
            if pte.is_valid() {
                return -1;
            }
       }
    }
    // all ptes in range has pass the test
    create_new_map_area(
        start_vpn.into(),
        end_vpn.into(),
        MapPermission::from_bits_truncate((port << 1) as u8) | MapPermission::U
    );
    0
}

/// munmap the mapped virtual addresses
pub fn sys_munmap(start: usize, len: usize) -> isize {
    if start >= MAXVA || start % PAGE_SIZE != 0 {
        return -1;
    }
    // avoid undefined situation
    let mut mlen = len;
    if start > MAXVA - len {
        mlen = MAXVA - start;
    }
    unmap_consecutive_area(start, mlen)
}