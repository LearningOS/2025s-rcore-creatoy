//! Process management syscalls
use core::mem::size_of;

use crate::{
    config::PAGE_SIZE,
    mm::{checked_translated_byte_buffer, PTEFlags},
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_syscall_times,
        insert_framed_area, remove_framed_area, suspend_current_and_run_next,
    },
    timer::{get_time, TimeVal},
};

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

    let offset = ts as usize & (PAGE_SIZE - 1);
    let page_remaining = PAGE_SIZE - offset;

    let time = get_time();

    let len = size_of::<TimeVal>();

    let src = core::ptr::addr_of!(time) as *const u8;
    let mut buffers = checked_translated_byte_buffer(
        current_user_token(),
        PTEFlags::U | PTEFlags::W,
        ts as *mut u8,
        len,
    );
    if page_remaining >= len {
        if buffers.is_empty() {
            -1
        } else {
            let buf = buffers[0].as_mut_ptr();
            unsafe { core::ptr::copy(src, buf, len) }

            0
        }
    } else {
        if buffers.len() < 2 {
            -1
        } else {
            let buf = buffers[0].as_mut_ptr();
            unsafe { core::ptr::copy(src, buf, page_remaining) }

            let buf = buffers[1].as_mut_ptr();
            unsafe { core::ptr::copy(src.wrapping_add(page_remaining), buf, len - page_remaining) }

            0
        }
    }
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    trace!(
        "sys_trace(request: {}, {}: {:0x}, data: {})",
        trace_request,
        if id < 2 { "addr" } else { "id" },
        id,
        data
    );

    match trace_request {
        // read a byte from id addr
        0 => {
            let buffers = checked_translated_byte_buffer(
                current_user_token(),
                PTEFlags::U | PTEFlags::R,
                id as *mut u8,
                1,
            );
            if buffers.is_empty() {
                -1
            } else {
                let buf = buffers[0].first();

                if let Some(byte) = buf {
                    *byte as isize
                } else {
                    -1
                }
            }
        }
        // write a byte to id addr
        1 => {
            let mut buffers = checked_translated_byte_buffer(
                current_user_token(),
                PTEFlags::U | PTEFlags::W,
                id as *mut u8,
                1,
            );

            if buffers.is_empty() {
                -1
            } else {
                let buf = buffers[0].first_mut();
                if let Some(byte) = buf {
                    *byte = data as u8;
                    0
                } else {
                    -1
                }
            }
        }
        2 => get_syscall_times(id),
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    insert_framed_area(start, len, prot)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    remove_framed_area(start, len)
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
