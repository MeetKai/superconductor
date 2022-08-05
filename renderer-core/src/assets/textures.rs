use super::HttpClient;
use crate::{spawn, Texture};
use std::borrow::Cow;
use std::io::Read;
use std::num::NonZeroU32;
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[derive(Clone)]
pub struct Settings {
    pub anisotropy_clamp: Option<std::num::NonZeroU8>,
}

#[derive(Clone)]
pub struct Context<T> {
    pub pipelines: Arc<crate::Pipelines>,
    pub bind_group_layouts: Arc<crate::BindGroupLayouts>,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub http_client: T,
    pub settings: Settings,
}

pub struct IblData {
    pub texture: Arc<Texture>,
    pub padded_sphere_harmonics_bytes: [u8; 144],
    pub num_levels: u32,
}

pub async fn load_ibl_cubemap<T: HttpClient>(
    context: Context<T>,
    url: &url::Url,
) -> anyhow::Result<IblData> {
    let mut header_bytes = [0; ktx2::Header::LENGTH];

    let fetched_header = context
        .http_client
        .fetch_bytes(url, Some(0..ktx2::Header::LENGTH))
        .await?;

    if fetched_header.len() != ktx2::Header::LENGTH {
        return Err(anyhow::anyhow!("File did not respect the range request. Expected a response length of {} but got {}. Is the file 404ing?", ktx2::Header::LENGTH, fetched_header.len()));
    }

    header_bytes.copy_from_slice(&fetched_header);

    let header = ktx2::Header::from_bytes(&header_bytes)?;

    if header.face_count != 6 {
        return Err(anyhow::anyhow!(
            "Expected 6 faces, got {}",
            header.face_count
        ));
    }

    if header.format != Some(ktx2::Format::BC6H_UFLOAT_BLOCK) {
        return Err(anyhow::anyhow!(
            "Got an unsupported format: {:?}",
            header.format
        ));
    }

    if header.supercompression_scheme != Some(ktx2::SupercompressionScheme::Zstandard) {
        return Err(anyhow::anyhow!(
            "Got an unsupported supercompression scheme: {:?}",
            header.supercompression_scheme
        ));
    }

    let key_value_data = context
        .http_client
        .fetch_bytes(
            url,
            Some(
                header.index.kvd_byte_offset as usize
                    ..header.index.kvd_byte_offset as usize + header.index.kvd_byte_length as usize,
            ),
        )
        .await?;

    let sphere_harmonics_bytes = match ktx2::KeyValueDataIterator::new(&key_value_data)
        .find(|&(key, _)| key == "sphere_harmonics")
    {
        Some((_, value)) => value,
        None => {
            return Err(anyhow::anyhow!(
                "Missing `sphere_harmonics` in the key-value section"
            ))
        }
    };

    // Pad the 9 float32x3 colours to float32x4 by writing to a zeroed buffer.
    // 144 = 4 * 4 * 9.
    let mut padded_sphere_harmonics_bytes = [0; 144];

    for (i, chunk) in sphere_harmonics_bytes.chunks(12).take(9).enumerate() {
        padded_sphere_harmonics_bytes[i * 16..(i * 16) + 12].copy_from_slice(chunk);
    }

    let mut level_indices = Vec::with_capacity(header.level_count as usize);

    {
        let mut reader = std::io::Cursor::new(
            context
                .http_client
                .fetch_bytes(
                    url,
                    Some(
                        ktx2::Header::LENGTH
                            ..ktx2::Header::LENGTH
                                + ktx2::LevelIndex::LENGTH * header.level_count as usize,
                    ),
                )
                .await?,
        );

        for _ in 0..header.level_count {
            let mut level_index_bytes = [0; ktx2::LevelIndex::LENGTH];

            reader.read_exact(&mut level_index_bytes)?;

            level_indices.push(ktx2::LevelIndex::from_bytes(&level_index_bytes));
        }
    }

    // Compressed textures made made of 4x4 blocks, so there are some issues
    // with textures that don't have a side length divisible by 4.
    // They're considered fine everywhere except D3D11 and old versions of D3D12
    // (according to jasperrlz in the Wgpu Users element chat).
    let base_width = header.pixel_width - (header.pixel_width % 4);
    let base_height = header.pixel_height - (header.pixel_height % 4);

    let bc6h_supported = context
        .device
        .features()
        .contains(wgpu::Features::TEXTURE_COMPRESSION_BC);

    let texture_descriptor = move || wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width: base_width,
            height: base_height,
            depth_or_array_layers: 6,
        },
        mip_level_count: header.level_count,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: if bc6h_supported {
            wgpu::TextureFormat::Bc6hRgbUfloat
        } else {
            wgpu::TextureFormat::Rg11b10Float
        },
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    };

    let texture = context.device.create_texture(&texture_descriptor());

    let texture = Arc::new(Texture {
        view: texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        }),
        texture,
    });

    spawn({
        let texture = Arc::clone(&texture);
        let device = Arc::clone(&context.device);
        let queue = Arc::clone(&context.queue);
        let pipelines = Arc::clone(&context.pipelines);
        let bind_group_layouts = Arc::clone(&context.bind_group_layouts);

        let http_client = context.http_client.clone();
        let url = url.clone();

        async move {
            for (i, level_index) in level_indices.into_iter().enumerate().rev() {
                let bytes = http_client
                    .fetch_bytes(
                        &url,
                        Some(
                            level_index.byte_offset as usize
                                ..(level_index.byte_offset + level_index.byte_length) as usize,
                        ),
                    )
                    .await?;

                let decompressed =
                    zstd::bulk::decompress(&bytes, level_index.uncompressed_byte_length as usize)?;

                if !bc6h_supported {
                    let mut command_encoder = device.create_command_encoder(&Default::default());

                    let stride = decompressed.len() / 6;

                    for face in 0..6 {
                        let bytes = &decompressed[face * stride..(face + 1) * stride];

                        let input_texture = device.create_texture_with_data(
                            &queue,
                            &wgpu::TextureDescriptor {
                                label: None,
                                size: wgpu::Extent3d {
                                    width: base_width >> (i + 2),
                                    height: base_height >> (i + 2),
                                    depth_or_array_layers: 1,
                                },
                                mip_level_count: 1,
                                sample_count: 1,
                                dimension: wgpu::TextureDimension::D2,
                                format: wgpu::TextureFormat::Rgba32Uint,
                                usage: wgpu::TextureUsages::TEXTURE_BINDING,
                            },
                            bytes,
                        );

                        let output_texture = device.create_texture(&wgpu::TextureDescriptor {
                            label: None,
                            size: wgpu::Extent3d {
                                width: base_width >> i,
                                height: base_height >> i,
                                depth_or_array_layers: 1,
                            },
                            mip_level_count: 1,
                            sample_count: 1,
                            dimension: wgpu::TextureDimension::D2,
                            format: wgpu::TextureFormat::Rg11b10Float,
                            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                                | wgpu::TextureUsages::COPY_SRC,
                        });

                        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                            label: None,
                            layout: &bind_group_layouts.uint_texture,
                            entries: &[wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::TextureView(
                                    &input_texture.create_view(&Default::default()),
                                ),
                            }],
                        });

                        let output_view = output_texture.create_view(&Default::default());

                        let mut render_pass =
                            command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                label: None,
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view: &output_view,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Load,
                                        store: true,
                                    },
                                })],
                                depth_stencil_attachment: None,
                            });

                        render_pass.set_pipeline(&pipelines.bc6h_decompression);

                        render_pass.set_bind_group(0, &bind_group, &[]);

                        render_pass.draw(0..3, 0..1);

                        drop(render_pass);

                        command_encoder.copy_texture_to_texture(
                            wgpu::ImageCopyTexture {
                                texture: &output_texture,
                                mip_level: 0,
                                origin: wgpu::Origin3d::ZERO,
                                aspect: wgpu::TextureAspect::All,
                            },
                            wgpu::ImageCopyTexture {
                                texture: &texture.texture,
                                mip_level: i as u32,
                                origin: wgpu::Origin3d {
                                    x: 0,
                                    y: 0,
                                    z: face as u32,
                                },
                                aspect: wgpu::TextureAspect::All,
                            },
                            wgpu::Extent3d {
                                width: base_width >> i,
                                height: base_width >> i,
                                depth_or_array_layers: 1,
                            },
                        );
                    }

                    queue.submit(std::iter::once(command_encoder.finish()));
                } else {
                    write_bytes_to_texture(
                        &queue,
                        &texture.texture,
                        i as u32,
                        &decompressed,
                        &texture_descriptor(),
                    );
                }
            }

            Ok(())
        }
    });

    Ok(IblData {
        texture,
        num_levels: header.level_count,
        padded_sphere_harmonics_bytes,
    })
}

fn write_bytes_to_texture(
    queue: &wgpu::Queue,
    texture: &wgpu::Texture,
    mip: u32,
    bytes: &[u8],
    desc: &wgpu::TextureDescriptor,
) {
    let format_info = desc.format.describe();

    let mip_size = desc
        .mip_level_size(mip)
        .expect("cannot be None with a sensible `mip`.");

    let mip_physical = mip_size.physical_size(desc.format);

    let width_blocks = mip_physical.width / format_info.block_dimensions.0 as u32;
    let height_blocks = mip_physical.height / format_info.block_dimensions.1 as u32;

    let bytes_per_row = width_blocks * format_info.block_size as u32;

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture,
            mip_level: mip,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        bytes,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(NonZeroU32::new(bytes_per_row).expect("invalid bytes per row")),
            rows_per_image: Some(NonZeroU32::new(height_blocks).expect("invalid height")),
        },
        mip_physical,
    );
}

pub(super) enum ImageSource<'a> {
    Url(url::Url),
    Bytes(&'a [u8]),
}

impl<'a> ImageSource<'a> {
    async fn get_bytes<T: HttpClient>(&self, http_client: &T) -> anyhow::Result<Cow<'a, [u8]>> {
        Ok(match self {
            Self::Url(url) => Cow::Owned(http_client.fetch_bytes(url, None).await?),
            Self::Bytes(bytes) => Cow::Borrowed(bytes),
        })
    }

    fn extension(&self) -> Option<&str> {
        match &self {
            ImageSource::Url(url) => Some(url.path_segments()?.last()?.rsplit_once('.')?.1),
            ImageSource::Bytes(_) => None,
        }
    }
}

pub(super) async fn load_image_with_mime_type<T: HttpClient>(
    source: ImageSource<'_>,
    srgb: bool,
    mime_type: Option<&str>,
    context: &Context<T>,
) -> anyhow::Result<Arc<Texture>> {
    match (mime_type, source.extension()) {
        (Some("image/ktx2"), _) | (_, Some("ktx2")) => match source {
            ImageSource::Url(url) => load_ktx2_async(context, &url, srgb, |_| {}).await,
            _ => {
                return Err(anyhow::anyhow!(
                    "Loading ktx2 images from embedded bytes is currently unsupported"
                ))
            }
        },
        _ => {
            let (image, _size) = load_image_crate_image(
                &source.get_bytes(&context.http_client).await?,
                srgb,
                true,
                context,
            )?;

            Ok(image)
        }
    }
}

pub fn load_image_crate_image<T>(
    bytes: &[u8],
    srgb: bool,
    generate_mipmaps: bool,
    context: &Context<T>,
) -> anyhow::Result<(Arc<Texture>, wgpu::Extent3d)> {
    let image = image::load_from_memory(bytes)?;
    let image = image.to_rgba8();

    let mip_level_count = if generate_mipmaps {
        mip_levels_for_image_size(image.width(), image.height())
    } else {
        1
    };

    let format = if srgb {
        wgpu::TextureFormat::Rgba8UnormSrgb
    } else {
        wgpu::TextureFormat::Rgba8Unorm
    };

    let size = wgpu::Extent3d {
        width: image.width(),
        height: image.height(),
        depth_or_array_layers: 1,
    };

    let texture = Texture::new(create_texture_with_first_mip_data(
        &context.device,
        &context.queue,
        &wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
        },
        &image,
    ));

    if !generate_mipmaps {
        return Ok((Arc::new(texture), size));
    }

    let temp_blit_textures: Vec<_> = (1..mip_level_count)
        .map(|level| {
            let mip_extent = wgpu::Extent3d {
                width: (image.width() >> level).max(1),
                height: (image.height() >> level).max(1),
                depth_or_array_layers: 1,
            };

            Texture::new(context.device.create_texture(&wgpu::TextureDescriptor {
                label: None,
                size: mip_extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::COPY_SRC
                    | wgpu::TextureUsages::TEXTURE_BINDING,
            }))
        })
        .collect();

    let source_view = texture.texture.create_view(&wgpu::TextureViewDescriptor {
        mip_level_count: Some(std::num::NonZeroU32::new(1).expect("cannot be None")),
        ..Default::default()
    });

    let sampler = context.device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::Repeat,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        anisotropy_clamp: None,
        ..Default::default()
    });

    let mut encoder = context
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("command encoder"),
        });

    for source_level in 0..mip_level_count - 1 {
        let bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &context.bind_group_layouts.sampled_texture,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(if source_level == 0 {
                            &source_view
                        } else {
                            &temp_blit_textures[source_level as usize - 1].view
                        }),
                    },
                ],
            });

        let temp_blit_texture = &temp_blit_textures[source_level as usize];

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("blit render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &temp_blit_texture.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(if srgb {
            &context.pipelines.srgb_blit
        } else {
            &context.pipelines.blit
        });
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        drop(render_pass);

        let target_level = source_level + 1;

        encoder.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                texture: &temp_blit_texture.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyTexture {
                texture: &texture.texture,
                mip_level: source_level + 1,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: image.width() >> target_level,
                height: image.height() >> target_level,
                depth_or_array_layers: 1,
            },
        );
    }

    context.queue.submit(std::iter::once(encoder.finish()));

    Ok((Arc::new(texture), size))
}

// Like the following, except without trying to write subsequent mips.
// https://github.com/gfx-rs/wgpu/blob/0b61a191244da0f0d987d53614a6698097a7622f/wgpu/src/util/device.rs#L79-L146
pub(super) fn create_texture_with_first_mip_data(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    desc: &wgpu::TextureDescriptor,
    data: &[u8],
) -> wgpu::Texture {
    // Implicitly add the COPY_DST usage
    let mut desc = desc.to_owned();
    desc.usage |= wgpu::TextureUsages::COPY_DST;
    let texture = device.create_texture(&desc);

    let format_info = desc.format.describe();
    let layer_iterations = desc.array_layer_count();

    let mut binary_offset = 0;
    for layer in 0..layer_iterations {
        let width_blocks = desc.size.width / format_info.block_dimensions.0 as u32;
        let height_blocks = desc.size.height / format_info.block_dimensions.1 as u32;

        let bytes_per_row = width_blocks * format_info.block_size as u32;
        let data_size = bytes_per_row * height_blocks * desc.size.depth_or_array_layers;

        let end_offset = binary_offset + data_size as usize;

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: layer,
                },
                aspect: wgpu::TextureAspect::All,
            },
            &data[binary_offset..end_offset],
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(NonZeroU32::new(bytes_per_row).expect("invalid bytes per row")),
                rows_per_image: Some(NonZeroU32::new(height_blocks).expect("invalid height")),
            },
            desc.size,
        );

        binary_offset = end_offset;
    }

    texture
}

fn mip_levels_for_image_size(width: u32, height: u32) -> u32 {
    (width.max(height) as f32).log2() as u32 + 1
}

pub(crate) async fn load_ktx2_async<F: Fn(u32) + Send + 'static, T: HttpClient>(
    context: &Context<T>,
    url: &url::Url,
    srgb: bool,
    on_level_load: F,
) -> anyhow::Result<Arc<Texture>> {
    fn downscaling_for_max_size(texture_size: u32, max_size: u32) -> u32 {
        let texture_size_log = (texture_size as f32).log2();
        let max_size_log = (max_size as f32).log2();

        (texture_size_log as u32).saturating_sub(max_size_log as u32)
    }

    let mut header_bytes = [0; ktx2::Header::LENGTH];

    let fetched_header = context
        .http_client
        .fetch_bytes(url, Some(0..ktx2::Header::LENGTH))
        .await?;

    header_bytes.copy_from_slice(&fetched_header);

    let header = ktx2::Header::from_bytes(&header_bytes)?;

    if let Some(format) = header.format {
        return Err(anyhow::anyhow!(
            "Expected a UASTC texture, got {:?}",
            format
        ));
    }

    if header.supercompression_scheme != Some(ktx2::SupercompressionScheme::Zstandard) {
        return Err(anyhow::anyhow!(
            "Got an unsupported supercompression scheme: {:?}",
            header.supercompression_scheme
        ));
    }

    let down_scaling_level = downscaling_for_max_size(
        header.pixel_width.max(header.pixel_width),
        context.device.limits().max_texture_dimension_2d,
    )
    .min(header.level_count - 1);

    let mut level_indices = Vec::with_capacity(header.level_count as usize);

    {
        let mut reader = std::io::Cursor::new(
            context
                .http_client
                .fetch_bytes(
                    url,
                    Some(
                        ktx2::Header::LENGTH
                            ..ktx2::Header::LENGTH
                                + ktx2::LevelIndex::LENGTH * header.level_count as usize,
                    ),
                )
                .await?,
        );

        for _ in 0..header.level_count {
            let mut level_index_bytes = [0; ktx2::LevelIndex::LENGTH];

            reader.read_exact(&mut level_index_bytes)?;

            level_indices.push(ktx2::LevelIndex::from_bytes(&level_index_bytes));
        }
    }

    let format = CompressedTextureFormat::new_from_features(context.device.features());

    let mut downscaled_width = header.pixel_width >> down_scaling_level;
    let mut downscaled_height = header.pixel_height >> down_scaling_level;

    // We round down the width and height below, but if they're less than 3 then they're rounded down to 0.
    // The smallest block size is 4x4 so we just round the sizes up to that here.
    downscaled_width = downscaled_width.max(4);
    downscaled_height = downscaled_height.max(4);

    let texture_descriptor = move || wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            // Compressed textures made made of 4x4 blocks, so there are some issues
            // with textures that don't have a side length divisible by 4.
            // They're considered fine everywhere except D3D11 and old versions of D3D12
            // (according to jasperrlz in the Wgpu Users element chat).
            width: downscaled_width - (downscaled_width % 4),
            height: downscaled_height - (downscaled_height % 4),
            depth_or_array_layers: 1,
        },
        mip_level_count: header.level_count - down_scaling_level,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: format.as_wgpu(srgb),
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    };

    let texture = Arc::new(Texture::new(
        context.device.create_texture(&texture_descriptor()),
    ));

    let mut levels = level_indices.into_iter().enumerate().rev();

    // Load the smallest (1x1 pixel) mip first before returning the texture
    {
        let (i, level_index) = match levels.next() {
            Some((i, level_index)) => (i, level_index),
            None => return Err(anyhow::anyhow!("No level indices in the file")),
        };

        let bytes = context
            .http_client
            .fetch_bytes(
                url,
                Some(
                    level_index.byte_offset as usize
                        ..(level_index.byte_offset + level_index.byte_length) as usize,
                ),
            )
            .await?;

        let transcoder = basis_universal::LowLevelUastcTranscoder::new();

        let decompressed =
            zstd::bulk::decompress(&bytes, level_index.uncompressed_byte_length as usize)?;

        let slice_width = header.pixel_width >> i;
        let slice_height = header.pixel_height >> i;

        let (block_width_pixels, block_height_pixels) = (4, 4);

        let transcoded = transcoder
            .transcode_slice(
                &decompressed,
                basis_universal::SliceParametersUastc {
                    num_blocks_x: ((slice_width + block_width_pixels - 1) / block_width_pixels)
                        .max(1),
                    num_blocks_y: ((slice_height + block_height_pixels - 1) / block_height_pixels)
                        .max(1),
                    has_alpha: false,
                    original_width: slice_width,
                    original_height: slice_height,
                },
                basis_universal::DecodeFlags::HIGH_QUALITY,
                format.as_transcoder_block_format(),
            )
            .map_err(|err| anyhow::anyhow!("Transcoder error: {:?}", err))?;

        write_bytes_to_texture(
            &context.queue,
            &texture.texture,
            i as u32 - down_scaling_level,
            &transcoded,
            &texture_descriptor(),
        );

        on_level_load(i as u32 - down_scaling_level)
    }

    // Load all other mips in the background.
    spawn({
        //let url = Rc::clone(url);
        let texture = Arc::clone(&texture);
        let url = url.clone();
        let http_client = context.http_client.clone();
        let queue = context.queue.clone();

        async move {
            for (i, level_index) in levels {
                if i < down_scaling_level as usize {
                    return Ok(());
                }

                let bytes = http_client
                    .fetch_bytes(
                        &url,
                        Some(
                            level_index.byte_offset as usize
                                ..(level_index.byte_offset + level_index.byte_length) as usize,
                        ),
                    )
                    .await?;

                let transcoder = basis_universal::LowLevelUastcTranscoder::new();

                let decompressed =
                    zstd::bulk::decompress(&bytes, level_index.uncompressed_byte_length as usize)?;

                let slice_width = header.pixel_width >> i;
                let slice_height = header.pixel_height >> i;

                let (block_width_pixels, block_height_pixels) = (4, 4);

                let transcoded = transcoder
                    .transcode_slice(
                        &decompressed,
                        basis_universal::SliceParametersUastc {
                            num_blocks_x: ((slice_width + block_width_pixels - 1)
                                / block_width_pixels)
                                .max(1),
                            num_blocks_y: ((slice_height + block_height_pixels - 1)
                                / block_height_pixels)
                                .max(1),
                            has_alpha: false,
                            original_width: slice_width,
                            original_height: slice_height,
                        },
                        basis_universal::DecodeFlags::HIGH_QUALITY,
                        format.as_transcoder_block_format(),
                    )
                    .map_err(|err| anyhow::anyhow!("Transcoder error: {:?}", err))?;

                write_bytes_to_texture(
                    &queue,
                    &texture.texture,
                    i as u32 - down_scaling_level,
                    &transcoded,
                    &texture_descriptor(),
                );

                on_level_load(i as u32 - down_scaling_level)
            }

            Ok(())
        }
    });

    Ok(texture)
}

#[derive(Clone, Copy, Debug)]
enum CompressedTextureFormat {
    Bc7,
    Astc,
    Rgba,
}

impl CompressedTextureFormat {
    // https://github.com/KhronosGroup/3D-Formats-Guidelines/blob/main/KTXDeveloperGuide.md#primary-transcode-targets
    // suggests we have Astc as 1st priority, Bc7 as second and then fallback to uncompressed rgba.
    fn new_from_features(features: wgpu::Features) -> Self {
        if features.contains(wgpu::Features::TEXTURE_COMPRESSION_ASTC_LDR) {
            Self::Astc
        } else if features.contains(wgpu::Features::TEXTURE_COMPRESSION_BC) {
            Self::Bc7
        } else {
            Self::Rgba
        }
    }

    fn as_transcoder_block_format(&self) -> basis_universal::TranscoderBlockFormat {
        match self {
            Self::Bc7 => basis_universal::TranscoderBlockFormat::BC7,
            Self::Astc => basis_universal::TranscoderBlockFormat::ASTC_4x4,
            Self::Rgba => basis_universal::TranscoderBlockFormat::RGBA32,
        }
    }

    fn as_wgpu(&self, srgb: bool) -> wgpu::TextureFormat {
        match self {
            Self::Astc => wgpu::TextureFormat::Astc {
                block: wgpu::AstcBlock::B4x4,
                channel: if srgb {
                    wgpu::AstcChannel::UnormSrgb
                } else {
                    wgpu::AstcChannel::Unorm
                },
            },
            Self::Rgba => {
                if srgb {
                    wgpu::TextureFormat::Rgba8UnormSrgb
                } else {
                    wgpu::TextureFormat::Rgba8Unorm
                }
            }
            Self::Bc7 => {
                if srgb {
                    wgpu::TextureFormat::Bc7RgbaUnormSrgb
                } else {
                    wgpu::TextureFormat::Bc7RgbaUnorm
                }
            }
        }
    }
}
