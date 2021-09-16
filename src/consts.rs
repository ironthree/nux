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

pub const AT_FDCWD: i32 = -100;
