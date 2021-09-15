//! # Implementations of wrapper functions for system calls
//!
//! This module contains `unsafe` wrapper functions for system calls as they
//! are implemented for `x86_64` / `AMD64` by the linux kernel.
//!
//! Types of function arguments and return values match the linux kernel
//! interface and / or the GNU libc implementations of these functions.

use core::ffi::c_void;

use crate::numbers;
use crate::structs;

// keep ordered by ascending system call number

pub unsafe fn read(fd: u32, buf: *mut u8, count: usize) -> i64 {
    let syscall = numbers::READ;
    let ret: i64;

    asm!(
    "syscall",
    in("rax") syscall,
    in("rdi") fd,
    in("rsi") buf,
    in("rdx") count,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn write(fd: u32, buf: *const u8, count: usize) -> i64 {
    let syscall = numbers::WRITE;
    let ret: i64;

    asm!(
    "syscall",
    in("rax") syscall,
    in("rdi") fd,
    in("rsi") buf,
    in("rdx") count,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn open(filename: *const u8, flags: i32, mode: u32) -> i32 {
    let syscall = numbers::OPEN;
    let ret: i32;

    asm!(
    "syscall",
    in("rax") syscall,
    in("rdi") filename,
    in("rsi") flags,
    in("rdx") mode,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn close(fd: u32) -> i32 {
    let syscall = numbers::CLOSE;
    let ret: i32;

    asm!(
    "syscall",
    in("rax") syscall,
    in("rdi") fd,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn stat(filename: *const u8, statbuf: *mut structs::Stat) -> i32 {
    let syscall = numbers::STAT;
    let ret: i32;

    asm!(
    "syscall",
    in("rax") syscall,
    in("rdi") filename,
    in("rsi") statbuf,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    // initialize padding
    (*statbuf).__pad0 = 0;
    (*statbuf).__unused = [0; 3];

    ret
}

pub unsafe fn fstat(fd: u32, statbuf: *mut structs::Stat) -> i32 {
    let syscall = numbers::FSTAT;
    let ret: i32;

    asm!(
    "syscall",
    in("rax") syscall,
    in("rdi") fd,
    in("rsi") statbuf,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    // initialize padding
    (*statbuf).__pad0 = 0;
    (*statbuf).__unused = [0; 3];

    ret
}

pub unsafe fn lstat(filename: *const u8, statbuf: *mut structs::Stat) -> i32 {
    let syscall = numbers::LSTAT;
    let ret: i32;

    asm!(
    "syscall",
    in("rax") syscall,
    in("rdi") filename,
    in("rsi") statbuf,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    // initialize padding
    (*statbuf).__pad0 = 0;
    (*statbuf).__unused = [0; 3];

    ret
}

pub unsafe fn lseek(fd: u32, offset: i64, whence: i32) -> i64 {
    let syscall = numbers::LSEEK;
    let ret: i64;

    asm!(
    "syscall",
    in("rax") syscall,
    in("rdi") fd,
    in("rsi") offset,
    in("rdx") whence,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn mmap(addr: *mut c_void, length: usize, prot: i32, flags: i32, fd: u32, offset: i64) -> *mut c_void {
    let syscall = numbers::MMAP;
    let ret: *mut c_void;

    asm!(
    "syscall",
    in("rax") syscall,
    in("rdi") addr,
    in("rsi") length,
    in("rdx") prot,
    in("r10") flags,
    in("r8") fd,
    in("r9") offset,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn mprotect(addr: *mut c_void, length: usize, prot: i32) -> i32 {
    let syscall = numbers::MPROTECT;
    let ret: i32;

    asm!(
    "syscall",
    in("rax") syscall,
    in("rdi") addr,
    in("rsi") length,
    in("rdx") prot,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn munmap(addr: *mut c_void, length: usize) -> i32 {
    let syscall = numbers::MUNMAP;
    let ret: i32;

    asm!(
    "syscall",
    in("rax") syscall,
    in("rdi") addr,
    in("rsi") length,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn brk(addr: *mut c_void) -> i32 {
    let syscall = numbers::BRK;
    let ret: i32;

    asm!(
    "syscall",
    in("rax") syscall,
    in("rdi") addr,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn getpid() -> u32 {
    let syscall = numbers::GETPID;
    let ret: u32;

    asm!(
    "syscall",
    in("rax") syscall,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn fsync(fd: u32) -> i32 {
    let syscall = numbers::FSYNC;
    let ret: i32;

    asm!(
    "syscall",
    in("rax") syscall,
    in("rdi") fd,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}
