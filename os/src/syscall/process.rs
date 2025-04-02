//! Process management syscalls
use crate::{
    mm::{get_mut_data, MapPermission}, task::{
        self, change_program_brk, current_mmap, current_munmap, exit_current_and_run_next, get_cur_syscall, suspend_current_and_run_next
    }, timer::get_time_us
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let token = task::current_user_token();
    let us = get_time_us();
    let ts = get_mut_data(token, _ts,MapPermission::W).unwrap();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        }
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    let token = task::current_user_token();
    match _trace_request {
        0 => {
            let Some(ptr) = get_mut_data::<u8>(token, _id as *const u8, MapPermission::R) else {
                return -1;
            };
            let res = (unsafe { *ptr }).into();
            println!("[trace]: read data: {}", res);
            res
        }
        1 => {
            let Some(ptr) = get_mut_data::<u8>(token, _id as *const u8, MapPermission::W) else {
                return -1;
            };
            let res: isize = (unsafe { *ptr }).into();
            println!("[trace]: read data {} and write {}", res,_data);
            unsafe { ptr.write_volatile(_data as u8) };
            0
        }
        2 => {
            let calls = get_cur_syscall(_id);
            // println!("trace: {} syscall count: {}", _id,calls);
            calls as isize
        }
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    current_mmap(_start, _len, _port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    current_munmap(_start, _len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
