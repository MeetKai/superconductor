use super::textures;
use crate::{BindGroupLayouts, Texture};
use std::sync::Arc;
use wgpu::util::DeviceExt;

fn load_single_pixel_image(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    format: wgpu::TextureFormat,
    bytes: &[u8; 4],
) -> Texture {
    Texture::new(device.create_texture_with_data(
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
    ))
}

pub struct Textures<T> {
    pub albedo: T,
    pub normal: T,
    pub metallic_roughness: T,
    pub emissive: T,
}

pub struct MaterialBindings {
    single_pixel_textures: Textures<Texture>,

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
            contents: bytemuck::bytes_of(material_settings),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        Self {
            bind_group_layouts,
            single_pixel_textures: Textures {
                emissive: load_single_pixel_image(
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
            },
            material_settings,
        }
    }

    pub fn create_initial_bind_group(
        &self,
        device: &wgpu::Device,
        settings: &textures::Settings,
    ) -> wgpu::BindGroup {
        self.create_bind_group(
            device,
            settings,
            Textures {
                albedo: None,
                normal: None,
                metallic_roughness: None,
                emissive: None,
            },
        )
    }

    pub fn create_bind_group(
        &self,
        device: &wgpu::Device,
        settings: &textures::Settings,
        incoming_textures: Textures<Option<Arc<Texture>>>,
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
                    resource: wgpu::BindingResource::TextureView(match &incoming_textures.albedo {
                        Some(albedo) => &albedo.view,
                        None => &self.single_pixel_textures.albedo.view,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(match &incoming_textures.normal {
                        Some(normal) => &normal.view,
                        None => &self.single_pixel_textures.normal.view,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(
                        match &incoming_textures.metallic_roughness {
                            Some(metallic_roughness) => &metallic_roughness.view,
                            None => &self.single_pixel_textures.metallic_roughness.view,
                        },
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(
                        match &incoming_textures.emissive {
                            Some(emissive) => &emissive.view,
                            None => &self.single_pixel_textures.emissive.view,
                        },
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: self.material_settings.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Sampler(&linear_sampler),
                },
            ],
        })
    }
}
