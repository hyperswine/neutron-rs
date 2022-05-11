// ---------
// ACPI POWER MANAGER
// ---------

// ACPI s, p, g state management. A loaded device driver by KernelManager should use these functions to control the device states

enum Sx_State {
    S0,
    S0x,
    S1,
    S2,
    S3,
    S4,
    S5,
}

enum Gx_State {
    G0,
    G1,
    G2,
    G3,
    G4,
    G5,
}

enum Px_State {
    P0,
    P1,
    P2,
    P3,
}

pub trait ACPIDevice {
    fn set_device_S_state();
    // should be set first and prioritised
    fn set_device_G_state();
    fn set_device_P_state();
}
