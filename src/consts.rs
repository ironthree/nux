//! # Definitions of constants, magic numbers, flags, etc.
//!
//! This module contains definitions of constants, magic numbers, flags, etc.
//! that are defined in various places in the linux kernel headers.
//!
//! This includes:
//!
//! - common file descriptors (`STDIN`, `STDOUT`, `STDERR`)
//! - option flags for `open`, `creat`, and `fcntl` system calls (`O_*`)
//! - mode flags for `open`, `creat`, and `fcntl` system calls  (`S_*`)
//! - whence flags for the `lseek` system call (`SEEK_*`)

pub const STDIN : u32 = 0;
pub const STDOUT: u32 = 1;
pub const STDERR: u32 = 2;

pub const O_RDONLY      : u32 = 0o00000000;
pub const O_WRONLY      : u32 = 0o00000001;
pub const O_RDWR        : u32 = 0o00000002;
pub const O_ACCMODE     : u32 = 0o00000003;
pub const O_CREAT       : u32 = 0o00000100;
pub const O_EXCL        : u32 = 0o00000200;
pub const O_NOCTTY      : u32 = 0o00000400;
pub const O_TRUNC       : u32 = 0o00001000;
pub const O_APPEND      : u32 = 0o00002000;
pub const O_NONBLOCK    : u32 = 0o00004000;
pub const O_NDELAY      : u32 = O_NONBLOCK;
pub const O_DSYNC       : u32 = 0o00010000;
pub const O_ASYNC       : u32 = 0o00020000;
pub const O_DIRECT      : u32 = 0o00040000;
pub const O_LARGEFILE   : u32 = 0o00100000;
pub const O_DIRECTORY   : u32 = 0o00200000;
pub const O_NOFOLLOW    : u32 = 0o00400000;
pub const O_NOATIME     : u32 = 0o01000000;
pub const O_CLOEXEC     : u32 = 0o02000000;
    const __O_SYNC      : u32 = 0o04000000;
pub const O_SYNC        : u32 = __O_SYNC  | O_DSYNC;
pub const O_PATH        : u32 = 0o10000000;
    const __O_TMPFILE   : u32 = 0o20000000;
pub const O_TMPFILE     : u32 = __O_TMPFILE | O_DIRECTORY;

/// user (file owner) has read, write, and execute permission
pub const S_IRWXU : u32 = 0o0700;
/// user has read permission
pub const S_IRUSR : u32 = 0o0400;
/// user has write permission
pub const S_IWUSR : u32 = 0o0200;
/// user has execute permission
pub const S_IXUSR : u32 = 0o0100;
/// group has read, write, and execute permission
pub const S_IRWXG : u32 = 0o0070;
/// group has read permission
pub const S_IRGRP : u32 = 0o0040;
/// group has write permission
pub const S_IWGRP : u32 = 0o0020;
/// group has execute permission
pub const S_IXGRP : u32 = 0o0010;
/// others have read, write, and execute permission
pub const S_IRWXO : u32 = 0o0007;
/// others have read permission
pub const S_IROTH : u32 = 0o0004;
/// others have write permission
pub const S_IWOTH : u32 = 0o0002;
/// others have execute permission
pub const S_IXOTH : u32 = 0o0001;
/// set-user-ID bit
pub const S_ISUID : u32 = 0o4000;
/// set-group-ID bit
pub const S_ISGID : u32 = 0o2000;
/// sticky bit
pub const S_ISVTX : u32 = 0o1000;

/// set file offset to `offset` bytes
pub const SEEK_SET : i32 = 0;
/// set file offset to `offset` bytes relative to the current offset
pub const SEEK_CUR : i32 = 1;
/// set file offset to `offset` bytes relative to the end of the file
pub const SEEK_END : i32 = 2;
/// set file offset to the next location in the file which is greater
/// than or equal to `offset` and that contains data
pub const SEEK_DATA: i32 = 3;
/// set file offset to the next location in the file which is greater
/// than or equal to `offset` and that does not contain data ("hole")
pub const SEEK_HOLE: i32 = 4;

/// there is data to read
pub const POLLIN        : i16 = 0x0001;
/// there is urgent data to read
pub const POLLPRI       : i16 = 0x0002;
/// writing now will not block
pub const POLLOUT       : i16 = 0x0004;
/// error condition
pub const POLLERR       : i16 = 0x0008;
/// hung up
pub const POLLHUP       : i16 = 0x0010;
/// invalid polling request
pub const POLLNVAL      : i16 = 0x0020;
/// normal data may be read
pub const POLLRDNORM    : i16 = 0x0040;
/// priority data may be read
pub const POLLRDBAND    : i16 = 0x0080;
/// writing now will not block
pub const POLLWRNORM    : i16 = 0x0100;
/// priority data may be written
pub const POLLWRBAND    : i16 = 0x0200;
pub const POLLMSG       : i16 = 0x0400;
pub const POLLREMOVE    : i16 = 0x1000;
pub const POLLRDHUP     : i16 = 0x2000;

// TODO: check if these are architecture-dependent
// defined in <bits/signum-arch.h> and <bits/signum-generic.h>
/// hangup
pub const SIGHUP        : i32 =  1;
/// interactive attention signal
pub const SIGINT        : i32 =  2;
/// quit
pub const SIGQUIT       : i32 =  3;
/// illegal instruction
pub const SIGILL        : i32 =  4;
/// trace / breakpoint trap
pub const SIGTRAP       : i32 =  5;
/// abnormal termination
pub const SIGABRT       : i32 =  6;
/// bus error
pub const SIGBUS        : i32 =  7;
/// erroneous arithmetic operation
pub const SIGFPE        : i32 =  8;
/// killed
pub const SIGKILL       : i32 =  9;
/// user-defined signal 1
pub const SIGUSR1       : i32 = 10;
/// invalid access to storage
pub const SIGSEGV       : i32 = 11;
/// user-defined signal 2
pub const SIGUSR2       : i32 = 12;
/// broken pipe
pub const SIGPIPE       : i32 = 13;
/// alarm clock
pub const SIGALRM       : i32 = 14;
/// termination request
pub const SIGTERM       : i32 = 15;
/// stack fault (obsolete)
pub const SIGSTKFLT     : i32 = 16;
/// child terminated or stopped
pub const SIGCHLD       : i32 = 17;
/// continue
pub const SIGCONT       : i32 = 18;
/// stop, unblockable
pub const SIGSTOP       : i32 = 19;
/// keyboard stop
pub const SIGTSTP       : i32 = 20;
/// background read from control terminal
pub const SIGTTIN       : i32 = 21;
/// background write to control terminal
pub const SIGTTOU       : i32 = 22;
/// urgent data is available at a socket
pub const SIGURG        : i32 = 23;
/// CPU time limit exceeded
pub const SIGXCPU       : i32 = 24;
/// file size limit exceeded
pub const SIGXFSZ       : i32 = 25;
/// virtual timer expired
pub const SIGVTALRM     : i32 = 26;
/// profiling timer expired
pub const SIGPROF       : i32 = 27;
/// window size change (4.3 BSD, Sun)
pub const SIGWINCH      : i32 = 28;
/// pollable event occurred (System V)
pub const SIGPOLL       : i32 = 29;
/// power failure imminent
pub const SIGPWR        : i32 = 30;
/// bad system call
pub const SIGSYS        : i32 = 31;
/// I/O now possible (4.2 BSD)
pub const SIGIO         : i32 = SIGPOLL;
/// IOT instruction, abort() on a PDP-11
pub const SIGIOT        : i32 = SIGABRT;
/// old System V name
pub const SIGCLD        : i32 = SIGCHLD;

/// don't send SIGCHLD when children stop
pub const SA_NOCLDSTOP  : i32 = 0x00000001;
/// don't create zombie on child death
pub const SA_NOCLDWAIT  : i32 = 0x00000002;
/// invoke signal-catching function with three arguments instead of one
pub const SA_SIGINFO    : i32 = 0x00000004;
/// use signal stack by using `sa_restorer'
pub const SA_ONSTACK    : i32 = 0x08000000;
/// restart syscall on signal return
pub const SA_RESTART    : i32 = 0x10000000;
/// historical no-op
pub const SA_INTERRUPT  : i32 = 0x20000000;
/// don't automatically block the signal when its handler is being executed
pub const SA_NODEFER    : i32 = 0x40000000;
/// reset to `SIG_DFL` on entry to handler
pub const SA_RESETHAND  : i32 = 0x80000000u32 as i32;
pub const SA_NOMASK     : i32 = SA_NODEFER;
pub const SA_ONESHOT    : i32 = SA_RESETHAND;
pub const SA_STACK      : i32 = SA_ONSTACK;

/// block signals
pub const SIG_BLOCK     : i32 = 0;
/// unblock signals
pub const SIG_UNBLOCK   : i32 = 1;
/// set the set of blocked signals
pub const SIG_SETMASK   : i32 = 2;

/// special file descriptor that refers to the current working directory
pub const AT_FDCWD: i32 = -100;
