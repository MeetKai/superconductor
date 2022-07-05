use wasm_bindgen::prelude::wasm_bindgen;

use superconductor::{
    bevy_ecs, components, renderer_core,
    resources::{Camera, FrameTime, KeyboardInputQueue, NewIblTextures, NewIblTexturesInner},
    url,
    winit::event::{ElementState, VirtualKeyCode},
    Mode, Vec3,
};
#[wasm_bindgen(start)]
pub fn main() {
    wasm_bindgen_futures::spawn_local(run());
}

pub async fn run() {
    //console_error_panic_hook::set_once();

    //console_log::init_with_level(log::Level::Info).unwrap();

    #[cfg(feature = "webgl")]
    let mode = select_mode_via_buttons().await;

    #[cfg(not(feature = "webgl"))]
    let mode = Mode::Desktop;

    let initialised_state = superconductor::initialise(mode).await;

    let mut app = bevy_app::App::new();

    app.add_plugin(SuperconductorPlugin::new(mode));

    superconductor::run_rendering_loop(app, initialised_state);
}

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::{Component, Query, Res, ResMut, With};
use futures::FutureExt;
use wasm_bindgen::JsCast;

pub struct SuperconductorPlugin {
    mode: Mode,
}

impl SuperconductorPlugin {
    fn new(mode: Mode) -> Self {
        Self { mode }
    }
}

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

        let camera_rig: dolly::rig::CameraRig = dolly::rig::CameraRig::builder()
            .with(dolly::drivers::Position::new(Vec3::new(0.0, 1.75, 0.0)))
            .with(dolly::drivers::YawPitch::new().pitch_degrees(0.0))
            .build();

        app.insert_resource(KeyboardState::default());
        app.insert_resource(camera_rig);

        app.add_system(rotate_entities);
        app.add_system(handle_keyboard_input);
        app.add_system(update_camera);

        superconductor::XrPlugin::new(self.mode).build(app);

        app.insert_resource(NewIblTextures(Some(NewIblTexturesInner {
            diffuse_cubemap: url::Url::parse("https://expenses.github.io/mateversum-web/environment_maps/helipad/diffuse_compressed.ktx2").unwrap(),
            specular_cubemap: url::Url::parse("https://expenses.github.io/mateversum-web/environment_maps/helipad/specular_compressed.ktx2").unwrap()
        })));
    }
}

#[cfg(feature = "webgl")]
pub async fn select_mode_via_buttons() -> superconductor::Mode {
    let vr_button = create_button("Start VR");
    let ar_button = create_button("Start AR");
    let desktop_button = create_button("Start Desktop");

    let start_vr_future = button_click_future(&vr_button);
    let start_ar_future = button_click_future(&ar_button);
    let start_desktop_future = button_click_future(&desktop_button);

    futures::select! {
        _ = Box::pin(start_vr_future.fuse()) => superconductor::Mode::Vr,
        _ = Box::pin(start_ar_future.fuse()) => superconductor::Mode::Ar,
        _ = Box::pin(start_desktop_future.fuse()) => superconductor::Mode::Desktop,
    }
}

#[cfg(feature = "webgl")]
async fn button_click_future(button: &web_sys::HtmlButtonElement) {
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _reject| {
        button.set_onclick(Some(&resolve))
    }))
    .await
    .unwrap();
}

#[cfg(feature = "webgl")]
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

#[derive(Default)]
struct KeyboardState {
    forwards: bool,
    right: bool,
    left: bool,
    backwards: bool,
}

fn rotate_entities(mut query: Query<&mut components::Instance, With<Spinning>>) {
    query.for_each_mut(|mut instance| {
        instance.0.rotation *= renderer_core::glam::Quat::from_rotation_y(0.01)
    });
}

fn handle_keyboard_input(
    mut input: ResMut<KeyboardInputQueue>,
    mut keyboard_state: ResMut<KeyboardState>,
) {
    for input in input.0.drain(..) {
        let pressed = input.state == ElementState::Pressed;

        match input.virtual_keycode {
            Some(VirtualKeyCode::W | VirtualKeyCode::Up) => {
                keyboard_state.forwards = pressed;
            }
            Some(VirtualKeyCode::A | VirtualKeyCode::Left) => {
                keyboard_state.left = pressed;
            }
            Some(VirtualKeyCode::S | VirtualKeyCode::Down) => {
                keyboard_state.backwards = pressed;
            }
            Some(VirtualKeyCode::D | VirtualKeyCode::Right) => {
                keyboard_state.right = pressed;
            }
            _ => {}
        }
    }
}

fn update_camera(
    keyboard_state: Res<KeyboardState>,
    mut camera: ResMut<Camera>,
    mut camera_rig: ResMut<dolly::rig::CameraRig>,
) {
    let forwards = keyboard_state.forwards as i32 - keyboard_state.backwards as i32;
    let right = keyboard_state.right as i32 - keyboard_state.left as i32;

    let move_vec = camera_rig.final_transform.rotation
        * Vec3::new(right as f32, 0.0, -forwards as f32).clamp_length_max(1.0);

    let delta_time = 1.0 / 60.0;
    let speed = 3.0;

    camera_rig
        .driver_mut::<dolly::drivers::Position>()
        .translate(move_vec * delta_time * speed);

    camera_rig.update(delta_time);

    camera.position = camera_rig.final_transform.position;
    camera.rotation = camera_rig.final_transform.rotation;
}
