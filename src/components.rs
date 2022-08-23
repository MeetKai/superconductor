use bevy_ecs::prelude::{Component, Entity};
use renderer_core::arc_swap::ArcSwapOption;
use renderer_core::assets::models;
use renderer_core::shared_structs::JointTransform;
use std::ops::Range;
use std::sync::Arc;

#[derive(Component)]
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

#[derive(Component)]
pub struct Instances(pub Vec<renderer_core::GpuInstance>);

#[derive(Component)]
pub struct InstanceRange(pub Range<u32>);

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
