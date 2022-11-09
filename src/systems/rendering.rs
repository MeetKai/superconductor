use crate::resources::{
    self, AnimatedVertexBuffers, Device, IndexBuffer, InstanceBuffer, LineBuffer, MainBindGroup,
    Pipelines, Queue, SkyboxUniformBindGroup, SurfaceFrameView, VertexBuffers,
};
use renderer_core::{
    arc_swap, assets::models::Ranges, permutations, pipelines::DEPTH_FORMAT, LineVertex,
    RawAnimatedVertexBuffers, RawVertexBuffers, VecGpuBuffer,
};
use std::ops::Range;
use std::sync::Arc;

use crate::components::{AnimatedModel, InstanceRanges, JointBuffers, Model};
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

type ModelQuery<'world, 'state, 'component> =
    Query<'world, 'state, (&'component Model, &'component InstanceRanges)>;
type AnimatedModelQuery<'world, 'state, 'component> = Query<
    'world,
    'state,
    (
        &'component AnimatedModel,
        &'component JointBuffers,
        &'component InstanceRanges,
    ),
>;

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn render_desktop(
    (device, queue, pipelines): (Res<Device>, Res<Queue>, Res<Pipelines>),
    main_bind_group: Res<MainBindGroup>,
    skybox_uniform_bind_group: Res<SkyboxUniformBindGroup>,
    surface_frame_view: Res<SurfaceFrameView>,
    pipeline_options: Res<renderer_core::PipelineOptions>,
    mut intermediate_depth_framebuffer: ResMut<resources::IntermediateDepthFramebuffer>,
    (index_buffer, vertex_buffers, animated_vertex_buffers, instance_buffer, line_buffer): (
        Res<IndexBuffer>,
        Res<VertexBuffers>,
        Res<AnimatedVertexBuffers>,
        Res<InstanceBuffer>,
        Res<LineBuffer>,
    ),
    static_models: ModelQuery,
    animated_models: AnimatedModelQuery,
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

    dbg!(&static_model_bind_groups.model_offsets);
    dbg!(&static_model_bind_groups.primitive_offsets);

    let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("command encoder"),
    });

    // todo: broken at the moment due to the addition of culling.
    /*
    if pipeline_options.depth_prepass {
        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("depth prepass render pass"),
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_attachment.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.set_vertex_buffer(0, vertex_buffers.position.slice(..));
        render_pass.set_vertex_buffer(1, instance_buffer.0.buffer.slice(..));

        render_pass.set_pipeline(&pipelines.opaque_depth_prepass.single);
        render_pass.set_bind_group(0, &main_bind_group, &[]);

        static_models.for_each(|(model, instance_ranges)| {
            let index_range = model.0.primitive_ranges.opaque.single.indices.clone();
            let instance_range = &instance_ranges.0[0];

            if !instance_range.is_empty() && !index_range.is_empty() {
                render_pass.draw_indexed(index_range, 0, instance_range.clone());
            }
        });

        render_pass.set_pipeline(&pipelines.opaque_depth_prepass.double);

        static_models.for_each(|(model, instance_ranges)| {
            let index_range = model.0.primitive_ranges.opaque.double.indices.clone();
            let instance_range = &instance_ranges.0[0];

            if !instance_range.is_empty() && !index_range.is_empty() {
                render_pass.draw_indexed(index_range, 0, instance_range.clone());
            }
        });
    }
    */

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
                load: if pipeline_options.depth_prepass {
                    wgpu::LoadOp::Load
                } else {
                    wgpu::LoadOp::Clear(0.0)
                },
                store: true,
            }),
            stencil_ops: None,
        }),
    });

    render_everything(
        &mut render_pass,
        Context {
            vertex_buffers: &vertex_buffers,
            animated_vertex_buffers: &animated_vertex_buffers,
            index_buffer: &index_buffer,
            instance_buffer: &instance_buffer.0.buffer,
            main_bind_group: &main_bind_group,
            skybox_uniform_bind_group: &skybox_uniform_bind_group.0,
            pipelines,
            static_model_bind_groups: &static_model_bind_groups,
            animated_model_bind_groups: &animated_model_bind_groups,
            line_buffer: &line_buffer.buffer,
        },
        &static_models,
        &animated_models,
    );

    drop(render_pass);

    queue.submit(std::iter::once(command_encoder.finish()));
}

#[allow(clippy::too_many_arguments)]
#[cfg(feature = "webgl")]
pub(crate) fn render_webxr(
    frame: bevy_ecs::prelude::NonSend<web_sys::XrFrame>,
    (device, queue, pipelines): (Res<Device>, Res<Queue>, Res<Pipelines>),
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
    static_models: ModelQuery,
    animated_models: AnimatedModelQuery,
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
            stencil_ops: None,
        }),
    });

    render_everything(
        &mut render_pass,
        Context {
            vertex_buffers: &vertex_buffers,
            animated_vertex_buffers: &animated_vertex_buffers,
            index_buffer: &index_buffer,
            instance_buffer: &instance_buffer.0.buffer,
            main_bind_group: &main_bind_group,
            skybox_uniform_bind_group: &skybox_uniform_bind_group.0,
            pipelines,
            static_model_bind_groups: &static_model_bind_groups,
            animated_model_bind_groups: &animated_model_bind_groups,
            line_buffer: &line_buffer.buffer,
        },
        &static_models,
        &animated_models,
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

fn render_mode<'a, R: Fn(&PrimitiveRanges) -> permutations::FaceSides<Ranges>>(
    render_pass: &mut wgpu::RenderPass<'a>,
    context: &Context<'a>,
    static_models: &'a ModelQuery,
    animated_models: &'a AnimatedModelQuery,
    pipelines: &'a permutations::ModelTypes<permutations::FaceSides<wgpu::RenderPipeline>>,
    range_getter: R,
) {
    bind_static_vertex_buffers(render_pass, context.vertex_buffers);

    render_pass.set_pipeline(&pipelines.stationary.single);

    render_all_primitives(
        render_pass,
        static_models,
        context.static_model_bind_groups,
        |primitive_ranges| range_getter(primitive_ranges).single.primitives,
    );

    render_pass.set_pipeline(&pipelines.stationary.double);

    render_all_primitives(
        render_pass,
        static_models,
        context.static_model_bind_groups,
        |primitive_ranges| range_getter(primitive_ranges).double.primitives,
    );

    bind_animated_vertex_buffers(render_pass, context.animated_vertex_buffers);

    render_pass.set_pipeline(&pipelines.animated.single);

    render_all_animated_primitives(
        render_pass,
        animated_models,
        context.animated_model_bind_groups,
        |primitive_ranges| range_getter(primitive_ranges).single.primitives,
    );

    render_pass.set_pipeline(&pipelines.animated.double);

    render_all_animated_primitives(
        render_pass,
        animated_models,
        context.animated_model_bind_groups,
        |primitive_ranges| range_getter(primitive_ranges).double.primitives,
    );
}

struct Context<'a> {
    vertex_buffers: &'a RawVertexBuffers<arc_swap::Guard<Arc<wgpu::Buffer>>>,
    animated_vertex_buffers: &'a RawAnimatedVertexBuffers<arc_swap::Guard<Arc<wgpu::Buffer>>>,
    index_buffer: &'a wgpu::Buffer,
    instance_buffer: &'a wgpu::Buffer,
    main_bind_group: &'a wgpu::BindGroup,
    skybox_uniform_bind_group: &'a wgpu::BindGroup,
    pipelines: &'a renderer_core::Pipelines,
    static_model_bind_groups: &'a ModelBindGroups,
    animated_model_bind_groups: &'a ModelBindGroups,
    line_buffer: &'a VecGpuBuffer<LineVertex>,
}

#[allow(clippy::too_many_arguments)]
fn render_everything<'a>(
    render_pass: &mut wgpu::RenderPass<'a>,
    context: Context<'a>,
    static_models: &'a ModelQuery,
    animated_models: &'a AnimatedModelQuery,
) {
    render_pass.set_index_buffer(context.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    render_pass.set_vertex_buffer(3, context.instance_buffer.slice(..));

    render_pass.set_bind_group(0, context.main_bind_group, &[]);

    render_mode(
        render_pass,
        &context,
        static_models,
        animated_models,
        &context.pipelines.pbr.opaque,
        |primitive_ranges| primitive_ranges.opaque.clone(),
    );

    render_mode(
        render_pass,
        &context,
        static_models,
        animated_models,
        &context.pipelines.pbr.alpha_clipped,
        |primitive_ranges| primitive_ranges.alpha_clipped.clone(),
    );

    if context.line_buffer.len() > 0 {
        render_pass.set_pipeline(&context.pipelines.line);
        render_pass.set_vertex_buffer(0, context.line_buffer.buffer.slice(..));
        render_pass.draw(0..context.line_buffer.len(), 0..1);
    }

    render_pass.set_pipeline(&context.pipelines.skybox);
    render_pass.set_bind_group(1, context.skybox_uniform_bind_group, &[]);
    render_pass.draw(0..3, 0..1);

    render_mode(
        render_pass,
        &context,
        static_models,
        animated_models,
        &context.pipelines.pbr.alpha_blended,
        |primitive_ranges| primitive_ranges.alpha_blended.clone(),
    );
}

// The model bind groups for the current frame
#[derive(Default)]
pub struct ModelBindGroups {
    bind_groups: Vec<arc_swap::Guard<Arc<wgpu::BindGroup>>>,
    // We use a `Vec` of offsets here to avoid needing a `Vec<Vec<Arc<wgpu::BindGroup>>>`
    // This means we can just clear the `Vec`s instead of re-allocating.
    model_offsets: Vec<usize>,
    primitive_offsets: Vec<usize>,
}

impl ModelBindGroups {
    fn collect(&mut self, query: &ModelQuery) {
        self.bind_groups.clear();
        self.model_offsets.clear();
        self.primitive_offsets.clear();

        // This is mutable because it involves potentially swapping out the dummy bind groups
        // for loaded ones.
        query.for_each(|(model, _)| {
            self.model_offsets.push(self.primitive_offsets.len());

            // Todo: we could do a check if the model has any instances here
            // and not write the bind groups if not, which would mean that we don't have to do a check
            // however many times we do a `render_all_primitives` call. But that'd be less clear and
            // I'm not sure if it's worthwhile.

            for primitive in &model.0.primitives {
                self.primitive_offsets.push(self.bind_groups.len());

                self.bind_groups
                    .extend(primitive.lods.iter().map(|lod| lod.bind_group.load()));
            }
        })
    }

    fn collect_animated(&mut self, query: &AnimatedModelQuery) {
        self.bind_groups.clear();
        self.model_offsets.clear();

        // This is mutable because it involves potentially swapping out the dummy bind groups
        // for loaded ones.
        query.for_each(|(model, ..)| {
            self.model_offsets.push(self.primitive_offsets.len());

            // Todo: we could do a check if the model has any instances here
            // and not write the bind groups if not, which would mean that we don't have to do a check
            // however many times we do a `render_all_primitives` call. But that'd be less clear and
            // I'm not sure if it's worthwhile.

            for primitive in &model.0.primitives {
                self.primitive_offsets.push(self.bind_groups.len());

                self.bind_groups
                    .extend(primitive.lods.iter().map(|lod| lod.bind_group.load()));
            }
        })
    }

    fn bind_groups_for_model_primitive(
        &self,
        model_index: usize,
        primitive_index: usize,
    ) -> &[arc_swap::Guard<Arc<wgpu::BindGroup>>] {
        let primitive_offset =
            self.primitive_offsets[self.model_offsets[model_index] + primitive_index];

        &self.bind_groups[primitive_offset..]
    }
}

fn render_all_primitives<'a, G: Fn(&PrimitiveRanges) -> Range<usize>>(
    render_pass: &mut wgpu::RenderPass<'a>,
    models: &ModelQuery,
    model_bind_groups: &'a ModelBindGroups,
    primitive_range_getter: G,
) {
    for (model_index, (model, instance_ranges)) in models.iter().enumerate() {
        // Get the range of primtives we're rendering
        let range = primitive_range_getter(&model.0.primitive_ranges);

        // Get the primitives we're rendering
        let primitives = &model.0.primitives[range.clone()];

        dbg!(&instance_ranges.0);

        for (lod, instance_ranges) in instance_ranges.0.iter().enumerate() {
            if instance_ranges.is_empty() {
                continue;
            }

            let instance_ranges = &instance_ranges[range.clone()];

            dbg!(&lod);

            if instance_ranges.is_empty() {
                continue;
            }

            for ((primitive_index, primitive), instance_range) in
                range.clone().zip(primitives).zip(instance_ranges)
            {
                let bind_groups = &model_bind_groups
                    .bind_groups_for_model_primitive(model_index, primitive_index);

                render_pass.set_bind_group(1, &bind_groups[lod], &[]);

                render_pass.draw_indexed(
                    primitive.lods[lod].index_buffer_range.clone(),
                    0,
                    instance_range.clone(),
                );
            }
        }
    }
}

fn render_all_animated_primitives<'a, G: Fn(&PrimitiveRanges) -> Range<usize>>(
    render_pass: &mut wgpu::RenderPass<'a>,
    models: &'a AnimatedModelQuery,
    model_bind_groups: &'a ModelBindGroups,
    primitive_range_getter: G,
) {
    for (model_index, (model, joint_buffers, instance_ranges)) in models.iter().enumerate() {
        // Get the range of primtives we're rendering
        let range = primitive_range_getter(&model.0.primitive_ranges);

        // Get the primitives we're rendering
        let primitives = &model.0.primitives[range.clone()];

        for (lod, instance_ranges) in instance_ranges.0.iter().enumerate() {
            let instance_ranges = &instance_ranges[range.clone()];

            if instance_ranges.is_empty() {
                continue;
            }

            for ((primitive_index, primitive), instance_range) in
                range.clone().zip(primitives).zip(instance_ranges)
            {
                let bind_groups = &model_bind_groups
                    .bind_groups_for_model_primitive(model_index, primitive_index);

                render_pass.set_bind_group(1, &bind_groups[lod], &[]);

                let mut joint_buffer_index = 0;
                let mut instance_offset = instance_range.start;

                // todo: Remove this ASAP when we can switch to WebGPU.
                while instance_offset < instance_range.end {
                    let end = (instance_offset + model.0.max_instances_per_joint_buffer())
                        .min(instance_range.end);

                    if let Some(joint_buffer) = joint_buffers.buffers.get(joint_buffer_index) {
                        render_pass.set_bind_group(2, &joint_buffer.bind_group, &[]);

                        render_pass.draw_indexed(
                            primitive.lods[lod].index_buffer_range.clone(),
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
