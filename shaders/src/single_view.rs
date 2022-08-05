use super::*;

#[spirv(vertex)]
pub fn vertex(
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
    instance_translation_and_scale: Vec4,
    instance_rotation: glam::Quat,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(position)] builtin_pos: &mut Vec4,
    out_position: &mut Vec3,
    out_normal: &mut Vec3,
    out_uv: &mut Vec2,
) {
    super::vertex(
        position,
        normal,
        uv,
        instance_translation_and_scale,
        instance_rotation,
        uniforms,
        builtin_pos,
        0,
        out_position,
        out_normal,
        out_uv,
    );
}

#[spirv(vertex)]
pub fn animated_vertex(
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
    instance_translation_and_scale: Vec4,
    instance_rotation: glam::Quat,
    #[spirv(flat)] joints_offset: u32,
    #[spirv(flat)] joint_indices: UVec4,
    joint_weights: Vec4,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 2, binding = 0, uniform)]
    joint_transforms: &[JointTransform; JointTransform::MAX_COUNT],
    #[spirv(position)] builtin_pos: &mut Vec4,
    out_position: &mut Vec3,
    out_normal: &mut Vec3,
    out_uv: &mut Vec2,
) {
    super::animated_vertex(
        position,
        normal,
        uv,
        instance_translation_and_scale,
        instance_rotation,
        joints_offset,
        joint_indices,
        joint_weights,
        uniforms,
        joint_transforms,
        builtin_pos,
        0,
        out_position,
        out_normal,
        out_uv,
    );
}

#[spirv(fragment)]
pub fn fragment(
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 0, binding = 1)] sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 2)] ibl_lut: &SampledImage,
    #[spirv(descriptor_set = 0, binding = 3)] ibl_cubemap: &Image!(cube, type=f32, sampled),
    #[spirv(descriptor_set = 0, binding = 4, uniform)] sphere_harmonics: &SphereHarmonics,
    #[spirv(descriptor_set = 1, binding = 0)] albedo_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 1)] normal_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 2)] metallic_roughness_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 3)] emissive_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 4, uniform)] material_settings: &MaterialSettings,
    #[spirv(descriptor_set = 1, binding = 5)] texture_sampler: &Sampler,
    #[spirv(front_facing)] front_facing: bool,
    output: &mut Vec4,
) {
    super::fragment(
        position,
        normal,
        uv,
        uniforms,
        sampler,
        ibl_lut,
        ibl_cubemap,
        sphere_harmonics,
        albedo_texture,
        normal_texture,
        metallic_roughness_texture,
        emissive_texture,
        material_settings,
        texture_sampler,
        0,
        front_facing,
        output,
    );
}

#[spirv(fragment)]
pub fn fragment_alpha_blended(
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 0, binding = 1)] sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 2)] ibl_lut: &SampledImage,
    #[spirv(descriptor_set = 0, binding = 3)] ibl_cubemap: &Image!(cube, type=f32, sampled),
    #[spirv(descriptor_set = 0, binding = 4, uniform)] sphere_harmonics: &SphereHarmonics,
    #[spirv(descriptor_set = 1, binding = 0)] albedo_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 1)] normal_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 2)] metallic_roughness_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 3)] emissive_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 4, uniform)] material_settings: &MaterialSettings,
    #[spirv(descriptor_set = 1, binding = 5)] texture_sampler: &Sampler,
    #[spirv(front_facing)] front_facing: bool,
    output: &mut Vec4,
) {
    super::fragment_alpha_blended(
        position,
        normal,
        uv,
        uniforms,
        sampler,
        ibl_lut,
        ibl_cubemap,
        sphere_harmonics,
        albedo_texture,
        normal_texture,
        metallic_roughness_texture,
        emissive_texture,
        material_settings,
        texture_sampler,
        0,
        front_facing,
        output,
    );
}

#[spirv(fragment)]
pub fn fragment_alpha_clipped(
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 0, binding = 1)] sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 2)] ibl_lut: &SampledImage,
    #[spirv(descriptor_set = 0, binding = 3)] ibl_cubemap: &Image!(cube, type=f32, sampled),
    #[spirv(descriptor_set = 0, binding = 4, uniform)] sphere_harmonics: &SphereHarmonics,
    #[spirv(descriptor_set = 1, binding = 0)] albedo_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 1)] normal_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 2)] metallic_roughness_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 3)] emissive_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 4, uniform)] material_settings: &MaterialSettings,
    #[spirv(descriptor_set = 1, binding = 5)] texture_sampler: &Sampler,
    #[spirv(front_facing)] front_facing: bool,
    output: &mut Vec4,
) {
    super::fragment_alpha_clipped(
        position,
        normal,
        uv,
        uniforms,
        sampler,
        ibl_lut,
        ibl_cubemap,
        sphere_harmonics,
        albedo_texture,
        normal_texture,
        metallic_roughness_texture,
        emissive_texture,
        material_settings,
        texture_sampler,
        0,
        front_facing,
        output,
    );
}

#[spirv(vertex)]
pub fn vertex_skybox(
    #[spirv(vertex_index)] vertex_index: i32,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 1, binding = 0, uniform)] skybox_uniforms: &SkyboxUniforms,
    #[spirv(position)] builtin_pos: &mut Vec4,
    ray: &mut Vec3,
) {
    super::vertex_skybox(vertex_index, uniforms, skybox_uniforms, builtin_pos, 0, ray);
}

#[spirv(vertex)]
pub fn line_vertex(
    position: Vec3,
    #[spirv(flat)] colour_id: u32,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(position)] builtin_pos: &mut Vec4,
    colour: &mut Vec3,
) {
    super::line_vertex(position, colour_id, uniforms, builtin_pos, 0, colour);
}

#[spirv(fragment)]
pub fn tonemap(
    uv: Vec2,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] _uniforms: &Uniforms,
    #[spirv(descriptor_set = 1, binding = 0)] sampler: &Sampler,
    #[spirv(descriptor_set = 1, binding = 1)] texture: &Image!(2D, type=f32, sampled),
    output: &mut Vec4,
) {
    let sample: Vec4 = texture.sample(*sampler, uv);

    let linear = aces_filmic(sample.truncate());

    *output = linear_to_srgb_approx(linear).extend(1.0)
}
