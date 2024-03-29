use bevy_app::{App, Plugin};
use bevy_ecs::schedule::{IntoSystemConfigs, SystemSet};
use std::ops::Range;
use std::sync::Arc;
use winit::{
    event,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub mod components;
pub mod resources;
mod systems;

pub use anyhow;
pub use bevy_app;
pub use bevy_ecs;
pub use renderer_core;
pub use url;
pub use wgpu;
pub use winit;

pub use renderer_core::{
    assets::{self, textures},
    culling::CullingFrustum,
    glam::Vec3,
};

use resources::{
    Camera, CullingParams, Device, EventQueue, HttpClient, NewIblCubemap, NewLightvolTextures,
    PipelineOptions, ProbesArrayInfo, Queue, SurfaceFrameView, TextureSettings, WindowChanges,
};

#[derive(SystemSet, Debug, PartialEq, Eq, Clone, Hash)]
pub enum StartupStage {
    PipelineCreation,
    BindGroupCreation,
}

#[derive(SystemSet, Debug, PartialEq, Eq, Clone, Hash)]
pub enum Stage {
    AssetLoading,
    BufferResetting,
    InstanceBuffering,
    BufferUploading,
    Rendering,
}

pub struct XrPlugin<T: assets::HttpClient = SimpleHttpClient> {
    pub mode: Mode,
    pub http_client: T,
}

impl<T: assets::HttpClient + Default> XrPlugin<T> {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            http_client: T::default(),
        }
    }
}

impl<T: assets::HttpClient> Plugin for XrPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(Camera::default());
        app.insert_resource(EventQueue(Default::default()));
        app.insert_resource(TextureSettings(textures::Settings {
            anisotropy_clamp: 16,
        }));
        app.insert_resource(NewIblCubemap(None));
        app.insert_resource(WindowChanges::default());
        app.insert_resource(HttpClient(self.http_client.clone()));
        app.insert_resource(CullingParams::default());
        app.insert_resource(ProbesArrayInfo::new(Vec3::ZERO, Vec3::ONE));
        app.insert_resource(NewLightvolTextures(None));

        app.add_systems(
            bevy_app::Startup,
            systems::create_bind_group_layouts_and_pipelines,
        );

        app.add_systems(bevy_app::PostStartup, systems::allocate_bind_groups);

        app.add_systems(
            bevy_app::Update,
            (
                systems::start_loading_models::<T>,
                systems::finish_loading_models,
                systems::update_ibl_resources::<T>,
                systems::update_lightvol_textures::<T>,
                systems::add_joints_to_instances,
            )
                .in_set(Stage::AssetLoading),
        );

        app.add_systems(
            bevy_app::Update,
            (
                systems::clear_instance_buffers,
                systems::clear_joint_buffers,
                systems::sample_animations,
                systems::clear_line_buffer,
                systems::clear_particle_buffer,
            )
                .in_set(Stage::BufferResetting)
                .after(Stage::AssetLoading),
        );

        match self.mode {
            Mode::Desktop => {
                app.add_systems(
                    bevy_app::Update,
                    systems::update_desktop_uniform_buffers
                        .in_set(Stage::BufferResetting)
                        .after(Stage::AssetLoading),
                );
            }
            #[cfg(feature = "webgl")]
            _ => {
                app.add_systems(
                    bevy_app::Update,
                    systems::update_webxr_uniform_buffers
                        .in_set(Stage::BufferResetting)
                        .after(Stage::AssetLoading),
                );
            }
        };

        app.add_systems(
            bevy_app::Update,
            (
                systems::push_entity_instances,
                systems::push_joints,
                // For debugging joints
                //systems::push_debug_joints_to_lines_buffer,
                //systems::push_debug_bounding_boxes_to_lines_buffer,
                // for debugging particles
                //systems::debugging::push_test_particles,
            )
                .in_set(Stage::InstanceBuffering)
                .after(Stage::BufferResetting),
        );

        app.add_systems(
            bevy_app::Update,
            (
                systems::upload_instances,
                systems::upload_joint_buffers,
                systems::progress_animation_times,
                systems::upload_lines,
                systems::upload_particles,
            )
                .in_set(Stage::BufferUploading)
                .after(Stage::InstanceBuffering),
        );

        match self.mode {
            Mode::Desktop => app.add_systems(
                bevy_app::Update,
                systems::rendering::render_desktop
                    .in_set(Stage::Rendering)
                    .after(Stage::BufferUploading),
            ),
            #[cfg(feature = "webgl")]
            _ => app.add_systems(
                bevy_app::Update,
                systems::rendering::render_webxr
                    .in_set(Stage::Rendering)
                    .after(Stage::BufferUploading),
            ),
        };
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    #[cfg(feature = "webgl")]
    Vr,
    #[cfg(feature = "webgl")]
    Ar,
    Desktop,
}

pub enum ModeSpecificState {
    #[cfg(feature = "webgl")]
    Xr {
        session: web_sys::XrSession,
        reference_space: web_sys::XrReferenceSpace,
    },
    Desktop {
        window: Window,
        event_loop: EventLoop<()>,
    },
}

pub struct InitialisedState {
    pub mode_specific: ModeSpecificState,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    pipeline_options: renderer_core::PipelineOptions,
}

pub async fn initialise(mode: Mode) -> InitialisedState {
    match mode {
        #[cfg(feature = "webgl")]
        Mode::Vr => initialise_xr(web_sys::XrSessionMode::ImmersiveVr).await,
        #[cfg(feature = "webgl")]
        Mode::Ar => initialise_xr(web_sys::XrSessionMode::ImmersiveAr).await,
        Mode::Desktop => initialise_desktop().await,
    }
}

#[cfg(feature = "webgl")]
pub async fn initialise_xr(xr_mode: web_sys::XrSessionMode) -> InitialisedState {
    let canvas = renderer_core::Canvas::default();
    let webgl2_context =
        canvas.create_webgl2_context(renderer_core::ContextCreationOptions { stencil: true });

    let navigator = web_sys::window().unwrap().navigator();
    let xr = navigator.xr();

    let required_features = js_sys::Array::of1(&"local-floor".into());

    let xr_session: web_sys::XrSession =
        wasm_bindgen_futures::JsFuture::from(xr.request_session_with_options(
            xr_mode,
            web_sys::XrSessionInit::new().required_features(&required_features),
        ))
        .await
        .unwrap()
        .into();

    let mut layer_init = web_sys::XrWebGlLayerInit::new();

    let pipeline_options = if xr_mode == web_sys::XrSessionMode::ImmersiveVr {
        renderer_core::PipelineOptions {
            multiview: Some(std::num::NonZeroU32::new(2).unwrap()),
            inline_tonemapping: true,
            framebuffer_format: wgpu::TextureFormat::Rgba8Unorm,
            // As we're doing multiview.
            flip_viewport: false,
            depth_prepass: false,
            reverse_z: false,
        }
    } else {
        renderer_core::PipelineOptions {
            multiview: None,
            inline_tonemapping: true,
            framebuffer_format: wgpu::TextureFormat::Rgba8Unorm,
            // As we're rendering directly to the framebuffer.
            flip_viewport: true,
            depth_prepass: false,
            reverse_z: false,
        }
    };

    layer_init
        .depth(pipeline_options.render_direct_to_framebuffer())
        .stencil(pipeline_options.render_direct_to_framebuffer());

    let backends = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        ..Default::default()
    });
    let surface = unsafe { instance.create_surface(&canvas) }.unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("No suitable GPU adapters found on the system!");

    let adapter_info = adapter.get_info();
    log::info!(
        "Using '{}' with the {:?} backend. Downlevel capabilities: {:?}",
        adapter_info.name,
        adapter_info.backend,
        adapter.get_downlevel_capabilities()
    );

    log::error!("Supported features: {:?}", adapter.features());

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("device"),
                features: adapter.features(),
                limits: adapter.limits(),
            },
            None,
        )
        .await
        .expect("Unable to find a suitable GPU adapter!");

    wasm_bindgen_futures::JsFuture::from(webgl2_context.make_xr_compatible())
        .await
        .expect("Failed to make the webgl context xr-compatible");

    let xr_gl_layer = web_sys::XrWebGlLayer::new_with_web_gl2_rendering_context_and_layer_init(
        &xr_session,
        &webgl2_context,
        &layer_init,
    )
    .unwrap();

    let mut render_state_init = web_sys::XrRenderStateInit::new();
    render_state_init
        .depth_near(0.001)
        // todo: reverse z is supposedly possible by setting depth_near and depth_far
        // to opposite values, but I'm not sure how.
        //.depth_near(1000.0)
        .base_layer(Some(&xr_gl_layer));
    xr_session.update_render_state_with_state(&render_state_init);

    let reference_space_type = match xr_mode {
        web_sys::XrSessionMode::Inline => web_sys::XrReferenceSpaceType::Viewer,
        _ => web_sys::XrReferenceSpaceType::LocalFloor,
    };

    let xr_reference_space: web_sys::XrReferenceSpace = wasm_bindgen_futures::JsFuture::from(
        xr_session.request_reference_space(reference_space_type),
    )
    .await
    .unwrap()
    .into();

    InitialisedState {
        mode_specific: ModeSpecificState::Xr {
            session: xr_session,
            reference_space: xr_reference_space,
        },
        device,
        queue,
        surface,
        pipeline_options,
    }
}

pub async fn initialise_desktop() -> InitialisedState {
    let event_loop = EventLoop::new();
    let builder = winit::window::WindowBuilder::new();

    let window = builder.build(&event_loop).unwrap();

    #[cfg(feature = "wasm")]
    {
        use winit::platform::web::WindowExtWebSys;
        // On wasm, append the canvas to the document body
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
    }

    let backends = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        ..Default::default()
    });
    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("No suitable GPU adapters found on the system!");

    let adapter_info = adapter.get_info();
    log::info!(
        "Using '{}' with the {:?} backend. Downlevel capabilities: {:?}",
        adapter_info.name,
        adapter_info.backend,
        adapter.get_downlevel_capabilities()
    );

    log::error!("Supported features: {:?}", adapter.features());

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("device"),
                features: adapter.features(),
                limits: adapter.limits(),
            },
            None,
        )
        .await
        .expect("Unable to find a suitable GPU adapter!");

    let surface_capabilites = surface.get_capabilities(&adapter);

    InitialisedState {
        mode_specific: ModeSpecificState::Desktop { window, event_loop },
        device,
        queue,
        pipeline_options: renderer_core::PipelineOptions {
            multiview: None,
            inline_tonemapping: true,
            framebuffer_format: surface_capabilites.formats[0],
            // wgpu handles this for us.
            flip_viewport: false,
            depth_prepass: false,
            // Reverse z has more accurate depth info.
            reverse_z: true,
        },
        surface,
    }
}

pub fn run_rendering_loop(mut app: bevy_app::App, initialised_state: InitialisedState) {
    let device = Arc::new(initialised_state.device);
    let framebuffer_format = initialised_state.pipeline_options.framebuffer_format;

    app.insert_resource(Device(device.clone()))
        .insert_resource(Queue(Arc::new(initialised_state.queue)))
        .insert_resource(PipelineOptions(initialised_state.pipeline_options));

    match initialised_state.mode_specific {
        #[cfg(feature = "webgl")]
        ModeSpecificState::Xr {
            session,
            reference_space,
        } => {
            renderer_core::run_rendering_loop(&session, move |time, frame| {
                let pose = match frame.get_viewer_pose(&reference_space) {
                    Some(pose) => pose,
                    // I'm not 100% sure in what circumstances this is `None`.
                    // We can't really do much without a pose though as this is how you fetch the views.
                    None => return,
                };

                app.insert_non_send_resource(frame);
                app.insert_non_send_resource(pose);
                app.insert_resource(resources::FrameTime(time));
                app.update();
            });
        }
        ModeSpecificState::Desktop { window, event_loop } => {
            let size = window.inner_size();

            let mut config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: framebuffer_format,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::AutoVsync,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![],
            };
            initialised_state.surface.configure(&device, &config);

            event_loop.run(move |event, _, control_flow| {
                match &event {
                    event::Event::WindowEvent { event, .. } => match &event {
                        event::WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        event::WindowEvent::Resized(new_size) => {
                            config.width = new_size.width;
                            config.height = new_size.height;
                            initialised_state.surface.configure(&device, &config);
                        }
                        _ => {}
                    },
                    event::Event::RedrawEventsCleared => {
                        window.request_redraw();
                    }
                    event::Event::RedrawRequested(_) => {
                        let frame = match initialised_state.surface.get_current_texture() {
                            Ok(frame) => frame,
                            Err(_) => {
                                initialised_state.surface.configure(&device, &config);
                                initialised_state
                                    .surface
                                    .get_current_texture()
                                    .expect("Failed to acquire next surface texture!")
                            }
                        };

                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        app.insert_resource(SurfaceFrameView {
                            view,
                            width: config.width,
                            height: config.height,
                        });

                        app.update();

                        // Reset event queue just in case nothing is consuming these.
                        app.world
                            .get_resource_or_insert_with(|| EventQueue(Default::default()))
                            .0
                            .clear();

                        if let Some(mut window_changes) =
                            app.world.get_resource_mut::<WindowChanges>()
                        {
                            if let Some(cursor_grab) = window_changes.cursor_grab {
                                if cursor_grab {
                                    // Try both methods of grabbing the cursor.
                                    let result = window
                                        .set_cursor_grab(winit::window::CursorGrabMode::Locked)
                                        .or_else(|_| {
                                            window.set_cursor_grab(
                                                winit::window::CursorGrabMode::Confined,
                                            )
                                        });

                                    if let Err(error) = result {
                                        log::error!(
                                            "Got an error when trying to set the cursor grab: {}",
                                            error
                                        );
                                    }
                                } else {
                                    // This can't fail.
                                    let _ =
                                        window.set_cursor_grab(winit::window::CursorGrabMode::None);
                                }
                            }

                            if let Some(cursor_visible) = window_changes.cursor_visible {
                                window.set_cursor_visible(cursor_visible);
                            }

                            if let Some(fullscreen) = window_changes.fullscreen {
                                window.set_fullscreen(if fullscreen {
                                    Some(winit::window::Fullscreen::Borderless(Some(
                                        window.current_monitor().unwrap(),
                                    )))
                                } else {
                                    None
                                })
                            }

                            *window_changes = Default::default();
                        }

                        frame.present();
                    }
                    _ => {}
                }

                if let Some(static_event) = event.to_static() {
                    app.world
                        .get_resource_or_insert_with(|| EventQueue(Default::default()))
                        .0
                        .push(static_event);
                }
            })
        }
    }
}

#[derive(Clone, Default)]
pub struct SimpleHttpClient(surf::Client);

impl renderer_core::assets::HttpClient for SimpleHttpClient {
    fn fetch_bytes(
        &self,
        url: &url::Url,
        byte_range: Option<Range<usize>>,
    ) -> renderer_core::assets::HttpClientFuture {
        fn byte_range_string(range: Range<usize>) -> String {
            format!("bytes={}-{}", range.start, range.end - 1)
        }

        let url = url.clone();

        let client = self.0.clone();

        Box::pin(async move {
            let mut request_builder = client
                .get(url.clone())
                .map_err(|err| anyhow::anyhow!("{}", err))?;

            if let Some(byte_range) = byte_range {
                request_builder = request_builder
                    .header("Range", byte_range_string(byte_range))
                    .map_err(|err| anyhow::anyhow!("{}", err))?;
            }

            log::debug!("Requesting {}", url);

            let bytes = request_builder
                .recv_bytes()
                .await
                .map_err(|err| anyhow::anyhow!("{}", err))?;

            log::debug!("Got bytes for {}: {}", url, bytes.len());

            Ok(bytes)
        })
    }
}
