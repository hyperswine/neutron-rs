// Define syscall ABI which kernel sets up in the interrupt table
// sparx/arch/riscv64/asm/syscalls.S contains the API which programs can link to
// Assuming: Single user, etc. For multi user, use separate partitions
// NO SYMLINKS, NO INCOMPLETE SYSCALLS/BASIC VERSIONS

// File: READ, WRITE, OPEN, CLOSE, MKDIR, RMDIR, DUP, UTIME, STAT, LSEEK
// Process: FORK, EXEC, KILL, NICE, WAITPID, GETPID, CHDIR, CWD
// System: GETTIME, UNAME, PUTENV

// -----------
// COMMON STRUCTS
// -----------

pub enum ArcSignal {}
pub struct HFSStat {}

pub enum ServiceErrorCode {}

use crate::types::KTimestamp;

// -----------
// FILE
// -----------

type FileDescriptor = u64;

pub enum RelativeFilePosition {
    START,
    CURRENT,
    END,
}

use alloc::string::String;

pub trait NeutronFileService {
    fn open(file: &str, flags: u64, mode: u64) -> Option<FileDescriptor>;

    fn close(fd: FileDescriptor) -> ServiceErrorCode;

    fn read(fd: u64, n_bytes: u64) -> Option<String>;

    fn write(fd: u64, _str: &str) -> ServiceErrorCode;

    fn mkdir(path: &str, mode: u64) -> ServiceErrorCode;

    fn rmdir(path: &str) -> ServiceErrorCode;

    fn duplicate(fd: u64) -> ServiceErrorCode;

    fn utime(path: &str, access_time: KTimestamp, modify_time: KTimestamp) -> ServiceErrorCode;

    fn stat(path: &str) -> Option<HFSStat>;

    fn lseek(fd: FileDescriptor, pos: i64, relative_to: RelativeFilePosition) -> ServiceErrorCode;
}

// -----------
// PROCESS
// -----------

type ProcessID = u64;

pub trait NeutronProcessService {
    // create a new process using the currrent one as a template
    fn fork() -> Option<ProcessID>;

    fn exec();

    fn kill(pid: u64, signal: ArcSignal) -> ServiceErrorCode;

    fn nice(increment: i32) -> ServiceErrorCode;

    fn waitpid();

    fn get_pid();

    fn chdir();

    fn cwd();
}

// -----------
// SYSTEM
// -----------

pub trait NeutronSystemService {
    // Seconds since last call -> user time, system time, child user, child system, elapsed real time
    fn times() -> (f32, f32, f32, f32, f32);

    // Seconds since Jan 1, 1970 at 12:00AM. Should sync to some network time
    fn gettime() -> f64;

    // sysname, nodename, release, version, machine
    fn uname() -> (String, String, String, String, String);

    fn putenv(varname: &str, value: &str) -> ServiceErrorCode;
}
