#![cfg_attr(target_arch = "spirv", no_std)]

use shared_structs::{
    eval_spherical_harmonics_nonlinear, JointTransform, MaterialSettings, Settings, Uniforms,
};
use spirv_std::{
    arch::IndexUnchecked,
    glam::{self, Mat3, UVec4, Vec2, Vec3, Vec4},
    num_traits::Float,
    spirv, Image, Sampler,
};

type SampledImage = Image!(2D, type=f32, sampled);
type Image3D = Image!(3D, type=f32, sampled);

mod single_view;

pub use single_view::{
    animated_vertex as _, depth_prepass_vertex as _, fragment as _, fragment_alpha_blended as _,
    fragment_alpha_clipped as _, line_vertex as _, tonemap as _, vertex as _, vertex_skybox as _,
};

#[spirv(vertex)]
pub fn vertex(
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
    instance_translation_and_scale: Vec4,
    instance_rotation: glam::Quat,
    _joints_offset: u32,
    material_index: u32,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 1, binding = 4, uniform)] material_settings: &MaterialSettings,
    #[spirv(position)] builtin_pos: &mut Vec4,
    #[spirv(view_index)] view_index: i32,
    out_position: &mut Vec3,
    out_normal: &mut Vec3,
    out_uv: &mut Vec2,
    #[spirv(flat)] out_material_index: &mut u32,
) {
    let instance_scale = instance_translation_and_scale.w;
    let instance_translation = instance_translation_and_scale.truncate();

    let position = instance_translation + (instance_rotation * (instance_scale * position));
    *builtin_pos = uniforms.projection_view(view_index) * position.extend(1.0);
    *out_position = position;
    *out_normal = instance_rotation * normal;
    *out_uv = material_settings.transform_uv(uv);
    *out_material_index = material_index;

    if uniforms.settings.contains(Settings::FLIP_VIEWPORT) {
        builtin_pos.y = -builtin_pos.y;
    }
}

#[spirv(vertex)]
pub fn animated_vertex(
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
    instance_translation_and_scale: Vec4,
    instance_rotation: glam::Quat,
    joints_offset: u32,
    material_index: u32,
    joint_indices: UVec4,
    joint_weights: Vec4,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 1, binding = 4, uniform)] material_settings: &MaterialSettings,
    #[spirv(descriptor_set = 2, binding = 0, uniform)]
    joint_transforms: &[JointTransform; JointTransform::MAX_COUNT],
    #[spirv(position)] builtin_pos: &mut Vec4,
    #[spirv(view_index)] view_index: i32,
    out_position: &mut Vec3,
    out_normal: &mut Vec3,
    out_uv: &mut Vec2,
    #[spirv(flat)] out_material_index: &mut u32,
) {
    let instance_scale = instance_translation_and_scale.w;
    let instance_translation = instance_translation_and_scale.truncate();

    let global_joint_indices = joint_indices + joints_offset;
    let joint_weights =
        joint_weights / (joint_weights.x + joint_weights.y + joint_weights.z + joint_weights.w);

    // Calculate the skinned position and normal by multiplying them by the joint transforms and perfoming a weighted average.

    #[rustfmt::skip]
    let position = unsafe {
        (*joint_transforms.index_unchecked(global_joint_indices.x as usize) * position * joint_weights.x)
            + (*joint_transforms.index_unchecked(global_joint_indices.y as usize) * position * joint_weights.y)
            + (*joint_transforms.index_unchecked(global_joint_indices.z as usize) * position * joint_weights.z)
            + (*joint_transforms.index_unchecked(global_joint_indices.w as usize) * position * joint_weights.w)
    };

    #[rustfmt::skip]
    let normal = unsafe {
        (joint_transforms.index_unchecked(global_joint_indices.x as usize).rotation * normal * joint_weights.x)
            + (joint_transforms.index_unchecked(global_joint_indices.y as usize).rotation * normal * joint_weights.y)
            + (joint_transforms.index_unchecked(global_joint_indices.z as usize).rotation * normal * joint_weights.z)
            + (joint_transforms.index_unchecked(global_joint_indices.w as usize).rotation * normal * joint_weights.w)
    };

    let position = instance_translation + (instance_rotation * (instance_scale * position));

    *builtin_pos = uniforms.projection_view(view_index) * position.extend(1.0);
    *out_position = position;
    *out_normal = instance_rotation * normal;
    *out_uv = material_settings.transform_uv(uv);
    *out_material_index = material_index;

    if uniforms.settings.contains(Settings::FLIP_VIEWPORT) {
        builtin_pos.y = -builtin_pos.y;
    }
}

struct TextureSampler<'a> {
    sampler: Sampler,
    texture: &'a SampledImage,
    uv: Vec2,
}

impl<'a> TextureSampler<'a> {
    fn new(texture: &'a SampledImage, sampler: Sampler, uv: Vec2) -> Self {
        Self {
            sampler,
            texture,
            uv,
        }
    }

    fn sample(&self) -> Vec4 {
        self.texture.sample(self.sampler, self.uv)
    }
}

struct ExtendedMaterialParams {
    base: glam_pbr::MaterialParams,
    alpha: f32,
    emissive: Vec3,
}

impl ExtendedMaterialParams {
    pub fn new(
        albedo_texture: &TextureSampler,
        metallic_roughness_texture: &TextureSampler,
        emissive_texture: &TextureSampler,
        material_settings: &MaterialSettings,
    ) -> Self {
        let albedo = albedo_texture.sample() * material_settings.base_color_factor;
        let emissive = emissive_texture.sample().truncate() * material_settings.emissive_factor();

        let metallic_roughness = metallic_roughness_texture.sample();
        let metallic = metallic_roughness.z * material_settings.metallic_factor;
        let roughness = metallic_roughness.y * material_settings.roughness_factor;

        Self {
            base: glam_pbr::MaterialParams {
                albedo_colour: albedo.truncate(),
                metallic,
                perceptual_roughness: glam_pbr::PerceptualRoughness(roughness),
                index_of_refraction: glam_pbr::IndexOfRefraction::default(),
                specular_colour: Vec3::ONE,
                specular_factor: 1.0,
            },
            alpha: albedo.w,
            emissive,
        }
    }
}

fn sample_spherical_harmonics(
    uniforms: &Uniforms,
    position: Vec3,
    sampler: Sampler,
    l_0: &Image3D,
    l_1_x: &Image3D,
    l_1_y: &Image3D,
    l_1_z: &Image3D,
) -> [Vec3; 4] {
    let rescaled = uniforms.probes_array().rescale(position);

    let sample_texture = |texture: &Image3D| {
        let sample: Vec4 = texture.sample_by_lod(sampler, rescaled, 0.0);
        sample.truncate()
    };

    [
        sample_texture(l_0),
        sample_texture(l_1_x),
        sample_texture(l_1_y),
        sample_texture(l_1_z),
    ]
}

#[spirv(fragment)]
pub fn fragment(
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
    #[spirv(flat)] _material_index: u32,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 0, binding = 1)] clamp_sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 3)] sh_l_0: &Image3D,
    #[spirv(descriptor_set = 0, binding = 4)] sh_l_1_x: &Image3D,
    #[spirv(descriptor_set = 0, binding = 5)] sh_l_1_y: &Image3D,
    #[spirv(descriptor_set = 0, binding = 6)] sh_l_1_z: &Image3D,
    #[spirv(descriptor_set = 1, binding = 0)] albedo_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 1)] normal_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 2)] metallic_roughness_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 3)] emissive_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 4, uniform)] material_settings: &MaterialSettings,
    #[spirv(descriptor_set = 1, binding = 5)] texture_sampler: &Sampler,
    #[spirv(view_index, flat)] view_index: i32,
    #[spirv(front_facing)] front_facing: bool,
    output: &mut Vec4,
) {
    let spherical_harmonics = sample_spherical_harmonics(
        uniforms,
        position,
        *clamp_sampler,
        sh_l_0,
        sh_l_1_x,
        sh_l_1_y,
        sh_l_1_z,
    );

    let albedo_texture = TextureSampler::new(albedo_texture, *texture_sampler, uv);
    let metallic_roughness_texture =
        TextureSampler::new(metallic_roughness_texture, *texture_sampler, uv);
    let normal_texture = TextureSampler::new(normal_texture, *texture_sampler, uv);
    let emissive_texture = TextureSampler::new(emissive_texture, *texture_sampler, uv);

    let material_params = ExtendedMaterialParams::new(
        &albedo_texture,
        &metallic_roughness_texture,
        &emissive_texture,
        &material_settings,
    );

    if material_settings.is_unlit != 0 {
        // we don't want to use tonemapping for unlit materials.
        *output = potentially_convert_linear_to_srgb(material_params.base.albedo_colour, uniforms)
            .extend(1.0);
        return;
    }

    let view_vector = (uniforms.eye_position(view_index) - position).normalize();

    let normal = calculate_normal(
        normal,
        uv,
        view_vector,
        &normal_texture,
        front_facing,
        material_settings.normal_map_scale,
    );
    let view = glam_pbr::View(view_vector);

    let diffuse_output = material_params.base.diffuse_colour()
        * eval_spherical_harmonics_nonlinear(spherical_harmonics, normal.0);

    let specular_output = spherical_harmonics_specular_approximation(
        spherical_harmonics,
        normal,
        view,
        material_params.base,
    );

    let combined_output = diffuse_output + specular_output + material_params.emissive;

    *output = potentially_tonemap(combined_output, uniforms).extend(1.0);
}

#[spirv(fragment)]
pub fn fragment_alpha_blended(
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
    #[spirv(flat)] _material_index: u32,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 0, binding = 1)] clamp_sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 3)] sh_l_0: &Image3D,
    #[spirv(descriptor_set = 0, binding = 4)] sh_l_1_x: &Image3D,
    #[spirv(descriptor_set = 0, binding = 5)] sh_l_1_y: &Image3D,
    #[spirv(descriptor_set = 0, binding = 6)] sh_l_1_z: &Image3D,
    #[spirv(descriptor_set = 1, binding = 0)] albedo_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 1)] normal_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 2)] metallic_roughness_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 3)] emissive_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 4, uniform)] material_settings: &MaterialSettings,
    #[spirv(descriptor_set = 1, binding = 5)] texture_sampler: &Sampler,
    #[spirv(view_index, flat)] view_index: i32,
    #[spirv(front_facing)] front_facing: bool,
    output: &mut Vec4,
) {
    let spherical_harmonics = sample_spherical_harmonics(
        uniforms,
        position,
        *clamp_sampler,
        sh_l_0,
        sh_l_1_x,
        sh_l_1_y,
        sh_l_1_z,
    );

    let albedo_texture = TextureSampler::new(albedo_texture, *texture_sampler, uv);
    let metallic_roughness_texture =
        TextureSampler::new(metallic_roughness_texture, *texture_sampler, uv);
    let normal_texture = TextureSampler::new(normal_texture, *texture_sampler, uv);
    let emissive_texture = TextureSampler::new(emissive_texture, *texture_sampler, uv);

    let material_params = ExtendedMaterialParams::new(
        &albedo_texture,
        &metallic_roughness_texture,
        &emissive_texture,
        &material_settings,
    );

    if material_settings.is_unlit != 0 {
        // we don't want to use tonemapping for unlit materials.
        *output = potentially_convert_linear_to_srgb(material_params.base.albedo_colour, uniforms)
            .extend(material_params.alpha);
        return;
    }

    let view_vector = (uniforms.eye_position(view_index) - position).normalize();

    let normal = calculate_normal(
        normal,
        uv,
        view_vector,
        &normal_texture,
        front_facing,
        material_settings.normal_map_scale,
    );
    let view = glam_pbr::View(view_vector);

    let diffuse_output = material_params.base.diffuse_colour()
        * eval_spherical_harmonics_nonlinear(spherical_harmonics, normal.0);

    let specular_output = spherical_harmonics_specular_approximation(
        spherical_harmonics,
        normal,
        view,
        material_params.base,
    );
    let combined_output = diffuse_output + specular_output + material_params.emissive;

    *output = potentially_tonemap(combined_output, uniforms).extend(material_params.alpha);
}

#[spirv(fragment)]
pub fn fragment_alpha_clipped(
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
    #[spirv(flat)] _material_index: u32,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 0, binding = 1)] clamp_sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 3)] sh_l_0: &Image3D,
    #[spirv(descriptor_set = 0, binding = 4)] sh_l_1_x: &Image3D,
    #[spirv(descriptor_set = 0, binding = 5)] sh_l_1_y: &Image3D,
    #[spirv(descriptor_set = 0, binding = 6)] sh_l_1_z: &Image3D,
    #[spirv(descriptor_set = 1, binding = 0)] albedo_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 1)] normal_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 2)] metallic_roughness_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 3)] emissive_texture: &SampledImage,
    #[spirv(descriptor_set = 1, binding = 4, uniform)] material_settings: &MaterialSettings,
    #[spirv(descriptor_set = 1, binding = 5)] texture_sampler: &Sampler,
    #[spirv(view_index, flat)] view_index: i32,
    #[spirv(front_facing)] front_facing: bool,
    output: &mut Vec4,
) {
    let spherical_harmonics = sample_spherical_harmonics(
        uniforms,
        position,
        *clamp_sampler,
        sh_l_0,
        sh_l_1_x,
        sh_l_1_y,
        sh_l_1_z,
    );

    let albedo_texture = TextureSampler::new(albedo_texture, *texture_sampler, uv);
    let metallic_roughness_texture =
        TextureSampler::new(metallic_roughness_texture, *texture_sampler, uv);
    let normal_texture = TextureSampler::new(normal_texture, *texture_sampler, uv);
    let emissive_texture = TextureSampler::new(emissive_texture, *texture_sampler, uv);

    let material_params = ExtendedMaterialParams::new(
        &albedo_texture,
        &metallic_roughness_texture,
        &emissive_texture,
        &material_settings,
    );

    let view_vector = (uniforms.eye_position(view_index) - position).normalize();

    let normal = calculate_normal(
        normal,
        uv,
        view_vector,
        &normal_texture,
        front_facing,
        material_settings.normal_map_scale,
    );
    let view = glam_pbr::View(view_vector);

    // We can only do this after we've sampled all textures for naga control flow reasons.
    if material_params.alpha < 0.5 {
        spirv_std::arch::kill();
    }

    if material_settings.is_unlit != 0 {
        // we don't want to use tonemapping for unlit materials.
        *output = potentially_convert_linear_to_srgb(material_params.base.albedo_colour, uniforms)
            .extend(1.0);
        return;
    }

    let diffuse_output = material_params.base.diffuse_colour()
        * eval_spherical_harmonics_nonlinear(spherical_harmonics, normal.0);

    let specular_output = spherical_harmonics_specular_approximation(
        spherical_harmonics,
        normal,
        view,
        material_params.base,
    );
    let combined_output = diffuse_output + specular_output + material_params.emissive;

    *output = potentially_tonemap(combined_output, uniforms).extend(1.0);
}

fn linear_to_srgb_approx(color_linear: Vec3) -> Vec3 {
    let gamma = 2.2;
    color_linear.powf(1.0 / gamma)
}

fn calculate_normal(
    interpolated_normal: Vec3,
    uv: Vec2,
    view_vector: Vec3,
    normal_map: &TextureSampler,
    front_facing: bool,
    normal_map_scale: f32,
) -> glam_pbr::Normal {
    let mut normal = interpolated_normal.normalize();

    if !front_facing {
        normal = -normal;
    }

    let map_normal = normal_map.sample();
    let map_normal = map_normal.truncate();
    let map_normal = map_normal * 255.0 / 127.0 - 128.0 / 127.0;
    // https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#_material_normaltextureinfo_scale
    let map_normal = (map_normal * Vec3::new(normal_map_scale, normal_map_scale, 1.0)).normalize();

    let normal = (compute_cotangent_frame(normal, -view_vector, uv) * map_normal).normalize();

    glam_pbr::Normal(normal)
}

// http://www.thetenthplanet.de/archives/1180
fn compute_cotangent_frame(normal: Vec3, position: Vec3, uv: Vec2) -> Mat3 {
    // get edge vectors of the pixel triangle
    let delta_pos_1 = spirv_std::arch::ddx_vector(position);
    let delta_pos_2 = spirv_std::arch::ddy_vector(position);
    let delta_uv_1 = spirv_std::arch::ddx_vector(uv);
    let delta_uv_2 = spirv_std::arch::ddy_vector(uv);

    // solve the linear system
    let delta_pos_2_perp = delta_pos_2.cross(normal);
    let delta_pos_1_perp = normal.cross(delta_pos_1);
    let t = delta_pos_2_perp * delta_uv_1.x + delta_pos_1_perp * delta_uv_2.x;
    let b = delta_pos_2_perp * delta_uv_1.y + delta_pos_1_perp * delta_uv_2.y;

    // construct a scale-invariant frame
    let invmax = 1.0 / t.length_squared().max(b.length_squared()).sqrt();
    Mat3::from_cols(t * invmax, b * invmax, normal)
}

#[spirv(vertex)]
pub fn fullscreen_tri(
    #[spirv(vertex_index)] vertex_index: i32,
    uv: &mut Vec2,
    #[spirv(position)] builtin_pos: &mut Vec4,
) {
    *uv = Vec2::new(((vertex_index << 1) & 2) as f32, (vertex_index & 2) as f32);
    let pos = 2.0 * *uv - Vec2::ONE;

    *builtin_pos = Vec4::new(pos.x, pos.y, 0.0, 1.0);
}

#[spirv(fragment)]
pub fn blit(
    mut uv: Vec2,
    #[spirv(descriptor_set = 0, binding = 0)] sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 1)] texture: &SampledImage,
    output: &mut Vec4,
) {
    uv.y = 1.0 - uv.y;
    *output = texture.sample_by_lod(*sampler, uv, 0.0);
}

fn saturate(value: Vec3) -> Vec3 {
    value.max(Vec3::ZERO).min(Vec3::ONE)
}

// https://knarkowicz.wordpress.com/2016/01/06/aces-filmic-tone-mapping-curve/
fn aces_filmic(x: Vec3) -> Vec3 {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    saturate((x * (a * x + b)) / (x * (c * x + d) + e))
}

fn potentially_convert_linear_to_srgb(mut output: Vec3, uniforms: &Uniforms) -> Vec3 {
    if uniforms.settings.contains(Settings::INLINE_SRGB) {
        output = linear_to_srgb_approx(output);
    }

    output
}

fn potentially_tonemap(mut output: Vec3, uniforms: &Uniforms) -> Vec3 {
    if uniforms.settings.contains(Settings::INLINE_TONEMAPPING) {
        output = aces_filmic(output);
    }
    output = potentially_convert_linear_to_srgb(output, uniforms);

    output
}

#[spirv(fragment)]
pub fn tonemap(
    uv: Vec2,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 1, binding = 0)] sampler: &Sampler,
    #[spirv(descriptor_set = 1, binding = 1)] texture: &Image!(2D, type=f32, sampled, arrayed),
    output: &mut Vec4,
) {
    let (array_index, uv) = if uv.x > 0.5 {
        (1, Vec2::new(uv.x * 2.0 - 1.0, uv.y))
    } else {
        (0, Vec2::new(uv.x * 2.0, uv.y))
    };

    let sample: Vec4 = texture.sample(*sampler, uv.extend(array_index as f32));

    let mut colour_output = sample.truncate();

    if !uniforms.settings.contains(Settings::INLINE_TONEMAPPING) {
        colour_output = aces_filmic(colour_output);
    }
    if !uniforms.settings.contains(Settings::INLINE_SRGB) {
        colour_output = linear_to_srgb_approx(colour_output);
    }

    *output = colour_output.extend(1.0)
}

#[spirv(vertex)]
pub fn vertex_skybox(
    #[spirv(vertex_index)] vertex_index: i32,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(position)] builtin_pos: &mut Vec4,
    #[spirv(view_index)] view_index: i32,
    ray: &mut Vec3,
) {
    // https://github.com/gfx-rs/wgpu/blob/9114283707a8b472412cf4fe685d364327d3a5b4/wgpu/examples/skybox/shader.wgsl#L21
    let pos = Vec4::new(
        (vertex_index / 2) as f32 * 4.0 - 1.0,
        (vertex_index & 1) as f32 * 4.0 - 1.0,
        // Set to 0 if reverse z, 1 if normal z.
        (1 - uniforms.settings.contains(Settings::REVERSE_Z) as u32) as f32,
        1.0,
    );

    let unprojected: Vec4 = uniforms.projection_inverse(view_index) * pos;

    *ray = glam::Quat::from_vec4(uniforms.view_inverse(view_index)) * unprojected.truncate();

    *builtin_pos = pos;

    if uniforms.settings.contains(Settings::FLIP_VIEWPORT) {
        builtin_pos.y = -builtin_pos.y;
    }
}

#[spirv(fragment)]
pub fn fragment_skybox(
    ray: Vec3,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(descriptor_set = 0, binding = 1)] sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 2)] ibl_cubemap: &Image!(cube, type=f32, sampled),
    output: &mut Vec4,
) {
    let sample: Vec4 = ibl_cubemap.sample_by_lod(*sampler, ray, 0.0);

    *output = potentially_tonemap(sample.truncate(), uniforms).extend(1.0);
}

#[spirv(vertex)]
pub fn line_vertex(
    position: Vec3,
    colour_id: u32,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(position)] builtin_pos: &mut Vec4,
    #[spirv(view_index)] view_index: i32,
    colour: &mut Vec3,
) {
    *builtin_pos = uniforms.projection_view(view_index) * position.extend(1.0);
    *colour = debug_colour_for_id(colour_id);

    if uniforms.settings.contains(Settings::FLIP_VIEWPORT) {
        builtin_pos.y = -builtin_pos.y;
    }
}

#[spirv(fragment)]
pub fn flat_colour(colour: Vec3, output: &mut Vec4) {
    *output = colour.extend(1.0);
}

const DEBUG_COLOURS: [Vec3; 16] = [
    Vec3::new(0.0, 0.0, 0.0),         // black
    Vec3::new(0.0, 0.0, 0.1647),      // darkest blue
    Vec3::new(0.0, 0.0, 0.3647),      // darker blue
    Vec3::new(0.0, 0.0, 0.6647),      // dark blue
    Vec3::new(0.0, 0.0, 0.9647),      // blue
    Vec3::new(0.0, 0.9255, 0.9255),   // cyan
    Vec3::new(0.0, 0.5647, 0.0),      // dark green
    Vec3::new(0.0, 0.7843, 0.0),      // green
    Vec3::new(1.0, 1.0, 0.0),         // yellow
    Vec3::new(0.90588, 0.75294, 0.0), // yellow-orange
    Vec3::new(1.0, 0.5647, 0.0),      // orange
    Vec3::new(1.0, 0.0, 0.0),         // bright red
    Vec3::new(0.8392, 0.0, 0.0),      // red
    Vec3::new(1.0, 0.0, 1.0),         // magenta
    Vec3::new(0.6, 0.3333, 0.7882),   // purple
    Vec3::new(1.0, 1.0, 1.0),         // white
];

fn debug_colour_for_id(id: u32) -> Vec3 {
    unsafe { *DEBUG_COLOURS.index_unchecked(id as usize % DEBUG_COLOURS.len()) }
}

#[spirv(vertex)]
pub fn depth_prepass_vertex(
    position: Vec3,
    instance_translation_and_scale: Vec4,
    instance_rotation: glam::Quat,
    #[spirv(descriptor_set = 0, binding = 0, uniform)] uniforms: &Uniforms,
    #[spirv(position)] builtin_pos: &mut Vec4,
    #[spirv(view_index)] view_index: i32,
) {
    let instance_scale = instance_translation_and_scale.w;
    let instance_translation = instance_translation_and_scale.truncate();

    let position = instance_translation + (instance_rotation * (instance_scale * position));
    *builtin_pos = uniforms.projection_view(view_index) * position.extend(1.0);

    if uniforms.settings.contains(Settings::FLIP_VIEWPORT) {
        builtin_pos.y = -builtin_pos.y;
    }
}

fn spherical_harmonics_specular_approximation(
    spherical_harmonics: [Vec3; 4],
    normal: glam_pbr::Normal,
    view: glam_pbr::View,
    material_params: glam_pbr::MaterialParams,
) -> Vec3 {
    let (red, green, blue) =
        shared_structs::spherical_harmonics_channel_vectors(spherical_harmonics);
    let average_light_direction = (red + green + blue) / 3.0;
    let directional_length = average_light_direction.length();

    let smoothness = 1.0 - material_params.perceptual_roughness.0;
    let adjusted_smoothness = smoothness * directional_length.sqrt();
    let adjusted_roughness = glam_pbr::PerceptualRoughness(1.0 - adjusted_smoothness);

    let actual_roughness = adjusted_roughness.as_actual_roughness();

    let light = glam_pbr::Light(average_light_direction / directional_length);
    let halfway = glam_pbr::Halfway::new(&view, &light);
    let view_dot_halfway = glam_pbr::Dot::new(&view, &halfway);

    let strength = spherical_harmonics[0] * directional_length;

    let f0 = glam_pbr::calculate_combined_f0(material_params);
    let f90 = glam_pbr::calculate_combined_f90(material_params);

    let fresnel = glam_pbr::fresnel_schlick(view_dot_halfway, f0, f90);

    let normal_dot_light = glam_pbr::Dot::new(&normal, &light);

    glam_pbr::specular_brdf(
        glam_pbr::Dot::new(&normal, &view),
        normal_dot_light,
        glam_pbr::Dot::new(&normal, &halfway),
        actual_roughness,
        fresnel,
    ) * strength
        * normal_dot_light.value
}
