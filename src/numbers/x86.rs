//! # Definitions of system call numbers
//!
//! This module contains definitions of system call numbers as they are defined
//! for `x86_64` / `AMD64` in `<asm/unistd_32.h>` from the linux kernel headers.

pub const FORK          : u32 =   2;
pub const READ          : u32 =   3;
pub const WRITE         : u32 =   4;
pub const OPEN          : u32 =   5;
pub const CLOSE         : u32 =   6;
pub const GETPID        : u32 =  20;
pub const ALARM         : u32 =  27;
pub const PAUSE         : u32 =  29;
pub const FSTAT         : u32 = 108;
pub const SCHED_YIELD   : u32 = 158;
pub const STAT64        : u32 = 195;
