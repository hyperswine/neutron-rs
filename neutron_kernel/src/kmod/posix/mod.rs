// FOR POSIX IMPLEMENTATION
// Based on https://docs.oracle.com/cd/E19048-01/chorus4/806-3328/6jcg1bm05/index.html
// And https://sceweb.sce.uhcl.edu/helm/WEBPAGE-Python/documentation/python_tutorial/lib/module-posix.html
// Only implements most common ones for rust-std

// REQUIRES IMPORTING FROM EACH ARCH

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

enum POSIX_SIGNAL {}

struct POSIX_STAT {}

#[no_mangle]
extern "C" fn stat(path: &str) -> POSIX_STAT {}

#[no_mangle]
extern "C" fn nice(increment: i32) -> u64 {}

#[no_mangle]
extern "C" fn kill(pid: u64, signal: POSIX_SIGNAL) -> u64 {}

#[no_mangle]
extern "C" fn pipe() -> (u64, u64) {}

#[no_mangle]
extern "C" fn gettimeofday() -> u64 {}
