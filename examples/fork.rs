use nux::syscalls;

fn main() {
    let pid = unsafe { syscalls::fork() };

    if pid < 0 {
        println!("Error {}", -pid);
    } else if pid == 0 {
        println!("Child process! PID {}", unsafe { syscalls::getpid() });
    } else {
        println!("Parent process! Child PID: {}", pid);
    }
}
