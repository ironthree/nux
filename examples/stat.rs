// print filesystem metadata for "Cargo.toml"

use std::mem::MaybeUninit;

use nux::structs::Stat;
use nux::syscalls;

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
