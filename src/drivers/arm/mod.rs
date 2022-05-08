pub mod bcm;
pub mod common;
pub mod gicv2;

// -------------------------
// Driver interfaces for ARM
// -------------------------

pub trait DeviceDriver {
    fn compatible(&self) -> &'static str;

    unsafe fn init(&self) -> Result<(), &'static str> {
        Ok(())
    }

    fn register_and_enable_irq_handler(&'static self) -> Result<(), &'static str> {
        Ok(())
    }

    fn virt_mmio_start_addr(&self) -> Option<usize> {
        None
    }
}

pub trait DriverManager {
    fn all_device_drivers(&self) -> &[&'static (dyn DeviceDriver + Sync)];

    fn early_print_device_drivers(&self) -> &[&'static (dyn DeviceDriver + Sync)];

    /// Return all drivers minus early-print drivers.
    fn non_early_print_device_drivers(&self) -> &[&'static (dyn DeviceDriver + Sync)];

    /// Initialization code that runs after the early print driver init.
    fn post_early_print_device_driver_init(&self);
}

// -------------------------
// State for ARM CPUs
// -------------------------

// State information about the kernel itself.

use core::sync::atomic::{AtomicU8, Ordering};

// ----------------
// Private Definitions
// ----------------

/// Different stages in the kernel execution.
#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Init,
    SingleCoreMain,
    MultiCoreMain,
}

/// Maintains the kernel state and state transitions.
pub struct StateManager(AtomicU8);

// ----------------
// Global instances
// ----------------

static STATE_MANAGER: StateManager = StateManager::new();

/// Return a reference to the global StateManager.
pub fn state_manager() -> &'static StateManager {
    &STATE_MANAGER
}

impl StateManager {
    const INIT: u8 = 0;
    const SINGLE_CORE_MAIN: u8 = 1;
    const MULTI_CORE_MAIN: u8 = 2;

    /// Create a new instance.
    pub const fn new() -> Self {
        Self(AtomicU8::new(Self::INIT))
    }

    /// Return the current state.
    fn state(&self) -> State {
        let state = self.0.load(Ordering::Acquire);

        match state {
            Self::INIT => State::Init,
            Self::SINGLE_CORE_MAIN => State::SingleCoreMain,
            Self::MULTI_CORE_MAIN => State::MultiCoreMain,
            _ => panic!("Invalid KERNEL_STATE"),
        }
    }

    /// Return if the kernel is init state.
    pub fn is_init(&self) -> bool {
        self.state() == State::Init
    }

    /// Transition from Init to SingleCoreMain.
    pub fn transition_to_single_core_main(&self) {
        if self
            .0
            .compare_exchange(
                Self::INIT,
                Self::SINGLE_CORE_MAIN,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_err()
        {
            panic!("transition_to_single_core_main() called while state != Init");
        }
    }
}
