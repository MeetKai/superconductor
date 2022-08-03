# Superconductor architecture overview

I've written this document to give a brief overview of how the rendering code of Superconductor is written. This document assumes graphics programming knowledge.

## Background

Superconductor is a renderer of 3d models written ontop of `wgpu`. It's mainly targetting mobile devices on the web, but also compiles and runs as a native binary on Linux/Windows/MacOS. On the web, it has support for rendering on VR and AR devices via the WebXR API. Because WebXR has no ability to interface with WebGPU, we're forced to use the WebGL 2 `wgpu` backend for the time being. This means no storage buffers, no compute shaders and no indirect draw calls. Which sucks.

## Assets

### Models

Superconductor only has support for loading glTF models. These are currently 2 types of model that glTFs can be loaded as, which I've named stationary and animated models. The main difference is that animated models have support for both skinned meshes and animations inside the glTF file, whereas stationary models completely ignore these elements. Despite the naming, animated models do not require any animations to be contained in the file, nor do any deformable skins need to be specified.

All models use indexed geometry and store vertex positions, uvs and normals. Animated models also store joint indices and joint weights.

Models are considered to be made up of what I've named primitives. All geometry in a primitive share a glTF material and store the textures, sampler and material settings (such as the base colour factor) in a bind group, as well as the geometry indices for the the primitive.

All primitives store albedo, metallic-roughness, normal and emission textures, even if the texture was not specified in the glTF. 1x1 dummy textures are used until a specified texture is loaded in to replace it.

### Textures

I've written fairly extensively about textures in https://github.com/MeetKai/superconductor/issues/11.

## Buffers

To reduce the amount of times that buffers are bound per-frame, as well as to allow for future (with WebGPU) improvements like Multi-Draw-Indirect, global index, vertex and instance buffers are used.

### Instance Buffer

All model draws are potentially instanced and read instance translation, rotations and (uniform) scaling from the global instance buffer.

The global instance buffer is essentially treated as a `Vec<Instance>`. It stores an offset and a capacity and the buffer object itself. Each frame it's cleared (resetting the offset to 0), instances are pushed onto (potentially resizing it) and it's then bound.

Resizing creates a new buffer at least twice the size of the old buffer and schedules a buffer-to-buffer copy in order to write the currently written instances to it.

### Index Buffer

The index buffer (and vertex buffers) work a little differently. It requires require A) consistency, where the location of any indices needs to be stable across inserts/removals and B) the ability to remove indices to free up space.

I've implemented this by sub-allocating the buffer using [a modified version of the `range-alloc` allocator](https://github.com/expenses/gfx/tree/range-allocator). When a new set of indices is inserted, the allocator finds a suitable range within the buffer to insert the indices into and resizes if necessary. Resizing is similary to the instance buffer, except that the allocator is resized to ensure that it contains enough space for the new indices, and the existing ranges are copied over to the new buffer.

### Vertex Buffers

Expanding on that, the vertex buffers are similar to the index buffer, except that we group the 3 buffers (position, normal, uv) with 1 allocator. The allocator stores ranges of vertices, where-as each buffer stores enough space for `num_vertices * size_of(element)` (notably, the uv buffer only has to store a float32x2 while the position and normal buffers have to store float32x3s).

Additionally, we have a seperate set of buffers for animated models, that store 5 buffers (position, normal, uv, joint indices, joint weights). This is because the vertex buffers are iterated through in lock-step, so there would have to be a lot of empty space in the joint indices/weights buffers for stationary models.

One future solution to this might be to bind the joint indices/weights buffers as storage buffers in the vertex shaders instead and index into them with a seperate system.

### Joint Transform Buffers

Animated models require a joint transform buffer that is bound in the vertex shader. We're limited to uniform buffers due to WebGL 2. For each animated model, we bind a uniform buffer of joints that is 65536 bytes in length, the max size for WebGL 2. As each joint transform is 32 bytes in size (float32x3 for translation, float32x4 for a rotation quaternion and a single float32 for a uniform scale. 8 floats in total), we can have a total of 2048 joints being used in a single draw call.

These joint transform buffers are uploaded from a CPU-side `arrayvec::ArrayVec<JointTransform, 2048>` and cannot be resized.

Ideally we'd use a single global storage buffer that contains all joint transforms for all models.

Until then, 2048 joints is not many. A lot of animated models have 30+ joints and reach this capacity pretty quickly when instancing. To support drawing more animated models we'd need to dynamically create more joint buffers for a model when the previous joint buffer reaches the capacity.

One other solution is to bake animations into textures, which is something that BabylonJS has implemented: https://doc.babylonjs.com/divingDeeper/animation/baked_texture_animations.

## Pipelines

As already mentioned, we support both stationary and animated models. As animated models use a seperate set of vertex buffers and a different vertex shader, they need to be rendered with a different pipeline. I've found the best way to represent different pipeline permutors to be a set of generic structs:

```rust
#[derive(Default, Debug, Clone)]
pub struct ModelTypes<T> {
    pub stationary: T,
    pub animated: T,
}
```

Another permutor is the different blend mode of the material, where alpha-clipped materials allow for fragment discarding and alpha-blended materials use a different blend mode:

```rust
#[derive(Default, Debug, Clone)]
pub struct BlendMode<T> {
    pub opaque: T,
    pub alpha_clipped: T,
    pub alpha_blended: T,
}
```

Finally, there's a permutor for whether the model is doing backface-culling or not:

```rust
#[derive(Default, Debug, Clone)]
pub struct FaceSides<T> {
    pub single: T,
    pub double: T,
}
```

Put together, we end up with a total of 12 pipelines for models:

```rust
pub struct Pipelines {
    pub pbr: permutations::BlendMode<
        permutations::ModelTypes<permutations::FaceSides<wgpu::RenderPipeline>>,
    >,
    ...
}
```

## Draw Calls

The process for recording model draw calls looks like this (in pseudo-code):

```rust
for blend_mode in (opaque, alpha_clipped, alpha_blended) {
    bind_global_stationary_vertex_buffers();

    for face_side in (single, double) {
        bind_appropriate_pipeline();

        for (model, instance_range) in stationary_models {
            for primitives in model.primitives.filter(blend_mode, face_side) {
                set_bind_group(primitive.bind_group);
                draw_indexed(primitive.index_range, instance_range);
            }
        }
    }

    bind_global_animated_vertex_buffers();

    for face_side in (single, double) {
        bind_appropriate_pipeline();

        for (model, joint_transforms_bind_group, instance_range) in animated_models {
            set_bind_group(joint_transforms_bind_group);

            for primitives in model.primitives.filter(blend_mode, face_side) {
                set_bind_group(primitive.bind_group);
                draw_indexed(primitive.index_range, instance_range);
            }
        }
    }
}
```

Ideally we'd have a Multi-Draw-Indirect setup that looks something like this, for a fixed number of 12 draw calls per-frame:

```rust
for blend_mode in (opaque, alpha_clipped, alpha_blended) {
    bind_global_stationary_vertex_buffers();

    for face_side in (single, double) {
        bind_appropriate_pipeline();
        draw_indexed_indirect(indirect_draw_buffers.filter(blend_mode, stationary, face_side));
    }

    bind_global_animated_vertex_buffers();

    for face_side in (single, double) {
        bind_appropriate_pipeline();
        draw_indexed_indirect(indirect_draw_buffers.filter(blend_mode, stationary, face_side));

    }
}
```

Currently the biggest blockers for having something like this are the fact that we need to have per-primitive bind groups for textures and per-model bind groups for joints. When WebGPU comes out we'll be able to elevate the per-model joint buffers to a single globally-bound storage buffer, but textures would remain a problem.

We'd need to wait for a texture array extension so that we could put all textures into one big globally bound array that is indexed into. See https://github.com/gpuweb/gpuweb/issues/822 for the status on this.

## Rendering

Todo :wink