---
layout: default
title: Terraformer3D Engine
parent: Supported Software
---

## Terraformer3D Overview

A simple unity/godot like game engine for building 3D games with rust-wgpu and blender. All types of models can be stored in `.blend` and it should be the format's responsibility to make it more efficient.

- specialisation 1: for making first/third person views. View distance medium and configurable.
- specialisation 2: for making top down 3D views of relatively small maps at a time. Or a mid-large sized map covered by fog.

## General Things

- wpgu is an abstraction over the default API, e.g. vulkan

## WGPU

Rust [wgpu](https://wgpu.rs/) itself tries to mimick vulkan's interface. But in reality it translates those wgpu calls to whatever API the host uses.

- [WebGPU](https://gpuweb.github.io/gpuweb/) is an interface specification meant for rendering things in a web environment/browser
- A [tutorial](https://sotrh.github.io/learn-wgpu/beginner/tutorial1-window/#boring-i-know) on wgpu

## Ray Tracing

What if instead of using the GPU designed kind of like the CPU but just scaled up more and each execution unit simplified (no branch prediction, etc), we rethink the problem?

- The concept of randomly generating rays per pixel in a cone to hit surfaces. Then rays reflecting and hitting other surfaces for at most `k` times unless they hit a light source
- Shadows are intrinsic. They arent mapped to the scene but are regions/pixels where rays dont often hit light sources
- Reflections are intrinsic. We absorb the color of each surface/material we hit and use a weighted average depending on whether the pixel hits much light

So we can replace a lot of the things like shadow mapping, cubemapping/other reflection methods, light calcs that would be traditionally done in the fragment shader. And instead use ray tracing to basically do everything when rendering a 3D scene each frame

- What if we used some other component that is much better at doing something like RT instead of the GPU?
- We would then reserve the GPU for 2D drawing like windows and etc and apply any post processing effects (e.g. Spatial AA & Temporal AA, HDR & Bloom, SSAO, Screen Space Reflections, Motion Blur) on the resultant framebuffer (in a separate pipeline)
- And a more specialised sub-GPGPU/TPU/Neural Cores for ML algorithms

### SpectroFloat12 (SF12)

We dont need too much precision or range for specific machine learning and graphics tasks. For RT, the range of values is pretty unimportant and should depend mostly on what the monitor is capable of outputting. Most current monitors have a precision of .0001 per R,G,B value. Given that each component is in [0, 1].

```
[ sign (1) | exponent (5) | fraction (6) ]
```

- pretty small range and less precision than FP16 or bfloat16. But up to 16/12 = 33% faster
- smaller component footprint means we can pack more SF12 execution units in the same area, possible 33% more as well
- total = 33% more execution units, each of which are 33% faster. That means 78% higher overall throughput. Calcs (1.33 * 1.33 = 1.78)

### Unsigned SF12 (USF12)

For many graphics applications, we only use non negative decimal values. A lot of the time they are either between 0-1 or 0-255. For an RT algorithm that simply scans for surface properties, which are:

- reflectiveness
- color (texture)
- normal (vector-3, negatives)
- any other maps like bump maps and height maps (vector-3, negatives)

Some of them have negatives and are vectors/pairs of individual scalar values.

```
[ exponent(6) | fraction (6) ]
```

- slightly higher range. Since the exponent was quite small in the first place, not too much improvement
- but no hardware overhead for calculating negative values and positive values together, if there is any. Maybe not

### Neuromorph I

What if we had a neuromorphic chip designed more like a grid of neurons with state, with input and output neurons. Maybe like an FPGA where we can decide how many nodes around the edge we want to use as input nodes and which ones we want to use as output nodes. Or perhaps let the compiler do it for us

- basically, some way to automatically assign the inputs and outputs to the edge nodes

Then some way to configure the grid to handle a task like RT. We dont want to hardcode it in if possible, have have the software make more of the decisions. But its not a bad idea

- for RT, maybe we train a graph neural net to somehow represent the problem / algorithm. It should be as precise as RT within the GPU (RT Cores). But instead we feed it a 3D scene with a camera frustum. Its job is to render from that camera's POV using the scene's vertex data. If possible we can try to cut out the vertices that arent visible or far away from the player with dynamic scene loading. Make each region a scene or each place the player is at generate a radius and encapsulate all the captured vertices whether they are in view or not. Prob easier to do it like other games where each area is a 'scene' that can be loaded on the fly when you get near it
- feed the scene vertices that are formatted like in opengl. Or perhaps use a 3D vector to store each vertex like a bounding box. Empty space do not have a value. The algorithm would therefore have a spatial representation of the scene as a box of discrete values in each index. More indices means higher resolution, so either we take the scene at 100% resolution or take every second vertex in both the x, y, z axis instead for 50% resolution
- the scene box should reflect the camera's POV. Maybe make it a frustum instead since we arent going to take into account everything outside of the frustum in color calcs anyway
- so for a perspective frustum of vertices, the algorithm has to somehow trace each individual pixel of the display into the frustum. This is the part that would make sense on a really highly parallel system where you can generate many values per pixel and feed it in. Then go to the next pixel and feed them into the cores again. Will have to do this millions of times to get the entire display, should be quite fast if done properly, e.g. RT cores with GHz speeds

Applying ML and monte carlo like heuristics:

- RT means we basically have to render from the displays' POV rather than anything else's. We can render from the lights POV and stuff like that too, and generate intermediate results and feed it back to the system
- but for monte carlo randomisation, maybe there is a way to include that within the algorithm itself rather than invoke an external PRNG

```rust
@async
for pixel in pixels {
    let pixel_color: Vec3
    // do it 100 times
    for i = 0..100 {
        // only generate rays 'forward'
        direction = random_vec3(0, 1)
        let current_properties = Property()
        let res = trace(pixel.origin, direction, current_properties)
        pixel_color.add_to_weighted_sum(res.color)
    }

    // final pixel color
    pixel_color.apply_weighted_sum()
    pixel.apply(pixel_color)
}

// in world
fn hit(origin, direction) {
    // get the first object/vertex a ray like this would hit
    let _ray = ray(origin, direction)
    let obj: *Obj = Obj(distance=Inf)

    for object in objects {
        if object.intercept(_ray) {
            if object.distance(_ray.origin) < obj.distance(_ray.origin) {
                obj = object
            }
        }
    }

    let [material_color, reflectiveness] = obj.hit_location.properties()
    return (material_color, reflectiveness)
}

fn trace(origin, direction, current_properties) -> &MaterialProperty {
    // find the object to be hit in this direction
    // if no object, then the ray should be cancelled and returned
    let res = world.hit(origin=origin, direction=direction)
    if !res {
        // black
        return [0,0,0]
    }
    
    // hit an object, got its color and other properties, mix it with the current one
    current_properties.mix(res)
    // should reflect according to the surface normal of the hit vertex/normal map of the object
    return trace(res.origin, res.direction, current_properties)
}
```

Could there be a specialised hardware circuitry for computing RT or something that closely matches RTing a frustum scene from a viewpoint?

### Meet the illuminator

Highly parallelised, SF12, matched to a certain display size.

```rust
annotation perspective {
    function => {
        if "--perspective" in compiler.args {
            compile
        }
        else {
            skip
        }
    }
}

@hardware-component
class Illuminator {
    constructor() {}

    @perspective
    fn run(scene_frustum: SceneFrustum) -> Frame {
        // scene frustum contains all the vertices and material properties per index matched to either 100% or 50% resolution

        let pixels: Array2D<RGBVec> = scene_frustum.near_plane

        // map() should be parallel
        pixels.map(
            // always use references as args and parameterise with the base type
            p -> trace100(p, &scene_frustum)
        )
    }

    // note class based methods override the names of higher level modules
    // that is why it is important to have descriptive function names
    // note: deduce can also deduce the function parameters if you always call it with the same 
    // types. If you always call it with a specific set of types, then use @deduce, template-deduce 
    // instead. Best to not include any other info at all if using @deduce
    @deduce
    fn trace100() {
        // do something 100 times in parallel and mix their results in parallel
        let res = (await parallel(n=100, &global.prng, trace(p))).average()

        return res
    }

    // key 1
    fn trace() {}

    // key 2
    fn hit() {}
}

// ARCH OVERVIEW

inputs (parallel) --feed--> [ discerner ] --scene_frustum--> [ scene frustum handler ]
                                |
                              pixels (parallel)
                                V
                            [ ray tracer ]
                                |
                                V
                    as many x [ linear array of cores that process 100 rays at time ] as possible
                        [ uses scene frustum handler for queries, 100 parallel connections to it ]
                                |
                            parallel sum
                                V
                            [ FMA ]

```

- basically just a lot of individual units. Up the the max supported resolution, e.g. 4K
- common case: feed in a 1440p grid of vec3 rgb values that need to be filled in
  - thats like at least 3.7million units in stage 1 to compute each pixel
  - then for each stage 1 unit, there is 100 stage 2 units for computing 100 rays in parallel, in random directions. Each ray will bounce at most `k` times, and so those stage 2 units may require up to `k` cycles to fully complete
  - then we combine all 100 results from each ray unit, preferrably at the same time. We should make `k` relatively small or according to how many bounces we are expecting on average before hitting a light, something absorbing or outside the scene. Parallel combine with a FMA unit
  - each stage 2 unit has direct access to the global scene handler and can somehow request a hit very fast. This is key 2 and kinda problematic as well
  - feedback the stage2 result to stage 1 and its corresponding pixel
- total: 370million stage 2 execution units. 3.7 million stage 1 execution units. If we can somehow parallel access a precalc'd scene frustum using the scene handler, then no need for more scene_handlers

### ML Way

Instead we can train a network to somehow recognise how a scene looks from a scene frustum and a view position and dir. Using a 3D matrix of values (base rgb, surface reflection, normal_dir, etc.) and a pos+dir 2D matrix of values (coord, dir (usually perspective)).

- it would be able to generate a 2D image directly. Might be a bit rough at first. But basically we offload it completely to the ML. If big enough, I think it prob can do it quite accurately. Just needs good data and diverse enough to get it going
- we can have 2 stages or 2 separate models to do it. Maybe on a neuromorphic chip or specialised chip like a tensor processing unit/scene processing unit specialised this kind of 3D of multiple-D data and 2D of 3D data

## Screen Space Reflections

So we rendered our image in the first framebuffer. We have a color buffer and a depth buffer. We can use that data to then figure out any reflections on certain vertices that have a reflective_index > 0. We would run another pipeline where for each vertex (coordinate and normal) with r_index, we scan the color framebuffer. We trace a 3d vector from that vertex coord and see if it hits another vertex in the 3d scene. If it hits a vertex with the same depth as the vertex in the depth buffer, we know that it is in the color buffer. So then we sample the color buffer for that hit vertex and mix the color with the current vertex depending on `r_index`.

- Cannot reflect anything not in the color buffer. So wouldnt work for a mirror if you are looking directly into it in first person. Even 3rd person wouldnt work since you need to see the front of your character

## Cubemapped Reflections

Terrible idea in terms of efficiency. You may also need several cubemap probes at certain intervals to fully capture all the surrounding colors.

- Will look pretty good in general since hard to do it badly
- But you will need to have a camera that captures 6 faces around a certain arbitrary point, e.g. the centre of the room. We store each face as a texture that we can sample from
- Say if you are trying to render a reflective mirror. Then for each vertex in the mirror that has a r_index > 0, we ray march into the 6-faced cubemap. And sample the face which matches the general direction and coordinates. Retrieve that color (rgba) value and mix it with the current vertex according to r_index
- You will probably have to regenerate the cubemap when you are inside the room/a certain distance away from it. If there is an algorithm to determine which vertices/objects not within a certain distance (render distance) then you can also cut it out before clip space calculation in the vertex shader
- Better if you have multiple probes at the general centres of each reflective object so then you have a more 'accurate' reflection distance and scaling

## Terrible Idea: Phong Shading

Its not completely terrible since it works quite well on modern GPUs. But it is only able to calculate generalised lighting conditions, i.e. no direct method for shadow calcs. Vertices blocked by another vertex in the depth buffer will simply be darkened and applied with ambient lighting only.

- If you want actual shadows that are casted from one object to another instead of just a region of generalised darkness, you have to also use shadow maps. This means you need to first render the scene a few times from each light's point of view. Then go into another pipeline where you loop through each rendered depth map to see which pixels are in the light and which ones in the dark based on the depth value. Then you can apply the final color calcs based on all lights in the scene
