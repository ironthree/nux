//! # Implementations of wrapper functions for system calls
//!
//! This module contains `unsafe` wrapper functions for system calls as they
//! are implemented for `x86` by the linux kernel.
//!
//! Types of function arguments and return values match the linux kernel
//! interface and / or the GNU libc implementations of these functions.

use crate::numbers;
//use crate::structs;

// keep ordered by ascending system call number

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
