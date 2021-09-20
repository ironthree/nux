//! # Implementations of wrapper functions for system calls
//!
//! This module contains `unsafe` wrapper functions for system calls as they
//! are implemented for `x86_64` / `AMD64` by the linux kernel.
//!
//! Types of function arguments and return values match the linux kernel
//! interface and / or the GNU libc implementations of these functions.

use crate::numbers;
use crate::structs;

// keep ordered by ascending system call number

pub unsafe fn read(fd: u32, buf: *mut u8, count: usize) -> i64 {
    let ret: i64;

    asm!(
    "syscall",
    in("rax") numbers::READ,
    in("rdi") fd,
    in("rsi") buf,
    in("rdx") count,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn write(fd: u32, buf: *const u8, count: usize) -> i64 {
    let ret: i64;

    asm!(
    "syscall",
    in("rax") numbers::WRITE,
    in("rdi") fd,
    in("rsi") buf,
    in("rdx") count,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn open(filename: *const u8, flags: i32, mode: u32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::OPEN,
    in("rdi") filename,
    in("rsi") flags,
    in("rdx") mode,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn close(fd: u32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::CLOSE,
    in("rdi") fd,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn stat(filename: *const u8, statbuf: *mut structs::Stat) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::STAT,
    in("rdi") filename,
    in("rsi") statbuf,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    // initialize padding
    (*statbuf).__pad0 = 0;
    (*statbuf).__unused = [0; 3];

    ret
}

pub unsafe fn fstat(fd: u32, statbuf: *mut structs::Stat) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::FSTAT,
    in("rdi") fd,
    in("rsi") statbuf,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    // initialize padding
    (*statbuf).__pad0 = 0;
    (*statbuf).__unused = [0; 3];

    ret
}

pub unsafe fn lstat(filename: *const u8, statbuf: *mut structs::Stat) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::LSTAT,
    in("rdi") filename,
    in("rsi") statbuf,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    // initialize padding
    (*statbuf).__pad0 = 0;
    (*statbuf).__unused = [0; 3];

    ret
}

pub unsafe fn lseek(fd: u32, offset: i64, whence: i32) -> i64 {
    let ret: i64;

    asm!(
    "syscall",
    in("rax") numbers::LSEEK,
    in("rdi") fd,
    in("rsi") offset,
    in("rdx") whence,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

// TODO: define mmap flags: MAP_*
pub unsafe fn mmap(addr: *mut u8, length: usize, prot: i32, flags: i32, fd: u32, offset: i64) -> *mut u8 {
    let ret: *mut u8;

    asm!(
    "syscall",
    in("rax") numbers::MMAP,
    in("rdi") addr,
    in("rsi") length,
    in("rdx") prot,
    in("r10") flags,
    in("r8") fd,
    in("r9") offset,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

// TODO: define mprotect flags: PROT_*
pub unsafe fn mprotect(addr: *mut u8, length: usize, prot: i32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::MPROTECT,
    in("rdi") addr,
    in("rsi") length,
    in("rdx") prot,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn munmap(addr: *mut u8, length: usize) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::MUNMAP,
    in("rdi") addr,
    in("rsi") length,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn brk(addr: *mut u8) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::BRK,
    in("rdi") addr,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn ioctl(fd: u32, cmd: u32, arg: u64) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::IOCTL,
    in("rdi") fd,
    in("rsi") cmd,
    in("rdx") arg,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn pread(fd: u32, buf: *mut u8, count: usize, pos: i64) -> i64 {
    let ret: i64;

    asm!(
    "syscall",
    in("rax") numbers::PREAD64,
    in("rdi") fd,
    in("rsi") buf,
    in("rdx") count,
    in("r10") pos,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn pwrite(fd: u32, buf: *const u8, count: usize, pos: i64) -> i64 {
    let ret: i64;

    asm!(
    "syscall",
    in("rax") numbers::PWRITE64,
    in("rdi") fd,
    in("rsi") buf,
    in("rdx") count,
    in("r10") pos,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn access(filename: *const u8, mode: u32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::ACCESS,
    in("rdi") filename,
    in("rsi") mode,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn pipe(pipefd: *mut [u32; 2]) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::PIPE,
    in("rdi") pipefd,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn sched_yield() -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::SCHED_YIELD,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

// TODO: define mremap flags: MREMAP_*
pub unsafe fn mremap(old_addr: *mut u8, old_size: usize, new_size: usize, flags: i32, new_addr: *mut u8) -> *mut u8 {
    let ret: *mut u8;

    asm!(
    "syscall",
    in("rax") numbers::MREMAP,
    in("rdi") old_addr,
    in("rsi") old_size,
    in("rdx") new_size,
    in("r10") flags,
    in("r8") new_addr,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

// TODO: define msync flags: MS_*
pub unsafe fn msync(addr: *mut u8, length: usize, flags: i32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::MSYNC,
    in("rdi") addr,
    in("rsi") length,
    in("rdx") flags,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn mincore(addr: *const u8, length: usize, vec: *mut u8) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::MINCORE,
    in("rdi") addr,
    in("rsi") length,
    in("rdx") vec,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

// TODO: define madvise flags: MADV_*
pub unsafe fn madvise(addr: *const u8, length: usize, advice: i32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::MADVISE,
    in("rdi") addr,
    in("rsi") length,
    in("rdx") advice,
    lateout("rax") ret,
    lateout("rcx") _,
    options(nostack),
    );

    ret
}

// TODO: define shmget flags: IPC_*, SHM_*
pub unsafe fn shmget(key: i32, size: usize, shmflg: i32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::SHMGET,
    in("rdi") key,
    in("rsi") size,
    in("rdx") shmflg,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn shmat(shmid: i32, shmaddr: *mut u8, shmflg: i32) -> *mut u8 {
    let ret: *mut u8;

    asm!(
    "syscall",
    in("rax") numbers::SHMAT,
    in("rdi") shmid,
    in("rsi") shmaddr,
    in("rdx") shmflg,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn dup(old_fd: u32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::DUP,
    in("rdi") old_fd,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn dup2(old_fd: u32, new_fd: u32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::DUP2,
    in("rdi") old_fd,
    in("rsi") new_fd,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn pause() -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::PAUSE,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn alarm(seconds: u64) -> u64 {
    let ret: u64;

    asm!(
    "syscall",
    in("rax") numbers::ALARM,
    in("rdi") seconds,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn getpid() -> u32 {
    let ret: u32;

    asm!(
    "syscall",
    in("rax") numbers::GETPID,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn sendfile(out_fd: u32, in_fd: u32, offset: i64, count: usize) -> i64 {
    let ret: i64;

    asm!(
    "syscall",
    in("rax") numbers::SENDFILE,
    in("rdi") out_fd,
    in("rsi") in_fd,
    in("rdx") offset,
    in("r10") count,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

// TODO: define socket domains AF_* and socket type SOCK_*
pub unsafe fn socket(domain: i32, stype: i32, protocol: i32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::SOCKET,
    in("rdi") domain,
    in("rsi") stype,
    in("rdx") protocol,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn connect(sockfd: u32, addr: *const structs::SockAddr, addrlen: u32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::CONNECT,
    in("rdi") sockfd,
    in("rsi") addr,
    in("rdx") addrlen,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn accept(sockfd: u32, addr: *mut structs::SockAddr, addrlen: u32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::ACCEPT,
    in("rdi") sockfd,
    in("rsi") addr,
    in("rdx") addrlen,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

// TODO: define sendto flags MSG_*
pub unsafe fn sendto(
    sockfd: u32,
    buf: *const u8,
    len: usize,
    flags: i32,
    dest_addr: *const structs::SockAddr,
    addrlen: u32,
) -> i64 {
    let ret: i64;

    asm!(
    "syscall",
    in("rax") numbers::SENDTO,
    in("rdi") sockfd,
    in("rsi") buf,
    in("rdx") len,
    in("r10") flags,
    in("r8") dest_addr,
    in("r9") addrlen,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

// TODO: define shutdown flags SHUT_*
pub unsafe fn shutdown(sockfd: u32, how: i32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::SHUTDOWN,
    in("rdi") sockfd,
    in("rsi") how,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn listen(sockfd: u32, backlog: u32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::LISTEN,
    in("rdi") sockfd,
    in("rsi") backlog,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn socketpair(domain: i32, stype: i32, protocol: i32, sv: *mut [i32; 2]) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::SOCKETPAIR,
    in("rdi") domain,
    in("rsi") stype,
    in("rdx") protocol,
    in("r10") sv,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn fork() -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::FORK,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn vfork() -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::VFORK,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn execve(filename: *const u8, argv: *const *const u8, envp: *const *const u8) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::EXECVE,
    in("rdi") filename,
    in("rsi") argv,
    in("rdx") envp,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn exit(status: i32) {
    asm!(
    "syscall",
    in("rax") numbers::EXIT,
    in("rdi") status,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );
}

pub unsafe fn kill(pid: u32, sig: i32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::KILL,
    in("rdi") pid,
    in("rsi") sig,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn shmdt(shmaddr: *const u8) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::SHMDT,
    in("rdi") shmaddr,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn fsync(fd: u32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::FSYNC,
    in("rdi") fd,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}

pub unsafe fn openat(dirfd: i32, filename: *const u8, flags: i32, mode: u32) -> i32 {
    let ret: i32;

    asm!(
    "syscall",
    in("rax") numbers::OPENAT,
    in("rdi") dirfd,
    in("rsi") filename,
    in("rdx") flags,
    in("r10") mode,
    lateout("rax") ret,
    lateout("rcx") _,
    lateout("r11") _,
    options(nostack),
    );

    ret
}
