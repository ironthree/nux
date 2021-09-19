//! # Implementations of wrapper functions for system calls
//!
//! This module contains `unsafe` wrapper functions for system calls as they
//! are implemented for `aarch64` by the linux kernel.
//!
//! Types of function arguments and return values match the linux kernel
//! interface and / or the GNU libc implementations of these functions.

use crate::consts::AT_FDCWD;

use crate::numbers;
use crate::structs;

// keep ordered by ascending system call number

pub unsafe fn openat(dirfd: i32, filename: *const u8, flags: i32, mode: u32) -> i32 {
    let ret: i32;

    asm!(
    "svc #0",
    in("x8") numbers::OPENAT,
    in("x0") dirfd,
    in("x1") filename,
    in("x2") flags,
    in("x3") mode,
    lateout("x0") ret,
    options(nostack),
    );

    ret
}

pub unsafe fn close(fd: u32) -> i32 {
    let ret: i32;

    asm!(
    "svc #0",
    in("x8") numbers::CLOSE,
    in("x0") fd,
    lateout("x0") ret,
    options(nostack),
    );

    ret
}

pub unsafe fn read(fd: u32, buf: *mut u8, count: usize) -> i64 {
    let ret: i64;

    asm!(
    "svc #0",
    in("x8") numbers::READ,
    in("x0") fd,
    in("x1") buf,
    in("x2") count,
    lateout("x0") ret,
    options(nostack),
    );

    ret
}

pub unsafe fn write(fd: u32, buf: *const u8, count: usize) -> i64 {
    let ret: i64;

    asm!(
    "svc #0",
    in("x8") numbers::WRITE,
    in("x0") fd,
    in("x1") buf,
    in("x2") count,
    lateout("x0") ret,
    options(nostack),
    );

    ret
}

pub unsafe fn fstat(fd: u32, statbuf: *mut structs::Stat) -> i32 {
    let ret: i32;

    asm!(
    "svc #0",
    in("x8") numbers::FSTAT,
    in("x0") fd,
    in("x1") statbuf,
    lateout("x0") ret,
    options(nostack),
    );

    // initialize padding
    (*statbuf).__pad1 = 0;
    (*statbuf).__pad2 = 0;
    (*statbuf).__unused = [0; 2];

    ret
}

// wrapper functions
// for compatibility with other architectures which have those syscalls

pub unsafe fn open(filename: *const u8, flags: i32, mode: u32) -> i32 {
    openat(AT_FDCWD, filename, flags, mode)
}

pub unsafe fn stat(filename: *const u8, statbuf: *mut structs::Stat) -> i32 {
    let fd = open(filename, 0, 0);
    if fd < 0 {
        return fd;
    }

    let ret = fstat(fd as u32, statbuf);

    close(fd as u32);

    if ret < 0 {
        return ret;
    }

    ret
}
