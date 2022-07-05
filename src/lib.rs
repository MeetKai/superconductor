use bevy_app::{App, Plugin};
use bevy_ecs::prelude::SystemStage;
use std::ops::Range;
use std::sync::Arc;

pub mod components;
pub mod resources;
mod systems;

pub use bevy_app;
pub use bevy_ecs;
pub use renderer_core;
pub use url;

pub use renderer_core::{assets::textures, glam::Vec3, utils::Swappable};

use components::Instance;
use resources::{Device, ModelUrls, NewIblTextures, Queue};

#[derive(bevy_ecs::prelude::StageLabel, Debug, PartialEq, Eq, Clone, Hash)]
pub enum StartupStage {
    PipelineCreation,
    BindGroupCreation,
}

#[derive(bevy_ecs::prelude::StageLabel, Debug, PartialEq, Eq, Clone, Hash)]
pub enum Stage {
    AssetLoading,
    BufferResetting,
    InstanceBuffering,
    BufferUploading,
    Rendering,
}

#[derive(Default)]
pub struct XrPlugin;

impl Plugin for XrPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ModelUrls(Default::default()));
        app.insert_resource(textures::Settings {
            anisotropy_clamp: Some(std::num::NonZeroU8::new(16).unwrap()),
        });
        app.insert_resource(NewIblTextures(None));

        app.add_startup_stage(
            StartupStage::PipelineCreation,
            SystemStage::single_threaded()
                .with_system(systems::create_bind_group_layouts_and_pipelines),
        );
        app.add_startup_stage_after(
            StartupStage::PipelineCreation,
            StartupStage::BindGroupCreation,
            SystemStage::single_threaded().with_system(systems::allocate_bind_groups),
        );

        app.add_stage(
            Stage::AssetLoading,
            SystemStage::single_threaded()
                .with_system(systems::start_loading_models)
                .with_system(systems::finish_loading_models)
                .with_system(systems::update_ibl_textures),
        );

        app.add_stage_after(
            Stage::AssetLoading,
            Stage::BufferResetting,
            SystemStage::single_threaded()
                .with_system(systems::update_uniform_buffers)
                .with_system(systems::clear_instance_buffers),
        );

        app.add_stage_after(
            Stage::BufferResetting,
            Stage::InstanceBuffering,
            SystemStage::single_threaded().with_system(systems::push_entity_instances),
        );

        app.add_stage_after(
            Stage::InstanceBuffering,
            Stage::BufferUploading,
            SystemStage::single_threaded().with_system(systems::upload_instances),
        );

        app.add_stage_after(
            Stage::BufferUploading,
            Stage::Rendering,
            SystemStage::single_threaded().with_system(systems::rendering::render),
        );

        app.world
            .spawn()
            .insert(Instance(renderer_core::Instance::new(
                Vec3::new(1.0, 1.0, -2.0),
                1.0,
                Default::default(),
            )));

        app.world
            .spawn()
            .insert(Instance(renderer_core::Instance::new(
                Vec3::new(-1.0, 1.0, -2.0),
                1.0,
                Default::default(),
            )));
    }
}

pub enum Mode {
    Vr,
    Ar,
    Desktop,
}

enum ModeSpecificState {
    Xr {
        session: web_sys::XrSession,
        reference_space: web_sys::XrReferenceSpace,
    },
    Desktop,
}

pub struct InitialisedState {
    mode_specific: ModeSpecificState,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline_options: renderer_core::PipelineOptions,
}

pub async fn initialise(mode: Mode) -> InitialisedState {
    match mode {
        Mode::Vr => initialise_xr(web_sys::XrSessionMode::ImmersiveVr).await,
        Mode::Ar => initialise_xr(web_sys::XrSessionMode::ImmersiveAr).await,
        Mode::Desktop => initialise_desktop().await,
    }
}

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
        }
    } else {
        renderer_core::PipelineOptions {
            multiview: None,
            inline_tonemapping: true,
        }
    };

    layer_init
        .depth(pipeline_options.render_direct_to_framebuffer())
        .stencil(pipeline_options.render_direct_to_framebuffer());

    let backend = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
    let instance = wgpu::Instance::new(backend);
    let surface = unsafe { instance.create_surface(&canvas) };
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

    log::info!("Supported features: {:?}", adapter.features());

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
        pipeline_options,
    }
}

pub async fn initialise_desktop() -> InitialisedState {
    panic!("Not yet implemented")
}

pub fn run_rendering_loop(mut app: bevy_app::App, initialised_state: InitialisedState) {
    app.insert_resource(Device(Arc::new(initialised_state.device)))
        .insert_resource(Queue(Arc::new(initialised_state.queue)))
        .insert_resource(initialised_state.pipeline_options);

    match initialised_state.mode_specific {
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
                app.schedule.run_once(&mut app.world);
            });
        }
        ModeSpecificState::Desktop => {}
    }
}

#[derive(Clone)]
struct SimpleHttpClient;

impl renderer_core::assets::HttpClient for SimpleHttpClient {
    type Future = std::pin::Pin<Box<dyn core::future::Future<Output = anyhow::Result<Vec<u8>>>>>;

    fn fetch_bytes(&self, url: &url::Url, byte_range: Option<Range<usize>>) -> Self::Future {
        async fn resolve_promise(
            promise: js_sys::Promise,
        ) -> anyhow::Result<wasm_bindgen::JsValue> {
            wasm_bindgen_futures::JsFuture::from(promise)
                .await
                .map_err(|err| anyhow::anyhow!("{:?}", err))
        }

        let url = url.clone();

        Box::pin(async move {
            let request_init = construct_request_init(byte_range.clone())?;

            let request = web_sys::Request::new_with_str_and_init(url.as_str(), &request_init)
                .map_err(|err| anyhow::anyhow!("{:?}", err))?;

            let response: web_sys::Response =
                resolve_promise(web_sys::window().unwrap().fetch_with_request(&request))
                    .await?
                    .into();

            let array_buffer: js_sys::ArrayBuffer = resolve_promise(
                response
                    .array_buffer()
                    .map_err(|err| anyhow::anyhow!("{:?}", err))?,
            )
            .await?
            .into();

            let uint8_array = js_sys::Uint8Array::new(&array_buffer);

            Ok(uint8_array.to_vec())
        })
    }
}

fn construct_request_init(
    byte_range: Option<Range<usize>>,
) -> anyhow::Result<web_sys::RequestInit> {
    let mut request_init = web_sys::RequestInit::new();

    fn byte_range_string(range: Range<usize>) -> String {
        format!("bytes={}-{}", range.start, range.end - 1)
    }

    if let Some(byte_range) = byte_range {
        let headers = js_sys::Object::new();
        js_sys::Reflect::set(
            &headers,
            &"Range".into(),
            &byte_range_string(byte_range).into(),
        )
        .map_err(|err| anyhow::anyhow!("Js Error: {:?}", err))?;
        request_init.headers(&headers);
    }

    Ok(request_init)
}
