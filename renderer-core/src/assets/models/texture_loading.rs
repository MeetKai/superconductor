#[cfg(feature = "wasm")]
pub type PendingTexture =
    future::Shared<std::pin::Pin<Box<dyn Future<Output = Option<Arc<Texture>>> + 'static>>>;

#[cfg(not(feature = "wasm"))]
pub type PendingTexture =
    future::Shared<std::pin::Pin<Box<dyn Future<Output = Option<Arc<Texture>>> + Send + 'static>>>;

use crate::assets::textures::{self, load_image_with_mime_type, ImageSource};
use crate::assets::HttpClient;
use crate::{spawn, Texture};
use futures::future::{self, FutureExt, OptionFuture};
use glam::{Vec2, Vec3};
use gltf_helpers::Extensions;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use wgpu::util::DeviceExt;

pub type MaterialBindGroup = Arc<crate::MutableBindGroup>;

pub fn image_index_from_texture_index(
    texture_index: usize,
    gltf: &goth_gltf::Gltf<Extensions>,
) -> anyhow::Result<usize> {
    let texture = match gltf.textures.get(texture_index) {
        Some(texture) => texture,
        None => {
            return Err(anyhow::anyhow!(
                "Texture index {} is out of range of {}",
                texture_index,
                gltf.textures.len()
            ))
        }
    };

    match texture
        .extensions
        .khr_texture_basisu
        .as_ref()
        .map(|ext| ext.source)
        .or(texture.source)
    {
        Some(source) => Ok(source),
        None => Err(anyhow::anyhow!("Texture {} has no source", texture_index)),
    }
}

fn load_single_pixel_image(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    format: wgpu::TextureFormat,
    bytes: &[u8; 4],
) -> Arc<Texture> {
    Arc::new(Texture::new(device.create_texture_with_data(
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
            view_formats: &[],
        },
        bytes,
    )))
}

pub fn start_loading_all_material_textures<T: HttpClient>(
    gltf: &goth_gltf::Gltf<Extensions>,
    root_url: url::Url,
    textures_context: textures::Context<T>,
    buffer_view_map: Arc<HashMap<usize, Vec<u8>>>,
) -> anyhow::Result<Vec<MaterialBindGroup>> {
    let mut pending_textures = Default::default();
    let mut materials = Vec::new();

    for material in &gltf.materials {
        let albedo_future =
            if let Some(tex_info) = material.pbr_metallic_roughness.base_color_texture.as_ref() {
                Some(start_loading_texture(
                    tex_info.index,
                    true,
                    gltf,
                    &mut pending_textures,
                    root_url.clone(),
                    textures_context.clone(),
                    buffer_view_map.clone(),
                )?)
            } else {
                None
            };

        let metallic_roughness_future = if let Some(tex_info) = material
            .pbr_metallic_roughness
            .metallic_roughness_texture
            .as_ref()
        {
            Some(start_loading_texture(
                tex_info.index,
                false,
                gltf,
                &mut pending_textures,
                root_url.clone(),
                textures_context.clone(),
                buffer_view_map.clone(),
            )?)
        } else {
            None
        };

        let normal_future = if let Some(tex_info) = material.normal_texture.as_ref() {
            Some(start_loading_texture(
                tex_info.index,
                false,
                gltf,
                &mut pending_textures,
                root_url.clone(),
                textures_context.clone(),
                buffer_view_map.clone(),
            )?)
        } else {
            None
        };

        let emissive_future = if let Some(tex_info) = material.emissive_texture.as_ref() {
            Some(start_loading_texture(
                tex_info.index,
                true,
                gltf,
                &mut pending_textures,
                root_url.clone(),
                textures_context.clone(),
                buffer_view_map.clone(),
            )?)
        } else {
            None
        };

        let linear_sampler = Arc::new(textures_context.device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::Repeat,
                address_mode_v: wgpu::AddressMode::Repeat,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                anisotropy_clamp: textures_context.settings.anisotropy_clamp,
                ..Default::default()
            },
        ));

        let material_settings = load_material_settings(material);

        let bind_group = Arc::new(crate::MutableBindGroup::new(
            &textures_context.device,
            &textures_context.bind_group_layouts.model,
            vec![
                crate::mutable_bind_group::Entry::Texture(load_single_pixel_image(
                    &textures_context.device,
                    &textures_context.queue,
                    wgpu::TextureFormat::Rgba8UnormSrgb,
                    &[255, 255, 255, 255],
                )),
                crate::mutable_bind_group::Entry::Texture(load_single_pixel_image(
                    &textures_context.device,
                    &textures_context.queue,
                    wgpu::TextureFormat::Rgba8Unorm,
                    &[127, 127, 255, 255],
                )),
                crate::mutable_bind_group::Entry::Texture(load_single_pixel_image(
                    &textures_context.device,
                    &textures_context.queue,
                    wgpu::TextureFormat::Rgba8Unorm,
                    &[0, 255, 255, 255],
                )),
                crate::mutable_bind_group::Entry::Texture(load_single_pixel_image(
                    &textures_context.device,
                    &textures_context.queue,
                    wgpu::TextureFormat::Rgba8UnormSrgb,
                    &[255, 255, 255, 255],
                )),
                // Note: previously I was creating a single material settings buffer per-model
                // and using different offsets for each material. Most browser/hardware combos
                // seem to accept this but I this seems to have caused an error on Chrome on a
                // M1 Mac.
                crate::mutable_bind_group::Entry::Buffer(
                    Arc::new(textures_context.device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("material settings"),
                            contents: bytemuck::bytes_of(&material_settings),
                            usage: wgpu::BufferUsages::UNIFORM,
                        },
                    )),
                    0,
                ),
                crate::mutable_bind_group::Entry::Sampler(linear_sampler),
            ],
        ));

        materials.push(bind_group.clone());

        let device = textures_context.device.clone();
        let bind_group_layouts = textures_context.bind_group_layouts.clone();

        spawn(async move {
            let (albedo_texture, metallic_roughness_texture, normal_texture, emissive_texture) =
                futures::future::join4(
                    OptionFuture::from(albedo_future).map(|option| option.flatten()),
                    OptionFuture::from(metallic_roughness_future).map(|option| option.flatten()),
                    OptionFuture::from(normal_future).map(|option| option.flatten()),
                    OptionFuture::from(emissive_future).map(|option| option.flatten()),
                )
                .await;

            bind_group.mutate(&device, &bind_group_layouts.model, |entries| {
                if let Some(albedo_texture) = albedo_texture {
                    entries[0] = crate::mutable_bind_group::Entry::Texture(albedo_texture);
                }

                if let Some(normal_texture) = normal_texture {
                    entries[1] = crate::mutable_bind_group::Entry::Texture(normal_texture);
                }

                if let Some(metallic_roughness_texture) = metallic_roughness_texture {
                    entries[2] =
                        crate::mutable_bind_group::Entry::Texture(metallic_roughness_texture);
                }

                if let Some(emissive_texture) = emissive_texture {
                    entries[3] = crate::mutable_bind_group::Entry::Texture(emissive_texture);
                }
            });

            Ok(())
        });
    }

    Ok(materials)
}

fn start_loading_texture<T: HttpClient>(
    texture_index: usize,
    srgb: bool,
    gltf: &goth_gltf::Gltf<Extensions>,
    pending_textures: &mut HashMap<usize, PendingTexture>,
    root_url: url::Url,
    textures_context: textures::Context<T>,
    buffer_view_map: Arc<HashMap<usize, Vec<u8>>>,
) -> anyhow::Result<PendingTexture> {
    let image_index = image_index_from_texture_index(texture_index, gltf)?;

    if let Some(future) = pending_textures.get(&image_index) {
        return Ok(future.clone());
    }

    let image: goth_gltf::Image = match gltf.images.get(image_index) {
        Some(image) => image.clone(),
        None => {
            return Err(anyhow::anyhow!(
                "Image index {} is out of range of {}",
                image_index,
                gltf.images.len()
            ))
        }
    };

    let future = async move {
        if let Some(uri) = &image.uri {
            let url = url::Url::options().base_url(Some(&root_url)).parse(uri)?;

            if url.scheme() == "data" {
                let (_mime_type, data) = url
                    .path()
                    .split_once(',')
                    .ok_or_else(|| anyhow::anyhow!("Failed to get data uri seperator"))?;

                let bytes = base64::decode(data)?;

                load_image_with_mime_type(
                    ImageSource::Bytes(&bytes),
                    srgb,
                    image.mime_type.as_ref().map(|string| &string[..]),
                    &textures_context,
                )
                .await
            } else {
                load_image_with_mime_type(
                    ImageSource::Url(url),
                    srgb,
                    image.mime_type.as_ref().map(|string| &string[..]),
                    &textures_context,
                )
                .await
            }
        } else if let Some(buffer_view_index) = image.buffer_view {
            let buffer_view_bytes = buffer_view_map.get(&buffer_view_index).unwrap();
            load_image_with_mime_type(
                ImageSource::Bytes(buffer_view_bytes),
                srgb,
                image.mime_type.as_ref().map(|string| &string[..]),
                &textures_context,
            )
            .await
        } else {
            Err(anyhow::anyhow!(
                "Neither an uri or a buffer view was specified for the image."
            ))
        }
    };

    let future = future.map(|result| match result {
        Ok(texture) => Some(texture),
        Err(error) => {
            log::error!("{}", error);
            None
        }
    });

    #[cfg(feature = "wasm")]
    let future = future.boxed_local().shared();

    #[cfg(not(feature = "wasm"))]
    let future = future.boxed().shared();

    pending_textures.insert(image_index, future.clone());

    Ok(future)
}

fn load_material_settings(
    material: &goth_gltf::Material<Extensions>,
) -> shared_structs::MaterialSettings {
    let unlit = material.extensions.khr_materials_unlit.is_some();

    let pbr = &material.pbr_metallic_roughness;

    let emissive_strength = material
        .extensions
        .khr_materials_emissive_strength
        .map(|emissive_strength| emissive_strength.emissive_strength)
        .unwrap_or(1.0);

    let texture_transform = pbr
        .base_color_texture
        .as_ref()
        .map(|info| info.extensions)
        .or_else(|| {
            pbr.metallic_roughness_texture
                .as_ref()
                .map(|info| info.extensions)
        })
        .or_else(|| material.normal_texture.as_ref().map(|info| info.extensions))
        .or_else(|| {
            material
                .emissive_texture
                .as_ref()
                .map(|info| info.extensions)
        })
        .and_then(|extensions| extensions.khr_texture_transform);

    let emissive_factor = Vec3::from(material.emissive_factor) * emissive_strength;

    let mut binary_settings = shared_structs::BinaryMaterialSettings::default();

    if unlit {
        binary_settings |= shared_structs::BinaryMaterialSettings::UNLIT;
    }

    shared_structs::MaterialSettings {
        base_color_factor: pbr.base_color_factor.into(),
        emissive_factor_x: emissive_factor.x,
        emissive_factor_y: emissive_factor.y,
        emissive_factor_z: emissive_factor.z,
        metallic_factor: pbr.metallic_factor,
        roughness_factor: pbr.roughness_factor,
        binary_settings,
        normal_map_scale: material
            .normal_texture
            .as_ref()
            .map(|info| info.scale)
            .unwrap_or(1.0),
        texture_transform_offset: texture_transform
            .map(|transform| Vec2::from(transform.offset))
            .unwrap_or(Vec2::ZERO),
        texture_transform_scale: texture_transform
            .map(|transform| Vec2::from(transform.scale))
            .unwrap_or(Vec2::ONE),
        texture_transform_rotation: texture_transform
            .map(|transform| transform.rotation)
            .unwrap_or(0.0),
    }
}
