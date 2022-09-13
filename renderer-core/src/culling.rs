use glam::{vec3, Mat4, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    #[inline]
    pub fn new(points: &[Vec3]) -> Self {
        let mut max = Vec3::splat(f32::MIN);
        let mut min = Vec3::splat(f32::MAX);

        for &point in points {
            max = max.max(point);
            min = min.min(point);
        }

        Self { max, min }
    }


    #[rustfmt::skip]
    pub fn line_points(&self) -> [Vec3; 24] {
        let min = self.min;
        let max = self.max;
        use glam::vec3;

        [
            // One side
            vec3(min.x, min.y, min.z), vec3(min.x, min.y, max.z),
            vec3(min.x, min.y, max.z), vec3(min.x, max.y, max.z),
            vec3(min.x, max.y, max.z), vec3(min.x, max.y, min.z),
            vec3(min.x, max.y, min.z), vec3(min.x, min.y, min.z),
            // The other side
            vec3(max.x, min.y, min.z), vec3(max.x, min.y, max.z),
            vec3(max.x, min.y, max.z), vec3(max.x, max.y, max.z),
            vec3(max.x, max.y, max.z), vec3(max.x, max.y, min.z),
            vec3(max.x, max.y, min.z), vec3(max.x, min.y, min.z),
            // Bridging lines between the sides.
            vec3(min.x, min.y, min.z), vec3(max.x, min.y, min.z),
            vec3(min.x, min.y, max.z), vec3(max.x, min.y, max.z),
            vec3(min.x, max.y, min.z), vec3(max.x, max.y, min.z),
            vec3(min.x, max.y, max.z), vec3(max.x, max.y, max.z),
        ]
    }
}

#[derive(Default)]
pub struct CullingFrustum {
    near_right: f32,
    near_top: f32,
    near_plane: f32,
    far_plane: f32,
}

impl CullingFrustum {
    pub fn new(vertical_fov: f32, aspect_ratio: f32, near_plane: f32, far_plane: f32) -> Self {
        let tan_fov = (0.5 * vertical_fov).tan();

        Self {
            near_right: aspect_ratio * near_plane * tan_fov,
            near_top: near_plane * tan_fov,
            near_plane: -near_plane,
            far_plane: -far_plane,
        }
    }
}

// Modified from https://bruop.github.io/improved_frustum_culling/.
// todo:
//
// This is a crazy complicated implementation but has basically zero false positives.
// When/if we move culling to the GPU we should use something simpler.
#[inline]
pub fn test_using_separating_axis_theorem(
    frustum: &CullingFrustum,
    vs_transform: Mat4,
    similarity: gltf_helpers::Similarity,
    aabb: &BoundingBox,
) -> bool {
    // Near, far
    let z_near = frustum.near_plane;
    let z_far = frustum.far_plane;
    // half width, half height
    let x_near = frustum.near_right;
    let y_near = frustum.near_top;

    // So first thing we need to do is obtain the normal directions of our OBB by transforming 4 of our AABB vertices
    let corners = [
        vec3(aabb.min.x, aabb.min.y, aabb.min.z),
        vec3(aabb.max.x, aabb.min.y, aabb.min.z),
        vec3(aabb.min.x, aabb.max.y, aabb.min.z),
        vec3(aabb.min.x, aabb.min.y, aabb.max.z),
    ];

    // Transform corners
    // This only translates to our OBB if our transform is affine
    let corners =
        corners.map(|corner| (vs_transform * (similarity * corner).extend(1.0)).truncate());

    struct OrientatedBoundingBox {
        center: Vec3,
        extents: Vec3,
        axes: [Vec3; 3],
    }

    let mut obb = OrientatedBoundingBox {
        axes: [
            corners[1] - corners[0],
            corners[2] - corners[0],
            corners[3] - corners[0],
        ],
        center: Default::default(),
        extents: Default::default(),
    };

    obb.center = corners[0] + 0.5 * (obb.axes[0] + obb.axes[1] + obb.axes[2]);
    obb.extents = vec3(
        (obb.axes[0]).length(),
        (obb.axes[1]).length(),
        (obb.axes[2]).length(),
    );
    obb.axes[0] /= obb.extents.x;
    obb.axes[1] /= obb.extents.y;
    obb.axes[2] /= obb.extents.z;
    obb.extents *= 0.5;

    {
        // Projected center of our OBB
        let m_dot_c = obb.center.z;
        // Projected size of OBB
        let mut radius = 0.0;
        for i in 0..3 {
            // dot(m, axes[i]) == axes[i].z;
            radius += obb.axes[i].z.abs() * obb.extents[i];
        }
        let obb_min = m_dot_c - radius;
        let obb_max = m_dot_c + radius;

        let tau_0 = z_far; // Since z is negative, far is smaller than near
        let tau_1 = z_near;

        if obb_min > tau_1 || obb_max < tau_0 {
            return false;
        }
    }

    {
        let planes = [
            vec3(z_near, 0.0, x_near),  // Left Plane
            vec3(-z_near, 0.0, x_near), // Right plane
            vec3(0.0, -z_near, y_near), // Top plane
            vec3(0.0, z_near, y_near),  // Bottom plane
        ];
        for m in planes {
            let m_dot_x = m.x.abs();
            let m_dot_y = m.y.abs();
            let m_dot_z = m.z;
            let m_dot_c = m.dot(obb.center);

            let mut obb_radius = 0.0;
            for i in 0..3 {
                obb_radius += m.dot(obb.axes[i]).abs() * obb.extents[i];
            }
            let obb_min = m_dot_c - obb_radius;
            let obb_max = m_dot_c + obb_radius;

            let p = x_near * m_dot_x + y_near * m_dot_y;

            let mut tau_0 = z_near * m_dot_z - p;
            let mut tau_1 = z_near * m_dot_z + p;

            if tau_0 < 0.0 {
                tau_0 *= z_far / z_near;
            }
            if tau_1 > 0.0 {
                tau_1 *= z_far / z_near;
            }

            if obb_min > tau_1 || obb_max < tau_0 {
                return false;
            }
        }
    }

    // OBB Axes
    {
        for (i, &m) in obb.axes.iter().enumerate() {
            let m_dot_x = m.x.abs();
            let m_dot_y = m.y.abs();
            let m_dot_z = m.z;
            let m_dot_c = m.dot(obb.center);

            let obb_radius = obb.extents[i];

            let obb_min = m_dot_c - obb_radius;
            let obb_max = m_dot_c + obb_radius;

            // Frustum projection
            let p = x_near * m_dot_x + y_near * m_dot_y;
            let mut tau_0 = z_near * m_dot_z - p;
            let mut tau_1 = z_near * m_dot_z + p;
            if tau_0 < 0.0 {
                tau_0 *= z_far / z_near;
            }
            if tau_1 > 0.0 {
                tau_1 *= z_far / z_near;
            }

            if obb_min > tau_1 || obb_max < tau_0 {
                return false;
            }
        }
    }

    // Now let's perform each of the cross products between the edges
    // First R x A_i
    {
        for m in obb.axes {
            let m = vec3(0.0, -m.z, m.y);
            let m_dot_x = 0.0;
            let m_dot_y = m.y.abs();
            let m_dot_z = m.z;
            let m_dot_c = m.y * obb.center.y + m.z * obb.center.z;

            let mut obb_radius = 0.0;
            for i in 0..3 {
                obb_radius += m.dot(obb.axes[i]).abs() * obb.extents[i];
            }

            let obb_min = m_dot_c - obb_radius;
            let obb_max = m_dot_c + obb_radius;

            // Frustum projection
            let p = x_near * m_dot_x + y_near * m_dot_y;
            let mut tau_0 = z_near * m_dot_z - p;
            let mut tau_1 = z_near * m_dot_z + p;
            if tau_0 < 0.0 {
                tau_0 *= z_far / z_near;
            }
            if tau_1 > 0.0 {
                tau_1 *= z_far / z_near;
            }

            if obb_min > tau_1 || obb_max < tau_0 {
                return false;
            }
        }
    }

    // U x A_i
    {
        for m in 0..obb.axes.len() {
            let m = vec3(obb.axes[m].z, 0.0, -obb.axes[m].x);
            let m_dot_x = m.x.abs();
            let m_dot_y = 0.0;
            let m_dot_z = m.z;
            let m_dot_c = m.x * obb.center.x + m.z * obb.center.z;

            let mut obb_radius = 0.0;
            for i in 0..3 {
                obb_radius += m.dot(obb.axes[i]).abs() * obb.extents[i];
            }

            let obb_min = m_dot_c - obb_radius;
            let obb_max = m_dot_c + obb_radius;

            // Frustum projection
            let p = x_near * m_dot_x + y_near * m_dot_y;
            let mut tau_0 = z_near * m_dot_z - p;
            let mut tau_1 = z_near * m_dot_z + p;
            if tau_0 < 0.0 {
                tau_0 *= z_far / z_near;
            }
            if tau_1 > 0.0 {
                tau_1 *= z_far / z_near;
            }

            if obb_min > tau_1 || obb_max < tau_0 {
                return false;
            }
        }
    }

    // Frustum Edges X Ai
    {
        for obb_edge_idx in 0..obb.axes.len() {
            let m = [
                vec3(-x_near, 0.0, z_near).cross(obb.axes[obb_edge_idx]), // Left Plane
                vec3(x_near, 0.0, z_near).cross(obb.axes[obb_edge_idx]),  // Right plane
                vec3(0.0, y_near, z_near).cross(obb.axes[obb_edge_idx]),  // Top plane
                vec3(0.0, -y_near, z_near).cross(obb.axes[obb_edge_idx]), // Bottom plane
            ];

            for m in m {
                let m_dot_x = m.x.abs();
                let m_dot_y = m.y.abs();
                let m_dot_z = m.z;

                let epsilon = 1e-4;
                if m_dot_x < epsilon && m_dot_y < epsilon && m_dot_z.abs() < epsilon {
                    continue;
                };

                let m_dot_c = m.dot(obb.center);

                let mut obb_radius = 0.0;
                for i in 0..3 {
                    obb_radius += m.dot(obb.axes[i]).abs() * obb.extents[i];
                }

                let obb_min = m_dot_c - obb_radius;
                let obb_max = m_dot_c + obb_radius;

                // Frustum projection
                let p = x_near * m_dot_x + y_near * m_dot_y;
                let mut tau_0 = z_near * m_dot_z - p;
                let mut tau_1 = z_near * m_dot_z + p;
                if tau_0 < 0.0 {
                    tau_0 *= z_far / z_near;
                }
                if tau_1 > 0.0 {
                    tau_1 *= z_far / z_near;
                }

                if obb_min > tau_1 || obb_max < tau_0 {
                    return false;
                }
            }
        }
    }

    // No intersections detected
    true
}
