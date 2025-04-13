//! RISC-V timer-related functionality

use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use riscv::register::time;
/// The number of ticks per second
const TICKS_PER_SEC: usize = 100;
#[allow(dead_code)]
/// The number of milliseconds per second
const MSEC_PER_SEC: usize = 1000;
/// The number of microseconds per second
#[allow(dead_code)]
const MICRO_PER_SEC: usize = 1_000_000;

/// Time value structure
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    /// seconds
    pub sec: usize,
    /// microseconds
    pub usec: usize,
}

/// Get the current time in ticks
pub fn get_time_tick() -> usize {
    time::read()
}

/// Get current time in [`TimeVal`]
pub fn get_time() -> TimeVal {
    let ticks = get_time_tick();
    let s = ticks / CLOCK_FREQ;
    let us = (ticks % CLOCK_FREQ) * MICRO_PER_SEC / CLOCK_FREQ;
    TimeVal { sec: s, usec: us }
}

/// get current time in milliseconds
#[allow(dead_code)]
pub fn get_time_ms() -> usize {
    time::read() * MSEC_PER_SEC / CLOCK_FREQ
}

/// get current time in microseconds
#[allow(dead_code)]
pub fn get_time_us() -> usize {
    time::read() * MICRO_PER_SEC / CLOCK_FREQ
}

/// Set the next timer interrupt
pub fn set_next_trigger() {
    set_timer(get_time_tick() + CLOCK_FREQ / TICKS_PER_SEC);
}
