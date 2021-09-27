// use alarm to wake from pause

use nux::structs::SigAction;
use nux::syscalls;

extern "C" fn alarm_handler(_sig: i32) {
    const MSG: &[u8] = b"Hello from signal handler!\n\0";
    unsafe { syscalls::write(nux::consts::STDOUT, MSG.as_ptr(), MSG.len()) };
}

fn main() {
    unsafe { syscalls::alarm(5) };

    let sigaction = SigAction::new(alarm_handler, 0, 0);

    let ret = unsafe {
        syscalls::rt_sigaction(
            nux::consts::SIGALRM,
            &sigaction,
            0 as *mut SigAction,
        )
    };

    if ret < 0 {
        // FIXME: plz help if you know how to fix this
        println!("Failed to set up signal handler: Error {}", -ret);
        return;
    }

    let ret = unsafe { syscalls::pause() };

    if ret.unsigned_abs() == nux::errno::EINTR {
        println!("Pause interrupted by alarm!");
    } else {
        println!("Pause interrupted by other signal!");
    }
}
