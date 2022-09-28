#![cfg_attr(not(feature = "std"), no_std)]

use core::ops::Mul;
use glam::{Mat2, Mat4, Vec2, Vec3, Vec4};

#[derive(Clone, Copy)]
#[cfg_attr(
    not(target_arch = "spirv"),
    derive(Debug, bytemuck::Zeroable, bytemuck::Pod)
)]
#[repr(C)]
pub struct Uniforms {
    pub left_projection_view: FlatMat4,
    pub right_projection_view: FlatMat4,
    pub left_eye_x: f32,
    pub left_eye_y: f32,
    pub left_eye_z: f32,
    pub right_eye_x: f32,
    pub right_eye_y: f32,
    pub right_eye_z: f32,
    pub flip_viewport: u32,
    pub inline_tonemapping: u32,
    pub inline_srgb: u32,
    pub reverse_z: u32,
    // As the struct is 16-byte aligned due to the Vec4s in the FlatMat4s,
    // we need to pad it to 16 bytes by adding 8 more bytes.
    #[cfg(not(target_arch = "spirv"))]
    pub _padding: [u32; 2],
}

impl Uniforms {
    pub fn projection_view(&self, view_index: i32) -> Mat4 {
        Mat4::from(if view_index != 0 {
            self.right_projection_view
        } else {
            self.left_projection_view
        })
    }

    pub fn eye_position(&self, view_index: i32) -> Vec3 {
        if view_index != 0 {
            Vec3::new(self.right_eye_x, self.right_eye_y, self.right_eye_z)
        } else {
            Vec3::new(self.left_eye_x, self.left_eye_y, self.left_eye_z)
        }
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(
    not(target_arch = "spirv"),
    derive(Debug, bytemuck::Zeroable, bytemuck::Pod)
)]
#[repr(C)]
pub struct SkyboxUniforms {
    pub left_projection_inverse: FlatMat4,
    pub right_projection_inverse: FlatMat4,
    pub left_view_inverse: Vec4,
    pub right_view_inverse: Vec4,
}

impl SkyboxUniforms {
    pub fn projection_inverse(&self, view_index: i32) -> Mat4 {
        Mat4::from(if view_index != 0 {
            self.right_projection_inverse
        } else {
            self.left_projection_inverse
        })
    }

    pub fn view_inverse(&self, view_index: i32) -> Vec4 {
        if view_index != 0 {
            self.right_view_inverse
        } else {
            self.left_view_inverse
        }
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(
    not(target_arch = "spirv"),
    derive(Debug, bytemuck::Zeroable, bytemuck::Pod)
)]
#[repr(C)]
pub struct FlatMat4 {
    col_0: Vec4,
    col_1: Vec4,
    col_2: Vec4,
    col_3: Vec4,
}

impl From<FlatMat4> for Mat4 {
    fn from(d: FlatMat4) -> Self {
        Self::from_cols(d.col_0, d.col_1, d.col_2, d.col_3)
    }
}

impl From<Mat4> for FlatMat4 {
    fn from(mat: Mat4) -> Self {
        Self {
            col_0: mat.col(0),
            col_1: mat.col(1),
            col_2: mat.col(2),
            col_3: mat.col(3),
        }
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(
    not(target_arch = "spirv"),
    derive(Debug, bytemuck::Zeroable, bytemuck::Pod)
)]
#[repr(C)]
pub struct MaterialSettings {
    pub base_color_factor: Vec4,
    pub texture_transform_offset: Vec2,
    pub texture_transform_scale: Vec2,
    pub emissive_factor_x: f32,
    pub emissive_factor_y: f32,
    pub emissive_factor_z: f32,
    pub texture_transform_rotation: f32,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub normal_map_scale: f32,
    pub is_unlit: u32,
}

impl MaterialSettings {
    pub fn transform_uv(self, uv: Vec2) -> Vec2 {
        self.texture_transform_offset
            + (Mat2::from_angle(self.texture_transform_rotation)
                * self.texture_transform_scale
                * uv)
    }

    pub fn emissive_factor(self) -> Vec3 {
        Vec3::new(self.emissive_factor_x, self.emissive_factor_y, self.emissive_factor_z)
    }

    pub fn default_unlit() -> Self {
        Self {
            base_color_factor: Vec4::ONE,
            emissive_factor_x: 0.0,
            emissive_factor_y: 0.0,
            emissive_factor_z: 0.0,
            metallic_factor: 0.0,
            roughness_factor: 1.0,
            normal_map_scale: 1.0,
            is_unlit: true as u32,
            texture_transform_offset: Vec2::ZERO,
            texture_transform_scale: Vec2::ONE,
            texture_transform_rotation: 0.0,
        }
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(
    not(target_arch = "spirv"),
    derive(Debug, bytemuck::Zeroable, bytemuck::Pod)
)]
#[repr(C)]
pub struct JointTransform {
    pub translation_and_scale: Vec4,
    pub rotation: glam::Quat,
}

impl JointTransform {
    pub const MAX_COUNT: usize = 65536 / core::mem::size_of::<Self>();

    pub fn new(translation: Vec3, scale: f32, rotation: glam::Quat) -> Self {
        Self {
            translation_and_scale: translation.extend(scale),
            rotation,
        }
    }

    fn translation(&self) -> Vec3 {
        self.translation_and_scale.truncate()
    }

    fn scale(&self) -> f32 {
        self.translation_and_scale.w
    }
}

impl Mul<Vec3> for JointTransform {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Vec3 {
        self.translation() + (self.scale() * (self.rotation * vector))
    }
}
