// FOR POSIX IMPLEMENTATION

#[no_mangle]
extern "C" fn open() {}

#[no_mangle]
extern "C" fn close() {}

#[no_mangle]
extern "C" fn read() {}

#[no_mangle]
extern "C" fn write() {}

#[no_mangle]
extern "C" fn exec() {}

#[no_mangle]
extern "C" fn fork() {}

#[no_mangle]
extern "C" fn exit() {}

#[no_mangle]
extern "C" fn gettime() {}
