pub mod syscall;

// -------------
// INTERFACE
// -------------

type Descriptor = u64;
type FileDescriptor = Descriptor;
type SocketDescriptor = Descriptor;

pub enum ErrNo {
    READ_VALID,
}

struct ServiceStatus {
    errno: ErrNo,
    status: i8,
}

#[test]
fn test_basic_mounting() {
    struct Device;
    // create a device
    let device = Device;
    // mount(0, "/dev/default");
}
