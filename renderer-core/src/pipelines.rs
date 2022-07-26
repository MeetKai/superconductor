use crate::bind_group_layouts::BindGroupLayouts;
use crate::permutations;

pub struct PipelineOptions {
    pub multiview: Option<std::num::NonZeroU32>,
    pub inline_tonemapping: bool,
    pub framebuffer_format: wgpu::TextureFormat,
    pub flip_viewport: bool,
}

impl PipelineOptions {
    // If we're not doing multiview rendering or a seperate tonemapping pass then we can render
    // meshes directly to the device framebuffer.
    pub fn render_direct_to_framebuffer(&self) -> bool {
        self.multiview.is_none() && self.inline_tonemapping
    }
}

pub struct Pipelines {
    pub pbr: permutations::BlendMode<
        permutations::ModelTypes<permutations::FaceSides<wgpu::RenderPipeline>>,
    >,
    pub stencil_write: wgpu::RenderPipeline,
    pub set_depth: wgpu::RenderPipeline,
    pub tonemap: wgpu::RenderPipeline,
    pub ui: wgpu::RenderPipeline,
    pub skybox: wgpu::RenderPipeline,
    pub skybox_mirrored: wgpu::RenderPipeline,
    pub bc6h_decompression: wgpu::RenderPipeline,
    pub blit: wgpu::RenderPipeline,
    pub srgb_blit: wgpu::RenderPipeline,
    pub line: wgpu::RenderPipeline,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        bind_group_layouts: &BindGroupLayouts,
        options: &PipelineOptions,
    ) -> Self {
        let target_format = if options.inline_tonemapping {
            options.framebuffer_format
        } else {
            wgpu::TextureFormat::Rgba16Float
        };

        let front_face = if options.flip_viewport {
            wgpu::FrontFace::Cw
        } else {
            wgpu::FrontFace::Ccw
        };

        let multiview = options.multiview;

        let uniform_only_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("uniform only pipeline layout"),
                bind_group_layouts: &[&bind_group_layouts.uniform],
                push_constant_ranges: &[],
            });

        let model_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("model pipeline layout"),
                bind_group_layouts: &[&bind_group_layouts.uniform, &bind_group_layouts.model],
                push_constant_ranges: &[],
            });

        let vertex_buffers = &[
            wgpu::VertexBufferLayout {
                array_stride: 3 * 4,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3],
            },
            wgpu::VertexBufferLayout {
                array_stride: 3 * 4,
                attributes: &wgpu::vertex_attr_array![1 => Float32x3],
                step_mode: wgpu::VertexStepMode::Vertex,
            },
            wgpu::VertexBufferLayout {
                array_stride: 2 * 4,
                attributes: &wgpu::vertex_attr_array![2 => Float32x2],
                step_mode: wgpu::VertexStepMode::Vertex,
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<super::GpuInstance>() as u64,
                attributes: &wgpu::vertex_attr_array![3 => Float32x4, 4 => Float32x4, 5 => Uint32],
                step_mode: wgpu::VertexStepMode::Instance,
            },
        ];

        let animated_model_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("animated model pipeline layout"),
                bind_group_layouts: &[
                    &bind_group_layouts.uniform,
                    &bind_group_layouts.model,
                    &bind_group_layouts.joints,
                ],
                push_constant_ranges: &[],
            });

        let animated_vertex_buffers = &[
            wgpu::VertexBufferLayout {
                array_stride: 3 * 4,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3],
            },
            wgpu::VertexBufferLayout {
                array_stride: 3 * 4,
                attributes: &wgpu::vertex_attr_array![1 => Float32x3],
                step_mode: wgpu::VertexStepMode::Vertex,
            },
            wgpu::VertexBufferLayout {
                array_stride: 2 * 4,
                attributes: &wgpu::vertex_attr_array![2 => Float32x2],
                step_mode: wgpu::VertexStepMode::Vertex,
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<super::GpuInstance>() as u64,
                attributes: &wgpu::vertex_attr_array![3 => Float32x4, 4 => Float32x4, 5 => Uint32],
                step_mode: wgpu::VertexStepMode::Instance,
            },
            wgpu::VertexBufferLayout {
                array_stride: 4 * 4,
                attributes: &wgpu::vertex_attr_array![6 => Uint32x4],
                step_mode: wgpu::VertexStepMode::Vertex,
            },
            wgpu::VertexBufferLayout {
                array_stride: 4 * 4,
                attributes: &wgpu::vertex_attr_array![7 => Float32x4],
                step_mode: wgpu::VertexStepMode::Vertex,
            },
        ];

        let line_vertex_buffers = &[wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<super::LineVertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Uint32],
        }];

        let prefix = if multiview.is_none() {
            "single_view::"
        } else {
            ""
        };

        let vertex_state = wgpu::VertexState {
            module: &device.create_shader_module(if multiview.is_none() {
                wgpu::include_spirv!("../../compiled-shaders/single_view_vertex.spv")
            } else {
                wgpu::include_spirv!("../../compiled-shaders/vertex.spv")
            }),
            entry_point: &format!("{}vertex", prefix),
            buffers: vertex_buffers,
        };

        let animated_vertex_state = wgpu::VertexState {
            module: &device.create_shader_module(if multiview.is_none() {
                wgpu::include_spirv!("../../compiled-shaders/single_view_animated_vertex.spv")
            } else {
                wgpu::include_spirv!("../../compiled-shaders/animated_vertex.spv")
            }),
            entry_point: &format!("{}animated_vertex", prefix),
            buffers: animated_vertex_buffers,
        };

        let normal_primitive_state = wgpu::PrimitiveState {
            front_face,
            cull_mode: Some(wgpu::Face::Back),
            ..Default::default()
        };

        let normal_depth_state = wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24PlusStencil8,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            bias: wgpu::DepthBiasState::default(),
            stencil: wgpu::StencilState::default(),
        };

        let fullscreen_tri_vertex_state = wgpu::VertexState {
            module: &device.create_shader_module(wgpu::include_spirv!(
                "../../compiled-shaders/fullscreen_tri.spv"
            )),
            entry_point: "fullscreen_tri",
            buffers: &[],
        };

        let tonemap_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layouts.uniform, &bind_group_layouts.tonemap],
                push_constant_ranges: &[],
            });

        let tonemap_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("tonemap pipeline"),
            layout: Some(&tonemap_pipeline_layout),
            vertex: fullscreen_tri_vertex_state.clone(),
            fragment: Some(wgpu::FragmentState {
                module: &device.create_shader_module(if multiview.is_none() {
                    wgpu::include_spirv!("../../compiled-shaders/single_view_tonemap.spv")
                } else {
                    wgpu::include_spirv!("../../compiled-shaders/tonemap.spv")
                }),
                entry_point: &format!("{}tonemap", prefix),
                targets: &[Some(options.framebuffer_format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: Default::default(),
        });

        let flat_blue_test_fragment_shader = device
            .create_shader_module(wgpu::include_spirv!("../../compiled-shaders/flat_blue.spv"));

        let stencil_write = wgpu::StencilFaceState {
            compare: wgpu::CompareFunction::Always,
            fail_op: wgpu::StencilOperation::Keep,
            depth_fail_op: wgpu::StencilOperation::Keep,
            pass_op: wgpu::StencilOperation::IncrementClamp,
        };

        let stencil_write_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("stencil write pipeline"),
                layout: Some(&uniform_only_pipeline_layout),
                vertex: vertex_state.clone(),

                fragment: Some(wgpu::FragmentState {
                    module: &flat_blue_test_fragment_shader,
                    entry_point: "flat_blue",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: target_format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::empty(),
                    })],
                }),
                primitive: normal_primitive_state,
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth24PlusStencil8,
                    depth_write_enabled: false,
                    depth_compare: wgpu::CompareFunction::Always,
                    bias: wgpu::DepthBiasState::default(),
                    stencil: wgpu::StencilState {
                        front: stencil_write,
                        back: stencil_write,
                        read_mask: 0xff,
                        write_mask: 0xff,
                    },
                }),
                multisample: Default::default(),
                multiview,
            });

        let set_depth_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("set depth pipeline"),
            layout: Some(&uniform_only_pipeline_layout),
            vertex: vertex_state.clone(),

            fragment: Some(wgpu::FragmentState {
                module: &flat_blue_test_fragment_shader,
                entry_point: "flat_blue",
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::empty(),
                })],
            }),
            primitive: normal_primitive_state,
            depth_stencil: Some(normal_depth_state),
            multisample: Default::default(),
            multiview,
        });

        let normal_primitive_state = wgpu::PrimitiveState {
            front_face,
            cull_mode: Some(wgpu::Face::Back),
            ..Default::default()
        };

        let always_depth_stencil_state = wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24PlusStencil8,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Always,
            bias: wgpu::DepthBiasState::default(),
            stencil: wgpu::StencilState::default(),
        };

        let ui_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ui pipeline layout"),
            bind_group_layouts: &[&bind_group_layouts.uniform, &bind_group_layouts.ui_texture],
            push_constant_ranges: &[],
        });

        let skybox_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("skybox pipeline layout"),
                bind_group_layouts: &[&bind_group_layouts.uniform, &bind_group_layouts.skybox],
                push_constant_ranges: &[],
            });

        let skybox_mirrored_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("skybox mirrored pipeline layout"),
                bind_group_layouts: &[
                    &bind_group_layouts.uniform,
                    &bind_group_layouts.skybox,
                    &bind_group_layouts.mirror_uniform,
                ],
                push_constant_ranges: &[],
            });

        let fragment_shader = device.create_shader_module(if multiview.is_none() {
            wgpu::include_spirv!("../../compiled-shaders/single_view_fragment.spv")
        } else {
            wgpu::include_spirv!("../../compiled-shaders/fragment.spv")
        });

        let fragment_opaque = wgpu::FragmentState {
            module: &fragment_shader,
            entry_point: &format!("{}fragment", prefix),
            targets: &[Some(target_format.into())],
        };

        let fragment_alpha_clipped = wgpu::FragmentState {
            module: &device.create_shader_module(if multiview.is_none() {
                wgpu::include_spirv!(
                    "../../compiled-shaders/single_view_fragment_alpha_clipped.spv"
                )
            } else {
                wgpu::include_spirv!("../../compiled-shaders/fragment_alpha_clipped.spv")
            }),
            entry_point: &format!("{}fragment_alpha_clipped", prefix),
            targets: &[Some(target_format.into())],
        };

        let fragment_alpha_blend = wgpu::FragmentState {
            module: &fragment_shader,
            entry_point: &format!("{}fragment", prefix),
            targets: &[Some(wgpu::ColorTargetState {
                format: target_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: Default::default(),
            })],
        };

        let bc6h_decompression_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("bc6h decompression pipeline layout"),
                bind_group_layouts: &[&bind_group_layouts.uint_texture],
                push_constant_ranges: &[],
            });

        let bc6h_decompression_target = wgpu::TextureFormat::Rg11b10Float;

        let blit_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layouts.sampled_texture],
            push_constant_ranges: &[],
        });

        let blit_fragment_shader =
            device.create_shader_module(wgpu::include_spirv!("../../compiled-shaders/blit.spv"));

        let skybox_fragment_shader = device.create_shader_module(wgpu::include_spirv!(
            "../../compiled-shaders/fragment_skybox.spv"
        ));

        let normal_depth_state = wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24PlusStencil8,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            bias: wgpu::DepthBiasState::default(),
            stencil: wgpu::StencilState::default(),
        };

        let read_only_depth_state = wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24PlusStencil8,
            depth_write_enabled: false,
            depth_compare: wgpu::CompareFunction::Less,
            bias: wgpu::DepthBiasState::default(),
            stencil: wgpu::StencilState::default(),
        };

        let backface_culling_primitive_state = wgpu::PrimitiveState {
            front_face,
            cull_mode: Some(wgpu::Face::Back),
            ..Default::default()
        };

        let double_sided_primitive_state = wgpu::PrimitiveState {
            front_face,
            cull_mode: None,
            ..Default::default()
        };

        Self {
            pbr: permutations::BlendMode {
                opaque: permutations::ModelTypes {
                    stationary: permutations::FaceSides {
                        single: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("opaque stationary single-sided pipeline"),
                            layout: Some(&model_pipeline_layout),
                            vertex: vertex_state.clone(),
                            fragment: Some(fragment_opaque.clone()),
                            primitive: backface_culling_primitive_state,
                            depth_stencil: Some(normal_depth_state.clone()),
                            multisample: Default::default(),
                            multiview,
                        }),
                        double: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("opaque stationary double-sided pipeline"),
                            layout: Some(&model_pipeline_layout),
                            vertex: vertex_state.clone(),
                            fragment: Some(fragment_opaque.clone()),
                            primitive: double_sided_primitive_state,
                            depth_stencil: Some(normal_depth_state.clone()),
                            multisample: Default::default(),
                            multiview,
                        }),
                    },
                    animated: permutations::FaceSides {
                        single: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("opaque animated single-sided pipeline"),
                            layout: Some(&animated_model_pipeline_layout),
                            vertex: animated_vertex_state.clone(),
                            fragment: Some(fragment_opaque.clone()),
                            primitive: backface_culling_primitive_state,
                            depth_stencil: Some(normal_depth_state.clone()),
                            multisample: Default::default(),
                            multiview,
                        }),
                        double: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("opaque animated double-sided pipeline"),
                            layout: Some(&animated_model_pipeline_layout),
                            vertex: animated_vertex_state.clone(),
                            fragment: Some(fragment_opaque.clone()),
                            primitive: double_sided_primitive_state,
                            depth_stencil: Some(normal_depth_state.clone()),
                            multisample: Default::default(),
                            multiview,
                        }),
                    },
                },
                alpha_clipped: permutations::ModelTypes {
                    stationary: permutations::FaceSides {
                        single: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("alpha clipped stationary single-sided pipeline"),
                            layout: Some(&model_pipeline_layout),
                            vertex: vertex_state.clone(),
                            fragment: Some(fragment_alpha_clipped.clone()),
                            primitive: backface_culling_primitive_state,
                            depth_stencil: Some(normal_depth_state.clone()),
                            multisample: Default::default(),
                            multiview,
                        }),
                        double: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("alpha clipped stationary double-sided pipeline"),
                            layout: Some(&model_pipeline_layout),
                            vertex: vertex_state.clone(),
                            fragment: Some(fragment_alpha_clipped.clone()),
                            primitive: double_sided_primitive_state,
                            depth_stencil: Some(normal_depth_state.clone()),
                            multisample: Default::default(),
                            multiview,
                        }),
                    },
                    animated: permutations::FaceSides {
                        single: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("alpha clipped animated single-sided pipeline"),
                            layout: Some(&animated_model_pipeline_layout),
                            vertex: animated_vertex_state.clone(),
                            fragment: Some(fragment_alpha_clipped.clone()),
                            primitive: backface_culling_primitive_state,
                            depth_stencil: Some(normal_depth_state.clone()),
                            multisample: Default::default(),
                            multiview,
                        }),
                        double: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("alpha clipped animated double-sided pipeline"),
                            layout: Some(&animated_model_pipeline_layout),
                            vertex: animated_vertex_state.clone(),
                            fragment: Some(fragment_alpha_clipped.clone()),
                            primitive: double_sided_primitive_state,
                            depth_stencil: Some(normal_depth_state.clone()),
                            multisample: Default::default(),
                            multiview,
                        }),
                    },
                },
                alpha_blended: permutations::ModelTypes {
                    stationary: permutations::FaceSides {
                        single: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("alpha blended stationary single-sided pipeline"),
                            layout: Some(&model_pipeline_layout),
                            vertex: vertex_state.clone(),
                            fragment: Some(fragment_alpha_blend.clone()),
                            primitive: backface_culling_primitive_state,
                            depth_stencil: Some(read_only_depth_state.clone()),
                            multisample: Default::default(),
                            multiview,
                        }),
                        double: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("alpha blended stationary double-sided pipeline"),
                            layout: Some(&model_pipeline_layout),
                            vertex: vertex_state.clone(),
                            fragment: Some(fragment_alpha_blend.clone()),
                            primitive: double_sided_primitive_state,
                            depth_stencil: Some(read_only_depth_state.clone()),
                            multisample: Default::default(),
                            multiview,
                        }),
                    },
                    animated: permutations::FaceSides {
                        single: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("alpha blended animated single-sided pipeline"),
                            layout: Some(&animated_model_pipeline_layout),
                            vertex: animated_vertex_state.clone(),
                            fragment: Some(fragment_alpha_blend.clone()),
                            primitive: backface_culling_primitive_state,
                            depth_stencil: Some(read_only_depth_state.clone()),
                            multisample: Default::default(),
                            multiview,
                        }),
                        double: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("alpha blended animated double-sided pipeline"),
                            layout: Some(&animated_model_pipeline_layout),
                            vertex: animated_vertex_state.clone(),
                            fragment: Some(fragment_alpha_blend.clone()),
                            primitive: double_sided_primitive_state,
                            depth_stencil: Some(read_only_depth_state),
                            multisample: Default::default(),
                            multiview,
                        }),
                    },
                },
            },
            stencil_write: stencil_write_pipeline,
            set_depth: set_depth_pipeline,
            tonemap: tonemap_pipeline,
            ui: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("ui pipeline"),
                layout: Some(&ui_pipeline_layout),
                vertex: vertex_state.clone(),
                fragment: Some(wgpu::FragmentState {
                    module: &device.create_shader_module(wgpu::include_spirv!(
                        "../../compiled-shaders/fragment_ui.spv"
                    )),
                    entry_point: "fragment_ui",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: target_format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: normal_primitive_state,
                depth_stencil: Some(normal_depth_state),
                multisample: Default::default(),
                multiview,
            }),
            skybox: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("skybox pipeline"),
                layout: Some(&skybox_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &device.create_shader_module(if multiview.is_none() {
                        wgpu::include_spirv!("../../compiled-shaders/single_view_vertex_skybox.spv")
                    } else {
                        wgpu::include_spirv!("../../compiled-shaders/vertex_skybox.spv")
                    }),
                    entry_point: &format!("{}vertex_skybox", prefix),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &skybox_fragment_shader,
                    entry_point: "fragment_skybox",
                    targets: &[Some(target_format.into())],
                }),
                primitive: Default::default(),
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth24PlusStencil8,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    bias: wgpu::DepthBiasState::default(),
                    stencil: wgpu::StencilState::default(),
                }),
                multisample: Default::default(),
                multiview,
            }),
            skybox_mirrored: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("skybox mirrored pipeline"),
                layout: Some(&skybox_mirrored_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &device.create_shader_module(if multiview.is_none() {
                        wgpu::include_spirv!(
                            "../../compiled-shaders/single_view_vertex_skybox_mirrored.spv"
                        )
                    } else {
                        wgpu::include_spirv!("../../compiled-shaders/vertex_skybox_mirrored.spv")
                    }),
                    entry_point: &format!("{}vertex_skybox_mirrored", prefix),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &skybox_fragment_shader,
                    entry_point: "fragment_skybox",
                    targets: &[Some(target_format.into())],
                }),
                primitive: Default::default(),
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth24PlusStencil8,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    bias: wgpu::DepthBiasState::default(),
                    stencil: wgpu::StencilState::default(),
                }),
                multisample: Default::default(),
                multiview,
            }),
            bc6h_decompression: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&bc6h_decompression_pipeline_layout),
                vertex: fullscreen_tri_vertex_state.clone(),
                fragment: Some(wgpu::FragmentState {
                    module: &device.create_shader_module(wgpu::include_spirv!(
                        "../../compiled-shaders/bc6.spv"
                    )),
                    entry_point: "main",
                    targets: &[Some(bc6h_decompression_target.into())],
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: Default::default(),
            }),
            blit: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("blit pipeline"),
                layout: Some(&blit_pipeline_layout),
                vertex: fullscreen_tri_vertex_state.clone(),
                fragment: Some(wgpu::FragmentState {
                    module: &blit_fragment_shader,
                    entry_point: "blit",
                    targets: &[Some(wgpu::TextureFormat::Rgba8Unorm.into())],
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: Default::default(),
            }),
            srgb_blit: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("srgb blit pipeline"),
                layout: Some(&blit_pipeline_layout),
                vertex: fullscreen_tri_vertex_state.clone(),
                fragment: Some(wgpu::FragmentState {
                    module: &blit_fragment_shader,
                    entry_point: "blit",
                    targets: &[Some(wgpu::TextureFormat::Rgba8UnormSrgb.into())],
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: Default::default(),
            }),
            line: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("line pipeline"),
                layout: Some(&uniform_only_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &device.create_shader_module(if multiview.is_none() {
                        wgpu::include_spirv!("../../compiled-shaders/single_view_line_vertex.spv")
                    } else {
                        wgpu::include_spirv!("../../compiled-shaders/line_vertex.spv")
                    }),
                    entry_point: &format!("{}line_vertex", prefix),
                    buffers: line_vertex_buffers,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &device.create_shader_module(wgpu::include_spirv!(
                        "../../compiled-shaders/flat_colour.spv"
                    )),
                    entry_point: "flat_colour",
                    targets: &[Some(target_format.into())],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::LineList,
                    ..Default::default()
                },
                depth_stencil: Some(always_depth_stencil_state),
                multisample: Default::default(),
                multiview,
            }),
        }
    }
}
