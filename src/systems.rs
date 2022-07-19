use crate::components::{
    AnimatedModel, AnimatedModelUrl, AnimationJoints, AnimationState, Instance, InstanceOf,
    InstanceRange, Instances, JointBuffer, Model, ModelUrl, PendingAnimatedModel, PendingModel,
};
use crate::resources::{
    AnimatedVertexBuffers, BindGroupLayouts, Camera, ClampSampler, CompositeBindGroup, Device,
    IndexBuffer, InstanceBuffer, IntermediateColorFramebuffer, IntermediateDepthFramebuffer,
    LineBuffer, MainBindGroup, NewIblCubemap, Pipelines, Queue, SkyboxUniformBindGroup,
    SkyboxUniformBuffer, SurfaceFrameView, UniformBuffer, VertexBuffers,
};
use bevy_ecs::prelude::{Added, Commands, Entity, Local, Query, Res, ResMut, Without};
use renderer_core::{
    arc_swap::ArcSwap,
    assets::{textures, HttpClient},
    bytemuck, create_main_bind_group,
    crevice::std140::AsStd140,
    ibl::IblTextures,
    shared_structs, spawn,
    utils::{Setter, Swappable},
    GpuInstance, Texture,
};
use std::sync::Arc;

pub(crate) mod rendering;

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

    query.for_each_mut(|mut instances| instances.0.clear());
}

pub(crate) fn clear_joint_buffers(mut query: Query<&mut JointBuffer>) {
    query.for_each_mut(|mut joint_buffer| {
        joint_buffer.staging.clear();
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
                    animation.animate(
                        &mut animation_joints.0,
                        animation_state.time,
                        &animated_model.0.animation_data.depth_first_nodes,
                    );
                }
            }
            Err(error) => {
                log::warn!("Got an error when sampling animations: {}", error);
            }
        }
    })
}

pub(crate) fn upload_joint_buffers(query: Query<&JointBuffer>, queue: Res<Queue>) {
    query.for_each(|joint_buffer| {
        queue.0.write_buffer(
            &joint_buffer.buffer,
            0,
            bytemuck::cast_slice(&joint_buffer.staging),
        );
    })
}

pub(crate) fn push_joints(
    instance_query: Query<(&InstanceOf, &AnimationJoints)>,
    mut model_query: Query<(&AnimatedModel, &mut JointBuffer)>,
) {
    instance_query.for_each(|(instance_of, animation_joints)| {
        match model_query.get_mut(instance_of.0) {
            Ok((animated_model, mut joint_buffer)) => {
                'joint_loop: for joint in animation_joints
                    .0
                    .iter(
                        &animated_model
                            .0
                            .animation_data
                            .joint_indices_to_node_indices,
                        &animated_model.0.animation_data.inverse_bind_transforms,
                    )
                    .map(|joint| {
                        shared_structs::JointTransform::new(
                            joint.translation,
                            joint.scale,
                            joint.rotation,
                        )
                    })
                {
                    if let Err(error) = joint_buffer.staging.try_push(joint) {
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

pub(crate) fn push_debug_joints_to_lines_buffer(
    instance_query: Query<(&InstanceOf, &AnimationJoints, &Instance)>,
    mut model_query: Query<&AnimatedModel>,
    mut line_buffer: ResMut<LineBuffer>,
) {
    instance_query.for_each(|(instance_of, animation_joints, instance)| {
        match model_query.get_mut(instance_of.0) {
            Ok(animated_model) => {
                for (id, (start, end)) in animation_joints
                    .0
                    .iter_lines(&animated_model.0.animation_data.depth_first_nodes)
                    .enumerate()
                {
                    let start =
                        instance.0.position + (instance.0.rotation * instance.0.scale * start);
                    let end = instance.0.position + (instance.0.rotation * instance.0.scale * end);

                    line_buffer.staging.extend_from_slice(&[
                        renderer_core::LineVertex {
                            position: start,
                            colour_id: id as u32,
                        },
                        renderer_core::LineVertex {
                            position: end,
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

// Here would be a good place to do culling.
pub(crate) fn push_entity_instances(
    mut instance_query: Query<(&InstanceOf, &Instance)>,
    mut model_query: Query<(&mut Instances, Option<&AnimatedModel>)>,
) {
    instance_query.for_each_mut(|(instance_of, instance)| {
        match model_query.get_mut(instance_of.0) {
            Ok((mut instances, animated_model)) => {
                let instance_index = instances.0.len() as u32;
                let num_joints = animated_model
                    .map(|animated_model| animated_model.0.num_joints())
                    .unwrap_or(0);

                instances.0.push(GpuInstance {
                    position: instance.0.position,
                    scale: instance.0.scale,
                    rotation: instance.0.rotation,
                    joints_offset: instance_index * num_joints,
                    _padding: Default::default(),
                });
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
    mut query: Query<(&Instances, &mut InstanceRange)>,
) {
    let mut command_encoder = device
        .0
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("command encoder"),
        });

    query.for_each_mut(|(instances, mut instance_range)| {
        instance_range.0 =
            instance_buffer
                .0
                .push(&instances.0, &device.0, &queue.0, &mut command_encoder);
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

pub(crate) fn allocate_bind_groups<T: HttpClient>(
    device: Res<Device>,
    queue: Res<Queue>,
    pipelines: Res<Pipelines>,
    bind_group_layouts: Res<BindGroupLayouts>,
    texture_settings: Res<textures::Settings>,
    http_client: Res<T>,
    mut commands: Commands,
) {
    let device = &device.0;
    let queue = &queue.0;
    let pipelines = &pipelines.0;
    let bind_group_layouts = &bind_group_layouts.0;

    let ibl_textures = Arc::new(IblTextures {
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
        sphere_harmonics: ArcSwap::from(Arc::new(device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("dummy sphere harmonics buffer"),
            size: 144,
            usage: wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: false,
        }))),
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

    let main_bind_group = Swappable::new(create_main_bind_group(
        device,
        &ibl_textures,
        &uniform_buffer,
        &clamp_sampler,
        bind_group_layouts,
    ));

    let main_bind_group_setter = main_bind_group.setter.clone();

    commands.insert_resource(UniformBuffer(uniform_buffer.clone()));
    commands.insert_resource(MainBindGroup(main_bind_group));
    commands.insert_resource(ClampSampler(clamp_sampler.clone()));
    commands.insert_resource(ibl_textures.clone());

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

    spawn(async move {
        let lut_url = url::Url::parse("http://localhost:8000/assets/lut_ggx.png").unwrap();

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
                ibl_textures.lut.store(lut_texture);

                main_bind_group_setter.set(create_main_bind_group(
                    &textures_context.device,
                    &ibl_textures,
                    &uniform_buffer,
                    &clamp_sampler,
                    &textures_context.bind_group_layouts,
                ));
            }
            Err(error) => {
                log::error!("Got an error while trying to load {}: {}", lut_url, error);
            }
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
pub(crate) fn update_ibl_textures<T: HttpClient>(
    device: Res<Device>,
    queue: Res<Queue>,
    pipelines: Res<Pipelines>,
    bind_group_layouts: Res<BindGroupLayouts>,
    texture_settings: Res<textures::Settings>,
    mut new_ibl_cubemap: ResMut<NewIblCubemap>,
    ibl_textures: Res<Arc<IblTextures>>,
    clamp_sampler: Res<ClampSampler>,
    main_bind_group: Res<MainBindGroup>,
    uniform_buffer: Res<UniformBuffer>,
    http_client: Res<T>,
) {
    let new_ibl_cubemap = match new_ibl_cubemap.0.take() {
        Some(new_ibl_cubemap) => new_ibl_cubemap,
        None => return,
    };

    let main_bind_group_setter = main_bind_group.0.setter.clone();

    let device = &device.0;
    let queue = &queue.0;
    let pipelines = &pipelines.0;
    let bind_group_layouts = &bind_group_layouts.0;
    let clamp_sampler = clamp_sampler.0.clone();
    let uniform_buffer = uniform_buffer.0.clone();
    let ibl_textures = ibl_textures.clone();

    let textures_context = renderer_core::assets::textures::Context {
        device: device.clone(),
        queue: queue.clone(),
        http_client: http_client.clone(),
        bind_group_layouts: bind_group_layouts.clone(),
        pipelines: pipelines.clone(),
        settings: texture_settings.clone(),
    };

    spawn(async move {
        match renderer_core::assets::textures::load_ktx2_cubemap(
            textures_context.clone(),
            &new_ibl_cubemap,
        )
        .await
        {
            Ok((specular_cubemap, Some(sphere_harmonics))) => {
                ibl_textures.cubemap.store(specular_cubemap);
                ibl_textures
                    .sphere_harmonics
                    .store(Arc::new(sphere_harmonics));

                main_bind_group_setter.set(create_main_bind_group(
                    &textures_context.device,
                    &ibl_textures,
                    &uniform_buffer,
                    &clamp_sampler,
                    &textures_context.bind_group_layouts,
                ));
            }
            _ => {
                log::error!("Error file loading cubemaps");
            }
        }
    });
}

pub(crate) fn set_desktop_uniform_buffers(
    pipeline_options: Res<renderer_core::PipelineOptions>,
    queue: Res<Queue>,
    uniform_buffer: Res<UniformBuffer>,
    skybox_uniform_buffer: Res<SkyboxUniformBuffer>,
    surface_frame_view: Res<SurfaceFrameView>,
    camera: Res<Camera>,
) {
    let queue = &queue.0;

    use renderer_core::glam::{Mat4, Vec4};

    // Adapted from the functions used in
    // https://crates.io/crates/ultraviolet.
    //
    // Corresponds to `perspective_vk` if `flip` is `true`,
    // `perspective_wgpu_dx` otherwise.
    //
    // todo: wait, why do we need to do this? I thought that wgpu handled viewport conversion
    // for us.
    fn create_perspective_matrix(
        vertical_fov: f32,
        aspect_ratio: f32,
        z_near: f32,
        z_far: f32,
        flip: bool,
    ) -> Mat4 {
        let t = (vertical_fov / 2.0).tan();
        let mut sy = 1.0 / t;
        let sx = sy / aspect_ratio;
        let nmf = z_near - z_far;

        if flip {
            sy = -sy;
        }

        Mat4::from_cols(
            Vec4::new(sx, 0.0, 0.0, 0.0),
            Vec4::new(0.0, sy, 0.0, 0.0),
            Vec4::new(0.0, 0.0, z_far / nmf, -1.0),
            Vec4::new(0.0, 0.0, z_near * z_far / nmf, 0.0),
        )
    }

    let perspective_matrix = create_perspective_matrix(
        59.0_f32.to_radians(),
        surface_frame_view.width as f32 / surface_frame_view.height as f32,
        0.01,
        1000.0,
        cfg!(not(feature = "wasm")),
    );

    let projection_view = perspective_matrix * camera.view_matrix();

    let uniforms = renderer_core::shared_structs::Uniforms {
        left_projection_view: projection_view.into(),
        right_projection_view: projection_view.into(),
        left_eye_position: camera.position,
        right_eye_position: camera.position,
        flip_viewport: false as u32,
        inline_tonemapping: pipeline_options.inline_tonemapping as u32,
        inline_srgb: false as u32,
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

#[cfg(feature = "wasm")]
pub(crate) fn update_uniform_buffers(
    pose: bevy_ecs::prelude::NonSend<web_sys::XrViewerPose>,
    pipeline_options: Res<renderer_core::PipelineOptions>,
    queue: Res<Queue>,
    uniform_buffer: Res<UniformBuffer>,
    skybox_uniform_buffer: Res<SkyboxUniformBuffer>,
) {
    let queue = &queue.0;

    use renderer_core::glam::Mat4;

    let parse_matrix = |vec| Mat4::from_cols_array(&<[f32; 16]>::try_from(vec).unwrap());

    let views = pose.views();

    let mut views_iter = views.iter();

    let left_view: web_sys::XrView = views_iter.next().unwrap().into();

    let left_proj = parse_matrix(left_view.projection_matrix());
    let left_inv = parse_matrix(left_view.transform().matrix()).inverse();

    let left_projection_view: renderer_core::shared_structs::FlatMat4 =
        (left_proj * left_inv).into();

    let left_instance = renderer_core::Instance::from_transform(left_view.transform(), 0.0);

    let (right_projection_view, right_proj, right_instance) = if let Some(right_view) =
        views_iter.next()
    {
        let right_view: web_sys::XrView = right_view.into();

        let right_proj = parse_matrix(right_view.projection_matrix());
        let right_inv = parse_matrix(right_view.transform().matrix()).inverse();

        let right_projection_view: renderer_core::shared_structs::FlatMat4 =
            (right_proj * right_inv).into();

        let right_instance = renderer_core::Instance::from_transform(right_view.transform(), 0.0);

        (right_projection_view, right_proj, right_instance)
    } else {
        Default::default()
    };

    let uniforms = renderer_core::shared_structs::Uniforms {
        left_projection_view,
        right_projection_view,
        left_eye_position: left_instance.position,
        right_eye_position: right_instance.position,
        flip_viewport: pipeline_options.flip_viewport as u32,
        inline_tonemapping: pipeline_options.inline_tonemapping as u32,
        inline_srgb: true as u32,
    };

    queue.write_buffer(
        &uniform_buffer.0,
        0,
        renderer_core::bytemuck::bytes_of(&uniforms.as_std140()),
    );

    let skybox_uniforms = shared_structs::SkyboxUniforms {
        left_projection_inverse: left_proj.inverse().into(),
        right_projection_inverse: right_proj.inverse().into(),
        left_view_inverse: left_instance.rotation.into(),
        right_view_inverse: right_instance.rotation.into(),
    };

    queue.write_buffer(
        &skybox_uniform_buffer.0,
        0,
        bytemuck::bytes_of(&skybox_uniforms.as_std140()),
    );
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

        let model_setter = Setter(Default::default());

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
                    Err(error) => {
                        log::error!(
                            "Got an error while trying to load a model from '{}': {}",
                            url,
                            error
                        );
                    }
                    Ok(model) => {
                        model_setter.set(model);
                    }
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

        let model_setter = Setter(Default::default());

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
                    Err(error) => {
                        log::error!(
                            "Got an error while trying to load a model from '{}': {}",
                            url,
                            error
                        );
                    }
                    Ok(model) => {
                        model_setter.set(model);
                    }
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
        if let Some(mut lock) = pending_model.0 .0.try_lock() {
            if let Some(loaded_model) = lock.take() {
                commands.entity(entity).insert(Model(loaded_model));
            }
        }
    });

    animated_models.for_each(|(entity, pending_model)| {
        if let Some(mut lock) = pending_model.0 .0.try_lock() {
            if let Some(loaded_model) = lock.take() {
                commands
                    .entity(entity)
                    .insert(AnimatedModel(loaded_model))
                    .insert(JointBuffer::new(&device.0, &bind_group_layouts.0));
            }
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
