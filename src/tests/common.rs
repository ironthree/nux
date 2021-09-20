//! This module contains tests for syscalls that are available on all
//! architectures.

use crate::syscalls;

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
