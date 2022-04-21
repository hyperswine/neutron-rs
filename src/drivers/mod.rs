// -------------------
// ARCH DEPENDENT
// -------------------

// Allow space for optimisation for a more optimal user xp

#[cfg(target_arch = "riscv64")]
pub mod spectro;

#[cfg(target_arch = "aarch64")]
pub mod pi4b;

#[cfg(target_arch = "aarch64")]
pub mod rk3399;

// -------------------
// NON ARCH DEPENDENT
// -------------------

// Mostly the device abstraction layer

// -------------------
// Neutron Device Tree
// -------------------

// Basically a better HAL

use alloc::collections::btree_map::BTreeMap;

type DeviceKey = u64;

struct NeutronDeviceTree {
    size: usize,
    tree: BTreeMap<DeviceKey, NeutronDevice>,
}

impl NeutronDeviceTree {
    fn new(size: usize, tree: BTreeMap<DeviceKey, NeutronDevice>) -> Self {
        Self { size, tree }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum DeviceType {
    Keyboard,
    Mouse,
    Monitor,
    MassStorage,
    Headset,
    Headphones,
    Speakers,
    Mic,
    EthernetAdapter,
    WifiAdapter,
    BTAdapter,
    NetworkAdapter,
}

struct NeutronDevice {
    dev_id: u64,
    dev_type: DeviceType,
}

// useful abstractions for kernel subsystems and certain device types
// for char devices
pub trait CharDeviceFunctions<Data> {
    fn dev_open(&self);
    fn dev_close(&self);
    fn dev_read(&self, data: Data);
    fn dev_write(&self, data: Data);
}

// -------------------
// Driver Registration
// -------------------

extern crate alloc;
use alloc::vec::Vec;

// A generic driver class
// can implement char device functionality, network functionality, block functionality, etc.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct Driver {
    driver_id: u64,
    device_type: DeviceType,
}

// Should be forwarded to KernelManager to observe the device tree
struct DriverManager {
    loaded_drivers: Vec<Driver>,
}

impl DriverManager {
    fn new(loaded_drivers: Vec<Driver>) -> Self {
        Self { loaded_drivers }
    }

    // Load a driver if it isnt loaded already
    fn load_driver(&mut self, driver: Driver) {
        let res = self.loaded_drivers.iter().find(|d| d == d);
        match res {
            Some(_) => return,
            None => {
                self.loaded_drivers.push(driver);
            }
        }
    }

    // check device tree for drivers that can be loaded for them
    // the device type must match. And if the id also matches, use that one
    // if no driver_id and dev_ids match, then load all possible drivers of that type and test them on the device. If device does not respond in an expected way, dont load them (Neutron Principle: Be very specific)

}

// most should be non arch dependent, e.g., Generic Mouse, Generic KB, Generic Headphones, Generic Mic
// implementations for those devices
pub mod generic;
