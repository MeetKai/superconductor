use crevice::std140::AsStd140;
use glam::{Vec2, Vec3};
use std::cell::RefCell;
use std::rc::Rc;
use wgpu::util::DeviceExt;

// TODO:
// * Use blitting to have mipmap for standard textures.

pub struct ModelLoadContext {
    pub device: Rc<wgpu::Device>,
    pub queue: Rc<wgpu::Queue>,
    pub fetched_images: Rc<RefCell<FetchedImages>>,
    pub model_bgl: Rc<wgpu::BindGroupLayout>,
    pub black_image: Rc<Texture>,
    pub white_image: Rc<Texture>,
    pub flat_normals_image: Rc<Texture>,
    pub default_metallic_roughness_image: Rc<Texture>,
    pub supported_features: wgpu::Features,
}

struct ModelBuffers {
    map: std::collections::HashMap<usize, Vec<u8>>,
}

struct MaterialTextures {
    normal_texture: Rc<Texture>,
    albedo_texture: Rc<Texture>,
    metallic_roughness_texture: Rc<Texture>,
    emissive_texture: Rc<Texture>,
}

pub struct ModelPrimitive {
    pub indices: wgpu::Buffer,
    pub positions: wgpu::Buffer,
    pub normals: wgpu::Buffer,
    pub uvs: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub num_indices: u32,
    // We hold handles onto the used textures here, so that when the model is dropped, the `Rc::strong_count`
    // of the textures goes down. Then we are able to unload the textures from GPU memory by `HashMap::retain`ing the fetched images..
    _textures: MaterialTextures,
}

struct StagingModelPrimitive {
    indices: Vec<u32>,
    positions: Vec<Vec3>,
    normals: Vec<Vec3>,
    uvs: Vec<Vec2>,
    material_index: usize,
    material_settings: wgpu::Buffer,
}

impl StagingModelPrimitive {
    async fn upload(
        self,
        gltf: &gltf::Gltf,
        context: &ModelLoadContext,
        buffers: &ModelBuffers,
        base_url: Option<url::Url>,
    ) -> ModelPrimitive {
        let material = gltf.materials().nth(self.material_index).unwrap();

        let pbr = material.pbr_metallic_roughness();

        let textures = MaterialTextures {
            albedo_texture: if let Some(albedo_texture) = pbr.base_color_texture() {
                load_image_from_gltf(
                    gltf,
                    &albedo_texture.texture(),
                    true,
                    buffers,
                    context,
                    base_url.as_ref(),
                )
                .await
            } else {
                context.white_image.clone()
            },
            normal_texture: if let Some(normal_texture) = material.normal_texture() {
                load_image_from_gltf(
                    gltf,
                    &normal_texture.texture(),
                    false,
                    buffers,
                    context,
                    base_url.as_ref(),
                )
                .await
            } else {
                context.flat_normals_image.clone()
            },
            metallic_roughness_texture: if let Some(metallic_roughness_texture) =
                pbr.metallic_roughness_texture()
            {
                load_image_from_gltf(
                    gltf,
                    &metallic_roughness_texture.texture(),
                    false,
                    buffers,
                    context,
                    base_url.as_ref(),
                )
                .await
            } else {
                context.default_metallic_roughness_image.clone()
            },
            emissive_texture: if let Some(emissive_texture) = material.emissive_texture() {
                load_image_from_gltf(
                    gltf,
                    &emissive_texture.texture(),
                    true,
                    buffers,
                    context,
                    base_url.as_ref(),
                )
                .await
            } else {
                context.black_image.clone()
            },
        };

        ModelPrimitive {
            bind_group: context
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &context.model_bgl,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(
                                &textures.albedo_texture.view,
                            ),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(
                                &textures.normal_texture.view,
                            ),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::TextureView(
                                &textures.metallic_roughness_texture.view,
                            ),
                        },
                        wgpu::BindGroupEntry {
                            binding: 3,
                            resource: wgpu::BindingResource::TextureView(
                                &textures.emissive_texture.view,
                            ),
                        },
                        wgpu::BindGroupEntry {
                            binding: 4,
                            resource: self.material_settings.as_entire_binding(),
                        },
                    ],
                }),
            num_indices: self.indices.len() as u32,
            indices: context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("indices"),
                    contents: bytemuck::cast_slice(&self.indices),
                    usage: wgpu::BufferUsages::INDEX,
                }),
            positions: context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("positions"),
                    contents: bytemuck::cast_slice(&self.positions),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            normals: context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("normals"),
                    contents: bytemuck::cast_slice(&self.normals),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            uvs: context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("uvs"),
                    contents: bytemuck::cast_slice(&self.uvs),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            _textures: textures,
        }
    }
}

#[derive(Default)]
pub struct Model {
    pub opaque_primitives: Rc<elsa::FrozenVec<Box<ModelPrimitive>>>,
    pub alpha_clipped_primitives: Rc<elsa::FrozenVec<Box<ModelPrimitive>>>,
}

pub async fn load_gltf_from_url(url: url::Url, context: Rc<ModelLoadContext>) -> Model {
    let bytes = fetch_bytes(&url).await;
    load_gltf_from_bytes(&bytes, Some(url), context).await
}

pub async fn load_gltf_from_bytes(
    bytes: &[u8],
    base_url: Option<url::Url>,
    context: Rc<ModelLoadContext>,
) -> Model {
    let gltf = gltf::Gltf::from_slice(bytes).unwrap();

    let mut buffers = ModelBuffers {
        map: Default::default(),
    };

    let node_tree = gltf_helpers::NodeTree::new(gltf.nodes());

    for buffer in gltf.buffers() {
        match buffer.source() {
            gltf::buffer::Source::Bin => {}
            gltf::buffer::Source::Uri(uri) => {
                let url = url::Url::options()
                    .base_url(base_url.as_ref())
                    .parse(uri)
                    .unwrap();

                if url.scheme() == "data" {
                    let (mime_type, data) = url.path().split_once(',').unwrap();
                    log::info!("Got: {}", mime_type);
                    buffers
                        .map
                        .insert(buffer.index(), base64::decode(data).unwrap());
                } else {
                    buffers.map.insert(buffer.index(), fetch_bytes(&url).await);
                }
            }
        }
    }

    let mut opaque_primitives = std::collections::HashMap::new();
    let mut alpha_clipped_primitives = std::collections::HashMap::new();

    for (node, mesh) in gltf
        .nodes()
        .filter_map(|node| node.mesh().map(|mesh| (node, mesh)))
    {
        let transform = node_tree.transform_of(node.index());

        for primitive in mesh.primitives() {
            let material = primitive.material();

            let primitive_map = match material.alpha_mode() {
                gltf::material::AlphaMode::Opaque => &mut opaque_primitives,
                _ => &mut alpha_clipped_primitives,
            };

            // We can't use `or_insert_with` here as that uses a closure and closures aren't async.
            let staging_primitive = match primitive_map.entry(material.index()) {
                std::collections::hash_map::Entry::Occupied(occupied) => occupied.into_mut(),
                std::collections::hash_map::Entry::Vacant(vacancy) => {
                    let pbr = material.pbr_metallic_roughness();

                    vacancy.insert(StagingModelPrimitive {
                        indices: Default::default(),
                        positions: Default::default(),
                        normals: Default::default(),
                        uvs: Default::default(),
                        material_settings: context.device.create_buffer_init(
                            &wgpu::util::BufferInitDescriptor {
                                label: Some("material settings"),
                                contents: bytemuck::bytes_of(
                                    &shared_structs::MaterialSettings {
                                        base_color_factor: pbr.base_color_factor().into(),
                                        emissive_factor: material.emissive_factor().into(),
                                        metallic_factor: pbr.metallic_factor(),
                                        roughness_factor: pbr.roughness_factor(),
                                    }
                                    .as_std140(),
                                ),
                                usage: wgpu::BufferUsages::UNIFORM,
                            },
                        ),
                        material_index: material.index().unwrap_or(0),
                    })
                }
            };

            let reader = primitive.reader(|buffer| match buffer.source() {
                gltf::buffer::Source::Bin => Some(gltf.blob.as_ref().unwrap()),
                gltf::buffer::Source::Uri(_) => {
                    buffers.map.get(&buffer.index()).map(|vec| &vec[..])
                }
            });

            staging_primitive.indices.extend(
                reader
                    .read_indices()
                    .unwrap()
                    .into_u32()
                    .map(|index| staging_primitive.positions.len() as u32 + index),
            );
            staging_primitive.positions.extend(
                reader
                    .read_positions()
                    .unwrap()
                    .map(|pos| transform * Vec3::from(pos)),
            );
            staging_primitive.normals.extend(
                reader
                    .read_normals()
                    .unwrap()
                    .map(|rot| transform.rotation * Vec3::from(rot)),
            );
            staging_primitive.uvs.extend(
                reader
                    .read_tex_coords(0)
                    .unwrap()
                    .into_f32()
                    .map(glam::Vec2::from),
            );
        }
    }

    let buffers = Rc::new(buffers);
    let gltf = Rc::new(gltf);

    let opaque_primitives_vec = Rc::new(elsa::FrozenVec::new());
    let alpha_clipped_primitives_vec = Rc::new(elsa::FrozenVec::new());

    for primitive in opaque_primitives.into_values() {
        let base_url = base_url.clone();
        let context = context.clone();
        let buffers = buffers.clone();
        let opaque_primitives_vec = opaque_primitives_vec.clone();
        let gltf = gltf.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let primitive = primitive.upload(&gltf, &context, &buffers, base_url).await;
            opaque_primitives_vec.push(Box::new(primitive));
        });
    }

    for primitive in alpha_clipped_primitives.into_values() {
        let base_url = base_url.clone();
        let context = context.clone();
        let buffers = buffers.clone();
        let alpha_clipped_primitives_vec = alpha_clipped_primitives_vec.clone();
        let gltf = gltf.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let primitive = primitive.upload(&gltf, &context, &buffers, base_url).await;
            alpha_clipped_primitives_vec.push(Box::new(primitive));
        })
    }

    Model {
        opaque_primitives: opaque_primitives_vec,
        alpha_clipped_primitives: alpha_clipped_primitives_vec,
    }
}

async fn load_image_from_gltf(
    gltf: &gltf::Gltf,
    texture: &gltf::Texture<'_>,
    srgb: bool,
    buffers: &ModelBuffers,
    context: &ModelLoadContext,
    base_url: Option<&url::Url>,
) -> Rc<Texture> {
    let image = texture.source();

    match image.source() {
        gltf::image::Source::View { view, mime_type } => {
            log::info!("{} {}", texture.index(), mime_type);
            let buffer = view.buffer();

            let buffer = match buffer.source() {
                gltf::buffer::Source::Bin => gltf.blob.as_ref().unwrap(),
                gltf::buffer::Source::Uri(_) => buffers
                    .map
                    .get(&buffer.index())
                    .map(|vec| &vec[..])
                    .unwrap(),
            };

            let bytes = &buffer[view.offset()..view.offset() + view.length()];

            Rc::new(load_image_from_mime_type(context, bytes, srgb, Some(mime_type)).await)
        }
        gltf::image::Source::Uri { uri, mime_type } => {
            let url = url::Url::options().base_url(base_url).parse(uri).unwrap();

            if url.scheme() == "data" {
                let (_mime_type, data) = url.path().split_once(',').unwrap();

                Rc::new(load_standard_image_format(
                    &context.device,
                    &context.queue,
                    &base64::decode(data).unwrap(),
                    srgb,
                ))
            } else {
                if let Some((image, cached_srgb)) = context.fetched_images.borrow().get(&url) {
                    if *cached_srgb == srgb {
                        return image.clone();
                    } else {
                        log::warn!(
                            "Same URL image is used twice, in both srgb and non-srgb formats: {}",
                            url
                        );
                    }
                }

                let bytes = fetch_bytes(&url).await;

                let image =
                    Rc::new(load_image_from_mime_type(context, &bytes, srgb, mime_type).await);

                context
                    .fetched_images
                    .borrow_mut()
                    .insert(url, (image.clone(), srgb));

                image
            }
        }
    }
}

async fn load_image_from_mime_type(
    context: &ModelLoadContext,
    bytes: &[u8],
    srgb: bool,
    mime_type: Option<&str>,
) -> Texture {
    if mime_type == Some("image/ktx2") {
        load_ktx2(&context.device, &context.queue, bytes)
    } else if mime_type == Some("image/x.basis") {
        load_basis(
            &context.device,
            &context.queue,
            context.supported_features,
            bytes,
            srgb,
        )
    } else {
        load_standard_image_format(&context.device, &context.queue, bytes, srgb)
    }
}

fn load_basis(
    device: &Rc<wgpu::Device>,
    queue: &wgpu::Queue,
    supported_features: wgpu::Features,
    bytes: &[u8],
    srgb: bool,
) -> Texture {
    let array = unsafe { js_sys::Uint8Array::view(bytes) };

    let file = basis_universal_wasm::BasisFile::new(&array);

    let image = 0;
    let format = if supported_features.contains(wgpu::Features::TEXTURE_COMPRESSION_ASTC_LDR) {
        Format::Astc
    } else if supported_features.contains(wgpu::Features::TEXTURE_COMPRESSION_BC) {
        Format::Bc7
    } else if supported_features.contains(wgpu::Features::TEXTURE_COMPRESSION_ETC2) {
        Format::Etc2Rgba
    } else {
        Format::Rgba
    };

    let num_levels = file.get_num_levels(image);

    let total_transcoded_size: u32 = (0..num_levels)
        .map(|level| file.get_image_transcoded_size_in_bytes(image, level, format as u32))
        .sum();

    let transcoded_data = vec![0; total_transcoded_size as usize];

    assert_eq!(file.start_transcoding(), 1);

    let mut offset = 0;

    for level in 0..num_levels {
        let size = file.get_image_transcoded_size_in_bytes(image, level, format as u32);

        let slice = unsafe {
            js_sys::Uint8Array::view(
                &transcoded_data[offset as usize..offset as usize + size as usize],
            )
        };

        offset += size;

        let res = file.transcode_image(&slice, image, level as u32, format as u32, 1, 0);

        assert_eq!(res, 1);
    }

    let width = file.get_image_width(image, 0);
    let height = file.get_image_height(image, 0);

    #[derive(Clone, Copy, Debug)]
    enum Format {
        Etc2Rgba = 1,
        Bc7 = 6,
        Astc = 10,
        Rgba = 13,
    }

    impl Format {
        fn as_wgpu(&self, srgb: bool) -> wgpu::TextureFormat {
            match self {
                Self::Etc2Rgba => {
                    if srgb {
                        wgpu::TextureFormat::Etc2Rgba8UnormSrgb
                    } else {
                        wgpu::TextureFormat::Etc2Rgba8Unorm
                    }
                }
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

    file.close();
    file.delete();

    let format = format.as_wgpu(srgb);

    Texture::new(device.create_texture_with_data(
        queue,
        &wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: num_levels,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
        },
        &transcoded_data,
    ))
}

fn load_ktx2(device: &Rc<wgpu::Device>, queue: &wgpu::Queue, bytes: &[u8]) -> Texture {
    let ktx2 = ktx2::Reader::new(bytes).unwrap();
    let header = ktx2.header();
    let mut levels = Vec::new();

    for level in ktx2.levels() {
        match header.supercompression_scheme {
            Some(ktx2::SupercompressionScheme::Zstandard) => {
                use std::io::Read;
                let mut cursor = std::io::Cursor::new(level);
                let mut decoded = Vec::new();
                ruzstd::StreamingDecoder::new(&mut cursor)
                    .unwrap()
                    .read_to_end(&mut decoded)
                    .unwrap();
                levels.push(std::borrow::Cow::Owned(decoded));
            }
            Some(other) => panic!("Unsupported: {:?}", other),
            None => {
                levels.push(std::borrow::Cow::Borrowed(level));
            }
        }
    }

    //let flattened: Vec<u8> = levels.iter().flat_map(|vec| vec.iter().cloned()).collect();

    for dfd in ktx2.data_format_descriptors() {
        if dfd.header == ktx2::DataFormatDescriptorHeader::BASIC {
            let basic_dfd = ktx2::BasicDataFormatDescriptor::parse(dfd.data).unwrap();
            let sample_information: Vec<_> = basic_dfd.sample_information().collect();
            log::info!("{:?} {:?}", basic_dfd.color_model, sample_information);
        }
    }

    todo!();
}

fn load_standard_image_format(
    device: &Rc<wgpu::Device>,
    queue: &wgpu::Queue,
    format_bytes: &[u8],
    srgb: bool,
) -> Texture {
    let image = image::load_from_memory(format_bytes).unwrap();

    let image = image.to_rgba8();

    let mip_level_count = mip_levels_for_image_size(image.width(), image.height());

    let format = if srgb {
        wgpu::TextureFormat::Rgba8UnormSrgb
    } else {
        wgpu::TextureFormat::Rgba8Unorm
    };

    let blit_textures: Vec<_> = (1..mip_level_count)
        .map(|i| {
            let width = image.width() >> i;
            let height = image.height() >> i;

            device.create_texture(&wgpu::TextureDescriptor {
                label: None,
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            })
        })
        .collect();

    Texture::new(create_texture_with_first_mip_data(
        device,
        queue,
        &wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: image.width(),
                height: image.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: if srgb {
                wgpu::TextureFormat::Rgba8UnormSrgb
            } else {
                wgpu::TextureFormat::Rgba8Unorm
            },
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
        },
        &*image,
    ))
}

pub fn load_single_pixel_image(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    format: wgpu::TextureFormat,
    bytes: &[u8; 4],
) -> Rc<Texture> {
    Rc::new(Texture::new(device.create_texture_with_data(
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
    )))
}

fn response_body_async_reader(response: web_sys::Response) -> impl futures::io::AsyncRead {
    use futures::{AsyncReadExt, StreamExt, TryStreamExt};

    let js_stream = wasm_streams::ReadableStream::from_raw(
        wasm_bindgen::JsValue::from(response.body().unwrap()).into(),
    );

    js_stream
        .into_stream()
        .map(|value| {
            let array: js_sys::Uint8Array = value.unwrap().into();
            let vec = array.to_vec();
            Ok(vec)
        })
        .into_async_read()
}

async fn fetch_bytes(url: &url::Url) -> Vec<u8> {
    let response: web_sys::Response = wasm_bindgen_futures::JsFuture::from(
        web_sys::window().unwrap().fetch_with_str(url.as_str()),
    )
    .await
    .unwrap()
    .into();

    let length = response.headers().get("content-length").unwrap().unwrap();
    let length: u64 = length.parse().unwrap();

    log::info!(
        "Fetching {}. Size in MB: {}",
        url,
        length as f32 / 1024.0 / 1024.0
    );

    use futures::AsyncReadExt;

    let mut async_read = response_body_async_reader(response);

    let mut buf = Vec::new();

    async_read.read_to_end(&mut buf).await.unwrap();

    buf
}

pub type FetchedImages = std::collections::HashMap<url::Url, (Rc<Texture>, bool)>;

pub fn prune_fetched_images(fetched_images: &mut FetchedImages) -> u32 {
    let mut removed = 0;

    fetched_images.retain(|_, (texture_ref, _)| {
        // Check the strong count. If a model is using the image then
        // it should be 2: the model + the one in this map. If less than 2
        // (it's normally impossible for strong_count to return 1 but w/e)
        // then we can drop it.
        if Rc::strong_count(texture_ref) < 2 {
            removed += 1;
            false
        } else {
            true
        }
    });

    removed
}

fn mip_levels_for_image_size(width: u32, height: u32) -> u32 {
    (width.max(height) as f32).log2() as u32 + 1
}

// Like the following, except without trying to write subsequent mips.
// https://github.com/gfx-rs/wgpu/blob/0b61a191244da0f0d987d53614a6698097a7622f/wgpu/src/util/device.rs#L79-L146
fn create_texture_with_first_mip_data(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    desc: &wgpu::TextureDescriptor,
    data: &[u8],
) -> wgpu::Texture {
    use std::num::NonZeroU32;

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

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl Texture {
    fn new(texture: wgpu::Texture) -> Self {
        Self {
            view: texture.create_view(&Default::default()),
            texture,
        }
    }
}
