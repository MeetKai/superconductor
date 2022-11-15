#[cfg(feature = "wasm")]
pub type PendingTexture =
    future::Shared<std::pin::Pin<Box<dyn Future<Output = Option<Arc<Texture>>> + 'static>>>;

#[cfg(not(feature = "wasm"))]
pub type PendingTexture =
    future::Shared<std::pin::Pin<Box<dyn Future<Output = Option<Arc<Texture>>> + Send + 'static>>>;

use crate::assets::materials::MaterialBindings;
use crate::assets::textures::{self, load_image_with_mime_type, ImageSource};
use crate::assets::HttpClient;
use crate::{spawn, Texture};
use arc_swap::ArcSwap;
use futures::future::{self, FutureExt, OptionFuture};
use glam::{Vec2, Vec3};
use gltf_helpers::Extensions;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

pub type MaterialBindGroup = Arc<ArcSwap<wgpu::BindGroup>>;

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

        let material_settings = load_material_settings(material);

        let material_bindings = MaterialBindings::new(
            &textures_context.device,
            &textures_context.queue,
            textures_context.bind_group_layouts.clone(),
            &material_settings,
        );

        let bind_group = Arc::new(ArcSwap::from_pointee(
            material_bindings
                .create_initial_bind_group(&textures_context.device, &textures_context.settings),
        ));

        materials.push(bind_group.clone());

        let device = textures_context.device.clone();
        let settings = textures_context.settings.clone();

        spawn(async move {
            let (albedo_texture, metallic_roughness_texture, normal_texture, emissive_texture) =
                futures::future::join4(
                    OptionFuture::from(albedo_future).map(|option| option.flatten()),
                    OptionFuture::from(metallic_roughness_future).map(|option| option.flatten()),
                    OptionFuture::from(normal_future).map(|option| option.flatten()),
                    OptionFuture::from(emissive_future).map(|option| option.flatten()),
                )
                .await;

            bind_group.store(Arc::new(material_bindings.create_bind_group(
                &device,
                &settings,
                crate::assets::materials::Textures {
                    albedo: albedo_texture,
                    metallic_roughness: metallic_roughness_texture,
                    normal: normal_texture,
                    emissive: emissive_texture,
                },
            )));

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

    shared_structs::MaterialSettings {
        base_color_factor: pbr.base_color_factor.into(),
        emissive_factor_x: emissive_factor.x,
        emissive_factor_y: emissive_factor.y,
        emissive_factor_z: emissive_factor.z,
        metallic_factor: pbr.metallic_factor,
        roughness_factor: pbr.roughness_factor,
        is_unlit: unlit as u32,
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
