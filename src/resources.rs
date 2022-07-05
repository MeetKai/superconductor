use bevy_ecs::prelude::Entity;
use renderer_core::glam::{Mat4, Quat, Vec3};
use renderer_core::utils::Swappable;
use std::collections::HashMap;
use std::sync::Arc;
use winit::event::KeyboardInput;

pub struct Device(pub(crate) Arc<wgpu::Device>);
pub struct Queue(pub(crate) Arc<wgpu::Queue>);

pub struct FrameTime(pub f64);

pub struct NewIblTextures(pub Option<NewIblTexturesInner>);

pub struct NewIblTexturesInner {
    pub diffuse_cubemap: url::Url,
    pub specular_cubemap: url::Url,
}

pub struct KeyboardInputQueue(pub Vec<KeyboardInput>);

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

pub(crate) struct SurfaceFrameView {
    pub(crate) view: wgpu::TextureView,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

pub struct Camera {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Camera {
    pub fn forwards(&self) -> Vec3 {
        self.rotation * Vec3::new(0.0, 0.0, -1.0)
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.position + self.forwards(), self.up())
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 1.75, 0.0),
            rotation: Quat::IDENTITY,
        }
    }
}