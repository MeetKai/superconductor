use super::materials::MaterialBindings;
use super::textures;
use super::HttpClient;
use crate::culling::{BoundingBox, BoundingSphere};
use crate::permutations;
use crate::{spawn, BindGroupLayouts};
use arc_swap::ArcSwap;
use glam::{Mat4, UVec4, Vec2, Vec3, Vec4};
use gltf_helpers::{
    animation::{read_animations, Animation, AnimationJoints},
    Extensions, Similarity,
};
use goth_gltf::extensions::CompressionMode;
use goth_gltf::AlphaMode;
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::sync::Arc;

mod accessors;
mod texture_loading;

use accessors::{read_buffer_with_accessor, read_f32, read_f32x3, read_f32x4, PrimitiveReader};
use texture_loading::{
    image_index_from_texture_index, start_loading_all_material_textures, PendingTexture,
};

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
    gltf: Arc<goth_gltf::Gltf<Extensions>>,
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
    gltf: Arc<goth_gltf::Gltf<Extensions>>,
    pending_textures: Arc<HashMap<usize, PendingTexture>>,
) -> Ranges {
    let primitives_start = primitives.len();
    let indices_start = staging_buffers.num_indices();

    for staging_primitive in staging_primitives {
        primitives.push(Primitive {
            bounding_box: staging_primitive.bounding_box,
            bounding_sphere: staging_primitive.bounding_sphere,
            transform: staging_primitive.transform,
            screen_coverages: staging_primitive.screen_coverages.clone(),
            lods: staging_primitive
                .lods
                .iter()
                .map(|lod| {
                    let material_bindings = MaterialBindings::new(
                        &context.device,
                        &context.queue,
                        context.bind_group_layouts.clone(),
                        &lod.material_settings,
                    );

                    let bind_group = Arc::new(ArcSwap::from_pointee(
                        material_bindings
                            .create_initial_bind_group(&context.device, &context.texture_settings),
                    ));

                    spawn_texture_loading_futures(
                        bind_group.clone(),
                        material_bindings,
                        lod.material_index,
                        gltf.clone(),
                        context,
                        pending_textures.clone(),
                    );

                    PrimitiveLod {
                        index_buffer_range: staging_buffers.collect(&lod.buffers),
                        bind_group,
                    }
                })
                .collect(),
        });
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
    gltf: &goth_gltf::Gltf<Extensions>,
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

        let uri = match &buffer.uri {
            Some(uri) => uri,
            None => continue,
        };

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
        let gltf = Arc::new(gltf);

        let node_tree = gltf_helpers::NodeTree::new(&gltf);

        let buffer_view_map = collect_buffer_view_map(&gltf, glb_buffer, root_url, context).await?;

        let mut staging_primitives: permutations::BlendMode<permutations::FaceSides<Vec<_>>> =
            Default::default();

        let pending_textures = Arc::new(start_loading_all_material_textures(
            &gltf,
            root_url.clone(),
            context.textures_context(),
            buffer_view_map.clone(),
        )?);

        let mut ignored_nodes: HashSet<usize> = HashSet::new();

        for node in &gltf.nodes {
            if let Some(msft_lod) = &node.extensions.msft_lod {
                ignored_nodes.extend(&msft_lod.ids);
            }
        }

        for (node_index, node, mesh_index) in gltf
            .nodes
            .iter()
            .enumerate()
            .filter(|(node_index, _)| !ignored_nodes.contains(node_index))
            .filter_map(|(node_index, node)| {
                node.mesh.map(|mesh_index| (node_index, node, mesh_index))
            })
        {
            let transform = node_tree.transform_of(node_index);

            let mesh = &gltf.meshes[mesh_index];

            let mesh_lods = std::iter::once(mesh).chain(
                node.extensions
                    .msft_lod
                    .iter()
                    .flat_map(|lod| &lod.ids)
                    .filter_map(|&node_index| gltf.nodes.get(node_index))
                    .filter_map(|node| node.mesh)
                    .filter_map(|mesh_index| gltf.meshes.get(mesh_index)),
            );

            let num_primitives = mesh.primitives.len();

            for mesh_lod in mesh_lods.clone() {
                assert_eq!(mesh_lod.primitives.len(), num_primitives);
            }

            for primitive_index in 0..num_primitives {
                let mut lods = Vec::new();

                for mesh in mesh_lods.clone() {
                    let primitive = &mesh.primitives[primitive_index];

                    let material_index = primitive.material.unwrap_or(0);
                    let material = &gltf.materials[material_index];

                    let reader = PrimitiveReader::new(&gltf, primitive, &buffer_view_map);

                    let buffers = StagingBuffers::new(&reader)?;

                    lods.push(StagingPrimitiveLod {
                        buffers,
                        material_settings: load_material_settings(material, &reader),
                        material_index,
                    });
                }

                let material = &gltf.materials[lods[0].material_index];

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

                primitive_vec.push(StagingPrimitive {
                    bounding_box: BoundingBox::new(&lods[0].buffers.positions),
                    bounding_sphere: BoundingSphere::new(&lods[0].buffers.positions),
                    lods,
                    transform,
                    screen_coverages: node.extras.msft_screencoverage.clone().unwrap_or_default(),
                });
            }
        }

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
            for lod in &mut primitive.lods {
                lod.index_buffer_range.start += index_buffer_range.start;
                lod.index_buffer_range.end += index_buffer_range.start;
            }
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
        let gltf = Arc::new(gltf);

        let node_tree = gltf_helpers::NodeTree::new(&gltf);

        let buffer_view_map = collect_buffer_view_map(&gltf, glb_buffer, root_url, context).await?;

        let mut staging_primitives: permutations::BlendMode<permutations::FaceSides<Vec<_>>> =
            Default::default();

        let pending_textures = Arc::new(start_loading_all_material_textures(
            &gltf,
            root_url.clone(),
            context.textures_context(),
            buffer_view_map.clone(),
        )?);

        for (node_index, mesh_index) in gltf
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(node_index, node)| node.mesh.map(|mesh_index| (node_index, mesh_index)))
        {
            let mesh = &gltf.meshes[mesh_index];

            for primitive in &mesh.primitives {
                let material_index = primitive.material.unwrap_or(0);
                let material = &gltf.materials[material_index];

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

                let buffers = StagingBuffers::new(&reader)?;

                primitive_vec.push(StagingPrimitive {
                    bounding_box: BoundingBox::new(&buffers.positions),
                    bounding_sphere: BoundingSphere::new(&buffers.positions),
                    lods: vec![StagingPrimitiveLod {
                        buffers: AnimatedStagingBuffers {
                            joint_indices: match reader.read_joints()? {
                                Some(joints) => joints.to_vec(),
                                None => std::iter::repeat(UVec4::splat(node_index as u32))
                                    .take(buffers.positions.len())
                                    .collect(),
                            },
                            joint_weights: match reader.read_weights()? {
                                Some(joint_weights) => joint_weights.to_vec(),
                                None => std::iter::repeat(Vec4::X)
                                    .take(buffers.positions.len())
                                    .collect(),
                            },
                            base: buffers,
                        },
                        material_settings: load_material_settings(material, &reader),
                        material_index,
                    }],
                    transform: Similarity::IDENTITY,
                    screen_coverages: Vec::new(),
                });
            }
        }

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
            for lod in &mut primitive.lods {
                lod.index_buffer_range.start += index_buffer_range.start;
                lod.index_buffer_range.end += index_buffer_range.start;
            }
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
    lods: Vec<StagingPrimitiveLod<T>>,
    bounding_box: BoundingBox,
    bounding_sphere: BoundingSphere,
    transform: Similarity,
    screen_coverages: Vec<f32>,
}

struct StagingPrimitiveLod<T> {
    buffers: T,
    material_settings: shared_structs::MaterialSettings,
    material_index: usize,
}

pub struct Primitive {
    pub lods: Vec<PrimitiveLod>,
    pub bounding_box: BoundingBox,
    pub bounding_sphere: BoundingSphere,
    pub transform: Similarity,
    pub screen_coverages: Vec<f32>,
}

pub struct PrimitiveLod {
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
    fn new(reader: &PrimitiveReader) -> anyhow::Result<Self> {
        let positions: Vec<Vec3> = reader
            .read_positions()?
            .ok_or_else(|| anyhow::anyhow!("Primitive doesn't specifiy vertex positions."))?
            .to_vec();

        Ok(Self {
            indices: match reader.read_indices()? {
                Some(indices) => indices.to_vec(),
                None => {
                    log::warn!("No indices specified, using inefficient per-vertex indices.");

                    (0..positions.len() as u32).collect()
                }
            },
            normals: match reader.read_normals()? {
                Some(normals) => normals.to_vec(),
                None => std::iter::repeat(Vec3::ZERO)
                    .take(positions.len())
                    .collect(),
            },
            uvs: match reader.read_uvs()? {
                Some(uvs) => uvs.to_vec(),
                None => std::iter::repeat(Vec2::ZERO)
                    .take(positions.len())
                    .collect(),
            },
            positions,
        })
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
    gltf: Arc<goth_gltf::Gltf<Extensions>>,
    context: &Context<T>,
    pending_textures: Arc<HashMap<usize, PendingTexture>>,
) {
    let material = match gltf.materials.get(material_index) {
        Some(material) => material.clone(),
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

            let (albedo_texture, metallic_roughness_texture, normal_texture, emissive_texture) =
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
                emissive: emissive_texture?,
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

fn load_material_settings(
    material: &goth_gltf::Material<Extensions>,
    reader: &PrimitiveReader,
) -> shared_structs::MaterialSettings {
    // Workaround for some exporters (Scaniverse) exporting scanned models that are meant to be
    // rendered unlit but don't set the material flag.
    let unlit = material.extensions.khr_materials_unlit.is_some()
        || reader.primitive.attributes.normal.is_none();

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
