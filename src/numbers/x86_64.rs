//! # Definitions of system call numbers
//!
//! This module contains definitions of system call numbers as they are defined
//! for `x86_64` / `AMD64` in `<asm/unistd_64.h>` from the linux kernel headers.

pub const READ          : u64 =   0;
pub const WRITE         : u64 =   1;
pub const OPEN          : u64 =   2;
pub const CLOSE         : u64 =   3;
pub const STAT          : u64 =   4;
pub const FSTAT         : u64 =   5;
pub const LSTAT         : u64 =   6;
pub const POLL          : u64 =   7;
pub const LSEEK         : u64 =   8;
pub const MMAP          : u64 =   9;
pub const MPROTECT      : u64 =  10;
pub const MUNMAP        : u64 =  11;
pub const BRK           : u64 =  12;
pub const RT_SIGACTION  : u64 =  13;
pub const RT_SIGRETURN  : u64 =  15;
pub const IOCTL         : u64 =  16;
// renamed pread -> pread64 in linux 2.6
pub const PREAD64       : u64 =  17;
// renamed pwrite -> pwrite64 in linux 2.6
pub const PWRITE64      : u64 =  18;
pub const ACCESS        : u64 =  21;
pub const PIPE          : u64 =  22;
pub const SCHED_YIELD   : u64 =  24;
pub const MREMAP        : u64 =  25;
pub const MSYNC         : u64 =  26;
pub const MINCORE       : u64 =  27;
pub const MADVISE       : u64 =  28;
pub const SHMGET        : u64 =  29;
pub const SHMAT         : u64 =  30;
pub const DUP           : u64 =  32;
pub const DUP2          : u64 =  33;
pub const PAUSE         : u64 =  34;
pub const ALARM         : u64 =  37;
pub const GETPID        : u64 =  39;
pub const SENDFILE      : u64 =  40;
pub const SOCKET        : u64 =  41;
pub const CONNECT       : u64 =  42;
pub const ACCEPT        : u64 =  43;
pub const SENDTO        : u64 =  44;
pub const RECVFROM      : u64 =  45;
pub const SHUTDOWN      : u64 =  48;
pub const BIND          : u64 =  49;
pub const LISTEN        : u64 =  50;
pub const GETSOCKNAME   : u64 =  51;
pub const GETPEERNAME   : u64 =  52;
pub const SOCKETPAIR    : u64 =  53;
pub const FORK          : u64 =  57;
pub const VFORK         : u64 =  58;
pub const EXECVE        : u64 =  59;
pub const EXIT          : u64 =  60;
pub const KILL          : u64 =  62;
pub const SEMGET        : u64 =  64;
pub const SHMDT         : u64 =  67;
pub const FSYNC         : u64 =  74;
pub const OPENAT        : u64 = 257;

// Not implemented (as of linux 5.13.15):

/*
__NR_rt_sigprocmask 14
__NR_readv 19
__NR_writev 20
__NR_select 23
__NR_shmctl 31
__NR_nanosleep 35
__NR_getitimer 36
__NR_setitimer 38
__NR_sendmsg 46
__NR_recvmsg 47
__NR_setsockopt 54
__NR_getsockopt 55
__NR_clone 56
__NR_wait4 61
__NR_uname 63
__NR_semop 65
__NR_semctl 66
__NR_msgget 68
__NR_msgsnd 69
__NR_msgrcv 70
__NR_msgctl 71
__NR_fcntl 72
__NR_flock 73
__NR_fdatasync 75
__NR_truncate 76
__NR_ftruncate 77
__NR_getdents 78
__NR_getcwd 79
__NR_chdir 80
__NR_fchdir 81
__NR_rename 82
__NR_mkdir 83
__NR_rmdir 84
__NR_creat 85
__NR_link 86
__NR_unlink 87
__NR_symlink 88
__NR_readlink 89
__NR_chmod 90
__NR_fchmod 91
__NR_chown 92
__NR_fchown 93
__NR_lchown 94
__NR_umask 95
__NR_gettimeofday 96
__NR_getrlimit 97
__NR_getrusage 98
__NR_sysinfo 99
__NR_times 100
__NR_ptrace 101
__NR_getuid 102
__NR_syslog 103
__NR_getgid 104
__NR_setuid 105
__NR_setgid 106
__NR_geteuid 107
__NR_getegid 108
__NR_setpgid 109
__NR_getppid 110
__NR_getpgrp 111
__NR_setsid 112
__NR_setreuid 113
__NR_setregid 114
__NR_getgroups 115
__NR_setgroups 116
__NR_setresuid 117
__NR_getresuid 118
__NR_setresgid 119
__NR_getresgid 120
__NR_getpgid 121
__NR_setfsuid 122
__NR_setfsgid 123
__NR_getsid 124
__NR_capget 125
__NR_capset 126
__NR_rt_sigpending 127
__NR_rt_sigtimedwait 128
__NR_rt_sigqueueinfo 129
__NR_rt_sigsuspend 130
__NR_sigaltstack 131
__NR_utime 132
__NR_mknod 133
__NR_uselib 134
__NR_personality 135
__NR_ustat 136
__NR_statfs 137
__NR_fstatfs 138
__NR_sysfs 139
__NR_getpriority 140
__NR_setpriority 141
__NR_sched_setparam 142
__NR_sched_getparam 143
__NR_sched_setscheduler 144
__NR_sched_getscheduler 145
__NR_sched_get_priority_max 146
__NR_sched_get_priority_min 147
__NR_sched_rr_get_interval 148
__NR_mlock 149
__NR_munlock 150
__NR_mlockall 151
__NR_munlockall 152
__NR_vhangup 153
__NR_modify_ldt 154
__NR_pivot_root 155
__NR__sysctl 156 // removed in linux 5.5
__NR_prctl 157
__NR_arch_prctl 158
__NR_adjtimex 159
__NR_setrlimit 160
__NR_chroot 161
__NR_sync 162
__NR_acct 163
__NR_settimeofday 164
__NR_mount 165
__NR_umount2 166
__NR_swapon 167
__NR_swapoff 168
__NR_reboot 169
__NR_sethostname 170
__NR_setdomainname 171
__NR_iopl 172
__NR_ioperm 173

linux 2.6:
supporting any kernel that's older than 2.6 probably makes no sense

__NR_create_module 174 // removed in linux 2.6
__NR_init_module 175
__NR_delete_module 176
__NR_get_kernel_syms 177 // removed in linux 2.6
__NR_query_module 178 // removed in linux 2.6
__NR_quotactl 179
__NR_nfsservctl 180 // removed in linux 3.1
__NR_gettid 186
__NR_readahead 187
__NR_setxattr 188
__NR_lsetxattr 189
__NR_fsetxattr 190
__NR_getxattr 191
__NR_lgetxattr 192
__NR_fgetxattr 193
__NR_listxattr 194
__NR_llistxattr 195
__NR_flistxattr 196
__NR_removexattr 197
__NR_lremovexattr 198
__NR_fremovexattr 199
__NR_tkill 200
__NR_time 201
__NR_futex 202
__NR_sched_setaffinity 203
__NR_sched_getaffinity 204
__NR_set_thread_area 205
__NR_io_setup 206
__NR_io_destroy 207
__NR_io_getevents 208
__NR_io_submit 209
__NR_io_cancel 210
__NR_get_thread_area 211
__NR_lookup_dcookie 212
__NR_epoll_create 213
__NR_epoll_ctl_old 214
__NR_epoll_wait_old 215
__NR_remap_file_pages 216 // deprecated since linux 3.16
__NR_getdents64 217
__NR_set_tid_address 218
__NR_restart_syscall 219
__NR_semtimedop 220
__NR_fadvise64 221
__NR_timer_create 222
__NR_timer_settime 223
__NR_timer_gettime 224
__NR_timer_getoverrun 225
__NR_timer_delete 226
__NR_clock_settime 227
__NR_clock_gettime 228
__NR_clock_getres 229
__NR_clock_nanosleep 230
__NR_exit_group 231
__NR_epoll_wait 232
__NR_epoll_ctl 233
__NR_tgkill 234
__NR_utimes 235

linux 2.6.6:

__NR_mbind 237
__NR_set_mempolicy 238
__NR_get_mempolicy 239
__NR_mq_open 240
__NR_mq_unlink 241
__NR_mq_timedsend 242
__NR_mq_timedreceive 243
__NR_mq_notify 244
__NR_mq_getsetattr 245

linux 2.6.10:

__NR_waitid 247
__NR_add_key 248
__NR_request_key 249
__NR_keyctl 250

linux 2.6.13:

__NR_kexec_load 246
__NR_ioprio_set 251
__NR_ioprio_get 252
__NR_inotify_init 253
__NR_inotify_add_watch 254
__NR_inotify_rm_watch 255

linux 2.6.16:

__NR_migrate_pages 256
__NR_mkdirat 258
__NR_mknodat 259
__NR_fchownat 260
__NR_futimesat 261
__NR_newfstatat 262
__NR_unlinkat 263
__NR_renameat 264
__NR_linkat 265
__NR_symlinkat 266
__NR_readlinkat 267
__NR_fchmodat 268
__NR_faccessat 269
__NR_pselect6 270
__NR_ppoll 271
__NR_unshare 272
__NR_utimensat 280

linux 2.6.17:

__NR_set_robust_list 273
__NR_get_robust_list 274
__NR_splice 275
__NR_tee 276
__NR_sync_file_range 277
__NR_vmsplice 278

linux 2.6.18:

__NR_move_pages 279

linux 2.6.19:

__NR_epoll_pwait 281
__NR_getcpu 309

linux 2.6.22:

__NR_signalfd 282
__NR_eventfd 284

linux 2.6.23:

__NR_fallocate 285

linux 2.6.25:

__NR_timerfd_create 283
__NR_timerfd_settime 286
__NR_timerfd_gettime 287

linux 2.6.27:

__NR_signalfd4 289
__NR_eventfd2 290
__NR_epoll_create1 291
__NR_dup3 292
__NR_pipe2 293
__NR_inotify_init1 294

linux 2.6.28:

__NR_accept4 288

linux 2.6.30:

__NR_preadv 295
__NR_pwritev 296

linux 2.6.31:

__NR_rt_tgsigqueueinfo 297

linux 2.6.32:

__NR_perf_event_open 298

linux 2.6.33:

__NR_recvmmsg 299

linux 2.6.36:

__NR_prlimit64 302

linux 2.6.37:

__NR_fanotify_init 300
__NR_fanotify_mark 301

linux 2.6.39:

__NR_name_to_handle_at 303
__NR_open_by_handle_at 304
__NR_clock_adjtime 305
__NR_syncfs 306

linux 3.0:

__NR_sendmmsg 307
__NR_setns 308

linux 3.2:

__NR_process_vm_readv 310
__NR_process_vm_writev 311

linux 3.5:

__NR_kcmp 312

linux 3.8:

__NR_finit_module 313

linux 3.14:

__NR_sched_setattr 314
__NR_sched_getattr 315

linux 3.15:

__NR_renameat2 316

linux 3.17:

__NR_seccomp 317
__NR_getrandom 318
__NR_memfd_create 319
__NR_kexec_file_load 320
__NR_membarrier 324

linux 3.18:

__NR_bpf 321

linux 3.19:

__NR_execveat 322

linux 4.3:

__NR_userfaultfd 323

linux 4.4:

__NR_mlock2 325

linux 4.5:

__NR_copy_file_range 326

linux 4.6:

__NR_preadv2 327
__NR_pwritev2 328

linux 4.8:

__NR_pkey_mprotect 329
__NR_pkey_alloc 330
__NR_pkey_free 331

linux 4.11:

__NR_statx 332

linux 4.18:

__NR_io_pgetevents 333
__NR_rseq 334

linux 5.1:

__NR_pidfd_send_signal 424
__NR_io_uring_setup 425
__NR_io_uring_enter 426
__NR_io_uring_register 427

linux 5.2:

__NR_open_tree 428
__NR_move_mount 429
__NR_fsopen 430
__NR_fsconfig 431
__NR_fsmount 432
__NR_fspick 433

linux 5.3:

__NR_pidfd_open 434
__NR_clone3 435

linux 5.6:

__NR_openat2 437
__NR_pidfd_getfd 438

linux 5.8:

__NR_faccessat2 439

linux 5.9:

__NR_close_range 436

linux 5.10:

__NR_process_madvise 440

linux 5.11:

__NR_epoll_pwait2 441

linux 5.12:

__NR_mount_setattr 442

linux 5.13:

__NR_landlock_create_ruleset 444
__NR_landlock_add_rule 445
__NR_landlock_restrict_self 446

unimplemented!():

__NR_getpmsg 181
__NR_putpmsg 182
__NR_tuxcall 184
__NR_security 185
__NR_afs_syscall 183
__NR_vserver 236

*/
