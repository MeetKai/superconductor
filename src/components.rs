use bevy_ecs::prelude::{Component, Entity};
use renderer_core::assets::models;
use renderer_core::shared_structs::JointTransform;
use renderer_core::utils::Setter;
use std::ops::Range;

#[derive(Component)]
pub struct Instance(pub renderer_core::Instance);

#[derive(Component)]
pub struct InstanceOf(pub Entity);

#[derive(Component)]
pub struct PendingModel(pub Setter<models::Model>);

#[derive(Component)]
pub struct Model(pub models::Model);

#[derive(Component)]
pub struct PendingAnimatedModel(pub Setter<models::AnimatedModel>);

#[derive(Component)]
pub struct AnimatedModel(pub models::AnimatedModel);

#[derive(Component)]
pub struct Instances(pub Vec<renderer_core::FullInstance>);

#[derive(Component)]
pub struct InstanceRange(pub Range<u32>);

#[derive(Component)]
pub struct ModelUrl(pub url::Url);

#[derive(Component)]
pub struct AnimatedModelUrl(pub url::Url);

#[derive(Component)]
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
