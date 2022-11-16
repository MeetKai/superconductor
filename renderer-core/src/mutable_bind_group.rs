use crate::Texture;
use arc_swap::ArcSwap;
use std::sync::Arc;

#[derive(Debug)]
pub struct MutableBindGroup {
    entries: parking_lot::Mutex<Vec<Entry>>,
    bind_group: ArcSwap<wgpu::BindGroup>,
}

impl MutableBindGroup {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, entries: Vec<Entry>) -> Self {
        Self {
            bind_group: ArcSwap::from_pointee(bind_group_from_entries(device, layout, &entries)),
            entries: parking_lot::Mutex::new(entries),
        }
    }

    pub fn mutate<F: FnOnce(&mut [Entry])>(
        &self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        func: F,
    ) {
        let mut entries = self.entries.lock();
        func(&mut entries);
        self.bind_group
            .store(Arc::new(bind_group_from_entries(device, layout, &entries)));
    }

    pub fn load(&self) -> arc_swap::Guard<Arc<wgpu::BindGroup>> {
        self.bind_group.load()
    }
}

fn bind_group_from_entries(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    entries: &[Entry],
) -> wgpu::BindGroup {
    let entries: Vec<_> = entries
        .iter()
        .enumerate()
        .map(|(i, entry)| wgpu::BindGroupEntry {
            binding: i as u32,
            resource: match entry {
                Entry::Buffer(buffer) => buffer.as_entire_binding(),
                Entry::Texture(texture) => wgpu::BindingResource::TextureView(&texture.view),
                Entry::Sampler(sampler) => wgpu::BindingResource::Sampler(sampler),
            },
        })
        .collect();

    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout,
        entries: &entries,
        label: None,
    })
}

#[derive(Debug)]
pub enum Entry {
    Buffer(Arc<wgpu::Buffer>),
    Texture(Arc<Texture>),
    Sampler(Arc<wgpu::Sampler>),
}
