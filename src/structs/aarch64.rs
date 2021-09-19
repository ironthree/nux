//! # Definitions of structs that are used to pass data to or from the kernel
//!
//! This module contains definitions of structs that are used to pass data
//! to or from the kernel in system calls. Names and data types of struct
//! members match the definition of those structs in the linux kernel header
//! files and their implementation in the [`libc`](https://docs.rs/libc) crate.

#![allow(non_camel_case_types)]

type __blkcnt_t = i64;
type __blksize_t = i32;
type __dev_t = u64;
type __gid_t = u32;
type __ino_t = u64;
type __mode_t = u32;
type __nlink_t = u32;
type __off_t = i64;
type __time_t = i64;
type __uid_t = u32;

#[repr(C)]
#[derive(Debug)]
pub struct Stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_mode: __mode_t,
    pub st_nlink: __nlink_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub st_rdev: __dev_t,

    pub(crate) __pad1: __dev_t,

    pub st_size: __off_t,
    pub st_blksize: __blksize_t,

    pub(crate) __pad2: i32,

    pub st_blocks: __blkcnt_t,

    pub st_atime: __time_t,
    pub st_atime_nsec: u64,
    pub st_mtime: __time_t,
    pub st_mtime_nsec: u64,
    pub st_ctime: __time_t,
    pub st_ctime_nsec: u64,

    pub(crate) __unused: [i32; 2],
}
