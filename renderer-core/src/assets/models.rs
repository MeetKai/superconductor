use super::materials::MaterialBindings;
use super::textures::{self, load_image_with_mime_type, ImageSource};
use super::HttpClient;
use crate::{
    spawn,
    utils::{Setter, Swappable},
    BindGroupLayouts, Texture,
};
use glam::{UVec4, Vec2, Vec3, Vec4};
use gltf_helpers::{
    animation::{read_animations, Animation},
    Similarity,
};
use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;

pub struct Context<T> {
    pub http_client: T,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub bind_group_layouts: Arc<BindGroupLayouts>,
    pub vertex_buffers: Arc<crate::buffers::VertexBuffers>,
    pub animated_vertex_buffers: Arc<crate::buffers::AnimatedVertexBuffers>,
    pub index_buffer: Arc<crate::buffers::IndexBuffer>,
    pub pipelines: Arc<crate::Pipelines>,
    pub texture_settings: textures::Settings,
}

#[derive(Default, Debug)]
pub struct PrimitiveRanges {
    pub opaque: Range<usize>,
    pub alpha_clipped: Range<usize>,
    pub opaque_double_sided: Range<usize>,
    pub alpha_clipped_double_sided: Range<usize>,
}

fn get_buffer<'a>(
    gltf: &'a gltf::Gltf,
    buffer_map: &'a HashMap<usize, Vec<u8>>,
    buffer: gltf::Buffer,
) -> Option<&'a [u8]> {
    match buffer.source() {
        gltf::buffer::Source::Bin => Some(gltf.blob.as_ref().unwrap()),
        gltf::buffer::Source::Uri(_) => buffer_map.get(&buffer.index()).map(|vec| &vec[..]),
    }
}

fn collect_primitives<
    'a,
    T: HttpClient,
    I: std::iter::Iterator<Item = &'a StagingPrimitive<StagingBuffers>>,
>(
    primitives: &mut Vec<Primitive>,
    staging_buffers: &mut StagingBuffers,
    staging_primitives: I,
    context: &Context<T>,
    gltf: Arc<gltf::Gltf>,
    buffer_map: Arc<HashMap<usize, Vec<u8>>>,
    root_url: &url::Url,
) -> Range<usize> {
    let primitives_start = primitives.len();

    for staging_primitive in staging_primitives {
        let material_bindings = MaterialBindings::new(
            &context.device,
            &context.queue,
            context.bind_group_layouts.clone(),
            &staging_primitive.material_settings,
        );

        let bind_group = Swappable::new(Arc::new(
            material_bindings.create_bind_group(&context.device, &context.texture_settings),
        ));

        let bind_group_setter = bind_group.setter.clone();

        primitives.push(Primitive {
            index_buffer_range: staging_buffers.collect(&staging_primitive.buffers),
            bind_group,
        });

        spawn_texture_loading_futures(
            bind_group_setter,
            material_bindings,
            staging_primitive.material_index,
            gltf.clone(),
            buffer_map.clone(),
            context,
            root_url,
        )
    }

    let primitives_end = primitives.len();

    primitives_start..primitives_end
}

fn collect_animated_primitives<
    'a,
    T: HttpClient,
    I: std::iter::Iterator<Item = &'a StagingPrimitive<AnimatedStagingBuffers>>,
>(
    primitives: &mut Vec<Primitive>,
    staging_buffers: &mut AnimatedStagingBuffers,
    staging_primitives: I,
    context: &Context<T>,
    gltf: Arc<gltf::Gltf>,
    buffer_map: Arc<HashMap<usize, Vec<u8>>>,
    root_url: &url::Url,
) -> Range<usize> {
    let primitives_start = primitives.len();

    for staging_primitive in staging_primitives {
        let material_bindings = MaterialBindings::new(
            &context.device,
            &context.queue,
            context.bind_group_layouts.clone(),
            &staging_primitive.material_settings,
        );

        let bind_group = Swappable::new(Arc::new(
            material_bindings.create_bind_group(&context.device, &context.texture_settings),
        ));

        let bind_group_setter = bind_group.setter.clone();

        primitives.push(Primitive {
            index_buffer_range: staging_buffers.collect(&staging_primitive.buffers),
            bind_group,
        });

        spawn_texture_loading_futures(
            bind_group_setter,
            material_bindings,
            staging_primitive.material_index,
            gltf.clone(),
            buffer_map.clone(),
            context,
            root_url,
        )
    }

    let primitives_end = primitives.len();

    primitives_start..primitives_end
}

pub struct AnimatedModelData {
    pub animations: Vec<Animation>,
    pub depth_first_nodes: Vec<(usize, Option<usize>)>,
    pub inverse_bind_transforms: Vec<Similarity>,
    pub joint_indices_to_node_indices: Vec<usize>,
}

pub struct Model {
    pub primitives: Vec<Primitive>,
    pub primitive_ranges: PrimitiveRanges,
    pub index_buffer_range: Range<u32>,
    pub vertex_buffer_range: Range<u32>,
}

impl Model {
    pub async fn load<T: HttpClient>(
        context: &Context<T>,
        root_url: &url::Url,
    ) -> anyhow::Result<Self> {
        let gltf: gltf::Gltf<()> =
            gltf::Gltf::from_slice(&context.http_client.fetch_bytes(root_url, None).await?)?;

        let mut buffer_map = HashMap::new();

        let node_tree = gltf_helpers::NodeTree::new(gltf.nodes());

        for buffer in gltf.buffers() {
            match buffer.source() {
                gltf::buffer::Source::Bin => {}
                gltf::buffer::Source::Uri(uri) => {
                    let url = url::Url::options().base_url(Some(root_url)).parse(uri)?;

                    if url.scheme() == "data" {
                        let (_mime_type, data) = url.path().split_once(',').unwrap();
                        log::warn!("Loading buffers from embedded base64 is inefficient. Consider moving the buffers into a seperate file.");
                        buffer_map.insert(buffer.index(), base64::decode(data)?);
                    } else {
                        buffer_map.insert(
                            buffer.index(),
                            context.http_client.fetch_bytes(&url, None).await?,
                        );
                    }
                }
            }
        }

        // What we're doing here is essentially collecting all the model primitives that share a meterial together
        // to reduce the number of draw calls.
        let mut opaque_primitives = HashMap::new();
        let mut alpha_clipped_primitives = HashMap::new();
        let mut opaque_double_sided_primitives = HashMap::new();
        let mut alpha_clipped_double_sided_primitives = HashMap::new();

        for (node, mesh) in gltf
            .nodes()
            .filter_map(|node| node.mesh().map(|mesh| (node, mesh)))
        {
            let transform = node_tree.transform_of(node.index());

            for primitive in mesh.primitives() {
                let material = primitive.material();

                // Note: it's possible to render double-sided objects with a backface-culling shader if we double the
                // triangles in the index buffer but with a backwards winding order. It's only worth doing this to keep
                // the number of shader permutations down.
                //
                // One thing to keep in mind is that we flip the shading normals according to the gltf spec:
                // https://www.khronos.org/registry/glTF/specs/2.0/glTF-2.0.html#double-sided

                let primitive_map = match (material.alpha_mode(), material.double_sided()) {
                    (gltf::material::AlphaMode::Opaque, false) => &mut opaque_primitives,
                    (_, false) => &mut alpha_clipped_primitives,
                    (gltf::material::AlphaMode::Opaque, true) => {
                        &mut opaque_double_sided_primitives
                    }
                    (_, true) => &mut alpha_clipped_double_sided_primitives,
                };

                let reader = primitive.reader(|buffer| get_buffer(&gltf, &buffer_map, buffer));

                // Workaround for some exporters (Scaniverse) exporting scanned models that are meant to be
                // rendered unlit but don't set the material flag.
                let unlit = material.unlit() || reader.read_normals().is_none();

                let staging_primitive =
                    primitive_map.entry(material.index()).or_insert_with(|| {
                        let pbr = material.pbr_metallic_roughness();

                        StagingPrimitive {
                            buffers: StagingBuffers::default(),
                            material_settings: shared_structs::MaterialSettings {
                                base_color_factor: pbr.base_color_factor().into(),
                                emissive_factor: material.emissive_factor().into(),
                                metallic_factor: pbr.metallic_factor(),
                                roughness_factor: pbr.roughness_factor(),
                                is_unlit: unlit as u32,
                                // It seems like uniform buffer padding works differently in the wgpu Vulkan backends vs the WebGL2 backend.
                                // todo: find a nicer way to resolve this.
                                #[cfg(not(feature = "wasm"))]
                                _padding: 0,
                            },
                            material_index: material.index().unwrap_or(0),
                        }
                    });

                staging_primitive.buffers.indices.extend(
                    reader
                        .read_indices()
                        .unwrap()
                        .into_u32()
                        .map(|index| staging_primitive.buffers.positions.len() as u32 + index),
                );

                let start_positions = staging_primitive.buffers.positions.len();

                staging_primitive.buffers.positions.extend(
                    reader
                        .read_positions()
                        .unwrap()
                        .map(|pos| transform * Vec3::from(pos)),
                );

                let num_positions = staging_primitive.buffers.positions.len() - start_positions;

                match reader.read_normals() {
                    Some(normals) => staging_primitive
                        .buffers
                        .normals
                        .extend(normals.map(|normal| transform.rotation * Vec3::from(normal))),
                    None => staging_primitive
                        .buffers
                        .normals
                        .extend(std::iter::repeat(Vec3::ZERO).take(num_positions)),
                }
                staging_primitive.buffers.uvs.extend(
                    reader
                        .read_tex_coords(0)
                        .unwrap()
                        .into_f32()
                        .map(glam::Vec2::from),
                );
            }
        }

        // Collect all the buffers for the primitives into one big staging buffer
        // and collect all the primitive ranges into one big vector.

        let mut staging_buffers = StagingBuffers::default();

        let mut primitives = Vec::new();
        let gltf = Arc::new(gltf);
        let buffer_map = Arc::new(buffer_map);

        let primitive_ranges = PrimitiveRanges {
            opaque: collect_primitives(
                &mut primitives,
                &mut staging_buffers,
                opaque_primitives.values(),
                context,
                gltf.clone(),
                buffer_map.clone(),
                root_url,
            ),
            alpha_clipped: collect_primitives(
                &mut primitives,
                &mut staging_buffers,
                alpha_clipped_primitives.values(),
                context,
                gltf.clone(),
                buffer_map.clone(),
                root_url,
            ),
            opaque_double_sided: collect_primitives(
                &mut primitives,
                &mut staging_buffers,
                opaque_double_sided_primitives.values(),
                context,
                gltf.clone(),
                buffer_map.clone(),
                root_url,
            ),
            alpha_clipped_double_sided: collect_primitives(
                &mut primitives,
                &mut staging_buffers,
                alpha_clipped_double_sided_primitives.values(),
                context,
                gltf,
                buffer_map,
                root_url,
            ),
        };

        let mut command_encoder =
            context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("command encoder"),
                });

        let vertex_buffer_range = context.vertex_buffers.insert(
            &staging_buffers.positions,
            &staging_buffers.normals,
            &staging_buffers.uvs,
            &context.device,
            &context.queue,
            &mut command_encoder,
        );

        // Make sure the indices point to the right vertices.
        for index in &mut staging_buffers.indices {
            *index += vertex_buffer_range.start;
        }

        let index_buffer_range = context.index_buffer.insert(
            &staging_buffers.indices,
            &context.device,
            &context.queue,
            &mut command_encoder,
        );

        context
            .queue
            .submit(std::iter::once(command_encoder.finish()));

        // Make sure the primitive index ranges are absolute from the start of the buffer.
        for primitive in &mut primitives {
            primitive.index_buffer_range.start += index_buffer_range.start;
            primitive.index_buffer_range.end += index_buffer_range.start;
        }

        Ok(Model {
            primitives,
            primitive_ranges,
            index_buffer_range,
            vertex_buffer_range,
        })
    }
}

pub struct AnimatedModel {
    pub primitives: Vec<Primitive>,
    pub primitive_ranges: PrimitiveRanges,
    pub index_buffer_range: Range<u32>,
    pub vertex_buffer_range: Range<u32>,
    pub animation_data: AnimatedModelData,
}

impl AnimatedModel {
    pub async fn load<T: HttpClient>(
        context: &Context<T>,
        root_url: &url::Url,
    ) -> anyhow::Result<Self> {
        let gltf: gltf::Gltf<()> =
            gltf::Gltf::from_slice(&context.http_client.fetch_bytes(root_url, None).await?)?;

        let mut buffer_map = HashMap::new();

        let node_tree = gltf_helpers::NodeTree::new(gltf.nodes());

        for buffer in gltf.buffers() {
            match buffer.source() {
                gltf::buffer::Source::Bin => {}
                gltf::buffer::Source::Uri(uri) => {
                    let url = url::Url::options().base_url(Some(root_url)).parse(uri)?;

                    if url.scheme() == "data" {
                        let (_mime_type, data) = url.path().split_once(',').unwrap();
                        log::warn!("Loading buffers from embedded base64 is inefficient. Consider moving the buffers into a seperate file.");
                        buffer_map.insert(buffer.index(), base64::decode(data)?);
                    } else {
                        buffer_map.insert(
                            buffer.index(),
                            context.http_client.fetch_bytes(&url, None).await?,
                        );
                    }
                }
            }
        }

        // What we're doing here is essentially collecting all the model primitives that share a meterial together
        // to reduce the number of draw calls.
        let mut opaque_primitives = HashMap::new();
        let mut alpha_clipped_primitives = HashMap::new();
        let mut opaque_double_sided_primitives = HashMap::new();
        let mut alpha_clipped_double_sided_primitives = HashMap::new();

        for (node, mesh) in gltf
            .nodes()
            .filter_map(|node| node.mesh().map(|mesh| (node, mesh)))
        {
            let transform = node_tree.transform_of(node.index());

            for primitive in mesh.primitives() {
                let material = primitive.material();

                // Note: it's possible to render double-sided objects with a backface-culling shader if we double the
                // triangles in the index buffer but with a backwards winding order. It's only worth doing this to keep
                // the number of shader permutations down.
                //
                // One thing to keep in mind is that we flip the shading normals according to the gltf spec:
                // https://www.khronos.org/registry/glTF/specs/2.0/glTF-2.0.html#double-sided

                let primitive_map = match (material.alpha_mode(), material.double_sided()) {
                    (gltf::material::AlphaMode::Opaque, false) => &mut opaque_primitives,
                    (_, false) => &mut alpha_clipped_primitives,
                    (gltf::material::AlphaMode::Opaque, true) => {
                        &mut opaque_double_sided_primitives
                    }
                    (_, true) => &mut alpha_clipped_double_sided_primitives,
                };

                let reader = primitive.reader(|buffer| get_buffer(&gltf, &buffer_map, buffer));

                // Workaround for some exporters (Scaniverse) exporting scanned models that are meant to be
                // rendered unlit but don't set the material flag.
                let unlit = material.unlit() || reader.read_normals().is_none();

                let staging_primitive =
                    primitive_map.entry(material.index()).or_insert_with(|| {
                        let pbr = material.pbr_metallic_roughness();

                        StagingPrimitive {
                            buffers: AnimatedStagingBuffers::default(),
                            material_settings: shared_structs::MaterialSettings {
                                base_color_factor: pbr.base_color_factor().into(),
                                emissive_factor: material.emissive_factor().into(),
                                metallic_factor: pbr.metallic_factor(),
                                roughness_factor: pbr.roughness_factor(),
                                is_unlit: unlit as u32,
                                // It seems like uniform buffer padding works differently in the wgpu Vulkan backends vs the WebGL2 backend.
                                // todo: find a nicer way to resolve this.
                                #[cfg(not(feature = "wasm"))]
                                _padding: 0,
                            },
                            material_index: material.index().unwrap_or(0),
                        }
                    });

                staging_primitive.buffers.indices.extend(
                    reader
                        .read_indices()
                        .unwrap()
                        .into_u32()
                        .map(|index| staging_primitive.buffers.positions.len() as u32 + index),
                );

                let start_positions = staging_primitive.buffers.positions.len();

                staging_primitive.buffers.positions.extend(
                    reader
                        .read_positions()
                        .unwrap()
                        .map(|pos| transform * Vec3::from(pos)),
                );

                let num_positions = staging_primitive.buffers.positions.len() - start_positions;

                match reader.read_normals() {
                    Some(normals) => staging_primitive
                        .buffers
                        .normals
                        .extend(normals.map(|normal| transform.rotation * Vec3::from(normal))),
                    None => staging_primitive
                        .buffers
                        .normals
                        .extend(std::iter::repeat(Vec3::ZERO).take(num_positions)),
                }
                staging_primitive.buffers.uvs.extend(
                    reader
                        .read_tex_coords(0)
                        .unwrap()
                        .into_f32()
                        .map(glam::Vec2::from),
                );

                staging_primitive.buffers.joint_indices.extend(
                    reader.read_joints(0).unwrap().into_u16().map(|indices| {
                        UVec4::new(
                            indices[0] as u32,
                            indices[1] as u32,
                            indices[2] as u32,
                            indices[3] as u32,
                        )
                    }),
                );

                staging_primitive.buffers.joint_weights.extend(
                    reader
                        .read_weights(0)
                        .unwrap()
                        .into_f32()
                        .map(|weights| Vec4::from(weights)),
                );
            }
        }

        // Collect all the buffers for the primitives into one big staging buffer
        // and collect all the primitive ranges into one big vector.

        let mut staging_buffers = AnimatedStagingBuffers::default();

        let mut primitives = Vec::new();
        let gltf = Arc::new(gltf);
        let buffer_map = Arc::new(buffer_map);

        let primitive_ranges = PrimitiveRanges {
            opaque: collect_animated_primitives(
                &mut primitives,
                &mut staging_buffers,
                opaque_primitives.values(),
                context,
                gltf.clone(),
                buffer_map.clone(),
                root_url,
            ),
            alpha_clipped: collect_animated_primitives(
                &mut primitives,
                &mut staging_buffers,
                alpha_clipped_primitives.values(),
                context,
                gltf.clone(),
                buffer_map.clone(),
                root_url,
            ),
            opaque_double_sided: collect_animated_primitives(
                &mut primitives,
                &mut staging_buffers,
                opaque_double_sided_primitives.values(),
                context,
                gltf.clone(),
                buffer_map.clone(),
                root_url,
            ),
            alpha_clipped_double_sided: collect_animated_primitives(
                &mut primitives,
                &mut staging_buffers,
                alpha_clipped_double_sided_primitives.values(),
                context,
                gltf.clone(),
                buffer_map.clone(),
                root_url,
            ),
        };

        let mut command_encoder =
            context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("command encoder"),
                });

        let vertex_buffer_range = context.animated_vertex_buffers.insert(
            &staging_buffers.positions,
            &staging_buffers.normals,
            &staging_buffers.uvs,
            &staging_buffers.joint_indices,
            &staging_buffers.joint_weights,
            &context.device,
            &context.queue,
            &mut command_encoder,
        );

        // Make sure the indices point to the right vertices.
        for index in &mut staging_buffers.indices {
            *index += vertex_buffer_range.start;
        }

        let index_buffer_range = context.index_buffer.insert(
            &staging_buffers.indices,
            &context.device,
            &context.queue,
            &mut command_encoder,
        );

        context
            .queue
            .submit(std::iter::once(command_encoder.finish()));

        // Make sure the primitive index ranges are absolute from the start of the buffer.
        for primitive in &mut primitives {
            primitive.index_buffer_range.start += index_buffer_range.start;
            primitive.index_buffer_range.end += index_buffer_range.start;
        }

        let animations = read_animations(gltf.animations(), |buffer| {
            get_buffer(&gltf, &buffer_map, buffer)
        });

        dbg!(&animations.len());

        if gltf.skins().count() > 1 {
            return Err(anyhow::anyhow!(
                "Expected <= 1 skin, got {}",
                gltf.skins().count()
            ));
        }

        let skin = gltf.skins().next();
        if let Some(skin) = skin.as_ref() {
            if skin.skeleton().is_some() {
                return Err(anyhow::anyhow!("Expected there not to be a skeleton"));
            }
        }

        let joint_indices_to_node_indices: Vec<_> = if let Some(skin) = skin.as_ref() {
            skin.joints().map(|node| node.index()).collect()
        } else {
            gltf.nodes().map(|node| node.index()).collect()
        };

        let inverse_bind_transforms: Vec<Similarity> = if let Some(skin) = skin.as_ref() {
            skin.reader(|buffer| get_buffer(&gltf, &buffer_map, buffer))
                .read_inverse_bind_matrices()
                .expect("Missing inverse bind matrices")
                .map(|matrix| {
                    let (translation, rotation, scale) =
                        gltf::scene::Transform::Matrix { matrix }.decomposed();
                    Similarity::new_from_gltf(translation, rotation, scale)
                })
                .collect()
        } else {
            gltf.nodes()
                .map(|node| node_tree.transform_of(node.index()).inverse())
                .collect()
        };

        Ok(AnimatedModel {
            primitives,
            primitive_ranges,
            index_buffer_range,
            vertex_buffer_range,
            animation_data: AnimatedModelData {
                animations,
                depth_first_nodes: node_tree.iter_depth_first().collect(),
                joint_indices_to_node_indices,
                inverse_bind_transforms,
            },
        })
    }
}

struct StagingPrimitive<T> {
    buffers: T,
    material_settings: shared_structs::MaterialSettings,
    material_index: usize,
}

pub struct Primitive {
    pub index_buffer_range: Range<u32>,
    pub bind_group: Swappable<Arc<wgpu::BindGroup>>,
}

#[derive(Default)]
struct StagingBuffers {
    indices: Vec<u32>,
    positions: Vec<Vec3>,
    normals: Vec<Vec3>,
    uvs: Vec<Vec2>,
}

impl StagingBuffers {
    fn collect(&mut self, new: &StagingBuffers) -> Range<u32> {
        let indices_start = self.indices.len() as u32;
        let num_vertices = self.positions.len() as u32;

        self.indices
            .extend(new.indices.iter().map(|index| index + num_vertices));

        self.positions.extend_from_slice(&new.positions);
        self.normals.extend_from_slice(&new.normals);
        self.uvs.extend_from_slice(&new.uvs);

        let indices_end = self.indices.len() as u32;

        indices_start..indices_end
    }
}

#[derive(Default)]
struct AnimatedStagingBuffers {
    indices: Vec<u32>,
    positions: Vec<Vec3>,
    normals: Vec<Vec3>,
    uvs: Vec<Vec2>,
    joint_indices: Vec<UVec4>,
    joint_weights: Vec<Vec4>,
}

impl AnimatedStagingBuffers {
    fn collect(&mut self, new: &AnimatedStagingBuffers) -> Range<u32> {
        let indices_start = self.indices.len() as u32;
        let num_vertices = self.positions.len() as u32;

        self.indices
            .extend(new.indices.iter().map(|index| index + num_vertices));

        self.positions.extend_from_slice(&new.positions);
        self.normals.extend_from_slice(&new.normals);
        self.uvs.extend_from_slice(&new.uvs);
        self.joint_indices.extend_from_slice(&new.joint_indices);
        self.joint_weights.extend_from_slice(&new.joint_weights);

        let indices_end = self.indices.len() as u32;

        indices_start..indices_end
    }
}

fn spawn_texture_loading_futures<T: HttpClient>(
    bind_group_setter: Setter<Arc<wgpu::BindGroup>>,
    material_bindings: MaterialBindings,
    material_index: usize,
    gltf: Arc<gltf::Gltf>,
    buffer_map: Arc<HashMap<usize, Vec<u8>>>,
    context: &Context<T>,
    root_url: &url::Url,
) {
    // Early exit if there aren't any materials set.
    if gltf.materials().nth(material_index).is_none() {
        log::warn!("Material index is invalid or model doesn't contain any materials.");
        return;
    }

    // This is a little messy. As we're spawning a future for each possible texture I want to make the code that calls
    // `load_image_from_source_with_followup` as small as possible.
    let image_context = ImageContext {
        gltf,
        buffer_map,
        root_url: root_url.clone(),
        textures_context: textures::Context {
            bind_group_layouts: context.bind_group_layouts.clone(),
            device: context.device.clone(),
            queue: context.queue.clone(),
            http_client: context.http_client.clone(),
            pipelines: context.pipelines.clone(),
            settings: context.texture_settings.clone(),
        },
        bind_group_setter,
        material_bindings: Arc::new(material_bindings),
    };

    spawn({
        let image_context = image_context.clone();

        async move {
            let material = image_context
                .gltf
                .materials()
                .nth(material_index)
                .expect("we checked this earlier");

            let pbr = material.pbr_metallic_roughness();
            if let Some(albedo_texture) = pbr.base_color_texture() {
                load_image_from_gltf_with_followup(
                    albedo_texture.texture(),
                    true,
                    &image_context,
                    |texture| {
                        image_context.material_bindings.albedo.store(texture);
                    },
                )
                .await;
            }
        }
    });

    spawn({
        let image_context = image_context.clone();

        async move {
            let material = image_context
                .gltf
                .materials()
                .nth(material_index)
                .expect("we checked this earlier");

            let pbr = material.pbr_metallic_roughness();
            if let Some(metallic_roughness_texture) = pbr.metallic_roughness_texture() {
                load_image_from_gltf_with_followup(
                    metallic_roughness_texture.texture(),
                    false,
                    &image_context,
                    |texture| {
                        image_context
                            .material_bindings
                            .metallic_roughness
                            .store(texture);
                    },
                )
                .await;
            }
        }
    });

    spawn({
        let image_context = image_context.clone();

        async move {
            let material = image_context
                .gltf
                .materials()
                .nth(material_index)
                .expect("we checked this earlier");

            if let Some(normal_texture) = material.normal_texture() {
                load_image_from_gltf_with_followup(
                    normal_texture.texture(),
                    false,
                    &image_context,
                    |texture| {
                        image_context.material_bindings.normal.store(texture);
                    },
                )
                .await;
            }
        }
    });

    spawn({
        async move {
            let material = image_context
                .gltf
                .materials()
                .nth(material_index)
                .expect("we checked this earlier");

            if let Some(emissive_texture) = material.emissive_texture() {
                load_image_from_gltf_with_followup(
                    emissive_texture.texture(),
                    true,
                    &image_context,
                    |texture| {
                        image_context.material_bindings.emission.store(texture);
                    },
                )
                .await;
            }
        }
    });
}

#[derive(Clone)]
struct ImageContext<T> {
    gltf: Arc<gltf::Gltf>,
    buffer_map: Arc<HashMap<usize, Vec<u8>>>,
    root_url: url::Url,
    textures_context: textures::Context<T>,
    bind_group_setter: Setter<Arc<wgpu::BindGroup>>,
    material_bindings: Arc<MaterialBindings>,
}

async fn load_image_from_gltf_with_followup<T: HttpClient, F: Fn(Arc<Texture>)>(
    texture: gltf::Texture<'_, ()>,
    srgb: bool,
    context: &ImageContext<T>,
    follow_up: F,
) {
    let result = load_image_from_gltf(texture, srgb, context).await;

    match result {
        Ok(texture) => {
            follow_up(texture);

            context
                .bind_group_setter
                .set(Arc::new(context.material_bindings.create_bind_group(
                    &context.textures_context.device,
                    &context.textures_context.settings,
                )));
        }
        Err(error) => {
            log::error!("Failed to load texture for {}: {}", context.root_url, error);
        }
    }
}

async fn load_image_from_gltf<T: HttpClient>(
    texture: gltf::Texture<'_, ()>,
    srgb: bool,
    context: &ImageContext<T>,
) -> anyhow::Result<Arc<Texture>> {
    match texture.source().source() {
        gltf::image::Source::View { mime_type, view } => {
            let buffer = get_buffer(&context.gltf, &context.buffer_map, view.buffer()).unwrap();

            let bytes = &buffer[view.offset()..view.offset() + view.length()];

            load_image_with_mime_type(
                ImageSource::Bytes(bytes),
                srgb,
                Some(mime_type),
                &context.textures_context,
            )
            .await
        }
        gltf::image::Source::Uri { uri, mime_type } => {
            let url = url::Url::options()
                .base_url(Some(&context.root_url))
                .parse(uri)?;

            if url.scheme() == "data" {
                let (_mime_type, data) = url
                    .path()
                    .split_once(',')
                    .ok_or_else(|| anyhow::anyhow!("Failed to get data uri seperator"))?;

                let bytes = base64::decode(data)?;

                load_image_with_mime_type(
                    ImageSource::Bytes(&bytes),
                    srgb,
                    mime_type,
                    &context.textures_context,
                )
                .await
            } else {
                load_image_with_mime_type(
                    ImageSource::Url(url),
                    srgb,
                    mime_type,
                    &context.textures_context,
                )
                .await
            }
        }
    }
}
