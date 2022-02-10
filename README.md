# Neutron
A rust based kernel built on first principles. Principles called the SPARX principles by yours truly.

## SPARX Principles?
Just a way to say "Shoddy, Pathetic, Abomination, Repetitive, eXceptionally horrendous" to express how software is written.

## Layout
- build/ -> for any temporary build files
- src/ -> for 99% of the kernel logic
- sparx/ -> for 1% of the logic including asm, link scripts, external deps (non-rust), build configs for arcboot, useful scripts
- tests/ -> cargo integration tests
- arctests/ -> arctest acceptance tests. #[feature = arctest] are technically system tests even though they are localised, at least for now

# Testing
A core idea is TDD. When in doubt, test.

- To run cargo tests, `cargo t` which builds the library for the host target and runs the `#[test]` functions.
- To run vm tests, `arcboot test` builds the a complete image with `feature = arctest` and a custom test harness. It basically runs `rustc test --no-run` for either the spectro/pi target. Then it runs `arcboot run` which boots the vm and loads the image, running the kernel with an arctest config instead of the usual config.

## Cargo Tests
- Mostly for the specific functions (local #[test] in each file) and functions together (things in tests/)
- NOTE: `tests/` is for cargo only and `#[test]` is also for cargo only. Arctest only relies on `feature = arctest` and files in `arctest/`
## Arctest
- Great for validation, blackbox and acceptance testing. Basically any high level stuff that you cant do directly with `cargo t`
- `arcboot test` -> not yet working but will be great for system testing and blackbox testing

# Dependencies
Rust (rustup recommended)
 - rust-src
 - target aarch64 and riscv64 (unknown-none)
 - arcboot
 - spectrovm
Toolchains (add to path or specify sysroot when using `cargo build`)
 - aarch64-gcc
 - riscv64-gcc

## Minimal Config
Since this is a multi target kind of thing in rust, we get a whole bunch of issues if we try to do it the standard way. Recommended to disable any language servers since they can spasm really hard. Maybe theres a way to configure it nicely but Idk I dont really wanna to configure VSCode too much.
- This means things like `.cargo/config.toml` should be very minimal. Mostly for cool things like aliases and stuff. Dont specify any main configs. You can do `[dependencies.X]` for X if you want but I rather leave it mostly vanila and rely on `arcboot` for more complex config and functionality
- Mostly using rust, the language itself and the cargo package management and test suite. I dont really care about the other stuff, at least for now.

# Spectral Graphics ISA
Based on RV64GC
- RISC, with mostly `add, sub, mult, div` on 2 registers
- Mostly optimised for high FLOPs -> FP32 and FP16 `add, sub, mult, div`. No FP64
- For tensor ops, can act on 4x4 matrices, i.e. 32 registers in one instruction
- Ability to branch and jump to any address in the GPU shader code
- Load/Store, A whole bunch of L/S units. So load byte, half, word
- No immediates. Dont need them
- Vectors -> basically riscv V extension. Very important for performance. Apparently Quad vectors (multiplying 4 vectors) can be implemented on ALU
- Special functions -> Trig, Inverse Fraction, Square Root, Logarithm, Power, Inverse Matrix
- Texture instructions -> Sample2D(*texture_image, coords) -> RGBA

GPU needs to be able to read from VRAM and store previously done calculations in L1/2 caches. No TLB, paging or interrupts.
- DMA is available to transfer directly from RAM -> VRAM and VRAM -> RAM. Most useful for GPU code to access RAM directly and load/store in its own registers
- In standard case, CPU sends an instruction to GPU to fetch directly from RAM at (offset, n_bytes) and copy it to (vram_offset), overwriting any existing data
- For something like glDrawElements, data should already be in VRAM. Should be placed there with glCreateBuffer and glBufferData. Driver should tell GPU to DMA the vertex data from RAM to its own VRAM and keep pointers/structural views of it in VRAM (some 0x0-0x60000 vertex table)
- When GPU shader program is told to start the pipeline, i.e. glDrawElements, it will consult the table about the vertices and start to manipulate them in each SP. The `main()` shader function can be called by each SP. The uniforms are stored in L2 cache and fetched on demand. In the local context `main()` local vars and stuff may need to be created. VRAM has its own stack and identity mapped (no paging) for each local var and function call frame. The uniforms that dont fit in Data cache are stored in the heap like all the other uniforms. Same for instructions
- The driver sits at the interface between vulkan and the GPU opcodes/ISA. Unlike CPU where you have to manually optimise your assembly code, a lot of the vulkan/opengl calls are streamlined and so are the ISA calls to do them. Multiple GPU instruction calls may be needed for a single 'driver/vulkan call'. If you are doing primitive assembly, instructions like `load`, `mult`, `multmatrix` may be called. In rasterisation, `load, mult, clip` may be called. The compiler should compile the vulkan c/c++ code into the right mmio calls to the graphics driver manager. The graphics driver code should then call another set of mmio code that tells the GPU exactly what GPU ISA instructions to execute sequentially
- For each stage of the pipeline, can separate the graphics driver ABI to handle them. Fragment shader stage is usually the most intensive, as texture sampling and other effects like lighting, shadows, etc are usually done here. If done in the vertex shader you can expect some less fidelity. So fragment shader driver code should be of the most importance.

## Instructions

| instruction | details |
| --- | --- |
| add | src_reg1, src_reg2, dst_reg |

## Driver code
The graphics driver is probably the most vital driver imo. A lot of work seems to be done to make them more efficient, featureful, less buggy and low latency. Should run in Ring-1. Would prob be fine in Ring-0 but some extra caution seems fine.

A lot of the driver code will simply make mmio calls to the GPU with all 20 of its instructions mapped to RAM. Hardware MMIO controller should be able to direct these to the GPU with low latency. Though may incur high bus traffic.

```rei
# for drawElements
gpu_draw_elem(shader_program_id) = {
    # tells the GPU to start executing a shader program with `id`. Note the shader program should have already been transferred to the GPU
    # after the GPU has completed its execution of the program, it should place the resulting framebuffer in 0x80000000
    # then when glClearBuffer is called, the GPU is told GPU_INSTR::CLEAR_BUFFER, and should write the framebuffer as an output stream to the monitor
    # OS WM and stuff should have control over the entire screen, and make mini screens/framebuffers for app windows to render in. Usually that means a glDrawElements and a glClearBuffer would actually clear it to a specified area or something
    mmio_write(GPU, GPU_INSTR::EXECUTE, shader_program_id)
}

gpu_send_shader(*shader: GPUShader) {
    mmio_write(GPU, GPU_INSTR::SEND_SHADER, shader)
}

# to compile shader code into gpu runnable binary code
compile_shader_code(*code) = {
    # mmio instructions for direct manipulation. Usually a full shader program is sent to the GPU for it to execute by itself
    # so usually not needed here
    _load() = mmio_write(GPU_INSTR::load, ...)
    _matrix_mult() = mmio_write(GPU_INSTR::matrixmult, ...)
    # ... more defs

    # invoke the compiler
    compiler.compile(code)
}

# in compiler
compile(*code) = {
    // parse

    // semantic analysis

    // if validated, continue, else, raise error.
    // NOTE: unlike a cpu program, a 'validated' but 'incorrect' graphics program can still run

    // compile to phantasm-graphics IR

    // assemble to spectral graphics shader binary (pseudo-ELF format with little endian, F32, program header and sections)
    // note format doesnt really matter as much since CPU isnt going to care about it and GPU is a brainless
    // device that only needs to crunch numbers, not ensure security and right stuff
}
```
