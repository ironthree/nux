// write bytes to STDOUT

use nux::syscalls;

fn main() {
    let buf = b"Hello, World!\n";

    let ret = unsafe { syscalls::write(nux::consts::STDOUT, buf.as_ptr(), buf.len()) } as i64;

    println!("Bytes written: {}", ret);
}
