# nux

This crate contains `no_std`-compatible bindings for linux kernel system calls.
The goal is to provide support for targets / architectures that are supported by
the unstable [`asm` feature](https://github.com/rust-lang/rust/issues/72016):

- `aarch64-unknown-linux-gnu`
- `armv7-unknown-linux-gnueabihf`
- `i686-unknown-linux-gnu`
- `powerpc64le-unknown-linux-gnu`
- `x86_64-unknown-linux-gnu`

## Current status: parked project

Due to lack of time (and very poor / contradictory documentation for linux
system calls), this side project will not be actively developed further, for
now. If you're looking for alternatives. you might be interested in one of the
following crates:

- [libc](https://crates.io/crates/libc) (unsafe wrapper around platform libc)
- [nix](https://crates.io/crates/nix) (safe wrapper around libc crate)
- [rsix](https://crates.io/crates/rsix) (bindings for *nix system calls), formerly `posish`
- [relibc](https://github.com/redox-os/relibc) (from-scratch libc implementation in Rust + C)

However, contributions are welcome. The basic project structure is set up so
adding bindings for more system calls should be easy for all architectures by
following the patterns that are already there. And if somebody out there knows
how to fix the FIXME items in the crate, that would be *wonderful* 😅
