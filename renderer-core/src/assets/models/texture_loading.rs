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
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

pub fn image_index_from_texture_index(
    texture_index: usize,
    gltf: &goth_gltf::Gltf,
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
    gltf: &goth_gltf::Gltf,
    root_url: url::Url,
    textures_context: textures::Context<T>,
    buffer_view_map: Arc<HashMap<usize, Vec<u8>>>,
) -> anyhow::Result<HashMap<usize, PendingTexture>> {
    let mut pending_textures = Default::default();
    //let mut materials = Vec::new();

    for material in &gltf.materials {
        let albedo_future =
            if let Some(tex_info) = material.pbr_metallic_roughness.base_color_texture {
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

        let metallic_roughness_future =
            if let Some(tex_info) = material.pbr_metallic_roughness.metallic_roughness_texture {
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

        let normal_future = if let Some(tex_info) = material.normal_texture {
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

        let emissive_future = if let Some(tex_info) = material.emissive_texture {
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

        spawn(async move {
            let (albedo_texture, metallic_roughness_texture, normal_texture, emissive_texture) =
                futures::future::join4(
                    OptionFuture::from(albedo_future),
                    OptionFuture::from(metallic_roughness_future),
                    OptionFuture::from(normal_future),
                    OptionFuture::from(emissive_future),
                )
                .await;

            // todo: do something with these
            let _incoming_textures = crate::assets::materials::Textures {
                albedo: albedo_texture,
                metallic_roughness: metallic_roughness_texture,
                normal: normal_texture,
                emissive: emissive_texture,
            };

            Ok(())
        });
    }

    Ok(pending_textures)
}

fn start_loading_texture<T: HttpClient>(
    texture_index: usize,
    srgb: bool,
    gltf: &goth_gltf::Gltf,
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
