//! minimal smoke tests for simple syscalls
//!
//! The unit tests in this module are expected to be simple, mostly for syscalls
//! that can be called without arguments (or with very simple-to-construct
//! arguments), with return values that are easy to check for errors, and
//! that don't have any unwanted side effects.

mod common;

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86")]
mod x86;

#[cfg(target_arch = "aarch64")]
mod aarch64;

#[cfg(target_arch = "arm")]
mod arm;

#[cfg(target_arch = "powerpc64")]
mod powerpc64;
