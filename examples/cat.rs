// minimal "cat" that only prints contents of "Cargo.toml" to STDOUT

use nux::syscalls;

const BUFSIZE: usize = 16;

fn main() {
    let mut buf = [0u8; BUFSIZE];

    #[cfg(not(target_arch = "aarch64"))]
    let fd = unsafe { syscalls::open("Cargo.toml\0".as_ptr(), 0, 0) };
    #[cfg(target_arch = "aarch64")]
    let fd = unsafe { syscalls::openat(nux::consts::AT_FDCWD, "Cargo.toml\0".as_ptr(), 0, 0) };

    if fd < 0 {
        eprintln!("Failed to open file: errno {}", -fd);
        return;
    }

    loop {
        let ret = unsafe { syscalls::read(fd as u32, buf.as_mut_ptr(), BUFSIZE) };

        if ret == 0 {
            break;
        } else if ret < 0 {
            eprintln!("Failed to read from file: errno {}", -ret);
            break;
        }

        let read = &buf[0usize..ret as usize];

        let written = unsafe { syscalls::write(nux::consts::STDOUT, read.as_ptr(), read.len()) };

        if written < 0 {
            eprintln!("Failed to write to stdout: errno {}", -written);
            break;
        }
    }

    let closed = unsafe { syscalls::close(fd as u32) };

    if closed < 0 {
        eprintln!("Failed to close file: errno {}", -closed);
        return;
    }
}
