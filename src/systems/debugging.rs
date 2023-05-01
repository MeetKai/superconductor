use crate::components::{AnimatedModel, AnimationJoints, Instance, InstanceOf, Model};
use crate::resources::{LineBuffer, ParticleBuffer};
use bevy_ecs::prelude::{Query, ResMut, Local};
use renderer_core::glam::Vec3;
use renderer_core::instance::ParticleInstance;

#[allow(dead_code)]
pub(crate) fn push_joints_to_lines_buffer(
    instance_query: Query<(&InstanceOf, &AnimationJoints, &Instance)>,
    model_query: Query<&AnimatedModel>,
    mut line_buffer: ResMut<LineBuffer>,
) {
    instance_query.for_each(|(instance_of, animation_joints, instance)| {
        match model_query.get(instance_of.0) {
            Ok(animated_model) => {
                for (id, (start, end)) in animation_joints
                    .0
                    .iter_lines(&animated_model.0.animation_data.depth_first_nodes)
                    .enumerate()
                {
                    line_buffer.staging.extend_from_slice(&[
                        renderer_core::LineVertex {
                            position: instance.0 * start,
                            colour_id: id as u32,
                        },
                        renderer_core::LineVertex {
                            position: instance.0 * end,
                            colour_id: id as u32,
                        },
                    ]);
                }
            }
            Err(error) => {
                log::warn!(
                    "Got an error when pushing joints to the lines buffer for debugging: {}",
                    error
                );
            }
        }
    })
}

#[allow(dead_code)]
pub(crate) fn push_bounding_boxes_to_lines_buffer(
    instance_query: Query<(&InstanceOf, &Instance)>,
    model_query: Query<&Model>,
    mut line_buffer: ResMut<LineBuffer>,
) {
    instance_query.for_each(|(instance_of, instance)| {
        if let Ok(model) = model_query.get(instance_of.0) {
            for (primitive_id, primitive) in model.0.primitives.iter().enumerate() {
                let vertices =
                    primitive
                        .bounding_box
                        .line_points()
                        .map(|point| renderer_core::LineVertex {
                            position: instance.0 * primitive.transform * point,
                            colour_id: primitive_id as u32,
                        });

                line_buffer.staging.extend_from_slice(&vertices);
            }
        }
    })
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


pub(crate) fn push_test_particle(mut particle_buffer: ResMut<ParticleBuffer>, mut time: Local<f32>) {
    for x in 0 .. 10 {
        for y in 0 .. 10 {

    particle_buffer.staging.push(ParticleInstance {
        position: Vec3::new(2.5 - x as f32 * 0.5, 0.5, 2.5 - y as f32 * 0.5),
        scale: 1.0 - x as f32 * 0.05 + y as f32 * 0.05,
        time: (*time + ((y ^ x) & 1) as f32 * 0.5) % 1.0,
        colour: DEBUG_COLOURS[(x + y * 10) as usize % DEBUG_COLOURS.len()]
    });
}
    }

    *time += 1.0 / 200.0;
}
