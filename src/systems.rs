use crate::components::{
    AnimatedModel, AnimatedModelUrl, AnimationJoints, AnimationState, Instance, InstanceOf,
    InstanceRanges, Instances, JointBuffer, JointBuffers, JointsOffset, Model, ModelUrl,
    PendingAnimatedModel, PendingModel,
};
use crate::resources::{
    AnimatedVertexBuffers, BindGroupLayouts, BoundingSphereParams, Camera, ClampSampler,
    CompositeBindGroup, CullingParams, Device, IndexBuffer, InstanceBuffer,
    IntermediateColorFramebuffer, IntermediateDepthFramebuffer, LineBuffer, LutUrl, MainBindGroup,
    NewIblCubemap, Pipelines, Queue, SkyboxUniformBindGroup, SkyboxUniformBuffer, SurfaceFrameView,
    UniformBuffer, VertexBuffers,
};
use bevy_ecs::prelude::{Added, Commands, Entity, Local, Query, Res, ResMut, Without};
use renderer_core::{
    arc_swap::{ArcSwap, ArcSwapOption},
    assets::{textures, HttpClient},
    bytemuck, create_main_bind_group,
    crevice::std140::AsStd140,
    culling::{BoundingSphereCullingParams, CullingFrustum},
    glam::Mat4,
    ibl::IblResources,
    shared_structs, spawn, GpuInstance, Texture,
};
use std::sync::Arc;

pub(crate) mod debugging;
pub(crate) mod rendering;

// todo: probably merge all the setup systems or move them into the main code.
pub(crate) fn create_bind_group_layouts_and_pipelines(
    device: Res<Device>,
    pipeline_options: Res<renderer_core::PipelineOptions>,
    mut commands: Commands,
) {
    let device = &device.0;

    let bind_group_layouts = renderer_core::BindGroupLayouts::new(device, &pipeline_options);

    let pipelines = renderer_core::Pipelines::new(device, &bind_group_layouts, &pipeline_options);

    commands.insert_resource(BindGroupLayouts(Arc::new(bind_group_layouts)));
    commands.insert_resource(Pipelines(Arc::new(pipelines)));
    commands.insert_resource(IntermediateColorFramebuffer(Default::default()));
    commands.insert_resource(IntermediateDepthFramebuffer(Default::default()));
    commands.insert_resource(CompositeBindGroup(None));
}

pub(crate) fn clear_instance_buffers(
    mut instance_buffer: ResMut<InstanceBuffer>,
    mut query: Query<&mut Instances>,
) {
    instance_buffer.0.clear();

    query.for_each_mut(|mut instances| instances.clear());
}

pub(crate) fn clear_joint_buffers(mut query: Query<&mut JointBuffers>) {
    query.for_each_mut(|mut joint_buffers| {
        joint_buffers.next_buffer = 0;

        for joint_buffer in &mut joint_buffers.buffers {
            joint_buffer.staging.clear();
        }
    })
}

pub(crate) fn clear_line_buffer(mut line_buffer: ResMut<LineBuffer>) {
    line_buffer.staging.clear();
    line_buffer.buffer.clear();
}

pub(crate) fn progress_animation_times(
    mut instance_query: Query<(&InstanceOf, &mut AnimationState)>,
    model_query: Query<&AnimatedModel>,
    mut times_error_reported: Local<u32>,
) {
    instance_query.for_each_mut(|(instance_of, mut animation_state)| {
        match model_query.get(instance_of.0) {
            Ok(animated_model) => {
                let animations = &animated_model
                .0
                .animation_data
                .animations;

                if let Some(animation) = animations
                    .get(animation_state.animation_index)
                {
                    animation_state.time =
                        (animation_state.time + 1.0 / 60.0) % animation.total_time();
                } else {
                    log::warn!("Got an error when progressing animations: animation index {} is out of range of {} animations", animation_state.animation_index, animations.len());
                }
            }
            Err(error) => {
                // todo: this is very messy.
                if *times_error_reported < 5 {
                    log::warn!("Got an error when progressing animations: {}", error);
                    *times_error_reported += 1;
                }
            }
        }
    })
}

pub(crate) fn sample_animations(
    mut instance_query: Query<(&InstanceOf, &mut AnimationJoints, &AnimationState)>,
    model_query: Query<&AnimatedModel>,
) {
    instance_query.for_each_mut(|(instance_of, mut animation_joints, animation_state)| {
        match model_query.get(instance_of.0) {
            Ok(animated_model) => {
                let animations = &animated_model.0.animation_data.animations;

                if let Some(animation) = animations.get(animation_state.animation_index) {
                    animation.animate(&mut animation_joints.0, animation_state.time);
                }
            }
            Err(error) => {
                log::warn!("Got an error when sampling animations: {}", error);
            }
        }
    })
}

pub(crate) fn upload_joint_buffers(query: Query<&JointBuffers>, queue: Res<Queue>) {
    query.for_each(|joint_buffers| {
        for joint_buffer in &joint_buffers.buffers[..joint_buffers.next_buffer + 1] {
            queue.0.write_buffer(
                &joint_buffer.buffer,
                0,
                bytemuck::cast_slice(&joint_buffer.staging),
            );
        }
    })
}

pub(crate) fn push_joints(
    mut instance_query: Query<(Entity, &InstanceOf, &mut AnimationJoints)>,
    mut model_query: Query<(&AnimatedModel, &mut JointBuffers)>,
    device: Res<Device>,
    bind_group_layouts: Res<BindGroupLayouts>,
    mut commands: Commands,
) {
    instance_query.for_each_mut(|(entity, instance_of, mut animation_joints)| {
        match model_query.get_mut(instance_of.0) {
            Ok((animated_model, mut joint_buffers)) => {
                if joint_buffers.buffers[joint_buffers.next_buffer]
                    .staging
                    .remaining_capacity()
                    < animated_model.0.num_joints() as usize
                {
                    joint_buffers.next_buffer += 1;

                    if joint_buffers.next_buffer >= joint_buffers.buffers.len() {
                        joint_buffers
                            .buffers
                            .push(JointBuffer::new(&device.0, &bind_group_layouts.0));
                    }
                }

                commands.entity(entity).insert(JointsOffset(
                    joint_buffers.buffers[joint_buffers.next_buffer]
                        .staging
                        .len() as u32,
                ));

                'joint_loop: for joint in animation_joints
                    .0
                    .iter(
                        &animated_model
                            .0
                            .animation_data
                            .joint_indices_to_node_indices,
                        &animated_model.0.animation_data.inverse_bind_transforms,
                        &animated_model.0.animation_data.depth_first_nodes,
                    )
                    .map(|joint| {
                        shared_structs::JointTransform::new(
                            joint.translation,
                            joint.scale,
                            joint.rotation,
                        )
                    })
                {
                    let next_buffer = joint_buffers.next_buffer;

                    if let Err(error) = joint_buffers.buffers[next_buffer].staging.try_push(joint) {
                        log::warn!("Got an error when pushing joints: {}", error);
                        break 'joint_loop;
                    }
                }
            }
            Err(error) => {
                log::warn!("Got an error when pushing joints: {}", error);
            }
        }
    })
}

pub(crate) fn push_entity_instances(
    camera: Res<Camera>,
    culling_params: Res<CullingParams>,
    mut instance_query: Query<(&InstanceOf, &Instance, Option<&JointsOffset>)>,
    mut model_query: Query<(&mut Instances, Option<&Model>, Option<&AnimatedModel>)>,
) {
    let view_matrix = camera.view_matrix();

    instance_query.for_each_mut(|(instance_of, instance, joints_offset)| {
        match model_query.get_mut(instance_of.0) {
            Ok((mut instances, model, animated_model)) => {
                if let Some(model) = model {
                    instances.reserve(model.0.primitives.len());

                    for (primitive_id, primitive) in model.0.primitives.iter().enumerate() {
                        let primitive_transform = instance.0 * primitive.transform;

                        let mut passed_culling_check = match culling_params.bounding_sphere_params {
                            BoundingSphereParams::SingleView(params) => {
                                renderer_core::culling::test_bounding_sphere(
                                    primitive.bounding_sphere,
                                    primitive_transform,
                                    params,
                                )
                            }
                            BoundingSphereParams::Vr { left, right } => {
                                renderer_core::culling::test_bounding_sphere(
                                    primitive.bounding_sphere,
                                    primitive_transform,
                                    left,
                                ) || renderer_core::culling::test_bounding_sphere(
                                    primitive.bounding_sphere,
                                    primitive_transform,
                                    right,
                                )
                            }
                        };

                        if let Some(frustum) = culling_params.frustum {
                            passed_culling_check &=
                                renderer_core::culling::test_using_separating_axis_theorem(
                                    frustum,
                                    view_matrix,
                                    primitive_transform,
                                    &primitive.bounding_box,
                                );
                        }

                        if !passed_culling_check {
                            continue;
                        }

                        instances.insert(
                            primitive_id,
                            GpuInstance {
                                similarity: primitive_transform,
                                joints_offset: joints_offset.map(|offset| offset.0).unwrap_or(0),
                                _padding: Default::default(),
                            },
                        );
                    }
                } else if let Some(animated_model) = animated_model {
                    instances.reserve(animated_model.0.primitives.len());

                    for (primitive_id, primitive) in animated_model.0.primitives.iter().enumerate()
                    {
                        let primitive_transform = instance.0 * primitive.transform;

                        // todo: culling for animated models.
                        instances.insert(
                            primitive_id,
                            GpuInstance {
                                similarity: primitive_transform,
                                joints_offset: joints_offset.map(|offset| offset.0).unwrap_or(0),
                                _padding: Default::default(),
                            },
                        );
                    }
                }
            }
            Err(error) => {
                log::warn!("Got an error when pushing an instance: {}", error);
            }
        }
    })
}

pub(crate) fn upload_instances(
    device: Res<Device>,
    queue: Res<Queue>,
    mut instance_buffer: ResMut<InstanceBuffer>,
    mut query: Query<(&Instances, &mut InstanceRanges)>,
) {
    let mut command_encoder = device
        .0
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("command encoder"),
        });

    query.for_each_mut(|(instances, mut instance_ranges)| {
        instance_ranges.0.clear();

        for primitive_instances in &instances.inner {
            instance_ranges.0.push(instance_buffer.0.push(
                primitive_instances,
                &device.0,
                &queue.0,
                &mut command_encoder,
            ));
        }
    });

    queue.0.submit(std::iter::once(command_encoder.finish()));
}

pub(crate) fn upload_lines(
    device: Res<Device>,
    queue: Res<Queue>,
    mut line_buffer: ResMut<LineBuffer>,
) {
    let mut command_encoder = device
        .0
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("command encoder"),
        });

    let LineBuffer { staging, buffer } = &mut *line_buffer;
    buffer.push(staging, &device.0, &queue.0, &mut command_encoder);

    queue.0.submit(std::iter::once(command_encoder.finish()));
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn allocate_bind_groups<T: HttpClient>(
    device: Res<Device>,
    queue: Res<Queue>,
    pipelines: Res<Pipelines>,
    bind_group_layouts: Res<BindGroupLayouts>,
    texture_settings: Res<textures::Settings>,
    http_client: Res<T>,
    lut_url: Res<LutUrl>,
    mut commands: Commands,
) {
    let device = &device.0;
    let queue = &queue.0;
    let pipelines = &pipelines.0;
    let bind_group_layouts = &bind_group_layouts.0;

    // todo: this is very messy.
    let ibl_resources = Arc::new(IblResources {
        lut: ArcSwap::from(Arc::new(Texture::new(device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("dummy ibl lut"),
                size: wgpu::Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
                sample_count: 1,
                mip_level_count: 1,
                dimension: wgpu::TextureDimension::D2,
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
                format: wgpu::TextureFormat::Rgba8Unorm,
            },
        )))),
        cubemap: ArcSwap::from(Arc::new(Texture::new_cubemap(device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("dummy ibl cubemap"),
                size: wgpu::Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 6,
                },
                sample_count: 1,
                mip_level_count: 1,
                dimension: wgpu::TextureDimension::D2,
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
                format: wgpu::TextureFormat::Rgba16Float,
            },
        )))),
        sphere_harmonics: device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("sphere harmonics buffer"),
            size: 144,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }),
    });

    let uniform_buffer = Arc::new(device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("uniform buffer"),
        size: std::mem::size_of::<<shared_structs::Uniforms as AsStd140>::Output>() as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        mapped_at_creation: false,
    }));

    let clamp_sampler = Arc::new(device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        anisotropy_clamp: texture_settings.anisotropy_clamp,
        ..Default::default()
    }));

    let skybox_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("skybox uniform buffer"),
        size: std::mem::size_of::<<shared_structs::SkyboxUniforms as AsStd140>::Output>() as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        mapped_at_creation: false,
    });

    let skybox_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("skybox uniform bind group"),
        layout: &bind_group_layouts.skybox,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: skybox_uniform_buffer.as_entire_binding(),
        }],
    });

    let main_bind_group = Arc::new(ArcSwap::from_pointee(create_main_bind_group(
        device,
        &ibl_resources,
        &uniform_buffer,
        &clamp_sampler,
        bind_group_layouts,
    )));

    commands.insert_resource(UniformBuffer(uniform_buffer.clone()));
    commands.insert_resource(MainBindGroup(main_bind_group.clone()));
    commands.insert_resource(ClampSampler(clamp_sampler.clone()));
    commands.insert_resource(ibl_resources.clone());

    commands.insert_resource(SkyboxUniformBuffer(skybox_uniform_buffer));
    commands.insert_resource(SkyboxUniformBindGroup(skybox_uniform_bind_group));

    let textures_context = renderer_core::assets::textures::Context {
        device: device.clone(),
        queue: queue.clone(),
        http_client: http_client.clone(),
        bind_group_layouts: bind_group_layouts.clone(),
        pipelines: pipelines.clone(),
        settings: texture_settings.clone(),
    };

    let lut_url = lut_url.0.clone();

    spawn(async move {
        // todo: yuck.
        // This results in only the skybox being rendered:
        //let bytes = &include_bytes!("../../lut_ggx.png")[..];
        let bytes = textures_context
            .http_client
            .fetch_bytes(&lut_url, None)
            .await
            .unwrap();

        let result = renderer_core::assets::textures::load_image_crate_image(
            &bytes[..],
            false,
            false,
            &textures_context,
        );

        match result {
            Ok((lut_texture, _size)) => {
                ibl_resources.lut.store(lut_texture);

                main_bind_group.store(Arc::new(create_main_bind_group(
                    &textures_context.device,
                    &ibl_resources,
                    &uniform_buffer,
                    &clamp_sampler,
                    &textures_context.bind_group_layouts,
                )));

                Ok(())
            }
            Err(error) => Err(anyhow::anyhow!(
                "Got an error while trying to load {}: {}",
                lut_url,
                error
            )),
        }
    });

    commands.insert_resource(IndexBuffer(Arc::new(renderer_core::IndexBuffer::new(
        1024, device,
    ))));
    commands.insert_resource(VertexBuffers(Arc::new(renderer_core::VertexBuffers::new(
        1024, device,
    ))));
    commands.insert_resource(AnimatedVertexBuffers(Arc::new(
        renderer_core::AnimatedVertexBuffers::new(1024, device),
    )));

    commands.insert_resource(InstanceBuffer(renderer_core::VecGpuBuffer::new(
        1,
        device,
        wgpu::BufferUsages::VERTEX,
        "instance buffer",
    )));

    commands.insert_resource(LineBuffer {
        buffer: renderer_core::VecGpuBuffer::new(
            1,
            device,
            wgpu::BufferUsages::VERTEX,
            "line buffer",
        ),
        staging: Vec::new(),
    });
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn update_ibl_resources<T: HttpClient>(
    device: Res<Device>,
    queue: Res<Queue>,
    pipelines: Res<Pipelines>,
    bind_group_layouts: Res<BindGroupLayouts>,
    texture_settings: Res<textures::Settings>,
    mut new_ibl_cubemap: ResMut<NewIblCubemap>,
    ibl_resources: Res<Arc<IblResources>>,
    clamp_sampler: Res<ClampSampler>,
    main_bind_group: Res<MainBindGroup>,
    uniform_buffer: Res<UniformBuffer>,
    http_client: Res<T>,
) {
    let new_ibl_cubemap = match new_ibl_cubemap.0.take() {
        Some(new_ibl_cubemap) => new_ibl_cubemap,
        None => return,
    };

    let main_bind_group = main_bind_group.0.clone();

    let device = &device.0;
    let queue = &queue.0;
    let pipelines = &pipelines.0;
    let bind_group_layouts = &bind_group_layouts.0;
    let clamp_sampler = clamp_sampler.0.clone();
    let uniform_buffer = uniform_buffer.0.clone();
    let ibl_resources = ibl_resources.clone();

    let textures_context = renderer_core::assets::textures::Context {
        device: device.clone(),
        queue: queue.clone(),
        http_client: http_client.clone(),
        bind_group_layouts: bind_group_layouts.clone(),
        pipelines: pipelines.clone(),
        settings: texture_settings.clone(),
    };

    let queue = queue.clone();

    spawn(async move {
        match renderer_core::assets::textures::load_ibl_cubemap(
            textures_context.clone(),
            &new_ibl_cubemap,
        )
        .await
        {
            Ok(ibl_data) => {
                ibl_resources.cubemap.store(ibl_data.texture);
                queue.write_buffer(
                    &ibl_resources.sphere_harmonics,
                    0,
                    &ibl_data.padded_sphere_harmonics_bytes,
                );

                main_bind_group.store(Arc::new(create_main_bind_group(
                    &textures_context.device,
                    &ibl_resources,
                    &uniform_buffer,
                    &clamp_sampler,
                    &textures_context.bind_group_layouts,
                )));

                Ok(())
            }
            Err(error) => Err(anyhow::anyhow!(
                "Error file loading ibl cubemap {}: {}",
                new_ibl_cubemap,
                error
            )),
        }
    });
}

pub(crate) fn update_desktop_uniform_buffers(
    pipeline_options: Res<renderer_core::PipelineOptions>,
    queue: Res<Queue>,
    uniform_buffer: Res<UniformBuffer>,
    skybox_uniform_buffer: Res<SkyboxUniformBuffer>,
    surface_frame_view: Res<SurfaceFrameView>,
    camera: Res<Camera>,
    mut culling_params: ResMut<CullingParams>,
) {
    let queue = &queue.0;

    let perspective_matrix = Mat4::perspective_infinite_reverse_rh(
        59.0_f32.to_radians(),
        surface_frame_view.width as f32 / surface_frame_view.height as f32,
        0.001,
    );

    *culling_params =
        CullingParams {
            frustum: Some(CullingFrustum::new(
                59.0_f32.to_radians(),
                surface_frame_view.width as f32 / surface_frame_view.height as f32,
                0.001,
                1000.0,
            )),
            bounding_sphere_params: BoundingSphereParams::SingleView(
                BoundingSphereCullingParams::new(camera.view_matrix(), perspective_matrix, 0.001),
            ),
        };

    let projection_view = perspective_matrix * camera.view_matrix();

    let is_webgpu = cfg!(all(feature = "wasm", not(feature = "webgl")));

    let uniforms = renderer_core::shared_structs::Uniforms {
        left_projection_view: projection_view.into(),
        right_projection_view: projection_view.into(),
        left_eye_position: camera.position,
        right_eye_position: camera.position,
        flip_viewport: false as u32,
        inline_tonemapping: pipeline_options.inline_tonemapping as u32,
        // Rendering to a srgb surface should be possible at some point, but doesn't currently seem to be.
        inline_srgb: is_webgpu as u32,
        reverse_z: true as u32,
        #[cfg(not(feature = "wasm"))]
        _padding: 0,
    };

    queue.write_buffer(
        &uniform_buffer.0,
        0,
        renderer_core::bytemuck::bytes_of(&uniforms.as_std140()),
    );

    let skybox_uniforms = shared_structs::SkyboxUniforms {
        left_projection_inverse: perspective_matrix.inverse().into(),
        right_projection_inverse: perspective_matrix.inverse().into(),
        left_view_inverse: camera.rotation.into(),
        right_view_inverse: camera.rotation.into(),
    };

    queue.write_buffer(
        &skybox_uniform_buffer.0,
        0,
        bytemuck::bytes_of(&skybox_uniforms.as_std140()),
    );
}

#[cfg(feature = "webgl")]
#[derive(Default)]
struct ViewData {
    projection: Mat4,
    view: Mat4,
    instance: renderer_core::Instance,
}

#[cfg(feature = "webgl")]
pub(crate) fn update_webxr_uniform_buffers(
    pose: bevy_ecs::prelude::NonSend<web_sys::XrViewerPose>,
    pipeline_options: Res<renderer_core::PipelineOptions>,
    queue: Res<Queue>,
    uniform_buffer: Res<UniformBuffer>,
    skybox_uniform_buffer: Res<SkyboxUniformBuffer>,
    mut culling_params: ResMut<CullingParams>,
) {
    let queue = &queue.0;

    let parse_matrix = |vec| Mat4::from_cols_array(&<[f32; 16]>::try_from(vec).unwrap());

    let views = pose.views();

    let mut views_iter = views.iter();

    let left_view: web_sys::XrView = views_iter.next().unwrap().into();

    let left_view_data = ViewData {
        projection: parse_matrix(left_view.projection_matrix()),
        view: parse_matrix(left_view.transform().matrix()).inverse(),
        instance: renderer_core::instance::instance_from_transform(left_view.transform(), 0.0),
    };

    let (right_view_data, is_vr) = if let Some(right_view) = views_iter.next() {
        let right_view: web_sys::XrView = right_view.into();

        (
            ViewData {
                projection: parse_matrix(right_view.projection_matrix()),
                view: parse_matrix(right_view.transform().matrix()).inverse(),
                instance: renderer_core::instance::instance_from_transform(
                    right_view.transform(),
                    0.0,
                ),
            },
            true,
        )
    } else {
        (Default::default(), false)
    };

    let uniforms = renderer_core::shared_structs::Uniforms {
        left_projection_view: (left_view_data.projection * left_view_data.view).into(),
        right_projection_view: (right_view_data.projection * right_view_data.view).into(),
        left_eye_position: left_view_data.instance.translation,
        right_eye_position: right_view_data.instance.translation,
        flip_viewport: pipeline_options.flip_viewport as u32,
        inline_tonemapping: pipeline_options.inline_tonemapping as u32,
        inline_srgb: true as u32,
        reverse_z: false as u32,
    };

    queue.write_buffer(
        &uniform_buffer.0,
        0,
        renderer_core::bytemuck::bytes_of(&uniforms.as_std140()),
    );

    let skybox_uniforms = shared_structs::SkyboxUniforms {
        left_projection_inverse: left_view_data.projection.inverse().into(),
        right_projection_inverse: right_view_data.projection.inverse().into(),
        left_view_inverse: left_view_data.instance.rotation.into(),
        right_view_inverse: right_view_data.instance.rotation.into(),
    };

    queue.write_buffer(
        &skybox_uniform_buffer.0,
        0,
        bytemuck::bytes_of(&skybox_uniforms.as_std140()),
    );

    *culling_params = CullingParams {
        frustum: None,
        bounding_sphere_params: if is_vr {
            BoundingSphereParams::Vr {
                left: BoundingSphereCullingParams::new(
                    left_view_data.view,
                    left_view_data.projection,
                    0.001,
                ),
                right: BoundingSphereCullingParams::new(
                    right_view_data.view,
                    right_view_data.projection,
                    0.001,
                ),
            }
        } else {
            BoundingSphereParams::SingleView(BoundingSphereCullingParams::new(
                left_view_data.view,
                left_view_data.projection,
                0.001,
            ))
        },
    };
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn start_loading_models<T: HttpClient>(
    static_models: Query<(Entity, &ModelUrl), Added<ModelUrl>>,
    animated_models: Query<(Entity, &AnimatedModelUrl), Added<AnimatedModelUrl>>,
    device: Res<Device>,
    queue: Res<Queue>,
    pipelines: Res<Pipelines>,
    bind_group_layouts: Res<BindGroupLayouts>,
    (index_buffer, vertex_buffers, animated_vertex_buffers): (
        Res<IndexBuffer>,
        Res<VertexBuffers>,
        Res<AnimatedVertexBuffers>,
    ),
    texture_settings: Res<textures::Settings>,
    http_client: Res<T>,
    mut commands: Commands,
) {
    let device = &device.0;
    let queue = &queue.0;

    static_models.for_each(|(entity, url)| {
        let url = url.0.clone();
        let vertex_buffers = vertex_buffers.0.clone();
        let animated_vertex_buffers = animated_vertex_buffers.0.clone();
        let index_buffer = index_buffer.0.clone();
        let texture_settings = texture_settings.clone();

        let model_setter = Arc::new(ArcSwapOption::empty());

        commands
            .entity(entity)
            .insert(PendingModel(model_setter.clone()));

        spawn({
            let device = device.clone();
            let queue = queue.clone();
            let bind_group_layouts = bind_group_layouts.0.clone();
            let pipelines = pipelines.0.clone();

            let context = renderer_core::assets::models::Context {
                device,
                queue,
                bind_group_layouts,
                http_client: http_client.clone(),
                index_buffer,
                vertex_buffers,
                animated_vertex_buffers,
                pipelines,
                texture_settings,
            };

            async move {
                let result = renderer_core::assets::models::Model::load(&context, &url).await;

                match result {
                    Ok(model) => {
                        model_setter.store(Some(Arc::new(model)));

                        Ok(())
                    }
                    Err(error) => Err(anyhow::anyhow!(
                        "Got an error while trying to load a model from '{}': {}",
                        url,
                        error
                    )),
                }
            }
        });
    });

    animated_models.for_each(|(entity, url)| {
        let url = url.0.clone();
        let vertex_buffers = vertex_buffers.0.clone();
        let animated_vertex_buffers = animated_vertex_buffers.0.clone();
        let index_buffer = index_buffer.0.clone();
        let texture_settings = texture_settings.clone();

        let model_setter = Arc::new(ArcSwapOption::empty());

        commands
            .entity(entity)
            .insert(PendingAnimatedModel(model_setter.clone()));

        spawn({
            let device = device.clone();
            let queue = queue.clone();
            let bind_group_layouts = bind_group_layouts.0.clone();
            let pipelines = pipelines.0.clone();

            let context = renderer_core::assets::models::Context {
                device,
                queue,
                bind_group_layouts,
                http_client: http_client.clone(),
                index_buffer,
                vertex_buffers,
                animated_vertex_buffers,
                pipelines,
                texture_settings,
            };

            async move {
                let result =
                    renderer_core::assets::models::AnimatedModel::load(&context, &url).await;

                match result {
                    Ok(model) => {
                        model_setter.store(Some(Arc::new(model)));
                        Ok(())
                    }
                    Err(error) => Err(anyhow::anyhow!(
                        "Got an error while trying to load a model from '{}': {}",
                        url,
                        error
                    )),
                }
            }
        });
    });
}

pub(crate) fn finish_loading_models(
    static_models: Query<(Entity, &PendingModel)>,
    animated_models: Query<(Entity, &PendingAnimatedModel)>,
    device: Res<Device>,
    bind_group_layouts: Res<BindGroupLayouts>,
    mut commands: Commands,
) {
    static_models.for_each(|(entity, pending_model)| {
        if let Some(loaded_model) = pending_model.0.swap(None) {
            commands.entity(entity).insert(Model(loaded_model));
        }
    });

    animated_models.for_each(|(entity, pending_model)| {
        if let Some(loaded_model) = pending_model.0.swap(None) {
            commands
                .entity(entity)
                .insert(AnimatedModel(loaded_model))
                .insert(JointBuffers::new(&device.0, &bind_group_layouts.0));
        }
    })
}

pub(crate) fn add_joints_to_instances(
    animated_models: Query<&AnimatedModel>,
    instances: Query<(Entity, &InstanceOf), Without<AnimationJoints>>,
    mut commands: Commands,
) {
    instances.for_each(|(entity, instance_of)| {
        if let Ok(animated_model) = animated_models.get(instance_of.0) {
            commands.entity(entity).insert(AnimationJoints(
                animated_model.0.animation_data.animation_joints.clone(),
            ));
        }
    })
}
