pub mod syscall;
pub mod time;

// -------------
// INTERFACE
// -------------

// use UNIX like constructs for file descriptors (Descriptor)

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

// ---------
// TEST
// ---------

#[test]
fn test_basic_mounting() {
    // ? represent a device in drivers. A driver should correspond to a device type
    // A device type can be either generic (GenericDevice) or specific (DeviceSpec)
    struct Device;
    // create a device
    let device = Device;
    // mount(0, "/dev/default");
}
