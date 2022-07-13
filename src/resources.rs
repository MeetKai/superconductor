use renderer_core::glam::{Mat4, Quat, Vec3};
use renderer_core::utils::Swappable;
use std::sync::Arc;

pub struct Device(pub Arc<wgpu::Device>);
pub struct Queue(pub Arc<wgpu::Queue>);

#[derive(Default)]
pub struct WindowChanges {
    pub cursor_grab: Option<bool>,
    pub cursor_visible: Option<bool>,
}

pub struct FrameTime(pub f64);

pub struct NewIblTextures(pub Option<NewIblTexturesInner>);

pub struct NewIblTexturesInner {
    pub diffuse_cubemap: url::Url,
    pub specular_cubemap: url::Url,
}

pub struct EventQueue(pub Vec<winit::event::Event<'static, ()>>);

pub struct Pipelines(pub Arc<renderer_core::Pipelines>);
pub struct BindGroupLayouts(pub Arc<renderer_core::BindGroupLayouts>);

pub(crate) struct UniformBuffer(pub(crate) Arc<wgpu::Buffer>);
pub(crate) struct MainBindGroup(pub(crate) Swappable<wgpu::BindGroup>);
pub(crate) struct SkyboxUniformBuffer(pub(crate) wgpu::Buffer);
pub(crate) struct SkyboxUniformBindGroup(pub(crate) wgpu::BindGroup);

pub struct IndexBuffer(pub Arc<renderer_core::IndexBuffer>);
pub struct VertexBuffers(pub Arc<renderer_core::VertexBuffers>);
pub struct AnimatedVertexBuffers(pub Arc<renderer_core::AnimatedVertexBuffers>);
pub(crate) struct InstanceBuffer(pub(crate) renderer_core::InstanceBuffer);

pub(crate) struct IntermediateDepthFramebuffer(pub(crate) CachedFramebuffer);
pub(crate) struct IntermediateColorFramebuffer(pub(crate) CachedFramebuffer);
pub(crate) struct CompositeBindGroup(pub(crate) Option<wgpu::BindGroup>);
pub(crate) struct LinearSampler(pub(crate) Arc<wgpu::Sampler>);

#[derive(Default)]
pub(crate) struct CachedFramebuffer {
    inner: Option<ResourceWithSize<renderer_core::Texture>>,
}

impl CachedFramebuffer {
    pub(crate) fn get(
        &mut self,
        device: &wgpu::Device,
        descriptor: &wgpu::TextureDescriptor,
    ) -> &renderer_core::Texture {
        let create_fn = || {
            let texture = device.create_texture(descriptor);

            let view = texture.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(if descriptor.size.depth_or_array_layers > 1 {
                    wgpu::TextureViewDimension::D2Array
                } else {
                    wgpu::TextureViewDimension::D2
                }),
                ..Default::default()
            });

            ResourceWithSize {
                resource: renderer_core::Texture { texture, view },
                size: descriptor.size,
            }
        };

        let cached = self.inner.get_or_insert_with(create_fn);

        if descriptor.size != cached.size {
            *cached = create_fn();
        }

        &cached.resource
    }
}

// A resource that's stored alongside a wgpu extent for cache invalidation purposes.
struct ResourceWithSize<T> {
    resource: T,
    size: wgpu::Extent3d,
}

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
