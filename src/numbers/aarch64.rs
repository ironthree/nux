//! # Definitions of system call numbers
//!
//! This module contains definitions of system call numbers as they are defined
//! for `aarch64` in `<asm-generic/unistd.h>` from the linux kernel headers.

pub const OPENAT        : u64 =  56;
pub const CLOSE         : u64 =  57;
pub const READ          : u64 =  63;
pub const WRITE         : u64 =  64;
pub const FSTAT         : u64 =  80;
pub const SCHED_YIELD   : u64 = 124;
pub const GETPID        : u64 = 172;

// Not implemented (as of linux 5.13.15):

/*
__NR_io_setup 0
__NR_io_destroy 1
__NR_io_submit 2
__NR_io_cancel 3
__NR_io_getevents 4
__NR_setxattr 5
__NR_lsetxattr 6
__NR_fsetxattr 7
__NR_getxattr 8
__NR_lgetxattr 9
__NR_fgetxattr 10
__NR_listxattr 11
__NR_llistxattr 12
__NR_flistxattr 13
__NR_removexattr 14
__NR_lremovexattr 15
__NR_fremovexattr 16
__NR_getcwd 17
__NR_lookup_dcookie 18
__NR_eventfd2 19
__NR_epoll_create1 20
__NR_epoll_ctl 21
__NR_epoll_pwait 22
__NR_dup 23
__NR_dup3 24
__NR3264_fcntl 25
__NR_inotify_init1 26
__NR_inotify_add_watch 27
__NR_inotify_rm_watch 28
__NR_ioctl 29
__NR_ioprio_set 30
__NR_ioprio_get 31
__NR_flock 32
__NR_mknodat 33
__NR_mkdirat 34
__NR_unlinkat 35
__NR_symlinkat 36
__NR_linkat 37
__NR_renameat 38
__NR_umount2 39
__NR_mount 40
__NR_pivot_root 41
__NR_nfsservctl 42
__NR3264_statfs 43
__NR3264_fstatfs 44
__NR3264_truncate 45
__NR3264_ftruncate 46
__NR_fallocate 47
__NR_faccessat 48
__NR_chdir 49
__NR_fchdir 50
__NR_chroot 51
__NR_fchmod 52
__NR_fchmodat 53
__NR_fchownat 54
__NR_fchown 55
__NR_vhangup 58
__NR_pipe2 59
__NR_quotactl 60
__NR_getdents64 61
__NR3264_lseek 62
__NR_readv 65
__NR_writev 66
__NR_pread64 67
__NR_pwrite64 68
__NR_preadv 69
__NR_pwritev 70
__NR3264_sendfile 71
__NR_pselect6 72
__NR_ppoll 73
__NR_signalfd4 74
__NR_vmsplice 75
__NR_splice 76
__NR_tee 77
__NR_readlinkat 78
__NR3264_fstatat 79
__NR3264_fstat 80
__NR_sync 81
__NR_fsync 82
__NR_fdatasync 83
__NR_sync_file_range2 84
__NR_sync_file_range 84
__NR_timerfd_create 85
__NR_timerfd_settime 86
__NR_timerfd_gettime 87
__NR_utimensat 88
__NR_acct 89
__NR_capget 90
__NR_capset 91
__NR_personality 92
__NR_exit 93
__NR_exit_group 94
__NR_waitid 95
__NR_set_tid_address 96
__NR_unshare 97
__NR_futex 98
__NR_set_robust_list 99
__NR_get_robust_list 100
__NR_nanosleep 101
__NR_getitimer 102
__NR_setitimer 103
__NR_kexec_load 104
__NR_init_module 105
__NR_delete_module 106
__NR_timer_create 107
__NR_timer_gettime 108
__NR_timer_getoverrun 109
__NR_timer_settime 110
__NR_timer_delete 111
__NR_clock_settime 112
__NR_clock_gettime 113
__NR_clock_getres 114
__NR_clock_nanosleep 115
__NR_syslog 116
__NR_ptrace 117
__NR_sched_setparam 118
__NR_sched_setscheduler 119
__NR_sched_getscheduler 120
__NR_sched_getparam 121
__NR_sched_setaffinity 122
__NR_sched_getaffinity 123
__NR_sched_get_priority_max 125
__NR_sched_get_priority_min 126
__NR_sched_rr_get_interval 127
__NR_restart_syscall 128
__NR_kill 129
__NR_tkill 130
__NR_tgkill 131
__NR_sigaltstack 132
__NR_rt_sigsuspend 133
__NR_rt_sigaction 134
__NR_rt_sigprocmask 135
__NR_rt_sigpending 136
__NR_rt_sigtimedwait 137
__NR_rt_sigqueueinfo 138
__NR_rt_sigreturn 139
__NR_setpriority 140
__NR_getpriority 141
__NR_reboot 142
__NR_setregid 143
__NR_setgid 144
__NR_setreuid 145
__NR_setuid 146
__NR_setresuid 147
__NR_getresuid 148
__NR_setresgid 149
__NR_getresgid 150
__NR_setfsuid 151
__NR_setfsgid 152
__NR_times 153
__NR_setpgid 154
__NR_getpgid 155
__NR_getsid 156
__NR_setsid 157
__NR_getgroups 158
__NR_setgroups 159
__NR_uname 160
__NR_sethostname 161
__NR_setdomainname 162
__NR_getrlimit 163
__NR_setrlimit 164
__NR_getrusage 165
__NR_umask 166
__NR_prctl 167
__NR_getcpu 168
__NR_gettimeofday 169
__NR_settimeofday 170
__NR_adjtimex 171
__NR_getppid 173
__NR_getuid 174
__NR_geteuid 175
__NR_getgid 176
__NR_getegid 177
__NR_gettid 178
__NR_sysinfo 179
__NR_mq_open 180
__NR_mq_unlink 181
__NR_mq_timedsend 182
__NR_mq_timedreceive 183
__NR_mq_notify 184
__NR_mq_getsetattr 185
__NR_msgget 186
__NR_msgctl 187
__NR_msgrcv 188
__NR_msgsnd 189
__NR_semget 190
__NR_semctl 191
__NR_semtimedop 192
__NR_semop 193
__NR_shmget 194
__NR_shmctl 195
__NR_shmat 196
__NR_shmdt 197
__NR_socket 198
__NR_socketpair 199
__NR_bind 200
__NR_listen 201
__NR_accept 202
__NR_connect 203
__NR_getsockname 204
__NR_getpeername 205
__NR_sendto 206
__NR_recvfrom 207
__NR_setsockopt 208
__NR_getsockopt 209
__NR_shutdown 210
__NR_sendmsg 211
__NR_recvmsg 212
__NR_readahead 213
__NR_brk 214
__NR_munmap 215
__NR_mremap 216
__NR_add_key 217
__NR_request_key 218
__NR_keyctl 219
__NR_clone 220
__NR_execve 221
__NR3264_mmap 222
__NR3264_fadvise64 223
__NR_swapon 224
__NR_swapoff 225
__NR_mprotect 226
__NR_msync 227
__NR_mlock 228
__NR_munlock 229
__NR_mlockall 230
__NR_munlockall 231
__NR_mincore 232
__NR_madvise 233
__NR_remap_file_pages 234
__NR_mbind 235
__NR_get_mempolicy 236
__NR_set_mempolicy 237
__NR_migrate_pages 238
__NR_move_pages 239
__NR_rt_tgsigqueueinfo 240
__NR_perf_event_open 241
__NR_accept4 242
__NR_recvmmsg 243
__NR_arch_specific_syscall 244
__NR_wait4 260
__NR_prlimit64 261
__NR_fanotify_init 262
__NR_fanotify_mark 263
__NR_name_to_handle_at         264
__NR_open_by_handle_at         265
__NR_clock_adjtime 266
__NR_syncfs 267
__NR_setns 268
__NR_sendmmsg 269
__NR_process_vm_readv 270
__NR_process_vm_writev 271
__NR_kcmp 272
__NR_finit_module 273
__NR_sched_setattr 274
__NR_sched_getattr 275
__NR_renameat2 276
__NR_seccomp 277
__NR_getrandom 278
__NR_memfd_create 279
__NR_bpf 280
__NR_execveat 281
__NR_userfaultfd 282
__NR_membarrier 283
__NR_mlock2 284
__NR_copy_file_range 285
__NR_preadv2 286
__NR_pwritev2 287
__NR_pkey_mprotect 288
__NR_pkey_alloc 289
__NR_pkey_free 290
__NR_statx 291
__NR_io_pgetevents 292
__NR_rseq 293
__NR_kexec_file_load 294
__NR_clock_gettime64 403
__NR_clock_settime64 404
__NR_clock_adjtime64 405
__NR_clock_getres_time64 406
__NR_clock_nanosleep_time64 407
__NR_timer_gettime64 408
__NR_timer_settime64 409
__NR_timerfd_gettime64 410
__NR_timerfd_settime64 411
__NR_utimensat_time64 412
__NR_pselect6_time64 413
__NR_ppoll_time64 414
__NR_io_pgetevents_time64 416
__NR_recvmmsg_time64 417
__NR_mq_timedsend_time64 418
__NR_mq_timedreceive_time64 419
__NR_semtimedop_time64 420
__NR_rt_sigtimedwait_time64 421
__NR_futex_time64 422
__NR_sched_rr_get_interval_time64 423
__NR_pidfd_send_signal 424
__NR_io_uring_setup 425
__NR_io_uring_enter 426
__NR_io_uring_register 427
__NR_open_tree 428
__NR_move_mount 429
__NR_fsopen 430
__NR_fsconfig 431
__NR_fsmount 432
__NR_fspick 433
__NR_pidfd_open 434
__NR_clone3 435
__NR_close_range 436
__NR_openat2 437
__NR_pidfd_getfd 438
__NR_faccessat2 439
__NR_process_madvise 440
__NR_epoll_pwait2 441
__NR_mount_setattr 442
__NR_landlock_create_ruleset 444
__NR_landlock_add_rule 445
__NR_landlock_restrict_self 446

#if __BITS_PER_LONG == 64 && !defined(__SYSCALL_COMPAT)
#define __NR_fcntl __NR3264_fcntl
#define __NR_statfs __NR3264_statfs
#define __NR_fstatfs __NR3264_fstatfs
#define __NR_truncate __NR3264_truncate
#define __NR_ftruncate __NR3264_ftruncate
#define __NR_lseek __NR3264_lseek
#define __NR_sendfile __NR3264_sendfile
#if defined(__ARCH_WANT_NEW_STAT) || defined(__ARCH_WANT_STAT64)
#define __NR_newfstatat __NR3264_fstatat
#define __NR_fstat __NR3264_fstat
#endif
#define __NR_mmap __NR3264_mmap
#define __NR_fadvise64 __NR3264_fadvise64
#ifdef __NR3264_stat
#define __NR_stat __NR3264_stat
#define __NR_lstat __NR3264_lstat
#endif
#else
#define __NR_fcntl64 __NR3264_fcntl
#define __NR_statfs64 __NR3264_statfs
#define __NR_fstatfs64 __NR3264_fstatfs
#define __NR_truncate64 __NR3264_truncate
#define __NR_ftruncate64 __NR3264_ftruncate
#define __NR_llseek __NR3264_lseek
#define __NR_sendfile64 __NR3264_sendfile
#if defined(__ARCH_WANT_NEW_STAT) || defined(__ARCH_WANT_STAT64)
#define __NR_fstatat64 __NR3264_fstatat
#define __NR_fstat64 __NR3264_fstat
#endif
#define __NR_mmap2 __NR3264_mmap
#define __NR_fadvise64_64 __NR3264_fadvise64
#ifdef __NR3264_stat
#define __NR_stat64 __NR3264_stat
#define __NR_lstat64 __NR3264_lstat
#endif
#endif


 */
