//! # Definitions of structs that are used to pass data to or from the kernel
//!
//! This module contains definitions of structs that are used to pass data
//! to or from the kernel in system calls. Names and data types of struct
//! members match the definition of those structs in the linux kernel header
//! files and their implementation in the [`libc`](https://docs.rs/libc) crate.

#![allow(non_camel_case_types)]

use core::arch::global_asm;

type __blkcnt_t = i64;
type __blksize_t = i64;
type __dev_t = u64;
type __gid_t = u32;
type __ino_t = u64;
type __mode_t = u32;
type __nlink_t = u64;
type __off_t = i64;
type __sa_family_t = u16;
type __sa_restorer_t = unsafe extern "C" fn() -> !;
type __sighandler_t = extern "C" fn(i32) -> ();
type __time_t = i64;
type __uid_t = u32;

/// Data structure describing a polling request, defined as `struct pollfd`
/// in the linux kernel header file `<sys/poll.h>`.
#[repr(C)]
#[derive(Debug)]
pub struct PollFd {
    /// file descriptor to poll
    pub fd: i32,
    /// types of events poller cares about
    pub events: i16,
    /// types of events that actually occurred
    pub revents: i16,
}

global_asm!(
    "
    .global __restore_rt
    __restore_rt:
        mov rax, 15
        syscall
    "
);

extern "C" {
    fn __restore_rt() -> !;
}

// FIXME
#[repr(C)]
pub struct SigAction {
    pub sa_handler: __sighandler_t,
    pub sa_flags: u64,
    pub sa_restorer: __sa_restorer_t,
    pub sa_mask: u64,
}

// FIXME
impl SigAction {
    pub fn new(handler: __sighandler_t, flags: u64, mask: u64) -> SigAction {
        SigAction {
            sa_handler: handler,
            sa_flags: flags,
            sa_restorer: __restore_rt,
            sa_mask: mask,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,

    pub(crate) __pad0: i32,

    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,

    pub st_atime: __time_t,
    pub st_atime_nsec: u64,
    pub st_mtime: __time_t,
    pub st_mtime_nsec: u64,
    pub st_ctime: __time_t,
    pub st_ctime_nsec: u64,

    pub(crate) __unused: [i64; 3],
}

#[repr(C)]
#[derive(Debug)]
pub struct SockAddr {
    sa_family: __sa_family_t,
    sa_data: [u8; 14],
}
