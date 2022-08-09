pub mod animation;
use glam::{Mat3, Mat4, Quat, Vec3};
use std::ops::Mul;

#[derive(Clone, Copy, Debug, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct Similarity {
    pub translation: Vec3,
    pub scale: f32,
    pub rotation: Quat,
}

impl Similarity {
    pub const IDENTITY: Self = Self {
        translation: Vec3::ZERO,
        scale: 1.0,
        rotation: Quat::IDENTITY,
    };

    pub fn as_mat4(self) -> Mat4 {
        Mat4::from_translation(self.translation)
            * Mat4::from_mat3(Mat3::from_quat(self.rotation))
            * Mat4::from_scale(Vec3::splat(self.scale))
    }

    pub fn inverse(&self) -> Self {
        Self {
            rotation: self.rotation.inverse(),
            translation: self.rotation.inverse() * (-self.translation),
            scale: 1.0 / self.scale,
        }
    }

    pub fn new_from_gltf(translation: [f32; 3], rotation: [f32; 4], scale: [f32; 3]) -> Self {
        let scale_x_y_approx_eq = (scale[0] - scale[1]).abs() <= std::f32::EPSILON * 10.0;
        let scale_x_z_approx_eq = (scale[0] - scale[2]).abs() <= std::f32::EPSILON * 10.0;

        let max_scale = scale[0].max(scale[1]).max(scale[2]);

        if !(scale_x_y_approx_eq && scale_x_z_approx_eq) {
            log::warn!("Node scales are not uniform: {:?}. Using the largest scale of {:?}. This may result in the model looking odd.", scale, max_scale);
        }

        Similarity {
            translation: translation.into(),
            rotation: Quat::from_array(rotation),
            scale: max_scale,
        }
    }

    pub fn new_from_gltf_node(node: &goth_gltf::Node) -> Self {
        match node.transform() {
            goth_gltf::NodeTransform::Matrix(_) => todo!(),
            goth_gltf::NodeTransform::Set {
                translation,
                rotation,
                scale,
            } => Self::new_from_gltf(translation, rotation, scale),
        }
    }
}

impl Mul<Similarity> for Similarity {
    type Output = Self;

    fn mul(self, child: Self) -> Self {
        Self {
            translation: self * child.translation,
            rotation: self.rotation * child.rotation,
            scale: self.scale * child.scale,
        }
    }
}

impl Mul<Vec3> for Similarity {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Vec3 {
        self.translation + (self.scale * (self.rotation * vector))
    }
}

pub struct NodeTree {
    inner: Vec<(Similarity, usize)>,
}

impl NodeTree {
    pub fn new(gltf: &goth_gltf::Gltf) -> Self {
        let mut inner = vec![(Similarity::IDENTITY, usize::max_value()); gltf.nodes.len()];

        for (index, node) in gltf.nodes.iter().enumerate() {
            inner[index].0 = Similarity::new_from_gltf_node(node);
            for child in &node.children {
                inner[*child].1 = index;
            }
        }

        Self { inner }
    }

    pub fn transform_of(&self, mut index: usize) -> Similarity {
        let mut transform_sum = Similarity::IDENTITY;

        while index != usize::max_value() {
            let (transform, parent_index) = self.inner[index];
            transform_sum = transform * transform_sum;
            index = parent_index;
        }

        transform_sum
    }

    // It turns out that we can just reverse the array to iter through nodes depth first! Useful for applying animations.
    pub fn iter_depth_first(&self) -> impl Iterator<Item = (usize, Option<usize>)> + '_ {
        self.inner
            .iter()
            .enumerate()
            .rev()
            .map(|(index, &(_, parent))| {
                (
                    index,
                    if parent != usize::max_value() {
                        Some(parent)
                    } else {
                        None
                    },
                )
            })
    }
}

#[derive(Debug)]
pub struct DepthFirstNodes {
    roots: Vec<usize>,
    children: Vec<Child>,
}

impl DepthFirstNodes {
    pub fn new(gltf: &goth_gltf::Gltf, node_tree: &NodeTree) -> Self {
        let roots: Vec<_> = node_tree
            .inner
            .iter()
            .enumerate()
            .filter_map(|(index, (_, parent))| {
                if *parent == usize::max_value() {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        let mut children = Vec::new();
        let mut stack = roots.clone();

        while let Some(parent) = stack.pop() {
            for child in &gltf.nodes[parent].children {
                children.push(Child {
                    index: *child,
                    parent,
                });

                stack.push(*child);
            }
        }

        Self { roots, children }
    }
}

#[derive(Debug)]
struct Child {
    index: usize,
    parent: usize,
}
