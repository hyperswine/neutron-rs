// COMMON

type FileDescriptor = u64;

pub enum RelativeFilePosition {
    START,
    CURRENT,
    END,
}

type ProcessID = u64;

// --------------
// File Ops
// --------------

pub trait NeutronFileServices {}

// --------------
// Device Ops
// --------------

pub trait NeutronDeviceServices {}

// --------------
// Communication Ops
// --------------

pub trait NeutronCommunicationServices {}

// --------------
// Process Ops
// --------------

pub trait NeutronProcessServices {}

// --------------
// System Ops
// --------------

pub trait NeutronSystemServices {}
