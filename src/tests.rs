use super::syscalls;

#[test]
fn sys_39_getpid() {
    let pid = unsafe { syscalls::getpid() };
    assert!(pid as i32 > 0);
}
