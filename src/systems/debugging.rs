use crate::components::{AnimatedModel, AnimationJoints, Instance, InstanceOf, Model};
use crate::resources::LineBuffer;
use bevy_ecs::prelude::{Query, ResMut};

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
