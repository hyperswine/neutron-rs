// -------------------
// ARCH DEPENDENT
// -------------------

// How to use drivers:
// compile neutron kernel with all drivers enabled
// load a bsp on the fly when you confirm what device you are using

// IDK if its better to store stuff like spectro under riscv/ and pi under aarch64/
// Maybe just leave it for now since it works and kinda makes sense

// GENERIC ARM DRIVERS
pub mod arm;

#[cfg(target_arch = "aarch64")]
pub mod pi4b;

#[cfg(target_arch = "aarch64")]
pub mod rk3399;

// GENERIC RISCV DRIVERS
pub mod riscv;

#[cfg(target_arch = "riscv64")]
pub mod spectro;

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

// Device types are pretty much inspired by linux

// ----------------
// PCI DEVICES
// ----------------

// Here https://docs.oracle.com/cd/E37670_01/E52461/html/ch06s04.html

// Usually an extra layer of comms or simply just PCI is enough
// Can implement several traits, PCI + Char for example
// where the char will have to also go through the PCI details

const MAX_PCI_NODE_CHILDREN: u8 = 12;

// PCI (not PCIe)
pub struct PCIBus {
    name: [u8; 48],
    parent: *mut PCIBus,
    children: [*mut PCIBus; MAX_PCI_NODE_CHILDREN as usize],
    bus_number: u8,
    // primary, secondary
    bridge_numbers: [u8; 2],
    max_bus_speed: u8,
    cur_bus_speed: u8,
}

pub struct PCISlot {}

// should be kept by NeutronDevice as NeutronPCIDevice or NeutronPCIeDevice
struct PCIDevice {}

struct PCIeDevice {}

struct NeutronPCIDevice {
    pci_device: PCIDevice,
}

struct NeutronPCIeDevice {
    pcie_device: PCIeDevice,
}

// ----------------
// CHAR DEVICES
// ----------------

// useful abstractions for kernel subsystems and certain device types
// for char devices like KB and MICE
// generic, spectro and rk3399 drivers should implement traits like this
pub trait CharDeviceFunctions<Data> {
    // BASIC INIT
    fn dev_open(&self);
    // used mainly for shutdown and sleep (S0x, S3)
    fn dev_close(&self);
    // if unplugged and shutdown
    fn dev_release(&mut self);

    // DATA
    fn dev_llseek(&mut self);
    fn dev_read(&self, data: Data);
    fn dev_write(&self, data: Data);

    // SPECIAL -> driver ext
}

// ----------------
// USB DEVICES
// ----------------

// Mostly a char like thing where you transfer blocks of data in parallel
// To and from the USB controller
pub trait USBDeviceFunctions {
    // INIT
    fn probe(&mut self);
    fn disconnect(&mut self);
    fn resume(&mut self);
    // pretty important for a well functioning system
    fn suspend(&mut self);
    fn reset(&mut self);

    // DATA
    fn transfer_to(&mut self, data: *const u8, size_bytes: u64);
    fn transfer_from(&mut self, kbuf: *const u8, size_bytes: u64);
}

// ----------------
// STORAGE DEVICES
// ----------------

pub const LBA_SIZE_BYTES: u64 = 4192;

// Basically Block Devices in UNIX world
// any SSD, HDD, USB-plugged in mass storage device
// transfers fixed blocks of data (4KiB) back and forths rather than a stream
// Should require kernel and user buffers as well as microcontroller drivers
// unlike char devices
pub trait StorageDeviceFunctions<Data> {
    // BASIC INIT
    fn dev_open(&self);
    // hot unplug -> uefi support as well
    fn dev_release(&mut self);

    // DATA
    fn dev_read(&self, data: Data);
    fn dev_write(&self, data: Data);

    // SPECIAL -> driver ext
}

// ----------------
// NETWORK DEVICES
// ----------------

// driver ext -> needs to be implemented by the specific driver
// struct NetworkSpecialFunction {
//     function: String,
// }

// Ethernet adapaters, Wifi, BT
pub trait NetworkDeviceFunctions<Data> {
    // BASICS
    fn dev_init(&self);
    fn dev_stop(&self);
    fn dev_open(&self);
    fn dev_close(&self);

    // CONFIG
    fn dev_set_mac_addr(&mut self);
    fn dev_set_timeout(&mut self);

    // DATA PASSING BACK AND FORTH
    fn dev_read(&self, data: Data);
    fn dev_write(&self, data: Data);

    // EXTENDED / SPECIAL -> basically ioctl, only in driver ext
    // fn dev_special_functionality(&self, functions: &[NetworkSpecialFunction]);
}

// ----------------
// GRAPHICS DEVICES
// ----------------

// Based on VESA Standards
// Basically a more complex version of a char driver
pub trait GraphicsDeviceFunctions<Data> {
    // BASICS
    fn dev_init(&self);
    fn dev_stop(&self);
    fn dev_open(&self);
    fn dev_close(&self);

    // FEATURES & CONFIG OPTIONS
    fn dev_hdr_settings(&mut self);
    fn dev_monitor_control_commands(&mut self);
    fn dev_channel_settings(&mut self);

    // OUTPUT
    fn dev_port_output(&mut self);

    // DATA PASSING BACK AND FORTH
    fn dev_read(&self, data: Data);
    fn dev_write(&self, data: Data);

    // EXTENDED / SPECIAL -> basically ioctl, only in driver ext
    // fn dev_special(&self);
}

// -------------------
// Driver Management
// -------------------

// Usually, when a driver is loaded, it shouldnt be unloaded during the runtime of the kernel
// even if the device is unplugged, and no devices are using the driver
// though driverd could apply some heuristics to handle it, just not here

extern crate alloc;
use alloc::string::String;
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
