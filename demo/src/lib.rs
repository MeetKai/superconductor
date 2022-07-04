use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_bindgen_futures::spawn_local(run());
}

async fn run() {
    console_error_panic_hook::set_once();

    console_log::init_with_level(log::Level::Info).unwrap();

    let mode = select_mode_via_buttons().await;

    let (xr_session, xr_reference_space, device, queue, pipeline_options) =
        superconductor::initialise(mode).await;

    let mut app = bevy_app::App::new();

    app.insert_resource(device)
        .insert_resource(queue)
        .insert_resource(pipeline_options)
        .add_plugin(SuperconductorPlugin::default());

    superconductor::renderer_core::run_rendering_loop(&xr_session, move |time, frame| {
        let pose = match frame.get_viewer_pose(&xr_reference_space) {
            Some(pose) => pose,
            // I'm not 100% sure in what circumstances this is `None`.
            // We can't really do much without a pose though as this is how you fetch the views.
            None => return,
        };

        app.insert_non_send_resource(frame);
        app.insert_non_send_resource(pose);
        app.insert_resource(superconductor::FrameTime(time));
        app.schedule.run_once(&mut app.world);
    });
}

pub use superconductor::*;

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::{Component, Query, With};
use futures::FutureExt;
use wasm_bindgen::JsCast;

#[derive(Default)]
pub struct SuperconductorPlugin;

impl Plugin for SuperconductorPlugin {
    fn build(&self, app: &mut App) {
        let model = app
            .world
            .spawn()
            .insert(components::ModelUrl(
                url::Url::parse("http://localhost:8000/assets/models/Alicia/AliciaSolid.vrm")
                    .unwrap(),
            ))
            .insert(components::Instances(Default::default()))
            .insert(components::InstanceRange(Default::default()))
            .id();

        app.world
            .spawn()
            .insert(components::InstanceOf(model))
            .insert(components::Instance(renderer_core::Instance::new(
                Vec3::new(1.0, 1.0, -2.0),
                1.0,
                Default::default(),
            )))
            .insert(Spinning);

        app.world
            .spawn()
            .insert(components::InstanceOf(model))
            .insert(components::Instance(renderer_core::Instance::new(
                Vec3::new(-1.0, 1.0, -2.0),
                1.0,
                Default::default(),
            )));

        let helmet = app
            .world
            .spawn()
            .insert(components::ModelUrl(
                url::Url::parse(
                    "http://localhost:8000/assets/models/FlightHelmet/FlightHelmet.gltf",
                )
                .unwrap(),
            ))
            .insert(components::Instances(Default::default()))
            .insert(components::InstanceRange(Default::default()))
            .id();

        app.world
            .spawn()
            .insert(components::InstanceOf(helmet))
            .insert(components::Instance(renderer_core::Instance::new(
                Vec3::new(0.0, 1.0, -3.0),
                2.0,
                Default::default(),
            )));

        app.add_system(rotate_entities);

        superconductor::XrPlugin.build(app);

        app.insert_resource(NewIblTextures(Some(NewIblTexturesInner {
            diffuse_cubemap: url::Url::parse("https://expenses.github.io/mateversum-web/environment_maps/helipad/diffuse_compressed.ktx2").unwrap(),
            specular_cubemap: url::Url::parse("https://expenses.github.io/mateversum-web/environment_maps/helipad/specular_compressed.ktx2").unwrap()
        })));
    }
}

pub async fn select_mode_via_buttons() -> superconductor::Mode {
    let vr_button = create_button("Start VR");
    let ar_button = create_button("Start AR");

    let start_vr_future = button_click_future(&vr_button);
    let start_ar_future = button_click_future(&ar_button);

    futures::select! {
        _ = Box::pin(start_vr_future.fuse()) => superconductor::Mode::Vr,
        _ = Box::pin(start_ar_future.fuse()) => superconductor::Mode::Ar,
    }
}

async fn button_click_future(button: &web_sys::HtmlButtonElement) {
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _reject| {
        button.set_onclick(Some(&resolve))
    }))
    .await
    .unwrap();
}

fn create_button(text: &str) -> web_sys::HtmlButtonElement {
    let button: web_sys::HtmlButtonElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("button")
        .unwrap()
        .unchecked_into();

    button.set_inner_text(text);

    let body = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap();

    body.append_child(&web_sys::Element::from(button.clone()))
        .unwrap();

    button
}

#[derive(Component)]
struct Spinning;

fn rotate_entities(mut query: Query<&mut components::Instance, With<Spinning>>) {
    query.for_each_mut(|mut instance| {
        instance.0.rotation *= renderer_core::glam::Quat::from_rotation_y(0.01)
    });
}