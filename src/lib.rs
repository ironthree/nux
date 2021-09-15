#![cfg_attr(not(feature = "std"), no_std)]
// https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html
#![feature(asm)]
// allow missing Safety docs for unsafe fns for now
#![allow(clippy::missing_safety_doc)]
//#![warn(missing_docs)]

#[cfg(not(target_os = "linux"))]
compile_error!("This crate is only compatible with linux.");

// shared modules
pub mod consts;
pub mod errno;
pub mod structs;

#[cfg(test)]
mod tests;

// architecture-specific modules
#[cfg(target_arch = "x86_64")]
#[path = "numbers/x86_64.rs"]
pub mod numbers;

#[cfg(target_arch = "x86")]
#[path = "numbers/x86.rs"]
pub mod numbers;

#[cfg(target_arch = "aarch64")]
#[path = "numbers/aarch64.rs"]
pub mod numbers;

#[cfg(target_arch = "arm")]
#[path = "numbers/arm.rs"]
pub mod numbers;

#[cfg(target_arch = "powerpc64")]
#[path = "numbers/powerpc64.rs"]
pub mod numbers;

#[cfg(target_arch = "x86_64")]
#[path = "syscalls/x86_64.rs"]
pub mod syscalls;

#[cfg(target_arch = "x86")]
#[path = "syscalls/x86.rs"]
pub mod syscalls;

#[cfg(target_arch = "aarch64")]
#[path = "syscalls/aarch64.rs"]
pub mod syscalls;

#[cfg(target_arch = "arm")]
#[path = "syscalls/arm.rs"]
pub mod syscalls;

#[cfg(target_arch = "powerpc64")]
#[path = "syscalls/powerpc64.rs"]
pub mod syscalls;
