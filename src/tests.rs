//! minimal smoke tests for simple syscalls
//!
//! The unit tests in this module are expected to be simple, mostly for syscalls
//! that can be called without arguments (or with very simple-to-construct
//! arguments), with return values that are easy to check for errors, and
//! that don't have any unwanted side effects.

use super::syscalls;

#[test]
fn sched_yield() {
    let ret = unsafe { syscalls::sched_yield() };
    assert_eq!(ret, 0);
}

#[test]
fn getpid() {
    let pid = unsafe { syscalls::getpid() };
    assert!(pid as i32 > 0);
}
