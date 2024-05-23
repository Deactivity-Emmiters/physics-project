#![allow(dead_code)]

mod constants;
mod controls;
mod physics;
mod scenes;
mod structs;
mod ui;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use controls::{
    apply_destruction_field, cathodes_spawn_electrons, update_electric_field, update_magnetic_field,
};
use physics::electrons::{electron_repulsion, update_electron_chunks, ElectronChunks};
use physics::{apply_plate_cathode_electric_field, move_by_magnetic_fields, move_by_velocity};
use structs::{CameraAngles, MagnetFieldArrow, MagneticField, UiState};
use ui::{camera_controls, change_background_color, ui_setup, update_magnet_arrow};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(scenes::scenes_plugin)
        .insert_resource(ClearColor(Color::rgb(255.0, 255.0, 255.0)))
        .insert_resource(structs::SpawnTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )))
        .insert_resource(ElectronChunks::default())
        .insert_resource(Time::<Fixed>::from_hz(500.0))
        .insert_resource(UiState {
            e_value: 2.0,
            b_value: 1.0,
            phi_value: 0.0,
            theta_value: 0.0,
            is_window_focused: false,
        })
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                // spawn_electrons,
                move_by_velocity,
                // apply_gravity,
                move_by_magnetic_fields,
                apply_plate_cathode_electric_field,
                apply_destruction_field,
                cathodes_spawn_electrons,
                update_electron_chunks,
                electron_repulsion.after(update_electron_chunks),
                update_magnetic_field,
                update_electric_field,
            ),
        )
        .add_systems(
            Update,
            (camera_controls, update_magnet_arrow.after(camera_controls)),
        )
        .add_systems(Update, ui_setup)
        .add_systems(Update, change_background_color);

    #[cfg(target_family = "wasm")]
    app.add_systems(Startup, update_canvas_size);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 50.0)),
            ..Default::default()
        },
        CameraAngles {
            vertical: Quat::from_rotation_x(0.0),
            horizontal: Quat::from_rotation_y(0.0),
        },
    ));

    commands.spawn(MagneticField(Vec3::new(0.0, 0.0, 1.0)));
    // commands.spawn(MagneticField(Vec3::new(0.0, 0.0, 1.0)));
    // commands.spawn(MagneticField(Vec3::new(0.0, 0.0, 1.0)));
    // commands.spawn(MagneticField(Vec3::new(0.0, 0.0, 1.0)));
    // commands.spawn(MagneticField(Vec3::new(0.0, 0.0, 1.0)));

    // global light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, -0.9, -1.5)),
        ..default()
    });

    ambient_light.brightness = 400.0;

    // magnet field arrow
    // arrow mesh
    let mesh = meshes.add(Mesh::from(Cuboid::new(0.3, 1.0, 0.3)));
    let material = materials.add(Color::rgb(0.0, 0.0, 1.0));
    commands.spawn((
        PbrBundle {
            mesh,
            material,
            ..Default::default()
        },
        MagnetFieldArrow,
    ));
}

#[cfg(target_family = "wasm")]
fn update_canvas_size(mut window: Query<&mut Window, With<bevy::window::PrimaryWindow>>) {
    (|| {
        let mut window = window.get_single_mut().ok()?;
        let browser_window = web_sys::window()?;
        let width = browser_window.inner_width().ok()?.as_f64()?;
        let height = browser_window.inner_height().ok()?.as_f64()?;
        window.resolution.set(width as f32, height as f32);
        Some(())
    })();
}
