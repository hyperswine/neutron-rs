// enable with #[cfg(feature = 'arcwm')]
pub mod arcwm;

// ------------------------
// USERSPACE GRAPHICS LAYER
// ------------------------

// Allow implementing of wgpu with these functions
// ASSUME Either MPU/RTU or Mali GPUs

// Primitive Assembly
fn assemble_verts() {}

// Vertex Shading
fn shade_vertices() {}

// Tessellate
fn tessellate() {}

// Geometry Shading
fn shade_geometry() {}

// Rasterise
fn rasterize() {}

// Fragment Shading
fn shade_fragments() {}

// Blend Colors
fn blend_colors() {}
