---
layout: default
title: Gaming on Neutron
parent: Terraformer3D Engine
grand_parent: Supported Software
---

## Overview

Neutron uses a framework called Terraformer3D for developing games optimised for Neutron@RiscV.

- We assume that games are 99% of the time: 3D based, assets created in mainstream editors like Blender and Maya, uses a single Window (ArcWindow), game devs know how to code in vulkan and wgpu.
- For most graphics programming, wgpu is used as a rust-based framework that wraps around the underlying Vulkan drivers on Neutron. For most things, wgpu should be enough, though if the dev wishes to leverage more control that vulkan offers, they can write vulkan `.vs,.fs,.gs` shaders and link them via vulkano-rs.

## Vulkan

A great api for writing code that runs on the gpu. Can be called from any program that includes the platform's vulkan headers.

- Can be used from wgpu-rs. If creating an executable in rust, just use webgpu. It will detect the gpu and drivers which use vulkan. The underlying window server should also use vulkan for rendering. So a wgpu executable can request a window to be created and an area to render to (framebuffer)

## Writing Games

An engine like Terraformer3D can be used to write a 3D game in rust. By default, uses the wgpu API which uses its own shader format. These shaders can then be embedded into the code itself rather than exist as separate shader files.

To make a game, one starts up Terraformer3D. Then they will be met with a Godot3D-like interface. The engine uses a scene-node structuring system for creating levels and characters.

- Most things are drag and droppable, and use of the GUI instead of the underlying code is recommended, unless you aren't a human

By default, we use rust-std and the terraformer3d library (wrappers around wgpu and etc). Offers great performance. But I want rei to also be an option as well as reiscript for lua-like scripting on a 'finished' game.

## User Testing

Unlike other software, Games are very high level and should require proper user testing like alpha/beta tests to gauge the performance, playability, fun, etc. in detail.

Unit testing and integration testing can still help out a lot to make sure you have the proper logic and no weird software bugs. When using a game engine, it may be a bit harder to test for edge cases so the engine itself has to do it well.

## Supporting Tools: Blender

It would be great to have a program like blender run on an OS. Technically you dont need blender to run to use blender models, i.e. `.blend` files or any exported files from blender on another platform. The web itself or any vnc server can offer something similar to the actual experience, just worse quality.

- Design 3D models in blender and any skeletal animations

For stuff like textures or mocap, you'd want to use other software. Though there are extensions to do them in blender.

## Supporting Tools: 3D Model Painters

It would also be great to have something that is optimised for painting large/complex 3D models. Perhaps ways to draw well, and in layers. Perhaps separating different meshes apart to color in things at a time. Ways to make normal maps, height maps, baking textures into the model itself rather than as a separate asset.

## Supporting Tools: VR Viewer

VR is very cool and having something to test out VR within the PC or connect it to the PC for debugging/soft running a game would be great.
