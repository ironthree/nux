//! # Definitions of structs that are used to pass data to or from the kernel
//!
//! This module contains definitions of structs that are used to pass data
//! to or from the kernel in system calls. Names and data types of struct
//! members match the definition of those structs in the linux kernel header
//! files and their implementation in the [`libc`](https://docs.rs/libc) crate.

#![allow(non_camel_case_types)]

type __dev_t = u64;
type __ino_t = u32;
type __mode_t = u32;
type __nlink_t = u32;
type __uid_t = u32;
type __gid_t = u32;
type __off_t = i32;
type __blksize_t = i32;
type __blkcnt_t = i32;
type __time_t = i32;
type __syscall_slong_t = i32;

// TODO: FIXME this is not correct :(

#[repr(C)]
#[derive(Debug)]
pub struct TimeSpec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

#[repr(C)]
#[derive(Debug)]
pub struct Stat {
    pub st_dev: __dev_t,

    pub(crate) __pad1: u16,

    pub st_ino: __ino_t,
    pub st_mode: __mode_t,
    pub st_nlink: __nlink_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub st_rdev: __dev_t,

    pub(crate) __pad2: u16,

    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,

    pub st_atim: TimeSpec,
    pub st_mtim: TimeSpec,
    pub st_ctim: TimeSpec,

    pub(crate) __unused: [u32; 2],
}
