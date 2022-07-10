---
layout: default
title: Graphics
has_children: true
---

## Neutron Graphics

Graphics is prob one of the biggest things if not the biggest.

Neutron employs high data flow theory to ensure graphics data gets processed with little latency and overhead as possible.

Upscaling and many post processing effects and multiple render passes are also something that affect performance. Terraformer3D is configured with as much "single pass" frame semantics as possible. Extra features like RT dramatically simplify and speedup otherwise slow features like shadows, reflections and proper lighting. It relies heavily on good scene setup and game state transitions for performance. Postprocessing effects should be used sparingly. Preprocessing effects should be applied at compile time if possible.

Other things involve good storage of asset data. Hardware compression/decompression ensure constant textures take 3x less space on disk. That means you should use 4K/8K textures (color, height, ao) and relatively complex models / animations. The act of transforming those models and applying textures through lighting (RT) is the bane of rendering.
