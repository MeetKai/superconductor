use crate::resources::{
    self, AnimatedVertexBuffers, Device, IndexBuffer, InstanceBuffer, LineBuffer, MainBindGroup,
    Pipelines, Queue, SkyboxUniformBindGroup, SurfaceFrameView, VertexBuffers,
};
use renderer_core::{
    arc_swap, permutations, pipelines::DEPTH_FORMAT, LineVertex, RawAnimatedVertexBuffers,
    RawVertexBuffers, VecGpuBuffer,
};
use std::ops::Range;
use std::sync::Arc;

use crate::components::{AnimatedModel, InstanceRange, JointBuffers, Model};
use bevy_ecs::prelude::{Local, Query, Res, ResMut};
use renderer_core::assets::models::PrimitiveRanges;
#[cfg(feature = "webgl")]
use renderer_core::create_view_from_device_framebuffer;

fn bind_static_vertex_buffers<'a>(
    render_pass: &mut wgpu::RenderPass<'a>,
    vertex_buffers: &'a RawVertexBuffers<arc_swap::Guard<Arc<wgpu::Buffer>>>,
) {
    render_pass.set_vertex_buffer(0, vertex_buffers.position.slice(..));
    render_pass.set_vertex_buffer(1, vertex_buffers.normal.slice(..));
    render_pass.set_vertex_buffer(2, vertex_buffers.uv.slice(..));
}

fn bind_animated_vertex_buffers<'a>(
    render_pass: &mut wgpu::RenderPass<'a>,
    vertex_buffers: &'a RawAnimatedVertexBuffers<arc_swap::Guard<Arc<wgpu::Buffer>>>,
) {
    render_pass.set_vertex_buffer(0, vertex_buffers.position.slice(..));
    render_pass.set_vertex_buffer(1, vertex_buffers.normal.slice(..));
    render_pass.set_vertex_buffer(2, vertex_buffers.uv.slice(..));
    render_pass.set_vertex_buffer(4, vertex_buffers.joint_indices.slice(..));
    render_pass.set_vertex_buffer(5, vertex_buffers.joint_weights.slice(..));
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn render_desktop(
    device: Res<Device>,
    queue: Res<Queue>,
    pipelines: Res<Pipelines>,
    main_bind_group: Res<MainBindGroup>,
    skybox_uniform_bind_group: Res<SkyboxUniformBindGroup>,
    surface_frame_view: Res<SurfaceFrameView>,
    mut intermediate_depth_framebuffer: ResMut<resources::IntermediateDepthFramebuffer>,
    (index_buffer, vertex_buffers, animated_vertex_buffers, instance_buffer, line_buffer): (
        Res<IndexBuffer>,
        Res<VertexBuffers>,
        Res<AnimatedVertexBuffers>,
        Res<InstanceBuffer>,
        Res<LineBuffer>,
    ),
    static_models: Query<(&Model, &InstanceRange)>,
    animated_models: Query<(&AnimatedModel, &JointBuffers, &InstanceRange)>,
    mut static_model_bind_groups: Local<ModelBindGroups>,
    mut animated_model_bind_groups: Local<ModelBindGroups>,
) {
    let device = &device.0;
    let queue = &queue.0;
    let pipelines = &pipelines.0;
    let main_bind_group = main_bind_group.0.load();

    let vertex_buffers = vertex_buffers.0.buffers.load();
    let animated_vertex_buffers = animated_vertex_buffers.0.buffers.load();
    let index_buffer = index_buffer.0.buffer.load();

    let depth_attachment = intermediate_depth_framebuffer.0.get(
        device,
        &wgpu::TextureDescriptor {
            label: Some("intermediate depth framebuffer"),
            size: wgpu::Extent3d {
                width: surface_frame_view.width,
                height: surface_frame_view.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
        },
    );

    static_model_bind_groups.collect(&static_models);
    animated_model_bind_groups.collect_animated(&animated_models);

    let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("command encoder"),
    });

    let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("main render pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &surface_frame_view.view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &depth_attachment.view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(0),
                store: true,
            }),
        }),
    });

    render_everything(
        &mut render_pass,
        &vertex_buffers,
        &animated_vertex_buffers,
        &index_buffer,
        &instance_buffer.0.buffer,
        &main_bind_group,
        &skybox_uniform_bind_group.0,
        pipelines,
        &static_models,
        &animated_models,
        &static_model_bind_groups,
        &animated_model_bind_groups,
        &line_buffer.buffer,
    );

    drop(render_pass);

    queue.submit(std::iter::once(command_encoder.finish()));
}

#[allow(clippy::too_many_arguments)]
#[cfg(feature = "webgl")]
pub(crate) fn render(
    frame: bevy_ecs::prelude::NonSend<web_sys::XrFrame>,
    device: Res<Device>,
    queue: Res<Queue>,
    pipelines: Res<Pipelines>,
    bind_group_layouts: Res<resources::BindGroupLayouts>,
    main_bind_group: Res<MainBindGroup>,
    skybox_uniform_bind_group: Res<SkyboxUniformBindGroup>,
    mut intermediate_color_framebuffer: ResMut<resources::IntermediateColorFramebuffer>,
    mut intermediate_depth_framebuffer: ResMut<resources::IntermediateDepthFramebuffer>,
    mut composite_bind_group: ResMut<resources::CompositeBindGroup>,
    pipeline_options: Res<renderer_core::PipelineOptions>,
    clamp_sampler: Res<resources::ClampSampler>,
    (index_buffer, vertex_buffers, animated_vertex_buffers, instance_buffer, line_buffer): (
        Res<IndexBuffer>,
        Res<VertexBuffers>,
        Res<AnimatedVertexBuffers>,
        Res<InstanceBuffer>,
        Res<LineBuffer>,
    ),
    static_models: Query<(&Model, &InstanceRange)>,
    animated_models: Query<(&AnimatedModel, &JointBuffers, &InstanceRange)>,
    (mut static_model_bind_groups, mut animated_model_bind_groups): (
        Local<ModelBindGroups>,
        Local<ModelBindGroups>,
    ),
) {
    use renderer_core::utils::BorrowedOrOwned;

    let device = &device.0;
    let queue = &queue.0;
    let pipelines = &pipelines.0;
    let bind_group_layouts = &bind_group_layouts.0;
    let main_bind_group = main_bind_group.0.load();

    let vertex_buffers = vertex_buffers.0.buffers.load();
    let animated_vertex_buffers = animated_vertex_buffers.0.buffers.load();
    let index_buffer = index_buffer.0.buffer.load();

    let xr_session: web_sys::XrSession = frame.session();

    let base_layer = xr_session.render_state().base_layer().unwrap();

    let framebuffer: web_sys::WebGlFramebuffer =
        js_sys::Reflect::get(&base_layer, &"framebuffer".into())
            .unwrap()
            .into();

    let framebuffer_colour_attachment = create_view_from_device_framebuffer(
        device,
        framebuffer.clone(),
        &base_layer,
        wgpu::TextureFormat::Rgba8Unorm,
        "device framebuffer (colour)",
    );

    let num_views = pipeline_options
        .multiview
        .map(|views| views.get())
        .unwrap_or(1);

    let (intermediate_color_framebuffer, composite_bind_group) =
        if pipeline_options.render_direct_to_framebuffer() {
            (None, None)
        } else {
            let intermediate_color_framebuffer = intermediate_color_framebuffer.0.get(
                device,
                &wgpu::TextureDescriptor {
                    label: Some("intermediate color framebuffer"),
                    size: wgpu::Extent3d {
                        width: base_layer.framebuffer_width() / num_views,
                        height: base_layer.framebuffer_height(),
                        depth_or_array_layers: num_views,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    // Always true at the moment.
                    format: if pipeline_options.inline_tonemapping {
                        wgpu::TextureFormat::Rgba8Unorm
                    } else {
                        wgpu::TextureFormat::Rgba16Float
                    },
                    usage: wgpu::TextureUsages::TEXTURE_BINDING
                        | wgpu::TextureUsages::RENDER_ATTACHMENT,
                },
            );

            // todo: we cache the colour framebuffer based on the size, creating a new framebuffer
            // when the size changes. But we don't re-create the bind group which is going to lead
            // to a crash if we ever try and resize the framebuffer.
            let composite_bind_group = composite_bind_group.0.get_or_insert_with(|| {
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("composite bind group"),
                    layout: &bind_group_layouts.tonemap,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Sampler(&clamp_sampler.0),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(
                                &intermediate_color_framebuffer.view,
                            ),
                        },
                    ],
                })
            });

            (
                Some(intermediate_color_framebuffer),
                Some(composite_bind_group),
            )
        };

    let depth_attachment = if pipeline_options.render_direct_to_framebuffer() {
        BorrowedOrOwned::Owned(create_view_from_device_framebuffer(
            device,
            framebuffer,
            &base_layer,
            DEPTH_FORMAT,
            "device framebuffer (depth)",
        ))
    } else {
        BorrowedOrOwned::Borrowed(intermediate_depth_framebuffer.0.get(
            device,
            &wgpu::TextureDescriptor {
                label: Some("intermediate depth framebuffer"),
                size: wgpu::Extent3d {
                    width: base_layer.framebuffer_width() / num_views,
                    height: base_layer.framebuffer_height(),
                    depth_or_array_layers: num_views,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: DEPTH_FORMAT,
                usage: wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::RENDER_ATTACHMENT,
            },
        ))
    };

    static_model_bind_groups.collect(&static_models);
    animated_model_bind_groups.collect_animated(&animated_models);

    let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("command encoder"),
    });

    let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("main render pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: if let Some(intermediate_color_framebuffer) = intermediate_color_framebuffer {
                &intermediate_color_framebuffer.view
            } else {
                &framebuffer_colour_attachment.view
            },
            resolve_target: None,
            ops: wgpu::Operations {
                // Note: when rendering to a Quest 2, clearing the intermediate framebuffer
                // makes the skybox only render on one eye! No clue why.
                load: wgpu::LoadOp::Load,
                store: true,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &depth_attachment.get().view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(0),
                store: true,
            }),
        }),
    });

    render_everything(
        &mut render_pass,
        &vertex_buffers,
        &animated_vertex_buffers,
        &index_buffer,
        &instance_buffer.0.buffer,
        &main_bind_group,
        &skybox_uniform_bind_group.0,
        pipelines,
        &static_models,
        &animated_models,
        &static_model_bind_groups,
        &animated_model_bind_groups,
        &line_buffer.buffer,
    );

    drop(render_pass);

    if let Some(composite_bind_group) = composite_bind_group {
        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("composite render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &framebuffer_colour_attachment.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&pipelines.tonemap);

        render_pass.set_bind_group(0, &main_bind_group, &[]);
        render_pass.set_bind_group(1, composite_bind_group, &[]);

        render_pass.draw(0..3, 0..1);

        drop(render_pass);
    }

    queue.submit(std::iter::once(command_encoder.finish()));
}

fn render_mode<'a, R: Fn(&PrimitiveRanges) -> permutations::FaceSides<Range<usize>>>(
    render_pass: &mut wgpu::RenderPass<'a>,
    vertex_buffers: &'a RawVertexBuffers<arc_swap::Guard<Arc<wgpu::Buffer>>>,
    animated_vertex_buffers: &'a RawAnimatedVertexBuffers<arc_swap::Guard<Arc<wgpu::Buffer>>>,
    static_models: &'a Query<(&Model, &InstanceRange)>,
    animated_models: &'a Query<(&AnimatedModel, &JointBuffers, &InstanceRange)>,
    static_model_bind_groups: &'a ModelBindGroups,
    animated_model_bind_groups: &'a ModelBindGroups,
    pipelines: &'a permutations::ModelTypes<permutations::FaceSides<wgpu::RenderPipeline>>,
    range_getter: R,
) {
    bind_static_vertex_buffers(render_pass, vertex_buffers);

    render_pass.set_pipeline(&pipelines.stationary.single);

    render_all_primitives(
        render_pass,
        static_models,
        static_model_bind_groups,
        |primitive_ranges| range_getter(primitive_ranges).single,
    );

    render_pass.set_pipeline(&pipelines.stationary.double);

    render_all_primitives(
        render_pass,
        static_models,
        static_model_bind_groups,
        |primitive_ranges| range_getter(primitive_ranges).double,
    );

    bind_animated_vertex_buffers(render_pass, animated_vertex_buffers);

    render_pass.set_pipeline(&pipelines.animated.single);

    render_all_animated_primitives(
        render_pass,
        animated_models,
        animated_model_bind_groups,
        |primitive_ranges| range_getter(primitive_ranges).single,
    );

    render_pass.set_pipeline(&pipelines.animated.double);

    render_all_animated_primitives(
        render_pass,
        animated_models,
        animated_model_bind_groups,
        |primitive_ranges| range_getter(primitive_ranges).double,
    );
}

#[allow(clippy::too_many_arguments)]
fn render_everything<'a>(
    render_pass: &mut wgpu::RenderPass<'a>,
    vertex_buffers: &'a RawVertexBuffers<arc_swap::Guard<Arc<wgpu::Buffer>>>,
    animated_vertex_buffers: &'a RawAnimatedVertexBuffers<arc_swap::Guard<Arc<wgpu::Buffer>>>,
    index_buffer: &'a wgpu::Buffer,
    instance_buffer: &'a wgpu::Buffer,
    main_bind_group: &'a wgpu::BindGroup,
    skybox_uniform_bind_group: &'a wgpu::BindGroup,
    pipelines: &'a renderer_core::Pipelines,
    static_models: &'a Query<(&Model, &InstanceRange)>,
    animated_models: &'a Query<(&AnimatedModel, &JointBuffers, &InstanceRange)>,
    static_model_bind_groups: &'a ModelBindGroups,
    animated_model_bind_groups: &'a ModelBindGroups,
    line_buffer: &'a VecGpuBuffer<LineVertex>,
) {
    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    render_pass.set_vertex_buffer(3, instance_buffer.slice(..));

    render_pass.set_bind_group(0, main_bind_group, &[]);

    render_mode(
        render_pass,
        vertex_buffers,
        animated_vertex_buffers,
        static_models,
        animated_models,
        static_model_bind_groups,
        animated_model_bind_groups,
        &pipelines.pbr.opaque,
        |primitive_ranges| primitive_ranges.opaque.clone(),
    );

    render_mode(
        render_pass,
        vertex_buffers,
        animated_vertex_buffers,
        static_models,
        animated_models,
        static_model_bind_groups,
        animated_model_bind_groups,
        &pipelines.pbr.alpha_clipped,
        |primitive_ranges| primitive_ranges.alpha_clipped.clone(),
    );

    if line_buffer.len() > 0 {
        render_pass.set_pipeline(&pipelines.line);
        render_pass.set_vertex_buffer(0, line_buffer.buffer.slice(..));
        render_pass.draw(0..line_buffer.len(), 0..1);
    }

    render_pass.set_pipeline(&pipelines.skybox);
    render_pass.set_bind_group(1, skybox_uniform_bind_group, &[]);
    render_pass.draw(0..3, 0..1);

    render_mode(
        render_pass,
        vertex_buffers,
        animated_vertex_buffers,
        static_models,
        animated_models,
        static_model_bind_groups,
        animated_model_bind_groups,
        &pipelines.pbr.alpha_blended,
        |primitive_ranges| primitive_ranges.alpha_blended.clone(),
    );
}

// The model bind groups for the current frame
#[derive(Default)]
pub struct ModelBindGroups {
    bind_groups: Vec<arc_swap::Guard<Arc<wgpu::BindGroup>>>,
    // We use a `Vec` of offsets here to avoid needing a `Vec<Vec<Arc<wgpu::BindGroup>>>`
    // This means we can just clear the `Vec`s instead of re-allocating.
    offsets: Vec<usize>,
}

impl ModelBindGroups {
    fn collect(&mut self, query: &Query<(&Model, &InstanceRange)>) {
        self.bind_groups.clear();
        self.offsets.clear();

        // This is mutable because it involves potentially swapping out the dummy bind groups
        // for loaded ones.
        query.for_each(|(model, _)| {
            self.offsets.push(self.bind_groups.len());

            // Todo: we could do a check if the model has any instances here
            // and not write the bind groups if not, which would mean that we don't have to do a check
            // however many times we do a `render_all_primitives` call. But that'd be less clear and
            // I'm not sure if it's worthwhile.
            self.bind_groups.extend(
                model
                    .0
                    .primitives
                    .iter()
                    .map(|primitive| primitive.bind_group.load()),
            );
        })
    }

    fn collect_animated(&mut self, query: &Query<(&AnimatedModel, &JointBuffers, &InstanceRange)>) {
        self.bind_groups.clear();
        self.offsets.clear();

        // This is mutable because it involves potentially swapping out the dummy bind groups
        // for loaded ones.
        query.for_each(|(model, ..)| {
            self.offsets.push(self.bind_groups.len());

            // Todo: we could do a check if the model has any instances here
            // and not write the bind groups if not, which would mean that we don't have to do a check
            // however many times we do a `render_all_primitives` call. But that'd be less clear and
            // I'm not sure if it's worthwhile.
            self.bind_groups.extend(
                model
                    .0
                    .primitives
                    .iter()
                    .map(|primitive| primitive.bind_group.load()),
            );
        })
    }

    fn bind_groups_for_model(
        &self,
        model_index: usize,
    ) -> &[arc_swap::Guard<Arc<wgpu::BindGroup>>] {
        &self.bind_groups[self.offsets[model_index]..]
    }
}

fn render_all_primitives<'a, G: Fn(&PrimitiveRanges) -> Range<usize>>(
    render_pass: &mut wgpu::RenderPass<'a>,
    models: &Query<(&Model, &InstanceRange)>,
    model_bind_groups: &'a ModelBindGroups,
    primitive_range_getter: G,
) {
    for (model_index, (model, instance_range)) in models.iter().enumerate() {
        // Don't issue commands for models with no (visible) instances.
        if !instance_range.0.is_empty() {
            // Get the range of primtives we're rendering
            let range = primitive_range_getter(&model.0.primitive_ranges);

            // Get the primitives we're rendering
            let primitives = &model.0.primitives[range.clone()];
            // And their associated material bind groups
            let bind_groups = &model_bind_groups.bind_groups_for_model(model_index)[range];

            for (primitive, bind_group) in primitives.iter().zip(bind_groups) {
                render_pass.set_bind_group(1, bind_group, &[]);

                render_pass.draw_indexed(
                    primitive.index_buffer_range.clone(),
                    0,
                    instance_range.0.clone(),
                );
            }
        }
    }
}

fn render_all_animated_primitives<'a, G: Fn(&PrimitiveRanges) -> Range<usize>>(
    render_pass: &mut wgpu::RenderPass<'a>,
    models: &'a Query<(&AnimatedModel, &JointBuffers, &InstanceRange)>,
    model_bind_groups: &'a ModelBindGroups,
    primitive_range_getter: G,
) {
    for (model_index, (model, joint_buffers, instance_range)) in models.iter().enumerate() {
        // Don't issue commands for models with no (visible) instances.
        if !instance_range.0.is_empty() {
            // Get the range of primtives we're rendering
            let range = primitive_range_getter(&model.0.primitive_ranges);

            // Get the primitives we're rendering
            let primitives = &model.0.primitives[range.clone()];
            // And their associated material bind groups
            let bind_groups = &model_bind_groups.bind_groups_for_model(model_index)[range];

            for (primitive, bind_group) in primitives.iter().zip(bind_groups) {
                render_pass.set_bind_group(1, bind_group, &[]);

                let mut joint_buffer_index = 0;
                let mut instance_offset = instance_range.0.start;

                while instance_offset < instance_range.0.end {
                    let end = (instance_offset + model.0.max_instances_per_joint_buffer())
                        .min(instance_range.0.end);

                    if let Some(joint_buffer) = joint_buffers.buffers.get(joint_buffer_index) {
                        render_pass.set_bind_group(2, &joint_buffer.bind_group, &[]);

                        render_pass.draw_indexed(
                            primitive.index_buffer_range.clone(),
                            0,
                            instance_offset..end,
                        );
                    }

                    instance_offset = end;
                    joint_buffer_index += 1;
                }
            }
        }
    }
}
