use bevy_ecs::prelude::Entity;
use renderer_core::utils::Swappable;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Device(pub(crate) Arc<wgpu::Device>);
pub struct Queue(pub(crate) Arc<wgpu::Queue>);

pub struct FrameTime(pub f64);

pub struct NewIblTextures(pub Option<NewIblTexturesInner>);

pub struct NewIblTexturesInner {
    pub diffuse_cubemap: url::Url,
    pub specular_cubemap: url::Url,
}

pub(crate) struct Pipelines(pub(crate) Arc<renderer_core::Pipelines>);
pub(crate) struct BindGroupLayouts(pub(crate) Arc<renderer_core::BindGroupLayouts>);

pub(crate) struct UniformBuffer(pub(crate) Arc<wgpu::Buffer>);
pub(crate) struct MainBindGroup(pub(crate) Swappable<wgpu::BindGroup>);
pub(crate) struct SkyboxUniformBuffer(pub(crate) wgpu::Buffer);
pub(crate) struct SkyboxUniformBindGroup(pub(crate) wgpu::BindGroup);

pub(crate) struct IndexBuffer(pub(crate) Arc<parking_lot::Mutex<renderer_core::IndexBuffer>>);
pub(crate) struct VertexBuffers(pub(crate) Arc<parking_lot::Mutex<renderer_core::VertexBuffers>>);
pub(crate) struct InstanceBuffer(pub(crate) renderer_core::InstanceBuffer);

pub(crate) struct IntermediateDepthFramebuffer(pub(crate) Option<renderer_core::Texture>);
pub(crate) struct IntermediateColorFramebuffer(pub(crate) Option<renderer_core::Texture>);
pub(crate) struct CompositeBindGroup(pub(crate) Option<wgpu::BindGroup>);
pub(crate) struct LinearSampler(pub(crate) Arc<wgpu::Sampler>);

pub(crate) struct ModelUrls(pub(crate) HashMap<url::Url, Entity>);
