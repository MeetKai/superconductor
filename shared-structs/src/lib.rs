#![cfg_attr(not(feature = "std"), no_std)]

use core::ops::{BitOr, BitOrAssign, Mul};
use glam::{Mat2, Mat4, Vec2, Vec3, Vec4};
#[cfg(target_arch = "spirv")]
use num_traits::Float;

#[derive(Clone, Copy)]
#[cfg_attr(
    not(target_arch = "spirv"),
    derive(Debug, bytemuck::Zeroable, bytemuck::Pod)
)]
#[repr(C)]
pub struct Uniforms {
    pub left_projection_view: FlatMat4,
    pub right_projection_view: FlatMat4,
    // For the skybox
    pub left_projection_inverse: FlatMat4,
    pub right_projection_inverse: FlatMat4,
    pub left_view_inverse: Vec4,
    pub right_view_inverse: Vec4,
    // Decomposed vectors
    pub left_eye_x: f32,
    pub left_eye_y: f32,
    pub left_eye_z: f32,
    pub right_eye_x: f32,
    pub right_eye_y: f32,
    pub right_eye_z: f32,
    pub settings: Settings,
    pub probes_array_bottom_left_x: f32,
    pub probes_array_bottom_left_y: f32,
    pub probes_array_bottom_left_z: f32,
    pub probes_array_scale_x: f32,
    pub probes_array_scale_y: f32,
    pub probes_array_scale_z: f32,
    // As the struct is 16-byte aligned due to the Vec4s in the FlatMat4s,
    // we need to pad it to 16 bytes by adding a few more bytes.
    #[cfg(not(target_arch = "spirv"))]
    pub _padding: [u32; 3],
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

    pub fn probes_array(&self) -> ProbesArray {
        ProbesArray {
            bottom_left: Vec3::new(
                self.probes_array_bottom_left_x,
                self.probes_array_bottom_left_y,
                self.probes_array_bottom_left_z,
            ),
            scale: Vec3::new(
                self.probes_array_scale_x,
                self.probes_array_scale_y,
                self.probes_array_scale_z,
            ),
        }
    }
}

pub struct ProbesArray {
    bottom_left: Vec3,
    scale: Vec3,
}

impl ProbesArray {
    pub fn rescale(&self, position: Vec3) -> Vec3 {
        (position - self.bottom_left) / self.scale
    }
}

#[derive(Clone, Copy, Default)]
#[cfg_attr(
    not(target_arch = "spirv"),
    derive(Debug, bytemuck::Zeroable, bytemuck::Pod)
)]
#[repr(transparent)]
pub struct Settings {
    bits: u32,
}

impl Settings {
    pub const FLIP_VIEWPORT: Self = Self { bits: 1 << 0 };
    pub const INLINE_TONEMAPPING: Self = Self { bits: 1 << 1 };
    pub const INLINE_SRGB: Self = Self { bits: 1 << 2 };
    pub const REVERSE_Z: Self = Self { bits: 1 << 3 };

    pub fn contains(self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }
}

impl BitOr for Settings {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits,
        }
    }
}

impl BitOrAssign for Settings {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
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
        Vec3::new(
            self.emissive_factor_x,
            self.emissive_factor_y,
            self.emissive_factor_z,
        )
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

pub type L1SphericalHarmonics = [Vec3; 4];

pub fn spherical_harmonics_channel_vectors(harmonics: L1SphericalHarmonics) -> (Vec3, Vec3, Vec3) {
    (
        Vec3::new(harmonics[1].x, harmonics[2].x, harmonics[3].x),
        Vec3::new(harmonics[1].y, harmonics[2].y, harmonics[3].y),
        Vec3::new(harmonics[1].z, harmonics[2].z, harmonics[3].z),
    )
}

// See https://grahamhazel.com/blog/2017/12/22/converting-sh-radiance-to-irradiance/
// https://web.archive.org/web/20160313132301/http://www.geomerics.com/wp-content/uploads/2015/08/CEDEC_Geomerics_ReconstructingDiffuseLighting1.pdf
// https://media.contentapi.ea.com/content/dam/eacom/frostbite/files/gdc2018-precomputedgiobalilluminationinfrostbite.pdf
pub fn eval_spherical_harmonics_nonlinear(harmonics: L1SphericalHarmonics, normal: Vec3) -> Vec3 {
    fn eval_scalar(r_0: f32, r_1_div_r_0: Vec3, normal: Vec3) -> f32 {
        let r1_r0_ratio = r_1_div_r_0.length();

        let a = (1.0 - r1_r0_ratio) / (1.0 + r1_r0_ratio);
        let p = 1.0 + 2.0 * r1_r0_ratio;
        let q = 0.5 * (1.0 + r_1_div_r_0.dot(normal));

        r_0 * (a + (1.0 - a) * (p + 1.0) * q.powf(p))
    }

    let (red_vector, green_vector, blue_vector) = spherical_harmonics_channel_vectors(harmonics);

    Vec3::new(
        eval_scalar(harmonics[0].x, red_vector, normal),
        eval_scalar(harmonics[0].y, green_vector, normal),
        eval_scalar(harmonics[0].z, blue_vector, normal),
    )
}
