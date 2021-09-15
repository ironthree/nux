//! # Definitions of common error codes
//!
//! This module contains definitions of common error codes as they are defined
//! in `<asm-generic/errno.h>` and `<asm-generic/errno-base.h>` from the linux
//! kernel headers.

/// Operation not permitted
pub const EPERM             : u32 =   1;
/// No such file or directory
pub const ENOENT            : u32 =   2;
/// No such process
pub const ESRCH             : u32 =   3;
/// Interrupted system call
pub const EINTR             : u32 =   4;
/// I/O error
pub const EIO               : u32 =   5;
/// No such device or address
pub const ENXIO             : u32 =   6;
/// Argument list too long
pub const E2BIG             : u32 =   7;
/// Exec format error
pub const ENOEXEC           : u32 =   8;
/// Bad file number
pub const EBADF             : u32 =   9;
/// No child processes
pub const ECHILD            : u32 =  10;
/// Try again
pub const EAGAIN            : u32 =  11;
/// Out of memory
pub const ENOMEM            : u32 =  12;
/// Permission denied
pub const EACCES            : u32 =  13;
/// Bad address
pub const EFAULT            : u32 =  14;
/// Block device required
pub const ENOTBLK           : u32 =  15;
/// Device or resource busy
pub const EBUSY             : u32 =  16;
/// File exists
pub const EEXIST            : u32 =  17;
/// Cross-device link
pub const EXDEV             : u32 =  18;
/// No such device
pub const ENODEV            : u32 =  19;
/// Not a directory
pub const ENOTDIR           : u32 =  20;
/// Is a directory
pub const EISDIR            : u32 =  21;
/// Invalid argument
pub const EINVAL            : u32 =  22;
/// File table overflow
pub const ENFILE            : u32 =  23;
/// Too many open files
pub const EMFILE            : u32 =  24;
/// Not a typewriter
pub const ENOTTY            : u32 =  25;
/// Text file busy
pub const ETXTBSY           : u32 =  26;
/// File too large
pub const EFBIG             : u32 =  27;
/// No space left on device
pub const ENOSPC            : u32 =  28;
/// Illegal seek
pub const ESPIPE            : u32 =  29;
/// Read-only file system
pub const EROFS             : u32 =  30;
/// Too many links
pub const EMLINK            : u32 =  31;
/// Broken pipe
pub const EPIPE             : u32 =  32;
/// Math argument out of domain of func
pub const EDOM              : u32 =  33;
/// Math result not representable
pub const ERANGE            : u32 =  34;
/// Resource deadlock would occur
pub const EDEADLK           : u32 =  35;
/// File name too long
pub const ENAMETOOLONG      : u32 =  36;
/// No record locks available
pub const ENOLCK            : u32 =  37;
/// Invalid system call number
pub const ENOSYS            : u32 =  38;
/// Directory not empty
pub const ENOTEMPTY         : u32 =  39;
/// Too many symbolic links encountered
pub const ELOOP             : u32 =  40;
/// Operation would block
pub const EWOULDBLOCK       : u32 = EAGAIN;
/// No message of desired type
pub const ENOMSG            : u32 =  42;
/// Identifier removed
pub const EIDRM             : u32 =  43;
/// Channel number out of range
pub const ECHRNG            : u32 =  44;
/// Level 2 not synchronized
pub const EL2NSYNC          : u32 =  45;
/// Level 3 halted
pub const EL3HLT            : u32 =  46;
/// Level 3 reset
pub const EL3RST            : u32 =  47;
/// Link number out of range
pub const ELNRNG            : u32 =  48;
/// Protocol driver not attached
pub const EUNATCH           : u32 =  49;
/// No CSI structure available
pub const ENOCSI            : u32 =  50;
/// Level 2 halted
pub const EL2HLT            : u32 =  51;
/// Invalid exchange
pub const EBADE             : u32 =  52;
/// Invalid request descriptor
pub const EBADR             : u32 =  53;
/// Exchange full
pub const EXFULL            : u32 =  54;
/// No anode
pub const ENOANO            : u32 =  55;
/// Invalid request code
pub const EBADRQC           : u32 =  56;
/// Invalid slot
pub const EBADSLT           : u32 =  57;
pub const EDEADLOCK         : u32 = EDEADLK;
/// Bad font file format
pub const EBFONT            : u32 =  59;
/// Device not a stream
pub const ENOSTR            : u32 =  60;
/// No data available
pub const ENODATA           : u32 =  61;
/// Timer expired
pub const ETIME             : u32 =  62;
/// Out of streams resources
pub const ENOSR             : u32 =  63;
/// Machine is not on the network
pub const ENONET            : u32 =  64;
/// Package not installed
pub const ENOPKG            : u32 =  65;
/// Object is remote
pub const EREMOTE           : u32 =  66;
/// Link has been severed
pub const ENOLINK           : u32 =  67;
/// Advertise error
pub const EADV              : u32 =  68;
/// Srmount error
pub const ESRMNT            : u32 =  69;
/// Communication error on send
pub const ECOMM             : u32 =  70;
/// Protocol error
pub const EPROTO            : u32 =  71;
/// Multihop attempted
pub const EMULTIHOP         : u32 =  72;
/// RFS specific error
pub const EDOTDOT           : u32 =  73;
/// Not a data message
pub const EBADMSG           : u32 =  74;
/// Value too large for defined data type
pub const EOVERFLOW         : u32 =  75;
/// Name not unique on network
pub const ENOTUNIQ          : u32 =  76;
/// File descriptor in bad state
pub const EBADFD            : u32 =  77;
/// Remote address changed
pub const EREMCHG           : u32 =  78;
/// Can not access a needed shared library
pub const ELIBACC           : u32 =  79;
/// Accessing a corrupted shared library
pub const ELIBBAD           : u32 =  80;
/// .lib section in a.out corrupted
pub const ELIBSCN           : u32 =  81;
/// Attempting to link in too many shared libraries
pub const ELIBMAX           : u32 =  82;
/// Cannot exec a shared library directly
pub const ELIBEXEC          : u32 =  83;
/// Illegal byte sequence
pub const EILSEQ            : u32 =  84;
/// Interrupted system call should be restarted
pub const ERESTART          : u32 =  85;
/// Streams pipe error
pub const ESTRPIPE          : u32 =  86;
/// Too many users
pub const EUSERS            : u32 =  87;
/// Socket operation on non-socket
pub const ENOTSOCK          : u32 =  88;
/// Destination address required
pub const EDESTADDRREQ      : u32 =  89;
/// Message too long
pub const EMSGSIZE          : u32 =  90;
/// Protocol wrong type for socket
pub const EPROTOTYPE        : u32 =  91;
/// Protocol not available
pub const ENOPROTOOPT       : u32 =  92;
/// Protocol not supported
pub const EPROTONOSUPPORT   : u32 =  93;
/// Socket type not supported
pub const ESOCKTNOSUPPORT   : u32 =  94;
/// Operation not supported on transport endpoint
pub const EOPNOTSUPP        : u32 =  95;
/// Protocol family not supported
pub const EPFNOSUPPORT      : u32 =  96;
/// Address family not supported by protocol
pub const EAFNOSUPPORT      : u32 =  97;
/// Address already in use
pub const EADDRINUSE        : u32 =  98;
/// Cannot assign requested address
pub const EADDRNOTAVAIL     : u32 =  99;
/// Network is down
pub const ENETDOWN          : u32 =  100;
/// Network is unreachable
pub const ENETUNREACH       : u32 =  101;
/// Network dropped connection because of reset
pub const ENETRESET         : u32 =  102;
/// Software caused connection abort
pub const ECONNABORTED      : u32 =  103;
/// Connection reset by peer
pub const ECONNRESET        : u32 =  104;
/// No buffer space available
pub const ENOBUFS           : u32 =  105;
/// Transport endpoint is already connected
pub const EISCONN           : u32 =  106;
/// Transport endpoint is not connected
pub const ENOTCONN          : u32 =  107;
/// Cannot send after transport endpoint shutdown
pub const ESHUTDOWN         : u32 =  108;
/// Too many references: cannot splice
pub const ETOOMANYREFS      : u32 =  109;
/// Connection timed out
pub const ETIMEDOUT         : u32 =  110;
/// Connection refused
pub const ECONNREFUSED      : u32 =  111;
/// Host is down
pub const EHOSTDOWN         : u32 =  112;
/// No route to host
pub const EHOSTUNREACH      : u32 =  113;
/// Operation already in progress
pub const EALREADY          : u32 =  114;
/// Operation now in progress
pub const EINPROGRESS       : u32 =  115;
/// Stale file handle
pub const ESTALE            : u32 =  116;
/// Structure needs cleaning
pub const EUCLEAN           : u32 =  117;
/// Not a XENIX named type file
pub const ENOTNAM           : u32 =  118;
/// No XENIX semaphores available
pub const ENAVAIL           : u32 =  119;
/// Is a named type file
pub const EISNAM            : u32 =  120;
/// Remote I/O error
pub const EREMOTEIO         : u32 =  121;
/// Quota exceeded
pub const EDQUOT            : u32 =  122;
/// No medium found
pub const ENOMEDIUM         : u32 =  123;
/// Wrong medium type
pub const EMEDIUMTYPE       : u32 =  124;
/// Operation Canceled
pub const ECANCELED         : u32 =  125;
/// Required key not available
pub const ENOKEY            : u32 =  126;
/// Key has expired
pub const EKEYEXPIRED       : u32 =  127;
/// Key has been revoked
pub const EKEYREVOKED       : u32 =  128;
/// Key was rejected by service
pub const EKEYREJECTED      : u32 =  129;
/// Owner died
pub const EOWNERDEAD        : u32 =  130;
/// State not recoverable
pub const ENOTRECOVERABLE   : u32 =  131;
/// Operation not possible due to RF-kill
pub const ERFKILL           : u32 =  132;
/// Memory page has hardware error
pub const EHWPOISON         : u32 =  133;
