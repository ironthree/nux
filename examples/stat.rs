// print filesystem metadata for "Cargo.toml"

// the "stat" syscall does not exist on aarch64
#![cfg_attr(target_arch = "aarch64", allow(unused))]

use std::mem::MaybeUninit;

use nux::structs::Stat;
use nux::syscalls;

#[cfg(not(target_arch = "aarch64"))]
fn main() {
    let mut statbuf: MaybeUninit<Stat> = MaybeUninit::uninit();

    let ret = unsafe { syscalls::stat("Cargo.toml\0".as_ptr(), statbuf.as_mut_ptr()) };

    if ret < 0 {
        eprintln!("Failed to stat file: errno {}", -ret);
        return;
    }

    let statbuf = unsafe { statbuf.assume_init() };

    println!("stat: {:#?}", statbuf);
}

#[cfg(target_arch = "aarch64")]
fn main() {}
