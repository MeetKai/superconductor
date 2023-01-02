use super::*;

#[spirv(vertex)]
pub fn vertex(
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
    lightmap_uv: Vec2,
    instance_translation_and_scale: Vec4,
    instance_rotation: glam::Quat,
    #[spirv(flat)] joints_offset: u32,
    #[spirv(flat)] material_index: u32,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 1, binding = 4, uniform)] material_settings: &MaterialSettings,
    #[spirv(position)] builtin_pos: &mut Vec4,
    out_position: &mut Vec3,
    out_normal: &mut Vec3,
    out_uv: &mut Vec2,
    out_lightmap_uv: &mut Vec2,
    #[spirv(flat)] out_material_index: &mut u32,
) {
    super::vertex(
        position,
        normal,
        uv,
        lightmap_uv,
        instance_translation_and_scale,
        instance_rotation,
        joints_offset,
        material_index,
        uniforms,
        material_settings,
        builtin_pos,
        0,
        out_position,
        out_normal,
        out_uv,
        out_lightmap_uv,
        out_material_index,
    );
}

#[spirv(vertex)]
pub fn animated_vertex(
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
    #[spirv(flat)] joint_indices: UVec4,
    instance_translation_and_scale: Vec4,
    instance_rotation: glam::Quat,
    #[spirv(flat)] joints_offset: u32,
    #[spirv(flat)] material_index: u32,
    joint_weights: Vec4,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 1, binding = 4, uniform)] material_settings: &MaterialSettings,
    #[spirv(descriptor_set = 2, binding = 0, uniform)]
    joint_transforms: &[JointTransform; JointTransform::MAX_COUNT],
    #[spirv(position)] builtin_pos: &mut Vec4,
    out_position: &mut Vec3,
    out_normal: &mut Vec3,
    out_uv: &mut Vec2,
    out_lightmap_uv: &mut Vec2,
    #[spirv(flat)] out_material_index: &mut u32,
) {
    super::animated_vertex(
        position,
        normal,
        uv,
        joint_indices,
        instance_translation_and_scale,
        instance_rotation,
        joints_offset,
        material_index,
        joint_weights,
        uniforms,
        material_settings,
        joint_transforms,
        builtin_pos,
        0,
        out_position,
        out_normal,
        out_uv,
        out_lightmap_uv,
        out_material_index,
    );
}

#[spirv(fragment)]
pub fn fragment(
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
    lightmap_uv: Vec2,
    #[spirv(flat)] material_index: u32,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 0, binding = 1)] clamp_sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 3)] sh_l_0: &Image!(3D, type=f32, sampled),
    #[spirv(descriptor_set = 0, binding = 4)] sh_l_1_x: &Image!(3D, type=f32, sampled),
    #[spirv(descriptor_set = 0, binding = 5)] sh_l_1_y: &Image!(3D, type=f32, sampled),
    #[spirv(descriptor_set = 0, binding = 6)] sh_l_1_z: &Image!(3D, type=f32, sampled),
    #[spirv(descriptor_set = 0, binding = 7)] lightmap: &SampledImage,

    #[spirv(descriptor_set = 0, binding = 8)] lightmap_x: &SampledImage,
    #[spirv(descriptor_set = 0, binding = 9)] lightmap_y: &SampledImage,
    #[spirv(descriptor_set = 0, binding = 10)] lightmap_z: &SampledImage,

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
        lightmap_uv,
        material_index,
        uniforms,
        clamp_sampler,
        sh_l_0,
        sh_l_1_x,
        sh_l_1_y,
        sh_l_1_z,
        lightmap,
        lightmap_x,
        lightmap_y,
        lightmap_z,
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
    lightmap_uv: Vec2,
    #[spirv(flat)] material_index: u32,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 0, binding = 1)] clamp_sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 3)] sh_l_0: &Image!(3D, type=f32, sampled),
    #[spirv(descriptor_set = 0, binding = 4)] sh_l_1_x: &Image!(3D, type=f32, sampled),
    #[spirv(descriptor_set = 0, binding = 5)] sh_l_1_y: &Image!(3D, type=f32, sampled),
    #[spirv(descriptor_set = 0, binding = 6)] sh_l_1_z: &Image!(3D, type=f32, sampled),
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
        lightmap_uv,
        material_index,
        uniforms,
        clamp_sampler,
        sh_l_0,
        sh_l_1_x,
        sh_l_1_y,
        sh_l_1_z,
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
    lightmap_uv: Vec2,
    #[spirv(flat)] material_index: u32,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 0, binding = 1)] clamp_sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 3)] sh_l_0: &Image!(3D, type=f32, sampled),
    #[spirv(descriptor_set = 0, binding = 4)] sh_l_1_x: &Image!(3D, type=f32, sampled),
    #[spirv(descriptor_set = 0, binding = 5)] sh_l_1_y: &Image!(3D, type=f32, sampled),
    #[spirv(descriptor_set = 0, binding = 6)] sh_l_1_z: &Image!(3D, type=f32, sampled),
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
        lightmap_uv,
        material_index,
        uniforms,
        clamp_sampler,
        sh_l_0,
        sh_l_1_x,
        sh_l_1_y,
        sh_l_1_z,
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
    #[spirv(position)] builtin_pos: &mut Vec4,
    ray: &mut Vec3,
) {
    super::vertex_skybox(vertex_index, uniforms, builtin_pos, 0, ray);
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

#[spirv(vertex)]
pub fn depth_prepass_vertex(
    position: Vec3,
    instance_translation_and_scale: Vec4,
    instance_rotation: glam::Quat,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(position)] builtin_pos: &mut Vec4,
) {
    super::depth_prepass_vertex(
        position,
        instance_translation_and_scale,
        instance_rotation,
        uniforms,
        builtin_pos,
        0,
    )
}
