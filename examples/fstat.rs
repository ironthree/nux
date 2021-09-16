// print filesystem metadata for "Cargo.toml"

use std::mem::MaybeUninit;

use nux::structs::Stat;
use nux::syscalls;

fn main() {
    let mut statbuf: MaybeUninit<Stat> = MaybeUninit::uninit();

    #[cfg(not(target_arch = "aarch64"))]
    let fd = unsafe { syscalls::open("Cargo.toml\0".as_ptr(), 0, 0) };
    #[cfg(target_arch = "aarch64")]
    let fd = unsafe { syscalls::openat(nux::consts::AT_FDCWD, "Cargo.toml\0".as_ptr(), 0, 0) };

    if fd < 0 {
        eprintln!("Failed to open file: errno {}", -fd);
        return;
    }

    let ret = unsafe { syscalls::fstat(fd as u32, statbuf.as_mut_ptr()) };

    if ret < 0 {
        eprintln!("Failed to stat file: errno {}", -ret);
        return;
    }

    let closed = unsafe { syscalls::close(fd as u32) };

    if closed < 0 {
        eprintln!("Failed to close file: errno {}", -closed);
        return;
    }

    let statbuf = unsafe { statbuf.assume_init() };

    println!("stat: {:#?}", statbuf);
}
