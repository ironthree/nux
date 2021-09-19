//! # Implementations of wrapper functions for system calls
//!
//! This module contains `unsafe` wrapper functions for system calls as they
//! are implemented for `x86` by the linux kernel.
//!
//! Types of function arguments and return values match the linux kernel
//! interface and / or the GNU libc implementations of these functions.

use crate::numbers;
use crate::structs;

// keep ordered by ascending system call number

pub unsafe fn fork() -> i32 {
    let ret: i32;

    asm!(
    "int 0x80",
    in("eax") numbers::FORK,
    lateout("eax") ret,
    options(nostack),
    );

    ret
}

pub unsafe fn read(fd: u32, buf: *mut u8, count: usize) -> i32 {
    let ret: i32;

    asm!(
    "int 0x80",
    in("eax") numbers::READ,
    in("ebx") fd,
    in("ecx") buf,
    in("edx") count,
    //in("esi")
    //in("edi")
    //in("ebp")
    lateout("eax") ret,
    options(nostack),
    );

    ret
}

pub unsafe fn write(fd: u32, buf: *const u8, count: usize) -> i32 {
    let ret: i32;

    asm!(
    "int 0x80",
    in("eax") numbers::WRITE,
    in("ebx") fd,
    in("ecx") buf,
    in("edx") count,
    //in("esi")
    //in("edi")
    //in("ebp")
    lateout("eax") ret,
    options(nostack),
    );

    ret
}

pub unsafe fn open(filename: *const u8, flags: i32, mode: u32) -> i32 {
    let ret: i32;

    asm!(
    "int 0x80",
    in("eax") numbers::OPEN,
    in("ebx") filename,
    in("ecx") flags,
    in("edx") mode,
    //in("esi")
    //in("edi")
    //in("ebp")
    lateout("eax") ret,
    options(nostack),
    );

    ret
}

pub unsafe fn close(fd: u32) -> i32 {
    let ret: i32;

    asm!(
    "int 0x80",
    in("eax") numbers::CLOSE,
    in("ebx") fd,
    lateout("eax") ret,
    options(nostack),
    );

    ret
}

pub unsafe fn getpid() -> i32 {
    let ret: i32;

    asm!(
    "int 0x80",
    in("eax") numbers::GETPID,
    lateout("eax") ret,
    options(nostack),
    );

    ret
}

// TODO: check signature on x86
pub unsafe fn alarm(seconds: u32) -> u32 {
    let ret: u32;

    asm!(
    "int 0x80",
    in("eax") numbers::ALARM,
    in("ebx") seconds,
    lateout("eax") ret,
    options(nostack),
    );

    ret
}

pub unsafe fn pause() -> i32 {
    let ret: i32;

    asm!(
    "int 0x80",
    in("eax") numbers::PAUSE,
    lateout("eax") ret,
    options(nostack),
    );

    ret
}

pub unsafe fn fstat(fd: u32, statbuf: *mut structs::Stat) -> i32 {
    let ret: i32;

    asm!(
    "int 0x80",
    in("eax") numbers::FSTAT,
    in("ebx") fd,
    in("ecx") statbuf,
    lateout("eax") ret,
    options(nostack),
    );

    // initialize padding
    (*statbuf).__pad1 = 0;
    (*statbuf).__pad2 = 0;
    (*statbuf).__unused = [0; 2];

    ret
}

pub unsafe fn sched_yield() -> i32 {
    let ret: i32;

    asm!(
    "int 0x80",
    in("eax") numbers::SCHED_YIELD,
    lateout("eax") ret,
    options(nostack),
    );

    ret
}

pub unsafe fn stat(filename: *const u8, statbuf: *mut structs::Stat) -> i32 {
    let ret: i32;

    asm!(
    "int 0x80",
    in("eax") numbers::STAT64,
    in("ebx") filename,
    in("ecx") statbuf,
    lateout("eax") ret,
    options(nostack),
    );

    // initialize padding
    (*statbuf).__pad1 = 0;
    (*statbuf).__pad2 = 0;
    (*statbuf).__unused = [0; 2];

    ret
}
