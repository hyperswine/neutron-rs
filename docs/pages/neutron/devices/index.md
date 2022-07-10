---
layout: default
title: Devices
parent: Neutron
---

## Neutron Devices

The main device categories of interest are PCIe devices (x86 & riscv), AMBA devices (ARM), and USB-C 3.2+. Most types of peripherals are able connect through these ports. Wireless peripherals would simply use the wifi/bt card connected via PCIe.

HDMI is obsolete with USB-C. No mini hdmi. Audio peripherals should be connected wirelessly. No headphone jack or mic in. The wireless card driver detects the types of peripherals connected to it and loads the appropriate NeutronDriver for it. Note Wireless drivers are basically PCIe/block drivers that use 4K blocks.

Bluetooth is recommended for headphones and audio and perhaps certain input devices like joycons (that dont need as low of a latency). And 'quick' throwaway connections. Wifi6E+ should be used for most other wireless objects. Mice, KB, etc. Wireless interference is a problem though... Quantii devices should be able to adjust channels on the fly if theres a lot of interference.

Wifi 6E has 7 120MHz bands being 6GHz. It has the entire 6-7.2GHz (or was it 4.8-6?) spectrum to use. It also carries more energy being higher frequency. But in reality this means pretty much nothing for speed increases. What is responsible for speed is the width of the band of the signal. For 2.4GHz wifi, your bandwidth is 10MHz. I think 5GHz is chooseable between 20/40/80MHz.

The max throughput over Wifi 6E is 9.6 Gbps (1.2 GB/s). For 4K/8K streaming and VR, could make a lot of sense. Wireless monitors too. Wifi 6E has a latency of 2-6ms (just for propagation? No that should be instant I think). Maybe even lower if the network stack is done properly. For graphics, you will need to render the frame, which could take 8-16 ms, then package it into 4K packets to send to the wifi card. If possible, DMA multiple pages at a time, which should be in theory quite fast. The card then needs to buffer and start transmitting. The monitor starts receiving and buffering until it gets the entire frame, which it can render directly through the ARC WIRELESS protocol.

A 4K120 frame should rendered every 8ms. A 24MB (24-bit true color) piece of data (including metadata/packet headers). 24MB needs to be sent by the wireless card to the monitor. Every 8ms. Compression? err. That could take extra time, which isnt really good. Maybe compress the actual stuff on the graphics card or memory / assets on disk. But not in mid stream. If alpha is used, we will have even more data (32MB) to transmit.

## Hardware Compression

Using hardware to accelerate certain algorithms can be done with an ASIC or an FPGA, if that would be faster (which it should be). Updates to the FPGA can then be pushed out with updates to the system. I dunno if the user really wants to know whats in the update. Maybe just briefly state what it does but provide hidden advanced details to devs.

I think we should have a few FPGA chips that can be programmed to do different things on the fly. For a gaming or creator profile, it can be reprogrammed to be a decompressor or an extra graphics accelerator. For casual users or business users, you might want better security or extra flashy features in general which can be implemented in "hardware".

I think Kraken makes a lot of sense. I do like zstd though. An ASIC chip fabricated with the same tech as the cpu/gpu for the most used zstd profiles would be very nice.

## Neutron Decompression

ZSTD/Huffman/Deflate but better compression ratio (>5) and implementable on an ASIC/FPGA. Works really well with NeFS, Terraformer3D and ARC Wireless protocols.

Speed? Uh.. In software it could take a long, long time. But in hardware we have algos like FFT and randomisation heuristics to speed things up. For streaming video, we can disable the losslessness, to allow lower latency/better performance. Esp when using multiple input and output displays.

The hardware compress/decompress complex is located between the PCIe complex and the DRAM complex. A PCIe mass storage device simply needs to send the blocks of data to the decompress instead, and buffer it there (up to a 1GB) before sending to the address in DRAM. And the other way around.

NeDec. The FPGA itself contains the driver algorithm. The memory contains the dictionary and packets. We always operate in 4K packets.

NeutronAPI and NeutronDrivers directly target and support NDF. Most of neutron's services use NDF by default for many things. The main `nzip` program creates `.nzip` files which are compressed archives. An archive is simply a mini directory, a 1D directory kind of. The metadata of the decompressed archive tells NeFS how to reconstruct the actual directory from the dec data. `nzip` is built as an executable program with a `Archive` app frontend in `/sys/bin/nzip`.

Or maybe call it `nz` or `narc` or `archive`. Build our app frontends with the arc dioxus library. Would be good to use rei, but err. Its best to use VSCode on the web and for live sharing / copilot / throwaway or cloud computing extensions. Maybe we should support that too. But I want something native on the desktop making full use of the HW available.

## Unified Memory

I really want something with unified memory. An APU with a CPU, GPGPU, multimedia accelerators and FPGA chiplets. They are all on the same chip. Fabricated with the same process.

DRAM might or might not be on the SoC. The problem is it would occupy a lot of gates. Would it be more efficient? I think so, definitely. Just banks of capacitors after all. Whats expensive are register banks and SRAM banks that use static transistors. These transistors dont seem to wear out over time, as the ones on the SSDs do. You arent really changing the state of the transistors though are you? Err, not really? SRAM you prob are.

## Neuromorphic Chip

A neuromorphic chip uses a grid of memristors. Each node has a state like a register. Except of constructing a register from your usual flipfops, you somehow use a memristor. Which would store 16/32/64/128 bits each. Then you can also simulate a brain or graph like netwrok. A conventional neural network as well.
