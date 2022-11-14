use bevy_ecs::prelude::{Component, Entity};
use renderer_core::arc_swap::ArcSwapOption;
use renderer_core::assets::models;
use renderer_core::shared_structs::JointTransform;
use std::ops::Range;
use std::sync::Arc;

#[derive(Component, Debug, Clone, Copy)]
pub struct Instance(pub renderer_core::Instance);

#[derive(Component)]
pub struct InstanceOf(pub Entity);

#[derive(Component)]
pub struct PendingModel(pub Arc<ArcSwapOption<models::Model>>);

#[derive(Component)]
pub struct Model(pub Arc<models::Model>);

#[derive(Component)]
pub struct PendingAnimatedModel(pub Arc<ArcSwapOption<models::AnimatedModel>>);

#[derive(Component)]
pub struct AnimatedModel(pub Arc<models::AnimatedModel>);

#[derive(Component, Default)]
pub struct Instances {
    // vec of primitives of lods of instances.
    pub primitives: Vec<instances::Primitive>,
}

mod instances {
    pub struct Primitive {
        pub lods: Vec<Lod>,
    }

    pub struct Lod {
        pub instances: Vec<renderer_core::GpuInstance>,
    }
}

impl Instances {
    pub fn clear(&mut self) {
        for primitives in &mut self.primitives {
            for lod in &mut primitives.lods {
                lod.instances.clear();
            }
        }
    }

    pub fn insert(
        &mut self,
        primitive_id: usize,
        lod: usize,
        instance: renderer_core::GpuInstance,
    ) {
        while self.primitives.len() <= primitive_id {
            self.primitives
                .push(instances::Primitive { lods: Vec::new() });
        }

        let lods = &mut self.primitives[primitive_id].lods;

        while lods.len() <= lod {
            lods.push(instances::Lod {
                instances: Vec::new(),
            });
        }

        lods[lod].instances.push(instance);
    }
}

#[derive(Component, Default)]
pub struct InstanceRanges {
    lods: Vec<instance_ranges::Lod>,
}

mod instance_ranges {
    use super::*;

    pub struct Lod {
        pub ranges: Vec<Range<u32>>,
    }
}

impl InstanceRanges {
    pub fn clear(&mut self) {
        for lod in &mut self.lods {
            lod.ranges.clear();
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &instance_ranges::Lod)> {
        self.lods.iter().enumerate()
    }

    pub fn extend(&mut self, lod: usize, ranges: impl Iterator<Item = Range<u32>>) {
        while lod >= self.lods.len() {
            self.lods.push(instance_ranges::Lod { ranges: Vec::new() });
        }
        self.lods[lod].ranges.extend(ranges);
    }
}

#[derive(Component)]
pub struct ModelUrl(pub url::Url);

#[derive(Component)]
pub struct AnimatedModelUrl(pub url::Url);

#[derive(Component, Debug)]
pub struct JointsOffset(pub u32);

#[derive(Component)]
pub struct JointBuffers {
    pub next_buffer: usize,
    pub buffers: Vec<JointBuffer>,
}

impl JointBuffers {
    pub fn new(
        device: &wgpu::Device,
        bind_group_layouts: &renderer_core::BindGroupLayouts,
    ) -> Self {
        Self {
            next_buffer: 0,
            buffers: vec![JointBuffer::new(device, bind_group_layouts)],
        }
    }
}

pub struct JointBuffer {
    pub staging: arrayvec::ArrayVec<JointTransform, { JointTransform::MAX_COUNT }>,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl JointBuffer {
    pub fn new(
        device: &wgpu::Device,
        bind_group_layouts: &renderer_core::BindGroupLayouts,
    ) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("joint buffer"),
            size: std::mem::size_of::<[JointTransform; JointTransform::MAX_COUNT]>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            staging: Default::default(),
            bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("joint buffer bind group"),
                layout: &bind_group_layouts.joints,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            }),
            buffer,
        }
    }
}

#[derive(Component)]
pub struct AnimationJoints(pub renderer_core::gltf_helpers::animation::AnimationJoints);

#[derive(Component)]
pub struct AnimationState {
    pub time: f32,
    pub animation_index: usize,
}
