#![allow(dead_code)]

mod constants;
mod physics;
mod structs;
mod ui;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use physics::{apply_magnetic_fields, move_by_velocity};
use structs::{CameraAngles, Electron, MagneticField, SpawnTimer, UiState, Velocity};
use ui::{camera_controls, change_background_color, ui_setup};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .insert_resource(ClearColor(Color::rgb(255.0, 255.0, 255.0)))
        .insert_resource(structs::SpawnTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .insert_resource(Time::<Fixed>::from_hz(200.0))
        .init_resource::<UiState>()
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                spawn_electrons,
                move_by_velocity,
                // apply_gravity,
                apply_magnetic_fields,
            ),
        )
        .add_systems(Update, camera_controls)
        .add_systems(Update, ui_setup)
        .add_systems(Update, change_background_color)
        .run();
}

fn setup(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
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
    commands.spawn(MagneticField(Vec3::new(0.0, 0.0, 1.0)));
    commands.spawn(MagneticField(Vec3::new(0.0, 0.0, 1.0)));
    commands.spawn(MagneticField(Vec3::new(0.0, 0.0, 1.0)));
    commands.spawn(MagneticField(Vec3::new(0.0, 0.0, 1.0)));

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
}

fn spawn_electrons(
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !spawn_timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let mesh = meshes.add(Sphere::new(1.0).mesh().ico(3).unwrap());

    let position = Vec3::new(0.0, 0.0, 0.0);

    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(Color::rgb(0.0, 0.0, 1.0)),
            transform: Transform::from_translation(position),
            ..Default::default()
        },
        Electron,
        Velocity(Vec3::new(0.0, -10.0, 1.0)),
    ));
}
