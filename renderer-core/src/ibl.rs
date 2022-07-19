use crate::Texture;
use arc_swap::ArcSwap;

pub struct IblTextures {
    pub lut: ArcSwap<Texture>,
    pub cubemap: ArcSwap<Texture>,
    pub sphere_harmonics: ArcSwap<wgpu::Buffer>,
}
