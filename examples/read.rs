// read bytes from STDIN and show statistics for processed bytes

use nux::syscalls;

const BUFSIZE: usize = 16;

fn main() {
    let mut buf = [0u8; BUFSIZE];

    loop {
        let ret = unsafe { syscalls::read(nux::consts::STDIN, buf.as_mut_ptr(), BUFSIZE) };

        if ret == 0 {
            break;
        }

        let read = &buf[0usize..ret as usize];

        println!("Read stdin: {:?}", read);
        println!("          : {}", String::from_utf8_lossy(read));
        println!("Bytes read: {}", ret);
    }
}
