// Define syscall ABI which kernel sets up in the interrupt table
// sparx/arch/riscv64/asm/syscalls.S contains the API which programs can link to
// Assuming: Single user, etc. For multi user, use separate partitions
// NO SYMLINKS, NO INCOMPLETE SYSCALLS/BASIC VERSIONS

// File: READ, WRITE, OPEN, CLOSE, MKDIR, RMDIR, DUP, UTIME, STAT, LSEEK
// Process: FORK, EXEC, KILL, NICE, WAITPID, GETPID, CHDIR, CWD
// System: GETTIME, UNAME, PUTENV

// -----------
// FILE
// -----------

pub trait NeutronSyscall {
    #[no_mangle]
extern "C" fn open(file: &str, flags: u64, mode: u64) -> u64 {}

#[no_mangle]
extern "C" fn close(fd: u64) -> u64 {}

#[no_mangle]
extern "C" fn read(fd: u64, n_bytes: u64) -> u64 {}

#[no_mangle]
extern "C" fn write(fd: u64, _str: &str) -> u64 {}

#[no_mangle]
extern "C" fn write(fd: u64, _str: &str) -> u64 {}

#[no_mangle]
extern "C" fn remove(path: &str) -> u64 {}

#[no_mangle]
extern "C" fn mkdir(path: &str, mode: u64) -> u64 {}

#[no_mangle]
extern "C" fn rmdir(path: &str) -> u64 {}

struct POSIX_ENV;

#[no_mangle]
extern "C" fn execve(path: &str, args: &[&str], env: &POSIX_ENV) -> u64 {}

#[no_mangle]
extern "C" fn fork() -> u64 {}

#[no_mangle]
extern "C" fn stat(path: &str) -> POSIX_STAT {}

#[no_mangle]
extern "C" fn nice(increment: i32) -> u64 {}

#[no_mangle]
extern "C" fn kill(pid: u64, signal: POSIX_SIGNAL) -> u64 {}

#[no_mangle]
extern "C" fn gettimeofday() -> u64 {}

}
