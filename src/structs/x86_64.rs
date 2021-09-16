//! # Definitions of structs that are used to pass data to or from the kernel
//!
//! This module contains definitions of structs that are used to pass data
//! to or from the kernel in system calls. Names and data types of struct
//! members match the definition of those structs in the linux kernel header
//! files and their implementation in the [`libc`](https://docs.rs/libc) crate.

#[repr(C)]
#[derive(Debug)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,

    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub(crate) __pad0: u32,

    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,

    pub st_atime: u64,
    pub st_atime_nsec: u64,
    pub st_mtime: u64,
    pub st_mtime_nsec: u64,
    pub st_ctime: u64,
    pub st_ctime_nsec: u64,
    pub(crate) __unused: [i64; 3],
}
