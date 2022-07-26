use super::textures;
use crate::{BindGroupLayouts, Texture};
use arc_swap::ArcSwap;
use crevice::std140::AsStd140;
use std::sync::Arc;
use wgpu::util::DeviceExt;

fn load_single_pixel_image(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    format: wgpu::TextureFormat,
    bytes: &[u8; 4],
) -> ArcSwap<Texture> {
    ArcSwap::from(Arc::new(Texture::new(device.create_texture_with_data(
        queue,
        &wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
        },
        bytes,
    ))))
}

pub struct MaterialBindings {
    pub albedo: ArcSwap<Texture>,
    pub normal: ArcSwap<Texture>,
    pub metallic_roughness: ArcSwap<Texture>,
    pub emission: ArcSwap<Texture>,

    material_settings: wgpu::Buffer,
    bind_group_layouts: Arc<BindGroupLayouts>,
}

impl MaterialBindings {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layouts: Arc<BindGroupLayouts>,
        material_settings: &shared_structs::MaterialSettings,
    ) -> Self {
        let material_settings = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("material settings"),
            contents: bytemuck::bytes_of(&material_settings.as_std140()),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        Self {
            bind_group_layouts,
            emission: load_single_pixel_image(
                device,
                queue,
                wgpu::TextureFormat::Rgba8UnormSrgb,
                &[255, 255, 255, 255],
            ),
            metallic_roughness: load_single_pixel_image(
                device,
                queue,
                wgpu::TextureFormat::Rgba8Unorm,
                &[0, 255, 0, 255],
            ),
            normal: load_single_pixel_image(
                device,
                queue,
                wgpu::TextureFormat::Rgba8Unorm,
                &[127, 127, 255, 255],
            ),
            albedo: load_single_pixel_image(
                device,
                queue,
                wgpu::TextureFormat::Rgba8UnormSrgb,
                &[255, 255, 255, 255],
            ),
            material_settings,
        }
    }

    pub fn create_bind_group(
        &self,
        device: &wgpu::Device,
        settings: &textures::Settings,
    ) -> wgpu::BindGroup {
        let linear_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            anisotropy_clamp: settings.anisotropy_clamp,
            ..Default::default()
        });

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.bind_group_layouts.model,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.albedo.load().view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&self.normal.load().view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(
                        &self.metallic_roughness.load().view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&self.emission.load().view),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: self.material_settings.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Sampler(&linear_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: wgpu::BindingResource::Sampler(&linear_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: wgpu::BindingResource::Sampler(&linear_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 8,
                    resource: wgpu::BindingResource::Sampler(&linear_sampler),
                },
            ],
        })
    }
}
