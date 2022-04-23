// FOR SPECTRO GPU
// CONTROL THE SPs, SMs, CONTROLLER, DISPATCH, VRAM ALLOCATION AND USAGE

// GPU SHOULD OUTPUT BYTESTREAM TO DISPLAY MONITOR
// FIRMWARE HANDLES THE REST, DRIVERS HANDLE TRANSLATIONS AND CALLS

// ------------------
// FRAMEBUFFER DRIVER
// ------------------

// Simply query the number of pixels and change each accordingly to their (x,y) rgba component
struct FramebufferDriver;

// possible to rasterise a png by Sample2D

// ------------------
// 3D DRIVER
// ------------------

struct GraphicsDriver3D;

// TODO: Implement GraphicsDriverFunctions

// Initialise GPU for the kernel. After bootloader, will need to be able to crunch a bunch of dynamic info
// Show a console by default
fn open() {}

// Does nothing for now
fn close() {}

// Read a value from an SM
// Vulkan -> Framebuffer in VRAM
fn read() {}

// Vulkan -> Write vertex data to VRAM
fn write() {}

// a framebuffer to be used/reused
struct Framebuffer;

// Output Framebuffer
fn output_framebuffer() -> Framebuffer {
    Framebuffer {}
}

#[test_case]
fn test_framebuffer() {
    output_framebuffer();
}

// a vulkan source code
// compile to a gpu program
// load into VRAM and start executing
