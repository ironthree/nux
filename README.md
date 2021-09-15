# nux

This crate contains `no_std`-compatible bindings for linux kernel system calls.
The goal is to provide support for targets / architectures that are supported by
the unstable [`asm` feature](https://github.com/rust-lang/rust/issues/72016):

- `aarch64-unknown-linux-gnu`
- `armv7-unknown-linux-gnueabihf`
- `i686-unknown-linux-gnu`
- `powerpc64le-unknown-linux-gnu`
- `x86_64-unknown-linux-gnu`

Some architectures that are of interest are only supported by the deprecated,
unstable [`llvm_asm` feature](https://github.com/rust-lang/rust/issues/70173)
yet, and if necessary, support for these can be added later, as well:

- `s390x-unknown-linux-gnu`
