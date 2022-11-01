use glam::{UVec4, Vec2, Vec3, Vec4};
use gltf_helpers::Extensions;
use goth_gltf::ComponentType;
use std::borrow::Cow;
use std::collections::HashMap;

fn unsigned_short_to_float(short: u16) -> f32 {
    short as f32 / 65535.0
}

fn unsigned_byte_to_float(byte: u8) -> f32 {
    byte as f32 / 255.0
}

fn signed_byte_to_float(byte: i8) -> f32 {
    (byte as f32 / 127.0).max(-1.0)
}

fn signed_short_to_float(short: i16) -> f32 {
    (short as f32 / 32767.0).max(-1.0)
}

fn byte_stride(
    accessor: &goth_gltf::Accessor,
    buffer_view: &goth_gltf::BufferView<Extensions>,
) -> usize {
    buffer_view
        .extensions
        .ext_meshopt_compression
        .as_ref()
        .map(|ext| ext.byte_stride)
        .or(buffer_view.byte_stride)
        .unwrap_or_else(|| {
            accessor.component_type.byte_size() * accessor.accessor_type.num_components()
        })
}

pub fn read_buffer_with_accessor<'a>(
    buffer_view_map: &'a HashMap<usize, Vec<u8>>,
    gltf: &'a goth_gltf::Gltf<Extensions>,
    accessor: &goth_gltf::Accessor,
) -> anyhow::Result<(&'a [u8], Option<usize>)> {
    let buffer_view_index = accessor
        .buffer_view
        .ok_or_else(|| anyhow::anyhow!("Accessor is missing buffer view"))?;
    let buffer_view = gltf.buffer_views.get(buffer_view_index).ok_or_else(|| {
        anyhow::anyhow!("Buffer view index {} is out of range", buffer_view_index)
    })?;

    let start = accessor.byte_offset;
    let end = start + accessor.count * byte_stride(accessor, buffer_view);

    let buffer_view_bytes = buffer_view_map.get(&buffer_view_index).ok_or_else(|| {
        anyhow::anyhow!("Buffer view index {} is out of range", buffer_view_index)
    })?;

    // Force the end of the slice to be in-bounds as either the maths for calculating
    // `end` is wrong or some files are a little odd.
    let end = end.min(buffer_view_bytes.len());

    let slice = &buffer_view_bytes[start..end];

    Ok((slice, buffer_view.byte_stride))
}

pub fn read_f32<'a>(
    slice: &'a [u8],
    byte_stride: Option<usize>,
    accessor: &goth_gltf::Accessor,
) -> anyhow::Result<Cow<'a, [f32]>> {
    Ok(
        match (accessor.component_type, accessor.normalized, byte_stride) {
            (ComponentType::Float, false, None) => Cow::Borrowed(bytemuck::cast_slice(slice)),
            other => {
                return Err(anyhow::anyhow!(
                "{}: Unsupported combination of component type, normalized and byte stride: {:?}",
                std::line!(),
                other
            ))
            }
        },
    )
}

pub fn read_f32x3<'a>(
    slice: &'a [u8],
    byte_stride: Option<usize>,
    accessor: &goth_gltf::Accessor,
) -> anyhow::Result<Cow<'a, [Vec3]>> {
    Ok(
        match (accessor.component_type, accessor.normalized, byte_stride) {
            (ComponentType::Float, false, None | Some(12)) => {
                let slice: &[f32] = bytemuck::cast_slice(slice);
                Cow::Owned(
                    slice
                        .chunks(3)
                        .map(|slice| Vec3::from(<[f32; 3]>::try_from(slice).unwrap()))
                        .collect(),
                )
            }
            (ComponentType::Short, true, Some(stride)) => {
                let slice: &[i16] = bytemuck::cast_slice(slice);
                Cow::Owned(
                    slice
                        .chunks(stride / 2)
                        .map(|slice| {
                            Vec3::from(std::array::from_fn(|i| signed_short_to_float(slice[i])))
                        })
                        .collect(),
                )
            }
            (ComponentType::UnsignedShort, false, Some(8)) => {
                let slice: &[u16] = bytemuck::cast_slice(slice);
                Cow::Owned(
                    slice
                        .chunks(4)
                        .map(move |slice| Vec3::from(std::array::from_fn(|i| slice[i] as f32)))
                        .collect(),
                )
            }
            (ComponentType::UnsignedShort, true, Some(8)) => {
                let slice: &[u16] = bytemuck::cast_slice(slice);
                Cow::Owned(
                    slice
                        .chunks(4)
                        .map(|slice| {
                            Vec3::from(std::array::from_fn(|i| unsigned_short_to_float(slice[i])))
                        })
                        .collect(),
                )
            }
            (ComponentType::Byte, true, Some(stride)) => Cow::Owned(
                slice
                    .chunks(stride)
                    .map(move |slice| {
                        Vec3::from(std::array::from_fn(|i| {
                            signed_byte_to_float(slice[i] as i8)
                        }))
                    })
                    .collect(),
            ),
            other => {
                return Err(anyhow::anyhow!(
                "{}: Unsupported combination of component type, normalized and byte stride: {:?}",
                std::line!(),
                other
            ))
            }
        },
    )
}

fn read_f32x2<'a>(
    slice: &'a [u8],
    byte_stride: Option<usize>,
    accessor: &goth_gltf::Accessor,
) -> anyhow::Result<Cow<'a, [Vec2]>> {
    Ok(
        match (accessor.component_type, accessor.normalized, byte_stride) {
            (ComponentType::Float, false, None | Some(8)) => {
                Cow::Borrowed(bytemuck::cast_slice(slice))
            }
            (ComponentType::Float, false, Some(stride)) => {
                let slice: &[f32] = bytemuck::cast_slice(slice);
                Cow::Owned(
                    slice
                        .chunks(stride / 4)
                        .map(move |slice| Vec2::from(std::array::from_fn(|i| slice[i])))
                        .collect(),
                )
            }
            (ComponentType::UnsignedShort, true, Some(stride)) => {
                let slice: &[u16] = bytemuck::cast_slice(slice);
                Cow::Owned(
                    slice
                        .chunks(stride / 2)
                        .map(move |slice| {
                            Vec2::from(std::array::from_fn(|i| unsigned_short_to_float(slice[i])))
                        })
                        .collect(),
                )
            }
            other => {
                return Err(anyhow::anyhow!(
                "{}: Unsupported combination of component type, normalized and byte stride: {:?}",
                std::line!(),
                other
            ))
            }
        },
    )
}

unsafe fn cast_slice<T>(bytes: &[u8]) -> &[T] {
    std::slice::from_raw_parts(
        bytes.as_ptr() as *const T,
        bytes.len() / std::mem::size_of::<T>(),
    )
}

pub fn read_f32x4<'a>(
    slice: &'a [u8],
    byte_stride: Option<usize>,
    accessor: &goth_gltf::Accessor,
) -> anyhow::Result<Cow<'a, [Vec4]>> {
    Ok(
        match (accessor.component_type, accessor.normalized, byte_stride) {
            (ComponentType::Float, false, None) => {
                // bytemuck::cast_slice panics with an alignment issue on wasm so we just use unsafe for this.
                // todo: might be wrong.
                Cow::Borrowed(unsafe { cast_slice(slice) })
            }
            (ComponentType::UnsignedByte, true, Some(4)) => Cow::Owned(
                slice
                    .chunks(4)
                    .map(move |slice| {
                        Vec4::from(std::array::from_fn(|i| unsigned_byte_to_float(slice[i])))
                    })
                    .collect(),
            ),
            (ComponentType::Short, true, None) => {
                let slice: &[[i16; 4]] = bytemuck::cast_slice(slice);
                Cow::Owned(
                    slice
                        .iter()
                        .map(|slice| {
                            Vec4::from(std::array::from_fn(|i| signed_short_to_float(slice[i])))
                        })
                        .collect(),
                )
            }
            other => {
                return Err(anyhow::anyhow!(
                "{}: Unsupported combination of component type, normalized and byte stride: {:?}",
                std::line!(),
                other
            ))
            }
        },
    )
}

fn read_u32<'a>(
    slice: &'a [u8],
    byte_stride: Option<usize>,
    accessor: &goth_gltf::Accessor,
) -> anyhow::Result<Cow<'a, [u32]>> {
    Ok(
        match (accessor.component_type, accessor.normalized, byte_stride) {
            (ComponentType::UnsignedShort, false, None) => {
                let slice: &[u16] = bytemuck::cast_slice(slice);
                Cow::Owned(slice.iter().map(|&i| i as u32).collect())
            }
            (ComponentType::UnsignedInt, false, None) => Cow::Borrowed(bytemuck::cast_slice(slice)),
            other => {
                return Err(anyhow::anyhow!(
                "{}: Unsupported combination of component type, normalized and byte stride: {:?}",
                std::line!(),
                other
            ))
            }
        },
    )
}

fn read_u32x4<'a>(
    slice: &'a [u8],
    byte_stride: Option<usize>,
    accessor: &goth_gltf::Accessor,
) -> anyhow::Result<Cow<'a, [UVec4]>> {
    Ok(
        match (accessor.component_type, accessor.normalized, byte_stride) {
            (ComponentType::UnsignedByte, false, Some(4) | None) => Cow::Owned(
                slice
                    .chunks(4)
                    .map(|slice| UVec4::from(std::array::from_fn(|i| slice[i] as u32)))
                    .collect(),
            ),
            other => {
                return Err(anyhow::anyhow!(
                "{}: Unsupported combination of component type, normalized and byte stride: {:?}",
                std::line!(),
                other
            ))
            }
        },
    )
}

pub struct PrimitiveReader<'a> {
    gltf: &'a goth_gltf::Gltf<Extensions>,
    pub primitive: &'a goth_gltf::Primitive,
    buffer_view_map: &'a HashMap<usize, Vec<u8>>,
}

impl<'a> PrimitiveReader<'a> {
    pub fn new(
        gltf: &'a goth_gltf::Gltf<Extensions>,
        primitive: &'a goth_gltf::Primitive,
        buffer_view_map: &'a HashMap<usize, Vec<u8>>,
    ) -> Self {
        Self {
            gltf,
            primitive,
            buffer_view_map,
        }
    }

    pub fn read_indices(&self) -> anyhow::Result<Option<Cow<'a, [u32]>>> {
        let accessor_index = match self.primitive.indices {
            Some(index) => index,
            None => return Ok(None),
        };

        let accessor =
            self.gltf.accessors.get(accessor_index).ok_or_else(|| {
                anyhow::anyhow!("Accessor index {} out of bounds", accessor_index)
            })?;
        let (slice, byte_stride) =
            read_buffer_with_accessor(self.buffer_view_map, self.gltf, accessor)?;

        Ok(Some(read_u32(slice, byte_stride, accessor)?))
    }

    pub fn read_positions(&self) -> anyhow::Result<Option<Cow<'a, [Vec3]>>> {
        let accessor_index = match self.primitive.attributes.position {
            Some(index) => index,
            None => return Ok(None),
        };

        let accessor =
            self.gltf.accessors.get(accessor_index).ok_or_else(|| {
                anyhow::anyhow!("Accessor index {} out of bounds", accessor_index)
            })?;
        let (slice, byte_stride) =
            read_buffer_with_accessor(self.buffer_view_map, self.gltf, accessor)?;

        Ok(Some(read_f32x3(slice, byte_stride, accessor)?))
    }

    pub fn read_normals(&self) -> anyhow::Result<Option<Cow<'a, [Vec3]>>> {
        let accessor_index = match self.primitive.attributes.normal {
            Some(index) => index,
            None => return Ok(None),
        };

        let accessor =
            self.gltf.accessors.get(accessor_index).ok_or_else(|| {
                anyhow::anyhow!("Accessor index {} out of bounds", accessor_index)
            })?;
        let (slice, byte_stride) =
            read_buffer_with_accessor(self.buffer_view_map, self.gltf, accessor)?;

        Ok(Some(read_f32x3(slice, byte_stride, accessor)?))
    }

    pub fn read_uvs(&self) -> anyhow::Result<Option<Cow<'a, [Vec2]>>> {
        let accessor_index = match self.primitive.attributes.texcoord_0 {
            Some(index) => index,
            None => return Ok(None),
        };

        let accessor =
            self.gltf.accessors.get(accessor_index).ok_or_else(|| {
                anyhow::anyhow!("Accessor index {} out of bounds", accessor_index)
            })?;
        let (slice, byte_stride) =
            read_buffer_with_accessor(self.buffer_view_map, self.gltf, accessor)?;

        Ok(Some(read_f32x2(slice, byte_stride, accessor)?))
    }

    pub fn read_joints(&self) -> anyhow::Result<Option<Cow<'a, [UVec4]>>> {
        let accessor_index = match self.primitive.attributes.joints_0 {
            Some(index) => index,
            None => return Ok(None),
        };

        let accessor =
            self.gltf.accessors.get(accessor_index).ok_or_else(|| {
                anyhow::anyhow!("Accessor index {} out of bounds", accessor_index)
            })?;

        let (slice, byte_stride) =
            read_buffer_with_accessor(self.buffer_view_map, self.gltf, accessor)?;

        Ok(Some(read_u32x4(slice, byte_stride, accessor)?))
    }

    pub fn read_weights(&self) -> anyhow::Result<Option<Cow<'a, [Vec4]>>> {
        let accessor_index = match self.primitive.attributes.weights_0 {
            Some(index) => index,
            None => return Ok(None),
        };

        let accessor =
            self.gltf.accessors.get(accessor_index).ok_or_else(|| {
                anyhow::anyhow!("Accessor index {} out of bounds", accessor_index)
            })?;
        let (slice, byte_stride) =
            read_buffer_with_accessor(self.buffer_view_map, self.gltf, accessor)?;

        Ok(Some(read_f32x4(slice, byte_stride, accessor)?))
    }
}
