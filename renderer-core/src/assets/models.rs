use super::materials::MaterialBindings;
use super::textures::{self, load_image_with_mime_type, ImageSource};
use super::HttpClient;
use crate::permutations;
use crate::{spawn, BindGroupLayouts, Texture};
use arc_swap::ArcSwap;
use futures::{
    future::{self, Future},
    FutureExt,
};
use glam::{Mat2, Mat4, UVec4, Vec2, Vec3, Vec4};
use gltf_helpers::{
    animation::{read_animations, Animation, AnimationJoints},
    Similarity,
};
use goth_gltf::extensions::CompressionMode;
use goth_gltf::AlphaMode;
use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;

mod accessors;

use accessors::{read_buffer_with_accessor, read_f32, read_f32x3, read_f32x4, PrimitiveReader};

#[derive(Clone)]
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

impl<T: Clone> Context<T> {
    fn textures_context(&self) -> textures::Context<T> {
        textures::Context {
            bind_group_layouts: self.bind_group_layouts.clone(),
            device: self.device.clone(),
            queue: self.queue.clone(),
            http_client: self.http_client.clone(),
            pipelines: self.pipelines.clone(),
            settings: self.texture_settings.clone(),
        }
    }
}

pub type PrimitiveRanges = permutations::BlendMode<permutations::FaceSides<Ranges>>;

#[derive(Clone, Debug)]
pub struct Ranges {
    pub indices: Range<u32>,
    pub primitives: Range<usize>,
}

// Collect all the buffers for the primitives into one big staging buffer
// and collect all the primitive ranges into one big vector.
fn collect_all_primitives<'a, T: HttpClient, B: 'a + CollectableBuffer + Default>(
    context: &Context<T>,
    gltf: Arc<goth_gltf::Gltf>,
    staging_primitives: &permutations::BlendMode<permutations::FaceSides<Vec<StagingPrimitive<B>>>>,
    pending_textures: Arc<HashMap<usize, PendingTexture>>,
) -> (PrimitiveRanges, Vec<Primitive>, B) {
    let mut primitives = Vec::new();
    let mut staging_buffers = B::default();

    let primitive_ranges = PrimitiveRanges {
        opaque: permutations::FaceSides {
            single: collect_primitives(
                &mut primitives,
                &mut staging_buffers,
                &staging_primitives.opaque.single,
                context,
                gltf.clone(),
                pending_textures.clone(),
            ),
            double: collect_primitives(
                &mut primitives,
                &mut staging_buffers,
                &staging_primitives.opaque.double,
                context,
                gltf.clone(),
                pending_textures.clone(),
            ),
        },
        alpha_clipped: permutations::FaceSides {
            single: collect_primitives(
                &mut primitives,
                &mut staging_buffers,
                &staging_primitives.alpha_clipped.single,
                context,
                gltf.clone(),
                pending_textures.clone(),
            ),
            double: collect_primitives(
                &mut primitives,
                &mut staging_buffers,
                &staging_primitives.alpha_clipped.double,
                context,
                gltf.clone(),
                pending_textures.clone(),
            ),
        },
        alpha_blended: permutations::FaceSides {
            single: collect_primitives(
                &mut primitives,
                &mut staging_buffers,
                &staging_primitives.alpha_blended.single,
                context,
                gltf.clone(),
                pending_textures.clone(),
            ),
            double: collect_primitives(
                &mut primitives,
                &mut staging_buffers,
                &staging_primitives.alpha_blended.double,
                context,
                gltf,
                pending_textures.clone(),
            ),
        },
    };

    (primitive_ranges, primitives, staging_buffers)
}

// Loop over each primitive, collecting the primitives together and spawning the texture loading
// futures.
fn collect_primitives<
    'a,
    T: HttpClient,
    B: CollectableBuffer + 'a,
    I: IntoIterator<Item = &'a StagingPrimitive<B>>,
>(
    primitives: &mut Vec<Primitive>,
    staging_buffers: &mut B,
    staging_primitives: I,
    context: &Context<T>,
    gltf: Arc<goth_gltf::Gltf>,
    pending_textures: Arc<HashMap<usize, PendingTexture>>,
) -> Ranges {
    let primitives_start = primitives.len();
    let indices_start = staging_buffers.num_indices();

    for staging_primitive in staging_primitives {
        let material_bindings = MaterialBindings::new(
            &context.device,
            &context.queue,
            context.bind_group_layouts.clone(),
            &staging_primitive.material_settings,
        );

        let bind_group = Arc::new(ArcSwap::from_pointee(
            material_bindings.create_initial_bind_group(&context.device, &context.texture_settings),
        ));

        primitives.push(Primitive {
            index_buffer_range: staging_buffers.collect(&staging_primitive.buffers),
            bind_group: bind_group.clone(),
        });

        spawn_texture_loading_futures(
            bind_group,
            material_bindings,
            staging_primitive.material_index,
            gltf.clone(),
            context,
            pending_textures.clone(),
        )
    }

    let primitives_end = primitives.len();
    let indices_end = staging_buffers.num_indices();

    Ranges {
        primitives: primitives_start..primitives_end,
        indices: indices_start..indices_end,
    }
}

pub struct AnimatedModelData {
    pub animations: Vec<Animation>,
    pub depth_first_nodes: gltf_helpers::DepthFirstNodes,
    pub inverse_bind_transforms: Vec<Similarity>,
    pub joint_indices_to_node_indices: Vec<usize>,
    pub animation_joints: AnimationJoints,
}

async fn collect_buffer_view_map<T: HttpClient>(
    gltf: &goth_gltf::Gltf,
    glb_buffer: Option<&[u8]>,
    root_url: &url::Url,
    context: &Context<T>,
) -> anyhow::Result<Arc<HashMap<usize, Vec<u8>>>> {
    use std::borrow::Cow;

    let mut buffer_map = HashMap::new();

    if let Some(glb_buffer) = glb_buffer {
        buffer_map.insert(0, Cow::Borrowed(glb_buffer));
    }

    for (index, buffer) in gltf.buffers.iter().enumerate() {
        if buffer
            .extensions
            .ext_meshopt_compression
            .as_ref()
            .map(|ext| ext.fallback)
            .unwrap_or(false)
        {
            continue;
        }

        match &buffer.uri {
            None => {}
            Some(uri) => {
                let url = url::Url::options().base_url(Some(root_url)).parse(uri)?;

                if url.scheme() == "data" {
                    let (_mime_type, data) = url
                        .path()
                        .split_once(',')
                        .ok_or_else(|| anyhow::anyhow!("Failed to get data uri split"))?;
                    log::warn!("Loading buffers from embedded base64 is inefficient. Consider moving the buffers into a seperate file.");
                    buffer_map.insert(index, Cow::Owned(base64::decode(data)?));
                } else {
                    buffer_map.insert(
                        index,
                        Cow::Owned(context.http_client.fetch_bytes(&url, None).await?),
                    );
                }
            }
        }
    }

    let mut buffer_view_map = HashMap::new();

    for (i, buffer_view) in gltf.buffer_views.iter().enumerate() {
        if let Some(ext) = buffer_view.extensions.ext_meshopt_compression.as_ref() {
            if let Some(buffer) = buffer_map.get(&ext.buffer) {
                let slice = &buffer[ext.byte_offset..ext.byte_offset + ext.byte_length];

                let filter = match ext.filter {
                    goth_gltf::extensions::CompressionFilter::None => None,
                    goth_gltf::extensions::CompressionFilter::Octahedral => {
                        Some(meshopt_decoder::Filter::Octahedral)
                    }
                    goth_gltf::extensions::CompressionFilter::Quaternion => {
                        Some(meshopt_decoder::Filter::Quaternion)
                    }
                    goth_gltf::extensions::CompressionFilter::Exponential => {
                        Some(meshopt_decoder::Filter::Exponential)
                    }
                };

                let bytes: Vec<u8> = match (ext.mode, ext.byte_stride) {
                    (CompressionMode::Triangles, 2) => {
                        meshopt_decoder::TriangleIterator::new(slice, ext.count)
                            .unwrap()
                            .flatten()
                            .flat_map(|index| (index as u16).to_le_bytes())
                            .collect()
                    }
                    (CompressionMode::Triangles, 4) => {
                        meshopt_decoder::TriangleIterator::new(slice, ext.count)
                            .unwrap()
                            .flatten()
                            .flat_map(|index| index.to_le_bytes())
                            .collect()
                    }
                    (CompressionMode::Attributes, byte_stride) => {
                        meshopt_decoder::decompress_attributes_to_vec(
                            slice,
                            ext.count,
                            filter,
                            byte_stride,
                        )
                        .unwrap()
                    }
                    x => panic!("{:?}", x),
                };

                buffer_view_map.insert(i, bytes);
            }
        } else if let Some(buffer) = buffer_map.get(&buffer_view.buffer) {
            buffer_view_map.insert(
                i,
                buffer[buffer_view.byte_offset..buffer_view.byte_offset + buffer_view.byte_length]
                    .to_vec(),
            );
        }
    }

    Ok(Arc::new(buffer_view_map))
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
        let bytes = context.http_client.fetch_bytes(root_url, None).await?;

        let (gltf, glb_buffer) = goth_gltf::Gltf::from_bytes(&bytes)?;

        let node_tree = gltf_helpers::NodeTree::new(&gltf);

        let buffer_view_map = collect_buffer_view_map(&gltf, glb_buffer, root_url, context).await?;

        // What we're doing here is essentially collecting all the model primitives that share a meterial together
        // to reduce the number of draw calls.
        let mut staging_primitives: permutations::BlendMode<permutations::FaceSides<Vec<_>>> =
            Default::default();

        let mut pending_textures = Default::default();

        for (node_index, mesh_index) in gltf
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(node_index, node)| node.mesh.map(|mesh_index| (node_index, mesh_index)))
        {
            let transform = node_tree.transform_of(node_index);
            let mesh = &gltf.meshes[mesh_index];

            for primitive in &mesh.primitives {
                let material_index = primitive.material;
                let material = gltf.materials[primitive.material.unwrap_or(0)];

                // Note: it's possible to render double-sided objects with a backface-culling shader if we double the
                // triangles in the index buffer but with a backwards winding order. It's only worth doing this to keep
                // the number of shader permutations down.
                //
                // One thing to keep in mind is that we flip the shading normals according to the gltf spec:
                // https://www.khronos.org/registry/glTF/specs/2.0/glTF-2.0.html#double-sided

                let primitive_vec = match (material.alpha_mode, material.double_sided) {
                    (AlphaMode::Opaque, false) => &mut staging_primitives.opaque.single,
                    (AlphaMode::Opaque, true) => &mut staging_primitives.opaque.double,

                    (AlphaMode::Mask, false) => &mut staging_primitives.alpha_clipped.single,
                    (AlphaMode::Mask, true) => &mut staging_primitives.alpha_clipped.double,

                    (AlphaMode::Blend, false) => &mut staging_primitives.alpha_blended.single,
                    (AlphaMode::Blend, true) => &mut staging_primitives.alpha_blended.double,
                };

                let reader = PrimitiveReader::new(&gltf, primitive, &buffer_view_map);

                let material_info = MaterialInfo::load(material, &reader);

                let mut staging_primitive = StagingPrimitive {
                    buffers: StagingBuffers::default(),
                    material_settings: material_info.settings,
                    material_index: material_index.unwrap_or(0),
                };

                staging_primitive.buffers.extend_from_reader(
                    &reader,
                    transform,
                    material_info.texture_transform,
                )?;

                primitive_vec.push(staging_primitive);

                start_loading_material_textures(
                    &material,
                    &gltf,
                    &mut pending_textures,
                    root_url.clone(),
                    context.textures_context(),
                    buffer_view_map.clone(),
                )?;
            }
        }

        let gltf = Arc::new(gltf);
        let pending_textures = Arc::new(pending_textures);

        // Collect all the buffers for the primitives into one big staging buffer
        // and collect all the primitive ranges into one big vector.
        let (mut primitive_ranges, mut primitives, mut staging_buffers) =
            collect_all_primitives(context, gltf, &staging_primitives, pending_textures);

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

        for range in primitive_ranges
            .iter_mut()
            .iter_mut()
            .flat_map(|blend_mode| blend_mode.iter_mut())
        {
            range.indices.start += index_buffer_range.start;
            range.indices.end += index_buffer_range.start;
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
        let bytes = context.http_client.fetch_bytes(root_url, None).await?;

        let (gltf, glb_buffer) = goth_gltf::Gltf::from_bytes(&bytes)?;

        let node_tree = gltf_helpers::NodeTree::new(&gltf);

        let buffer_view_map = collect_buffer_view_map(&gltf, glb_buffer, root_url, context).await?;

        let mut staging_primitives: permutations::BlendMode<permutations::FaceSides<Vec<_>>> =
            Default::default();

        let mut pending_textures = Default::default();

        for (node_index, mesh_index) in gltf
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(node_index, node)| node.mesh.map(|mesh_index| (node_index, mesh_index)))
        {
            let mesh = &gltf.meshes[mesh_index];

            for primitive in &mesh.primitives {
                let material_index = primitive.material;
                let material = gltf.materials[primitive.material.unwrap_or(0)];

                // Note: it's possible to render double-sided objects with a backface-culling shader if we double the
                // triangles in the index buffer but with a backwards winding order. It's only worth doing this to keep
                // the number of shader permutations down.
                //
                // One thing to keep in mind is that we flip the shading normals according to the gltf spec:
                // https://www.khronos.org/registry/glTF/specs/2.0/glTF-2.0.html#double-sided

                let primitive_vec = match (material.alpha_mode, material.double_sided) {
                    (AlphaMode::Opaque, false) => &mut staging_primitives.opaque.single,
                    (AlphaMode::Opaque, true) => &mut staging_primitives.opaque.double,

                    (AlphaMode::Mask, false) => &mut staging_primitives.alpha_clipped.single,
                    (AlphaMode::Mask, true) => &mut staging_primitives.alpha_clipped.double,

                    (AlphaMode::Blend, false) => &mut staging_primitives.alpha_blended.single,
                    (AlphaMode::Blend, true) => &mut staging_primitives.alpha_blended.double,
                };

                let reader = PrimitiveReader::new(&gltf, primitive, &buffer_view_map);

                let material_info = MaterialInfo::load(material, &reader);

                let mut staging_primitive = StagingPrimitive {
                    buffers: AnimatedStagingBuffers::default(),
                    material_settings: material_info.settings,
                    material_index: material_index.unwrap_or(0),
                };

                let num_vertices = staging_primitive.buffers.base.extend_from_reader(
                    &reader,
                    Similarity::IDENTITY,
                    material_info.texture_transform,
                )?;

                match reader.read_joints()? {
                    Some(joints) => {
                        staging_primitive
                            .buffers
                            .joint_indices
                            .extend(joints.map(|indices| {
                                UVec4::new(
                                    indices[0] as u32,
                                    indices[1] as u32,
                                    indices[2] as u32,
                                    indices[3] as u32,
                                )
                            }))
                    }
                    None => staging_primitive.buffers.joint_indices.extend(
                        std::iter::repeat(UVec4::splat(node_index as u32)).take(num_vertices),
                    ),
                }

                match reader.read_weights()? {
                    Some(joint_weights) => staging_primitive
                        .buffers
                        .joint_weights
                        .extend(joint_weights.map(Vec4::from)),
                    None => staging_primitive
                        .buffers
                        .joint_weights
                        .extend(std::iter::repeat(Vec4::X).take(num_vertices)),
                };

                primitive_vec.push(staging_primitive);

                start_loading_material_textures(
                    &material,
                    &gltf,
                    &mut pending_textures,
                    root_url.clone(),
                    context.textures_context(),
                    buffer_view_map.clone(),
                )?;
            }
        }

        let gltf = Arc::new(gltf);
        let pending_textures = Arc::new(pending_textures);

        // Collect all the buffers for the primitives into one big staging buffer
        // and collect all the primitive ranges into one big vector.
        let (mut primitive_ranges, mut primitives, mut staging_buffers) =
            collect_all_primitives(context, gltf.clone(), &staging_primitives, pending_textures);

        let mut command_encoder =
            context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("command encoder"),
                });

        let vertex_buffer_range = context.animated_vertex_buffers.insert(
            &staging_buffers.base.positions,
            &staging_buffers.base.normals,
            &staging_buffers.base.uvs,
            &staging_buffers.joint_indices,
            &staging_buffers.joint_weights,
            &context.device,
            &context.queue,
            &mut command_encoder,
        );

        // Make sure the indices point to the right vertices.
        for index in &mut staging_buffers.base.indices {
            *index += vertex_buffer_range.start;
        }

        let index_buffer_range = context.index_buffer.insert(
            &staging_buffers.base.indices,
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

        for range in primitive_ranges
            .iter_mut()
            .iter_mut()
            .flat_map(|blend_mode| blend_mode.iter_mut())
        {
            range.indices.start += index_buffer_range.start;
            range.indices.end += index_buffer_range.start;
        }

        let animations = read_animations(
            &gltf,
            |accessor| {
                let (slice, byte_stride) =
                    read_buffer_with_accessor(&buffer_view_map, &gltf, accessor).unwrap();
                read_f32(slice, byte_stride, accessor).unwrap()
            },
            |accessor| {
                let (slice, byte_stride) =
                    read_buffer_with_accessor(&buffer_view_map, &gltf, accessor).unwrap();
                read_f32x3(slice, byte_stride, accessor).unwrap()
            },
            |accessor| {
                let (slice, byte_stride) =
                    read_buffer_with_accessor(&buffer_view_map, &gltf, accessor).unwrap();
                read_f32x4(slice, byte_stride, accessor).unwrap()
            },
        );

        if gltf.skins.len() > 1 {
            log::warn!("Got {} skins. Using the first.", gltf.skins.len());
        }

        let skin = gltf.skins.first();

        let joint_indices_to_node_indices: Vec<_> = match skin.as_ref() {
            Some(skin) => skin.joints.clone(),
            None => (0..gltf.nodes.len()).collect(),
        };

        let inverse_bind_transforms: Vec<Similarity> = match skin.as_ref() {
            Some(skin) => {
                let accessor_index = skin
                    .inverse_bind_matrices
                    .ok_or_else(|| anyhow::anyhow!("Missing inverse bind matrices accessor"))?;
                let accessor = &gltf.accessors[accessor_index];

                let (slice, _byte_stride) =
                    read_buffer_with_accessor(&buffer_view_map, &gltf, accessor)?;

                let matrices: &[[f32; 16]] = bytemuck::cast_slice(slice);

                matrices
                    .iter()
                    .map(|matrix| Similarity::new_from_mat4(Mat4::from_cols_array(matrix)))
                    .collect()
            }
            None => (0..gltf.nodes.len())
                .map(|_| Similarity::IDENTITY)
                .collect(),
        };

        let depth_first_nodes = gltf_helpers::DepthFirstNodes::new(&gltf, &node_tree);

        let animation_joints = AnimationJoints::new(&gltf, &depth_first_nodes);

        Ok(AnimatedModel {
            primitives,
            primitive_ranges,
            index_buffer_range,
            vertex_buffer_range,
            animation_data: AnimatedModelData {
                animations,
                depth_first_nodes,
                joint_indices_to_node_indices,
                inverse_bind_transforms,
                animation_joints,
            },
        })
    }

    pub fn num_joints(&self) -> u32 {
        self.animation_data.joint_indices_to_node_indices.len() as u32
    }

    pub fn max_instances_per_joint_buffer(&self) -> u32 {
        shared_structs::JointTransform::MAX_COUNT as u32 / self.num_joints()
    }
}

struct StagingPrimitive<T> {
    buffers: T,
    material_settings: shared_structs::MaterialSettings,
    material_index: usize,
}

pub struct Primitive {
    pub index_buffer_range: Range<u32>,
    pub bind_group: Arc<ArcSwap<wgpu::BindGroup>>,
}

trait CollectableBuffer {
    fn collect(&mut self, new: &Self) -> Range<u32>;
    fn num_indices(&self) -> u32;
}

#[derive(Default)]
struct StagingBuffers {
    indices: Vec<u32>,
    positions: Vec<Vec3>,
    normals: Vec<Vec3>,
    uvs: Vec<Vec2>,
}

impl StagingBuffers {
    fn extend_from_reader(
        &mut self,
        reader: &PrimitiveReader,
        transform: Similarity,
        texture_transform: Option<goth_gltf::extensions::KhrTextureTransform>,
    ) -> anyhow::Result<usize> {
        let vertices_offset = self.positions.len();

        self.positions.extend(
            reader
                .read_positions()?
                .ok_or_else(|| anyhow::anyhow!("Primitive doesn't specifiy vertex positions."))?
                .map(|pos| transform * Vec3::from(pos)),
        );

        let num_vertices = self.positions.len() - vertices_offset;

        match reader.read_indices()? {
            Some(indices) => self
                .indices
                .extend(indices.map(|index| vertices_offset as u32 + index)),
            None => {
                log::warn!("No indices specified, using inefficient per-vertex indices.");

                self.indices
                    .extend(vertices_offset as u32..vertices_offset as u32 + num_vertices as u32);
            }
        };

        match reader.read_normals()? {
            Some(normals) => self
                .normals
                .extend(normals.map(|normal| transform.rotation * Vec3::from(normal))),
            None => self
                .normals
                .extend(std::iter::repeat(Vec3::ZERO).take(num_vertices)),
        }

        match reader.read_uvs()? {
            Some(uvs) => match texture_transform {
                Some(transform) => self.uvs.extend(uvs.map(|uv| {
                    Vec2::from(transform.offset)
                        + (Mat2::from_angle(transform.rotation)
                            * Vec2::from(transform.scale)
                            * Vec2::from(uv))
                })),
                None => self.uvs.extend(uvs.map(Vec2::from)),
            },
            None => self
                .uvs
                .extend(std::iter::repeat(Vec2::ZERO).take(num_vertices)),
        }

        Ok(num_vertices)
    }
}

impl CollectableBuffer for StagingBuffers {
    fn collect(&mut self, new: &Self) -> Range<u32> {
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

    fn num_indices(&self) -> u32 {
        self.indices.len() as u32
    }
}

#[derive(Default)]
struct AnimatedStagingBuffers {
    base: StagingBuffers,
    joint_indices: Vec<UVec4>,
    joint_weights: Vec<Vec4>,
}

impl CollectableBuffer for AnimatedStagingBuffers {
    fn collect(&mut self, new: &Self) -> Range<u32> {
        self.joint_indices.extend_from_slice(&new.joint_indices);
        self.joint_weights.extend_from_slice(&new.joint_weights);

        self.base.collect(&new.base)
    }

    fn num_indices(&self) -> u32 {
        self.base.indices.len() as u32
    }
}

fn spawn_texture_loading_futures<T: HttpClient>(
    bind_group: Arc<ArcSwap<wgpu::BindGroup>>,
    material_bindings: MaterialBindings,
    material_index: usize,
    gltf: Arc<goth_gltf::Gltf>,
    context: &Context<T>,
    pending_textures: Arc<HashMap<usize, PendingTexture>>,
) {
    let material = match gltf.materials.get(material_index) {
        Some(material) => *material,
        None => {
            log::warn!(
                "Material index {} is out of range of {}",
                material_index,
                gltf.materials.len()
            );
            return;
        }
    };

    let pbr = material.pbr_metallic_roughness;
    let context = context.clone();

    spawn({
        async move {
            let albedo_texture = {
                let pending_textures = pending_textures.clone();
                let gltf = gltf.clone();

                async move {
                    anyhow::Ok(match pbr.base_color_texture {
                        Some(texture) => {
                            pending_textures
                                .get(&image_index_from_texture_index(texture.index, &gltf)?)
                                .unwrap()
                                .clone()
                                .await
                        }
                        None => None,
                    })
                }
            };

            let metallic_roughness_texture = {
                let pending_textures = pending_textures.clone();
                let gltf = gltf.clone();

                async move {
                    anyhow::Ok(match pbr.metallic_roughness_texture {
                        Some(texture) => {
                            pending_textures
                                .get(&image_index_from_texture_index(texture.index, &gltf)?)
                                .unwrap()
                                .clone()
                                .await
                        }
                        None => None,
                    })
                }
            };

            let normal_texture = {
                let pending_textures = pending_textures.clone();
                let gltf = gltf.clone();

                async move {
                    anyhow::Ok(match material.normal_texture {
                        Some(texture) => {
                            pending_textures
                                .get(&image_index_from_texture_index(texture.index, &gltf)?)
                                .unwrap()
                                .clone()
                                .await
                        }
                        None => None,
                    })
                }
            };

            let emissive_texture = {
                let pending_textures = pending_textures.clone();
                let gltf = gltf.clone();

                async move {
                    anyhow::Ok(match material.emissive_texture {
                        Some(texture) => {
                            pending_textures
                                .get(&image_index_from_texture_index(texture.index, &gltf)?)
                                .unwrap()
                                .clone()
                                .await
                        }
                        None => None,
                    })
                }
            };

            let (albedo_texture, metallic_roughness_texture, normal_texture, emission_texture) =
                futures::future::join4(
                    albedo_texture,
                    metallic_roughness_texture,
                    normal_texture,
                    emissive_texture,
                )
                .await;
            let incoming_textures = super::materials::Textures {
                albedo: albedo_texture?,
                metallic_roughness: metallic_roughness_texture?,
                normal: normal_texture?,
                emission: emission_texture?,
            };

            bind_group.store(Arc::new(material_bindings.create_bind_group(
                &context.device,
                &context.texture_settings,
                incoming_textures,
            )));

            Ok(())
        }
    });
}

type PendingTexture =
    future::Shared<std::pin::Pin<Box<dyn Future<Output = Option<Arc<Texture>>> + Send + 'static>>>;

fn image_index_from_texture_index(
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

fn start_loading_material_textures<T: HttpClient>(
    material: &goth_gltf::Material,
    gltf: &goth_gltf::Gltf,
    pending_textures: &mut HashMap<usize, PendingTexture>,
    root_url: url::Url,
    textures_context: textures::Context<T>,
    buffer_view_map: Arc<HashMap<usize, Vec<u8>>>,
) -> anyhow::Result<()> {
    if let Some(tex_info) = material.pbr_metallic_roughness.base_color_texture {
        start_loading_texture(
            tex_info.index,
            true,
            gltf,
            pending_textures,
            root_url.clone(),
            textures_context.clone(),
            buffer_view_map.clone(),
        )?;
    }

    if let Some(tex_info) = material.pbr_metallic_roughness.metallic_roughness_texture {
        start_loading_texture(
            tex_info.index,
            false,
            gltf,
            pending_textures,
            root_url.clone(),
            textures_context.clone(),
            buffer_view_map.clone(),
        )?;
    }

    if let Some(tex_info) = material.normal_texture {
        start_loading_texture(
            tex_info.index,
            false,
            gltf,
            pending_textures,
            root_url.clone(),
            textures_context.clone(),
            buffer_view_map.clone(),
        )?;
    }

    if let Some(tex_info) = material.emissive_texture {
        start_loading_texture(
            tex_info.index,
            true,
            gltf,
            pending_textures,
            root_url,
            textures_context,
            buffer_view_map,
        )?;
    }

    Ok(())
}

fn start_loading_texture<T: HttpClient>(
    texture_index: usize,
    srgb: bool,
    gltf: &goth_gltf::Gltf,
    pending_textures: &mut HashMap<usize, PendingTexture>,
    root_url: url::Url,
    textures_context: textures::Context<T>,
    buffer_view_map: Arc<HashMap<usize, Vec<u8>>>,
) -> anyhow::Result<()> {
    let image_index = image_index_from_texture_index(texture_index, gltf)?;

    if pending_textures.contains_key(&image_index) {
        return Ok(());
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

    let future = future
        .map(|result| match result {
            Ok(texture) => Some(texture),
            Err(error) => {
                log::error!("{}", error);
                None
            }
        })
        .boxed()
        .shared();

    pending_textures.insert(image_index, future);

    Ok(())
}

struct MaterialInfo {
    settings: shared_structs::MaterialSettings,
    texture_transform: Option<goth_gltf::extensions::KhrTextureTransform>,
}

impl MaterialInfo {
    fn load(material: goth_gltf::Material, reader: &PrimitiveReader) -> Self {
        // Workaround for some exporters (Scaniverse) exporting scanned models that are meant to be
        // rendered unlit but don't set the material flag.
        let unlit = material.extensions.khr_materials_unlit.is_some()
            || reader.primitive.attributes.normal.is_none();

        let pbr = material.pbr_metallic_roughness;

        let texture_transform = pbr
            .base_color_texture
            .map(|info| info.extensions)
            .or_else(|| pbr.metallic_roughness_texture.map(|info| info.extensions))
            .or_else(|| material.normal_texture.map(|info| info.extensions))
            .or_else(|| material.emissive_texture.map(|info| info.extensions))
            .and_then(|extensions| extensions.khr_texture_transform);

        let emissive_strength = material
            .extensions
            .khr_materials_emissive_strength
            .map(|emissive_strength| emissive_strength.emissive_strength)
            .unwrap_or(1.0);

        let settings = shared_structs::MaterialSettings {
            base_color_factor: pbr.base_color_factor.into(),
            emissive_factor: Vec3::from(material.emissive_factor) * emissive_strength,
            metallic_factor: pbr.metallic_factor,
            roughness_factor: pbr.roughness_factor,
            is_unlit: unlit as u32,
            normal_map_scale: material
                .normal_texture
                .map(|info| info.scale)
                .unwrap_or(1.0),
            // It seems like uniform buffer padding works differently in the wgpu Vulkan backends vs the WebGL2 backend.
            // todo: find a nicer way to resolve this.
            #[cfg(not(feature = "wasm"))]
            _padding: 0,
        };

        Self {
            settings,
            texture_transform,
        }
    }
}
