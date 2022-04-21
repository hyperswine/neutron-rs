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

use alloc::borrow::ToOwned;
use alloc::collections::btree_map::BTreeMap;

type DeviceKey = u64;

pub struct NeutronDeviceTree {
    size: usize,
    tree: BTreeMap<DeviceKey, NeutronDevice>,
}

impl NeutronDeviceTree {
    pub fn new(size: usize, tree: BTreeMap<DeviceKey, NeutronDevice>) -> Self {
        Self { size, tree }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum DeviceType {
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

// all devices plugged in are considered neutron devices
// it is the drivermanager's job to load the most compatible driver to
// expose its MMIO API as a char/block/network API for other kernel subsystems
pub struct NeutronDevice {
    dev_id: u64,
    dev_type: DeviceType,
    // a device with the same ddid as the driver's driver_id is 100% compatible
    device_driver_id: u64,
}

impl NeutronDevice {
    pub fn new(dev_id: u64, dev_type: DeviceType, device_driver_id: u64) -> Self {
        Self {
            dev_id,
            dev_type,
            device_driver_id,
        }
    }

    pub fn device_driver_id(&self) -> u64 {
        self.device_driver_id
    }

    pub fn dev_id(&mut self) -> &mut u64 {
        &mut self.dev_id
    }

    pub fn dev_type(&self) -> DeviceType {
        self.dev_type
    }
}

// useful abstractions for kernel subsystems and certain device types
// for char devices
// generic, spectro and rk3399 drivers should implement traits like this
pub trait CharDeviceFunctions<Data> {
    fn dev_open(&self);
    fn dev_close(&self);
    fn dev_read(&self, data: Data);
    fn dev_write(&self, data: Data);
}

// -------------------
// Driver Management
// -------------------

extern crate alloc;
use alloc::vec::Vec;

// A generic driver class
// can implement char device functionality, network functionality, block functionality, etc.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct NeutronDriver {
    driver_id: u64,
    device_type: DeviceType,
}

impl NeutronDriver {
    pub fn new(driver_id: u64, device_type: DeviceType) -> Self {
        Self {
            driver_id,
            device_type,
        }
    }

    pub fn driver_id(&self) -> u64 {
        self.driver_id
    }

    pub fn device_type(&self) -> DeviceType {
        self.device_type
    }
}

#[inline]
pub fn test_driver_on_device(driver: &NeutronDriver, device: &NeutronDevice) -> bool {
    // TODO: try testing the device with the driver using the driver's implemented functions dev_read, etc. and look at the expected outputs with the trait ExpectedDeviceFunctionality

    false
}

// Should be forwarded to KernelManager to observe the device tree
pub struct DriverManager {
    // list of possible drivers on the system that can be loaded at will
    registered_drivers: Vec<NeutronDriver>,
    loaded_drivers: Vec<NeutronDriver>,
}

impl DriverManager {
    pub fn new(registered_drivers: Vec<NeutronDriver>, loaded_drivers: Vec<NeutronDriver>) -> Self {
        Self {
            registered_drivers,
            loaded_drivers,
        }
    }

    // Load a driver if it isnt loaded already
    pub fn load_driver(&mut self, driver: NeutronDriver) {
        let res = self
            .loaded_drivers
            .iter()
            .find(|d| d.driver_id() == driver.driver_id());
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
    pub fn load_compatible_driver(&mut self, device: NeutronDevice) {
        let __d = match self
            .registered_drivers
            .iter()
            .find(|d| d.driver_id() == device.device_driver_id())
        {
            Some(driver) => {
                // 100% compatible driver exists for device, load if not loaded already
                driver
            }
            None => {
                // cant find a fully supported driver, test manually
                let mut res = self
                    .registered_drivers
                    .iter()
                    .filter(|d| d.device_type() == device.dev_type());

                let _driver = res.find(|r| test_driver_on_device(r, &device));

                match _driver {
                    Some(d) => d,
                    None => return,
                }
            }
        };

        self.load_driver(__d.to_owned());
    }

    // register a driver if it isnt registered already
    // drivers to be registers should be stored in code within generic/ and etc
    // and be registered there
    pub fn register_driver(&mut self, driver: NeutronDriver) {
        let res = self
            .registered_drivers
            .iter()
            .find(|d| d.driver_id() == driver.driver_id());
        match res {
            Some(_) => return,
            None => {
                self.registered_drivers.push(driver);
            }
        }
    }
}

// most should be non arch dependent, e.g., Generic Mouse, Generic KB, Generic Headphones, Generic Mic
// implementations for those devices
pub mod generic;
