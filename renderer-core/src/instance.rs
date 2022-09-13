use glam::Vec3;

#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct LineVertex {
    pub position: Vec3,
    pub colour_id: u32,
}

#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct GpuInstance {
    pub similarity: gltf_helpers::Similarity,
    pub joints_offset: u32,
    pub _padding: [u32; 3],
}

pub type Instance = gltf_helpers::Similarity;

#[cfg(feature = "wasm")]
pub fn instance_from_transform(transform: web_sys::XrRigidTransform, scale: f32) -> Instance {
    let rotation = transform.orientation();
    let position = transform.position();

    Instance {
        translation: glam::DVec3::new(position.x(), position.y(), position.z()).as_vec3(),
        rotation: glam::DQuat::from_xyzw(rotation.x(), rotation.y(), rotation.z(), rotation.w())
            .as_f32(),
        scale,
    }
}
