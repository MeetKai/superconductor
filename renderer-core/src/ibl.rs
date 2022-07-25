use crate::Texture;
use arc_swap::ArcSwap;

pub struct IblResources {
    pub lut: ArcSwap<Texture>,
    pub cubemap: ArcSwap<Texture>,
    pub sphere_harmonics: wgpu::Buffer,
}
