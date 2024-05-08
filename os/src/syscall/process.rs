//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    mm::translated_byte_buffer,
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next,
        suspend_current_and_run_next, TaskStatus, get_syscall_times, get_task_status, get_times
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
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
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    if ts.is_null() {
        return -1;
    }

    const SIZE: usize = core::mem::size_of::<TimeVal>();
    let translated_ts = translated_byte_buffer(current_user_token(), ts as *const u8, SIZE);
    if translated_ts.is_empty() {
        return -1;
    }
    let us = get_time_us();
    unsafe {
        if translated_ts[0].len() < SIZE {
            return -1;
        }
        let translated_ts_ptr = translated_ts[0].as_ptr() as *mut TimeVal;
        *translated_ts_ptr = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");

    if ti.is_null() {
        return -1;
    }
    let mut translated_ti = translated_byte_buffer(
        current_user_token(),
        ti as *const u8,
        core::mem::size_of::<TaskInfo>(),
    );

    // Check if the translated buffer is empty
    if translated_ti.is_empty() {
        return -1;
    }

    // Ensure that the translated buffer has enough bytes to store a `TaskInfo`
    if translated_ti[0].len() < core::mem::size_of::<TaskInfo>() {
        return -1;
    }

    // Fill in the task information
    unsafe {
        let ti_ptr = translated_ti[0].as_mut_ptr() as *mut TaskInfo;
        (*ti_ptr).status = get_task_status();
        (*ti_ptr).syscall_times = get_syscall_times();
        (*ti_ptr).time = get_times();
    }

    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
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
