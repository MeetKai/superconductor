use crate::{DepthFirstNodes, Similarity};
use glam::{Mat4, Quat, Vec3};
use goth_gltf::{Interpolation, TargetPath};
use std::fmt;
use std::ops::{Add, Mul};

pub fn read_animations<'a, F1, I1, F3, I3, F4, I4>(
    gltf: &'a goth_gltf::Gltf,
    read_f32: F1,
    read_f32x3: F3,
    read_f32x4: F4,
) -> Vec<Animation>
where
    I1: Iterator<Item = f32>,
    F1: Fn(&goth_gltf::Accessor) -> I1,
    I3: Iterator<Item = [f32; 3]>,
    F3: Fn(&goth_gltf::Accessor) -> I3,
    I4: Iterator<Item = [f32; 4]>,
    F4: Fn(&goth_gltf::Accessor) -> I4,
{
    gltf.animations
        .iter()
        .map(|animation| {
            let mut translation_channels: Vec<Channel<Vec3>> = Vec::new();
            let mut rotation_channels: Vec<Channel<Quat>> = Vec::new();
            let mut scale_channels: Vec<Channel<f32>> = Vec::new();

            for channel in &animation.channels {
                let sampler = &animation.samplers[channel.sampler];

                let input_accessor = &gltf.accessors[sampler.input];

                let output_accessor = &gltf.accessors[sampler.output];

                let inputs = read_f32(input_accessor).collect();

                match channel.target.path {
                    TargetPath::Translation => {
                        translation_channels.push(Channel {
                            interpolation: sampler.interpolation,
                            inputs,
                            node_index: channel.target.node.unwrap(),
                            outputs: read_f32x3(output_accessor).map(Vec3::from).collect(),
                        });
                    }
                    TargetPath::Rotation => {
                        rotation_channels.push(Channel {
                            interpolation: sampler.interpolation,
                            inputs,
                            node_index: channel.target.node.unwrap(),
                            outputs: {
                                read_f32x4(output_accessor).map(Quat::from_array).collect()
                            },
                        });
                    }
                    TargetPath::Scale => {
                        scale_channels.push(Channel {
                            interpolation: sampler.interpolation,
                            inputs,
                            node_index: channel.target.node.unwrap(),
                            outputs: read_f32x3(output_accessor)
                                .map(|scale| scale[0].max(scale[1]).max(scale[2]))
                                .collect(),
                        });
                    }
                    TargetPath::Weights => {
                        log::warn!("Weight animations are not supported, ignoring.");
                    }
                }
            }

            let total_time = translation_channels
                .iter()
                .map(|channel| channel.inputs[channel.inputs.len() - 1])
                .chain(
                    rotation_channels
                        .iter()
                        .map(|channel| channel.inputs[channel.inputs.len() - 1]),
                )
                .chain(
                    scale_channels
                        .iter()
                        .map(|channel| channel.inputs[channel.inputs.len() - 1]),
                )
                .max_by_key(|&time| ordered_float::OrderedFloat(time))
                .unwrap();

            Animation {
                total_time,
                translation_channels,
                rotation_channels,
                scale_channels,
            }
        })
        .collect()
}

#[derive(Clone, Debug)]
pub struct AnimationJoints {
    global_transforms: Vec<Similarity>,
    local_transforms: Vec<Similarity>,
}

impl AnimationJoints {
    pub fn new(gltf: &goth_gltf::Gltf, depth_first_nodes: &DepthFirstNodes) -> Self {
        let node_transforms: Vec<_> = gltf
            .nodes
            .iter()
            .map(|node| match node.transform() {
                goth_gltf::NodeTransform::Matrix(matrix) => {
                    Similarity::new_from_mat4(Mat4::from_cols_array(&matrix))
                }
                goth_gltf::NodeTransform::Set {
                    translation,
                    rotation,
                    scale,
                } => Similarity::new_from_gltf(translation, rotation, scale),
            })
            .collect();

        let mut joints = Self {
            global_transforms: node_transforms.clone(),
            local_transforms: node_transforms,
        };

        joints.update(depth_first_nodes);

        joints
    }

    pub fn iter<'a>(
        &'a mut self,
        joint_indices_to_node_indices: &'a [usize],
        inverse_bind_transforms: &'a [Similarity],
        depth_first_nodes: &DepthFirstNodes,
    ) -> impl Iterator<Item = Similarity> + 'a {
        self.update(depth_first_nodes);

        joint_indices_to_node_indices
            .iter()
            .enumerate()
            .map(move |(joint_index, &node_index)| {
                self.global_transforms[node_index] * inverse_bind_transforms[joint_index]
            })
    }

    pub fn update(&mut self, depth_first_nodes: &DepthFirstNodes) {
        for &index in &depth_first_nodes.roots {
            self.global_transforms[index] = self.local_transforms[index];
        }

        for child in &depth_first_nodes.children {
            let parent_transform = self.global_transforms[child.parent];
            self.global_transforms[child.index] =
                parent_transform * self.local_transforms[child.index];
        }
    }

    pub fn iter_lines<'a>(
        &'a self,
        depth_first_nodes: &'a DepthFirstNodes,
    ) -> impl Iterator<Item = (Vec3, Vec3)> + 'a {
        depth_first_nodes.children.iter().map(|child| {
            (
                self.global_transforms[child.parent].translation,
                self.global_transforms[child.index].translation,
            )
        })
    }

    pub fn get_joint_mut(
        &mut self,
        index: usize,
        joint_indices_to_node_indices: &[usize],
    ) -> &mut Similarity {
        &mut self.local_transforms[joint_indices_to_node_indices[index]]
    }
}

struct Channel<T> {
    interpolation: Interpolation,
    inputs: Vec<f32>,
    outputs: Vec<T>,
    node_index: usize,
}

impl<T> fmt::Debug for Channel<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Channel")
            .field("interpolation", &self.interpolation)
            .field("num_values", &self.inputs.len())
            .field("node_index", &self.node_index)
            .finish()
    }
}

impl<T: Interpolate> Channel<T> {
    fn sample(&self, t: f32) -> Option<(usize, T)> {
        if t < self.inputs[0] || t > self.inputs[self.inputs.len() - 1] {
            return None;
        }

        let index = self
            .inputs
            .binary_search_by_key(&ordered_float::OrderedFloat(t), |t| {
                ordered_float::OrderedFloat(*t)
            });
        let i = match index {
            Ok(exact) => exact,
            Err(would_be_inserted_at) => would_be_inserted_at - 1,
        };

        let previous_time = self.inputs[i];
        let next_time = self.inputs.get(i + 1)?;
        let delta = next_time - previous_time;
        let from_start = t - previous_time;
        let factor = from_start / delta;

        let value = match self.interpolation {
            Interpolation::Step => self.outputs[i],
            Interpolation::Linear => {
                let previous_value = self.outputs[i];
                let next_value = self.outputs[i + 1];

                previous_value.linear(next_value, factor)
            }
            Interpolation::CubicSpline => {
                // See the bottom of:
                // https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#animations
                //
                // The keyframes are grouped in sets of 3, arranged as:
                // * An in-tangent
                // * A value
                // * An out-tangent
                //
                // We don't care about the in-tangent for the starting point, or the out-tangent for
                // the ending point so we don't load those.

                let starting_point = self.outputs[i * 3 + 1];
                let starting_out_tangent = self.outputs[i * 3 + 2];

                let ending_in_tangent = self.outputs[i * 3 + 3];
                let ending_point = self.outputs[i * 3 + 4];

                Interpolate::cubic_spline(
                    starting_point,
                    starting_out_tangent,
                    ending_point,
                    ending_in_tangent,
                    delta,
                    factor,
                )
            }
        };

        Some((self.node_index, value))
    }
}

#[derive(Debug)]
pub struct Animation {
    total_time: f32,
    translation_channels: Vec<Channel<Vec3>>,
    rotation_channels: Vec<Channel<Quat>>,
    scale_channels: Vec<Channel<f32>>,
}

impl Animation {
    pub fn total_time(&self) -> f32 {
        self.total_time
    }

    pub fn animate(&self, animation_joints: &mut AnimationJoints, time: f32) {
        self.translation_channels
            .iter()
            .filter_map(move |channel| channel.sample(time))
            .for_each(|(node_index, translation)| {
                animation_joints.local_transforms[node_index].translation = translation;
            });

        self.rotation_channels
            .iter()
            .filter_map(move |channel| channel.sample(time))
            .for_each(|(node_index, rotation)| {
                animation_joints.local_transforms[node_index].rotation = rotation;
            });

        self.scale_channels
            .iter()
            .filter_map(move |channel| channel.sample(time))
            .for_each(|(node_index, scale)| {
                animation_joints.local_transforms[node_index].scale = scale;
            });
    }
}

trait Interpolate: Copy {
    fn linear(self, other: Self, t: f32) -> Self;

    fn cubic_spline(
        starting_point: Self,
        starting_out_tangent: Self,
        ending_in_point: Self,
        ending_out_tangent: Self,
        time_between_keyframes: f32,
        t: f32,
    ) -> Self;
}

impl Interpolate for Vec3 {
    fn linear(self, other: Self, t: f32) -> Self {
        self.lerp(other, t)
    }

    fn cubic_spline(
        starting_point: Self,
        starting_out_tangent: Self,
        ending_in_point: Self,
        ending_out_tangent: Self,
        time_between_keyframes: f32,
        t: f32,
    ) -> Self {
        cubic_spline_interpolate(
            starting_point,
            starting_out_tangent,
            ending_in_point,
            ending_out_tangent,
            time_between_keyframes,
            t,
        )
    }
}

impl Interpolate for Quat {
    fn linear(self, other: Self, t: f32) -> Self {
        self.slerp(other, t)
    }

    fn cubic_spline(
        starting_point: Self,
        starting_out_tangent: Self,
        ending_in_point: Self,
        ending_out_tangent: Self,
        time_between_keyframes: f32,
        t: f32,
    ) -> Self {
        cubic_spline_interpolate(
            starting_point,
            starting_out_tangent,
            ending_in_point,
            ending_out_tangent,
            time_between_keyframes,
            t,
        )
        .normalize()
    }
}

impl Interpolate for f32 {
    fn linear(self, other: Self, t: f32) -> Self {
        self * (1.0 - t) + other * t
    }

    fn cubic_spline(
        starting_point: Self,
        starting_out_tangent: Self,
        ending_in_point: Self,
        ending_out_tangent: Self,
        time_between_keyframes: f32,
        t: f32,
    ) -> Self {
        cubic_spline_interpolate(
            starting_point,
            starting_out_tangent,
            ending_in_point,
            ending_out_tangent,
            time_between_keyframes,
            t,
        )
    }
}

// For a full explanation see:
// https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#appendix-c-spline-interpolation
fn cubic_spline_interpolate<T>(
    starting_point: T,
    starting_out_tangent: T,
    ending_point: T,
    ending_in_tangent: T,
    time_between_keyframes: f32,
    t: f32,
) -> T
where
    T: Add<T, Output = T> + Mul<f32, Output = T> + Copy,
{
    let p0 = starting_point;
    let m0 = starting_out_tangent * time_between_keyframes;
    let p1 = ending_point;
    let m1 = ending_in_tangent * time_between_keyframes;

    let t2 = t * t;
    let t3 = t * t * t;

    p0 * (2.0 * t3 - 3.0 * t2 + 1.0)
        + m0 * (t3 - 2.0 * t2 + t)
        + p1 * (-2.0 * t3 + 3.0 * t2)
        + m1 * (t3 - t2)
}
