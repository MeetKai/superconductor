use arc_swap::ArcSwap;
use std::mem::size_of;
use std::ops::Range;
use std::sync::Arc;

use super::Instance;
use glam::{UVec4, Vec2, Vec3, Vec4};

pub type InstanceBuffer = VecGpuBuffer<Instance>;

pub struct VecGpuBuffer<T: bytemuck::Pod> {
    offset: u32,
    capacity: u32,
    pub buffer: wgpu::Buffer,
    usage: wgpu::BufferUsages,
    _phantom: std::marker::PhantomData<T>,
    label: &'static str,
}

impl<T: bytemuck::Pod> VecGpuBuffer<T> {
    fn size_in_bytes(size: u32) -> u64 {
        size as u64 * size_of::<T>() as u64
    }

    pub fn new(
        capacity: u32,
        device: &wgpu::Device,
        usage: wgpu::BufferUsages,
        label: &'static str,
    ) -> Self {
        Self {
            offset: Default::default(),
            capacity,
            buffer: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: Self::size_in_bytes(capacity),
                usage: usage | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            usage,
            label,
            _phantom: Default::default(),
        }
    }

    pub fn clear(&mut self) {
        self.offset = 0;
    }

    pub fn push(
        &mut self,
        instances: &[T],
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        command_encoder: &mut wgpu::CommandEncoder,
    ) -> Range<u32> {
        let start = self.offset;
        let end = start + instances.len() as u32;

        if end > self.capacity {
            self.resize(end, device, command_encoder);
        }

        queue.write_buffer(
            &self.buffer,
            Self::size_in_bytes(start),
            bytemuck::cast_slice(instances),
        );

        self.offset = end;

        start..end
    }

    fn resize(
        &mut self,
        required_capacity: u32,
        device: &wgpu::Device,
        command_encoder: &mut wgpu::CommandEncoder,
    ) {
        let copy_size = Self::size_in_bytes(self.offset);

        let new_capacity = required_capacity.max(self.capacity * 2);

        log::info!(
            "Growing {} from {} to {}",
            self.label,
            self.capacity,
            new_capacity
        );

        let new_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(self.label),
            size: Self::size_in_bytes(new_capacity),
            usage: self.usage | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        command_encoder.copy_buffer_to_buffer(&self.buffer, 0, &new_buffer, 0, copy_size);

        self.buffer = new_buffer;
        self.capacity = new_capacity;
    }
}

pub struct IndexBuffer {
    allocator: parking_lot::Mutex<range_alloc::RangeAllocator<u32>>,
    pub buffer: arc_swap::ArcSwap<wgpu::Buffer>,
}

impl IndexBuffer {
    fn size_in_bytes(size: u32) -> u64 {
        size as u64 * size_of::<u32>() as u64
    }

    pub fn new(capacity: u32, device: &wgpu::Device) -> Self {
        Self {
            allocator: parking_lot::Mutex::new(range_alloc::RangeAllocator::new(0..capacity)),
            buffer: arc_swap::ArcSwap::from_pointee(device.create_buffer(
                &wgpu::BufferDescriptor {
                    label: Some("index buffer"),
                    size: Self::size_in_bytes(capacity),
                    usage: wgpu::BufferUsages::INDEX
                        | wgpu::BufferUsages::COPY_DST
                        | wgpu::BufferUsages::COPY_SRC,
                    mapped_at_creation: false,
                },
            )),
        }
    }

    pub fn insert(
        &self,
        indices: &[u32],
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        command_encoder: &mut wgpu::CommandEncoder,
    ) -> Range<u32> {
        let length = indices.len() as u32;

        // Use the allocator to find a range in the buffer to write to,
        // resizing the buffer in needed and returning the correct buffer to write to
        // (as `ArcSwap::load` does not always return the newest value).
        let (buffer, range) = {
            let mut allocator = self.allocator.lock();

            match allocator.allocate_range(length) {
                Ok(range) => (self.buffer.load_full(), range),
                Err(_) => {
                    let new_buffer = Self::resize(
                        &mut allocator,
                        &self.buffer,
                        length,
                        device,
                        command_encoder,
                    );
                    let range = allocator.allocate_range(length).expect("just resized");
                    (new_buffer, range)
                }
            }
        };

        queue.write_buffer(
            &buffer,
            Self::size_in_bytes(range.start),
            bytemuck::cast_slice(indices),
        );

        range
    }

    fn resize(
        allocator: &mut range_alloc::RangeAllocator<u32>,
        buffer: &ArcSwap<wgpu::Buffer>,
        required_capacity: u32,
        device: &wgpu::Device,
        command_encoder: &mut wgpu::CommandEncoder,
    ) -> Arc<wgpu::Buffer> {
        let copy_range = allocator
            .allocated_ranges()
            .last()
            .map(|range| range.end)
            .unwrap_or(0);

        let old_capacity = allocator.initial_range().end;

        let new_capacity = (old_capacity + required_capacity).max(old_capacity * 2);

        log::info!(
            "Growing index buffer from {} to {}",
            old_capacity,
            new_capacity
        );

        allocator.grow_to(new_capacity);

        let new_buffer = Arc::new(device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("index buffer"),
            size: Self::size_in_bytes(new_capacity),
            usage: wgpu::BufferUsages::INDEX
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        }));

        command_encoder.copy_buffer_to_buffer(
            &buffer.load(),
            0,
            &new_buffer,
            0,
            Self::size_in_bytes(copy_range),
        );

        buffer.store(new_buffer.clone());

        new_buffer
    }
}

pub struct RawVertexBuffers<T> {
    pub position: T,
    pub normal: T,
    pub uv: T,
}

impl RawVertexBuffers<ArcSwap<wgpu::Buffer>> {
    pub fn load(&self) -> RawVertexBuffers<arc_swap::Guard<Arc<wgpu::Buffer>>> {
        RawVertexBuffers {
            position: self.position.load(),
            normal: self.normal.load(),
            uv: self.uv.load(),
        }
    }
}

pub struct VertexBuffers {
    allocator: parking_lot::Mutex<range_alloc::RangeAllocator<u32>>,
    pub buffers: RawVertexBuffers<ArcSwap<wgpu::Buffer>>,
}

impl VertexBuffers {
    pub fn new(capacity: u32, device: &wgpu::Device) -> Self {
        Self {
            allocator: parking_lot::Mutex::new(range_alloc::RangeAllocator::new(0..capacity)),
            buffers: RawVertexBuffers {
                position: ArcSwap::from(create_buffer(
                    device,
                    "position buffer",
                    capacity,
                    size_of::<Vec3>(),
                )),
                normal: ArcSwap::from(create_buffer(
                    device,
                    "normal buffer",
                    capacity,
                    size_of::<Vec3>(),
                )),
                uv: ArcSwap::from(create_buffer(
                    device,
                    "normal buffer",
                    capacity,
                    size_of::<Vec2>(),
                )),
            },
        }
    }

    pub fn insert(
        &self,
        positions: &[Vec3],
        normals: &[Vec3],
        uvs: &[Vec2],
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        command_encoder: &mut wgpu::CommandEncoder,
    ) -> Range<u32> {
        let length = positions.len() as u32;

        debug_assert_eq!(positions.len(), normals.len());
        debug_assert_eq!(positions.len(), uvs.len());

        let (buffers, range) = {
            let mut allocator = self.allocator.lock();

            match allocator.allocate_range(length) {
                Ok(range) => {
                    let buffers = RawVertexBuffers {
                        position: self.buffers.position.load_full(),
                        normal: self.buffers.normal.load_full(),
                        uv: self.buffers.uv.load_full(),
                    };

                    (buffers, range)
                }
                Err(_) => {
                    let new_buffers = Self::resize(
                        &mut allocator,
                        &self.buffers,
                        length,
                        device,
                        command_encoder,
                    );
                    let range = allocator.allocate_range(length).expect("just resized");
                    (new_buffers, range)
                }
            }
        };

        queue.write_buffer(
            &buffers.position,
            size_in_bytes(range.start, size_of::<Vec3>()),
            bytemuck::cast_slice(positions),
        );

        queue.write_buffer(
            &buffers.normal,
            size_in_bytes(range.start, size_of::<Vec3>()),
            bytemuck::cast_slice(normals),
        );

        queue.write_buffer(
            &buffers.uv,
            size_in_bytes(range.start, size_of::<Vec2>()),
            bytemuck::cast_slice(uvs),
        );

        range
    }

    fn resize(
        allocator: &mut range_alloc::RangeAllocator<u32>,
        buffers: &RawVertexBuffers<ArcSwap<wgpu::Buffer>>,
        required_capacity: u32,
        device: &wgpu::Device,
        command_encoder: &mut wgpu::CommandEncoder,
    ) -> RawVertexBuffers<Arc<wgpu::Buffer>> {
        let copy_range = allocator
            .allocated_ranges()
            .last()
            .map(|range| range.end)
            .unwrap_or(0);

        let old_capacity = allocator.initial_range().end;

        let new_capacity = (old_capacity + required_capacity).max(old_capacity * 2);

        log::info!(
            "Growing vertex buffers from {} to {}",
            old_capacity,
            new_capacity
        );

        allocator.grow_to(new_capacity);

        let new_buffers = RawVertexBuffers {
            position: create_buffer(device, "position buffer", new_capacity, size_of::<Vec3>()),
            normal: create_buffer(device, "normal buffer", new_capacity, size_of::<Vec3>()),
            uv: create_buffer(device, "uv buffer", new_capacity, size_of::<Vec2>()),
        };

        let current_buffers = buffers.load();

        command_encoder.copy_buffer_to_buffer(
            &current_buffers.position,
            0,
            &new_buffers.position,
            0,
            size_in_bytes(copy_range, size_of::<Vec3>()),
        );
        command_encoder.copy_buffer_to_buffer(
            &current_buffers.normal,
            0,
            &new_buffers.normal,
            0,
            size_in_bytes(copy_range, size_of::<Vec3>()),
        );
        command_encoder.copy_buffer_to_buffer(
            &current_buffers.uv,
            0,
            &new_buffers.uv,
            0,
            size_in_bytes(copy_range, size_of::<Vec2>()),
        );

        buffers.position.store(new_buffers.position.clone());
        buffers.normal.store(new_buffers.normal.clone());
        buffers.uv.store(new_buffers.uv.clone());

        new_buffers
    }
}

pub struct RawAnimatedVertexBuffers<T> {
    pub position: T,
    pub normal: T,
    pub uv: T,
    pub joint_indices: T,
    pub joint_weights: T,
}

impl RawAnimatedVertexBuffers<ArcSwap<wgpu::Buffer>> {
    pub fn load(&self) -> RawAnimatedVertexBuffers<arc_swap::Guard<Arc<wgpu::Buffer>>> {
        RawAnimatedVertexBuffers {
            position: self.position.load(),
            normal: self.normal.load(),
            uv: self.uv.load(),
            joint_indices: self.joint_indices.load(),
            joint_weights: self.joint_weights.load(),
        }
    }
}

fn size_in_bytes(size: u32, size_of_field: usize) -> u64 {
    size as u64 * size_of_field as u64
}

fn create_buffer(
    device: &wgpu::Device,
    label: &str,
    capacity: u32,
    size_of_field: usize,
) -> Arc<wgpu::Buffer> {
    Arc::new(device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: size_in_bytes(capacity, size_of_field),
        usage: wgpu::BufferUsages::VERTEX
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    }))
}

pub struct AnimatedVertexBuffers {
    allocator: parking_lot::Mutex<range_alloc::RangeAllocator<u32>>,
    pub buffers: RawAnimatedVertexBuffers<ArcSwap<wgpu::Buffer>>,
}

impl AnimatedVertexBuffers {
    pub fn new(capacity: u32, device: &wgpu::Device) -> Self {
        Self {
            allocator: parking_lot::Mutex::new(range_alloc::RangeAllocator::new(0..capacity)),
            buffers: RawAnimatedVertexBuffers {
                position: ArcSwap::from(create_buffer(
                    device,
                    "position buffer",
                    capacity,
                    size_of::<Vec3>(),
                )),
                normal: ArcSwap::from(create_buffer(
                    device,
                    "normal buffer",
                    capacity,
                    size_of::<Vec3>(),
                )),
                uv: ArcSwap::from(create_buffer(
                    device,
                    "normal buffer",
                    capacity,
                    size_of::<Vec2>(),
                )),
                joint_indices: ArcSwap::from(create_buffer(
                    device,
                    "joint indices buffer",
                    capacity,
                    size_of::<UVec4>(),
                )),
                joint_weights: ArcSwap::from(create_buffer(
                    device,
                    "joint weights buffer",
                    capacity,
                    size_of::<Vec4>(),
                )),
            },
        }
    }

    pub fn insert(
        &self,
        positions: &[Vec3],
        normals: &[Vec3],
        uvs: &[Vec2],
        joint_indices: &[UVec4],
        joint_weights: &[Vec4],
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        command_encoder: &mut wgpu::CommandEncoder,
    ) -> Range<u32> {
        let length = positions.len() as u32;

        debug_assert_eq!(positions.len(), normals.len());
        debug_assert_eq!(positions.len(), uvs.len());
        debug_assert_eq!(positions.len(), joint_indices.len());
        debug_assert_eq!(positions.len(), joint_weights.len());

        let (buffers, range) = {
            let mut allocator = self.allocator.lock();

            match allocator.allocate_range(length) {
                Ok(range) => {
                    let buffers = RawAnimatedVertexBuffers {
                        position: self.buffers.position.load_full(),
                        normal: self.buffers.normal.load_full(),
                        uv: self.buffers.uv.load_full(),
                        joint_indices: self.buffers.joint_indices.load_full(),
                        joint_weights: self.buffers.joint_weights.load_full(),
                    };

                    (buffers, range)
                }
                Err(_) => {
                    let new_buffers = Self::resize(
                        &mut allocator,
                        &self.buffers,
                        length,
                        device,
                        command_encoder,
                    );
                    let range = allocator.allocate_range(length).expect("just resized");
                    (new_buffers, range)
                }
            }
        };

        queue.write_buffer(
            &buffers.position,
            size_in_bytes(range.start, size_of::<Vec3>()),
            bytemuck::cast_slice(positions),
        );

        queue.write_buffer(
            &buffers.normal,
            size_in_bytes(range.start, size_of::<Vec3>()),
            bytemuck::cast_slice(normals),
        );

        queue.write_buffer(
            &buffers.uv,
            size_in_bytes(range.start, size_of::<Vec2>()),
            bytemuck::cast_slice(uvs),
        );

        queue.write_buffer(
            &buffers.joint_indices,
            size_in_bytes(range.start, size_of::<UVec4>()),
            bytemuck::cast_slice(joint_indices),
        );

        queue.write_buffer(
            &buffers.joint_weights,
            size_in_bytes(range.start, size_of::<Vec4>()),
            bytemuck::cast_slice(joint_weights),
        );

        range
    }

    fn resize(
        allocator: &mut range_alloc::RangeAllocator<u32>,
        buffers: &RawAnimatedVertexBuffers<ArcSwap<wgpu::Buffer>>,
        required_capacity: u32,
        device: &wgpu::Device,
        command_encoder: &mut wgpu::CommandEncoder,
    ) -> RawAnimatedVertexBuffers<Arc<wgpu::Buffer>> {
        let copy_range = allocator
            .allocated_ranges()
            .last()
            .map(|range| range.end)
            .unwrap_or(0);

        let old_capacity = allocator.initial_range().end;

        let new_capacity = (old_capacity + required_capacity).max(old_capacity * 2);

        log::info!(
            "Growing animated vertex buffers from {} to {}",
            old_capacity,
            new_capacity
        );

        allocator.grow_to(new_capacity);

        let new_buffers = RawAnimatedVertexBuffers {
            position: create_buffer(device, "position buffer", new_capacity, size_of::<Vec3>()),
            normal: create_buffer(device, "normal buffer", new_capacity, size_of::<Vec3>()),
            uv: create_buffer(device, "uv buffer", new_capacity, size_of::<Vec2>()),
            joint_indices: create_buffer(
                device,
                "joint indices buffer",
                new_capacity,
                size_of::<UVec4>(),
            ),
            joint_weights: create_buffer(
                device,
                "joint weights buffer",
                new_capacity,
                size_of::<Vec4>(),
            ),
        };

        let current_buffers = buffers.load();

        command_encoder.copy_buffer_to_buffer(
            &current_buffers.position,
            0,
            &new_buffers.position,
            0,
            size_in_bytes(copy_range, size_of::<Vec3>()),
        );
        command_encoder.copy_buffer_to_buffer(
            &current_buffers.normal,
            0,
            &new_buffers.normal,
            0,
            size_in_bytes(copy_range, size_of::<Vec3>()),
        );
        command_encoder.copy_buffer_to_buffer(
            &current_buffers.uv,
            0,
            &new_buffers.uv,
            0,
            size_in_bytes(copy_range, size_of::<Vec2>()),
        );
        command_encoder.copy_buffer_to_buffer(
            &current_buffers.joint_indices,
            0,
            &new_buffers.joint_indices,
            0,
            size_in_bytes(copy_range, size_of::<UVec4>()),
        );
        command_encoder.copy_buffer_to_buffer(
            &current_buffers.joint_weights,
            0,
            &new_buffers.joint_weights,
            0,
            size_in_bytes(copy_range, size_of::<Vec4>()),
        );

        buffers.position.store(new_buffers.position.clone());
        buffers.normal.store(new_buffers.normal.clone());
        buffers.uv.store(new_buffers.uv.clone());
        buffers
            .joint_indices
            .store(new_buffers.joint_indices.clone());
        buffers
            .joint_weights
            .store(new_buffers.joint_weights.clone());

        new_buffers
    }
}
