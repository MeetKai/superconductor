use crate::bind_group_layouts::BindGroupLayouts;
use crate::permutations;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
// `Rg11b10Float` isn't supported in WebGPU yet.
#[cfg(all(feature = "wasm", not(feature = "webgl")))]
pub const BC6H_DECOMPRESSION_TARGET_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
#[cfg(not(all(feature = "wasm", not(feature = "webgl"))))]
pub const BC6H_DECOMPRESSION_TARGET_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rg11b10Float;

pub struct PipelineOptions {
    pub multiview: Option<std::num::NonZeroU32>,
    pub inline_tonemapping: bool,
    pub framebuffer_format: wgpu::TextureFormat,
    pub flip_viewport: bool,
    pub depth_prepass: bool,
    pub reverse_z: bool,
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
    pub opaque_depth_prepass: permutations::FaceSides<wgpu::RenderPipeline>,
    pub tonemap: wgpu::RenderPipeline,
    pub skybox: wgpu::RenderPipeline,
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

        let front_face = if false {//options.flip_viewport {
            wgpu::FrontFace::Cw
        } else {
            wgpu::FrontFace::Ccw
        };

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
            // instance
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<super::GpuInstance>() as u64,
                attributes: &wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4, 2 => Uint32, 3 => Uint32, 4 => Uint32],
                step_mode: wgpu::VertexStepMode::Instance,
            },
            // position, normal, uv, lightmap uv
            wgpu::VertexBufferLayout {
                array_stride: 3 * 4,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![5 => Float32x3],
            },
            wgpu::VertexBufferLayout {
                array_stride: 3 * 4,
                attributes: &wgpu::vertex_attr_array![6 => Float32x3],
                step_mode: wgpu::VertexStepMode::Vertex,
            },
            wgpu::VertexBufferLayout {
                array_stride: 2 * 4,
                attributes: &wgpu::vertex_attr_array![7 => Float32x2],
                step_mode: wgpu::VertexStepMode::Vertex,
            },
            wgpu::VertexBufferLayout {
                array_stride: 2 * 4,
                attributes: &wgpu::vertex_attr_array![8 => Float32x2],
                step_mode: wgpu::VertexStepMode::Vertex,
            },
        ];

        let stationary_depth_prepass_vertex_buffers = &[
            wgpu::VertexBufferLayout {
                array_stride: 3 * 4,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<super::GpuInstance>() as u64,
                attributes: &wgpu::vertex_attr_array![1 => Float32x4, 2 => Float32x4, 3 => Uint32, 4 => Uint32],
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
            // instance
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<super::GpuInstance>() as u64,
                attributes: &wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4, 2 => Uint32, 3 => Uint32, 4 => Uint32],
                step_mode: wgpu::VertexStepMode::Instance,
            },
            // position, normal, uv, joint indices, joint weights
            wgpu::VertexBufferLayout {
                array_stride: 3 * 4,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![5 => Float32x3],
            },
            wgpu::VertexBufferLayout {
                array_stride: 3 * 4,
                attributes: &wgpu::vertex_attr_array![6 => Float32x3],
                step_mode: wgpu::VertexStepMode::Vertex,
            },
            wgpu::VertexBufferLayout {
                array_stride: 2 * 4,
                attributes: &wgpu::vertex_attr_array![7 => Float32x2],
                step_mode: wgpu::VertexStepMode::Vertex,
            },
            // joint indices
            wgpu::VertexBufferLayout {
                array_stride: 4 * 4,
                attributes: &wgpu::vertex_attr_array![8 => Uint32x4],
                step_mode: wgpu::VertexStepMode::Vertex,
            },
            // joint weights
            wgpu::VertexBufferLayout {
                array_stride: 4 * 4,
                attributes: &wgpu::vertex_attr_array![9 => Float32x4],
                step_mode: wgpu::VertexStepMode::Vertex,
            },
        ];

        let line_vertex_buffers = &[wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<super::LineVertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Uint32],
        }];

        let prefix = if options.multiview.is_none() {
            "single_view__"
        } else {
            ""
        };

        let vertex_state = wgpu::VertexState {
            module: &device.create_shader_module(if options.multiview.is_none() {
                wgpu::include_spirv!("../../compiled-shaders/single_view_vertex.spv")
            } else {
                wgpu::include_spirv!("../../compiled-shaders/vertex.spv")
            }),
            entry_point: &format!("{}vertex", prefix),
            buffers: vertex_buffers,
        };

        let stationary_depth_prepass_vertex_state = wgpu::VertexState {
            module: &device.create_shader_module(if options.multiview.is_none() {
                wgpu::include_spirv!("../../compiled-shaders/single_view_depth_prepass_vertex.spv")
            } else {
                wgpu::include_spirv!("../../compiled-shaders/depth_prepass_vertex.spv")
            }),
            entry_point: &format!("{}depth_prepass_vertex", prefix),
            buffers: stationary_depth_prepass_vertex_buffers,
        };

        let animated_vertex_state = wgpu::VertexState {
            module: &device.create_shader_module(if options.multiview.is_none() {
                wgpu::include_spirv!("../../compiled-shaders/single_view_animated_vertex.spv")
            } else {
                wgpu::include_spirv!("../../compiled-shaders/animated_vertex.spv")
            }),
            entry_point: &format!("{}animated_vertex", prefix),
            buffers: animated_vertex_buffers,
        };

        let normal_depth_state = wgpu::DepthStencilState {
            format: DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: if options.reverse_z {
                wgpu::CompareFunction::Greater
            } else {
                wgpu::CompareFunction::Less
            },
            bias: wgpu::DepthBiasState::default(),
            stencil: wgpu::StencilState::default(),
        };

        let eq_depth_state = wgpu::DepthStencilState {
            format: DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Equal,
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
                module: &device.create_shader_module(if options.multiview.is_none() {
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

        let always_depth_stencil_state = wgpu::DepthStencilState {
            format: DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Always,
            bias: wgpu::DepthBiasState::default(),
            stencil: wgpu::StencilState::default(),
        };

        let fragment_opaque = wgpu::FragmentState {
            module: &device.create_shader_module(if options.multiview.is_none() {
                wgpu::include_spirv!("../../compiled-shaders/single_view_fragment.spv")
            } else {
                wgpu::include_spirv!("../../compiled-shaders/fragment.spv")
            }),
            entry_point: &format!("{}fragment", prefix),
            targets: &[Some(target_format.into())],
        };

        let fragment_alpha_clipped = wgpu::FragmentState {
            module: &device.create_shader_module(if options.multiview.is_none() {
                wgpu::include_spirv!(
                    "../../compiled-shaders/single_view_fragment_alpha_clipped.spv"
                )
            } else {
                wgpu::include_spirv!("../../compiled-shaders/fragment_alpha_clipped.spv")
            }),
            entry_point: &format!("{}fragment_alpha_clipped", prefix),
            targets: &[Some(target_format.into())],
        };

        let fragment_alpha_blended = wgpu::FragmentState {
            module: &device.create_shader_module(if options.multiview.is_none() {
                wgpu::include_spirv!(
                    "../../compiled-shaders/single_view_fragment_alpha_blended.spv"
                )
            } else {
                wgpu::include_spirv!("../../compiled-shaders/fragment_alpha_blended.spv")
            }),
            entry_point: &format!("{}fragment_alpha_blended", prefix),
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
                            depth_stencil: Some(if options.depth_prepass {
                                eq_depth_state.clone()
                            } else {
                                normal_depth_state.clone()
                            }),
                            multisample: Default::default(),
                            multiview: options.multiview,
                        }),
                        double: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("opaque stationary double-sided pipeline"),
                            layout: Some(&model_pipeline_layout),
                            vertex: vertex_state.clone(),
                            fragment: Some(fragment_opaque.clone()),
                            primitive: double_sided_primitive_state,
                            depth_stencil: Some(if options.depth_prepass {
                                eq_depth_state
                            } else {
                                normal_depth_state.clone()
                            }),
                            multisample: Default::default(),
                            multiview: options.multiview,
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
                            multiview: options.multiview,
                        }),
                        double: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("opaque animated double-sided pipeline"),
                            layout: Some(&animated_model_pipeline_layout),
                            vertex: animated_vertex_state.clone(),
                            fragment: Some(fragment_opaque.clone()),
                            primitive: double_sided_primitive_state,
                            depth_stencil: Some(normal_depth_state.clone()),
                            multisample: Default::default(),
                            multiview: options.multiview,
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
                            multiview: options.multiview,
                        }),
                        double: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("alpha clipped stationary double-sided pipeline"),
                            layout: Some(&model_pipeline_layout),
                            vertex: vertex_state.clone(),
                            fragment: Some(fragment_alpha_clipped.clone()),
                            primitive: double_sided_primitive_state,
                            depth_stencil: Some(normal_depth_state.clone()),
                            multisample: Default::default(),
                            multiview: options.multiview,
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
                            multiview: options.multiview,
                        }),
                        double: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("alpha clipped animated double-sided pipeline"),
                            layout: Some(&animated_model_pipeline_layout),
                            vertex: animated_vertex_state.clone(),
                            fragment: Some(fragment_alpha_clipped.clone()),
                            primitive: double_sided_primitive_state,
                            depth_stencil: Some(normal_depth_state.clone()),
                            multisample: Default::default(),
                            multiview: options.multiview,
                        }),
                    },
                },
                alpha_blended: permutations::ModelTypes {
                    stationary: permutations::FaceSides {
                        single: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("alpha blended stationary single-sided pipeline"),
                            layout: Some(&model_pipeline_layout),
                            vertex: vertex_state.clone(),
                            fragment: Some(fragment_alpha_blended.clone()),
                            primitive: backface_culling_primitive_state,
                            depth_stencil: Some(normal_depth_state.clone()),
                            multisample: Default::default(),
                            multiview: options.multiview,
                        }),
                        double: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("alpha blended stationary double-sided pipeline"),
                            layout: Some(&model_pipeline_layout),
                            vertex: vertex_state.clone(),
                            fragment: Some(fragment_alpha_blended.clone()),
                            primitive: double_sided_primitive_state,
                            depth_stencil: Some(normal_depth_state.clone()),
                            multisample: Default::default(),
                            multiview: options.multiview,
                        }),
                    },
                    animated: permutations::FaceSides {
                        single: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("alpha blended animated single-sided pipeline"),
                            layout: Some(&animated_model_pipeline_layout),
                            vertex: animated_vertex_state.clone(),
                            fragment: Some(fragment_alpha_blended.clone()),
                            primitive: backface_culling_primitive_state,
                            depth_stencil: Some(normal_depth_state.clone()),
                            multisample: Default::default(),
                            multiview: options.multiview,
                        }),
                        double: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("alpha blended animated double-sided pipeline"),
                            layout: Some(&animated_model_pipeline_layout),
                            vertex: animated_vertex_state.clone(),
                            fragment: Some(fragment_alpha_blended.clone()),
                            primitive: double_sided_primitive_state,
                            depth_stencil: Some(normal_depth_state.clone()),
                            multisample: Default::default(),
                            multiview: options.multiview,
                        }),
                    },
                },
            },
            opaque_depth_prepass: permutations::FaceSides {
                single: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("opaque stationary depth-prepass single-sided pipeline"),
                    layout: Some(&uniform_only_pipeline_layout),
                    vertex: stationary_depth_prepass_vertex_state.clone(),
                    fragment: None,
                    primitive: backface_culling_primitive_state,
                    depth_stencil: Some(normal_depth_state.clone()),
                    multisample: Default::default(),
                    multiview: options.multiview,
                }),
                double: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("opaque stationary depth-prepass double-sided pipeline"),
                    layout: Some(&uniform_only_pipeline_layout),
                    vertex: stationary_depth_prepass_vertex_state.clone(),
                    fragment: None,
                    primitive: double_sided_primitive_state,
                    depth_stencil: Some(normal_depth_state),
                    multisample: Default::default(),
                    multiview: options.multiview,
                }),
            },
            tonemap: tonemap_pipeline,
            skybox: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("skybox pipeline"),
                layout: Some(&uniform_only_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &device.create_shader_module(if options.multiview.is_none() {
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
                    format: DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: if options.reverse_z {
                        wgpu::CompareFunction::GreaterEqual
                    } else {
                        wgpu::CompareFunction::LessEqual
                    },
                    bias: wgpu::DepthBiasState::default(),
                    stencil: wgpu::StencilState::default(),
                }),
                multisample: Default::default(),
                multiview: options.multiview,
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
                    targets: &[Some(BC6H_DECOMPRESSION_TARGET_FORMAT.into())],
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
                    module: &device.create_shader_module(if options.multiview.is_none() {
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
                multiview: options.multiview,
            }),
        }
    }
}
