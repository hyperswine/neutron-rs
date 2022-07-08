---
layout: default
title: VideoCore
parent: Arm
grand_parent: Arch
---

## VideoCore GPUs

Not too bad it seems. But pretty bad compared to others.

Very freq used in Pi stuff. Well broadcom SoCs, which Pi freq uses. Pi 1, Pi 2 and Pi 3 all use this Videocore IV. Videocore V was used in BCM7251, but didnt seem like many or any major brands used it at all. Videocore VI was used in Pi 4B.

## Videocore IV

Default config:

1. 250MHz, 40nm process, OpenGL-ES 2.0 and OpenVG 1.1
2. 720p output HDMI, 25 Million Triangles per second

So if aiming 60fps, we have 25M/60 = 416K triangles max. to be rendered each frame. Given that the latency between other components and comms are mostly irrelevant. THeres prob ways to almost force it too. Maybe just cut out triangles/meshes that would overcount, or simplify the scene using some triangle simplifier before rendering.

### Quad Processor

It processes Quads (QPU) as its native data. Instructions are 64-bits.

We have 4 way parallelism and 16-way virtual parallelism. The 4-way is multiplexed over 4 successive clock cycles.

We also have a "dual issue" ALU (FP32?) with one add + one multiply in one cycle. So its just MAC.
