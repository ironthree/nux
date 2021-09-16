# nux

This crate contains `no_std`-compatible bindings for linux kernel system calls.
The goal is to provide support for targets / architectures that are supported by
the unstable [`asm` feature](https://github.com/rust-lang/rust/issues/72016):

- `aarch64-unknown-linux-gnu`
- `armv7-unknown-linux-gnueabihf`
- `i686-unknown-linux-gnu`
- `powerpc64le-unknown-linux-gnu`
- `x86_64-unknown-linux-gnu`

