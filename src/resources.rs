use bevy_ecs::system::Resource;
use renderer_core::{
    assets::textures,
    culling::{BoundingSphereCullingParams, CullingFrustum},
    glam::{Mat4, Quat, Vec3},
    GpuInstance, LineVertex, MutableBindGroup,
};
use std::sync::Arc;

#[derive(Resource)]
pub struct Device(pub Arc<wgpu::Device>);
#[derive(Resource)]
pub struct Queue(pub Arc<wgpu::Queue>);

#[derive(Default, Resource)]
pub struct WindowChanges {
    pub cursor_grab: Option<bool>,
    pub cursor_visible: Option<bool>,
    pub fullscreen: Option<bool>,
}

#[derive(Resource)]
pub struct FrameTime(pub f64);

#[derive(Resource)]
pub struct NewIblCubemap(pub Option<url::Url>);

#[derive(Resource)]
pub struct EventQueue(pub Vec<winit::event::Event<'static, ()>>);

#[derive(Resource)]
pub struct Pipelines(pub Arc<renderer_core::Pipelines>);
#[derive(Resource)]
pub struct BindGroupLayouts(pub Arc<renderer_core::BindGroupLayouts>);

#[derive(Resource)]
pub(crate) struct UniformBuffer(pub(crate) Arc<wgpu::Buffer>);
#[derive(Resource)]
pub(crate) struct MainBindGroup(pub(crate) Arc<MutableBindGroup>);

#[derive(Resource)]
pub struct IndexBuffer(pub Arc<renderer_core::IndexBuffer>);
#[derive(Resource)]
pub struct VertexBuffers(pub Arc<renderer_core::VertexBuffers>);
#[derive(Resource)]
pub struct AnimatedVertexBuffers(pub Arc<renderer_core::AnimatedVertexBuffers>);
#[derive(Resource)]
pub(crate) struct InstanceBuffer(pub(crate) renderer_core::VecGpuBuffer<GpuInstance>);

#[derive(Resource)]
pub(crate) struct LineBuffer {
    pub(crate) staging: Vec<LineVertex>,
    pub(crate) buffer: renderer_core::VecGpuBuffer<LineVertex>,
}

#[derive(Resource)]
pub(crate) struct IntermediateDepthFramebuffer(pub(crate) CachedFramebuffer);
#[derive(Resource)]
pub(crate) struct IntermediateColorFramebuffer(pub(crate) CachedFramebuffer);
#[derive(Resource)]
pub(crate) struct CompositeBindGroup(pub(crate) Option<wgpu::BindGroup>);

#[derive(Default, Resource)]
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
#[derive(Resource)]
struct ResourceWithSize<T> {
    resource: T,
    size: wgpu::Extent3d,
}

#[derive(Resource)]
pub(crate) struct SurfaceFrameView {
    pub(crate) view: wgpu::TextureView,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

#[derive(Resource)]
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

#[derive(Default, Resource)]
pub struct CullingParams {
    pub bounding_sphere_params: BoundingSphereParams,
    pub frustum: Option<CullingFrustum>,
}

pub enum BoundingSphereParams {
    SingleView(BoundingSphereCullingParams),
    Vr {
        left: BoundingSphereCullingParams,
        right: BoundingSphereCullingParams,
    },
}

impl Default for BoundingSphereParams {
    fn default() -> Self {
        Self::SingleView(Default::default())
    }
}

#[derive(Resource)]
pub struct TextureSettings(pub textures::Settings);

#[derive(Resource)]
pub struct PipelineOptions(pub renderer_core::PipelineOptions);

#[derive(Resource)]
pub struct HttpClient<T: renderer_core::assets::HttpClient>(pub T);

#[derive(Resource)]
pub struct ProbesArrayInfo {
    pub bottom_left: Vec3,
    pub scale: Vec3,
}

impl ProbesArrayInfo {
    pub fn new(center: Vec3, scale: Vec3) -> Self {
        Self {
            bottom_left: center - scale / 2.0,
            scale,
        }
    }
}

#[derive(Resource)]
pub struct NewLightvolTextures(pub Option<LightvolTextures>);

pub struct LightvolTextures {
    pub sh0: url::Url,
    pub sh1_x: url::Url,
    pub sh1_y: url::Url,
    pub sh1_z: url::Url,
}
