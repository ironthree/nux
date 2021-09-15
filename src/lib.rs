#![cfg_attr(not(feature = "std"), no_std)]
// https://doc.rust-lang.org/nightly/unstable-book/library-features/asm.html
#![feature(asm)]
// allow missing Safety docs for unsafe fns for now
#![allow(clippy::missing_safety_doc)]
//#![warn(missing_docs)]

#[cfg(not(target_arch = "x86_64"))]
compile_error!("This crate only contains implementations for x86_64 yet.");

#[cfg(not(target_os = "linux"))]
compile_error!("This crate is only compatible with linux.");

pub mod consts;
pub mod errno;

#[cfg(target_arch = "x86_64")]
#[path = "numbers/x86_64.rs"]
pub mod numbers;

pub mod structs;

#[cfg(target_arch = "x86_64")]
#[path = "syscalls/x86_64.rs"]
pub mod syscalls;

#[cfg(test)]
mod tests;
