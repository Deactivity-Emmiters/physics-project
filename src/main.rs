#![allow(dead_code)]

mod constants;
mod physics;
mod structs;
mod ui;
use crate::physics::move_by_velocity;
use crate::structs::{Plate, PlateCathode};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use physics::{apply_cathode_electric_field, apply_desruction_field, cathodes_spawn_electrons, move_by_magnetic_fields};
use structs::{
    CameraAngles, Electron, MagneticField, PlateDestructionField, SpawnTimer, UiState, Velocity,
};
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
        .insert_resource(Time::<Fixed>::from_hz(2000.0))
        .init_resource::<UiState>()
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                // spawn_electrons,
                move_by_velocity,
                // apply_gravity,
                move_by_magnetic_fields,
                apply_cathode_electric_field,
                apply_desruction_field,
                cathodes_spawn_electrons,
            ),
        )
        .add_systems(Update, camera_controls)
        .add_systems(Update, ui_setup)
        .add_systems(Update, change_background_color)
        .run();
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
    commands.spawn(MagneticField(Vec3::new(0.0, 0.0, 1.0)));
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

    // cathode plate
    let plate_cathode = PlateCathode {
        e_field: 10.0,
        emmisivness: 10,
    };
    let plate = Plate {
        height: 200.0,
        width: 30.0,
        depth: 1.0,
    };
    let plate_transform = Transform {
        translation: Vec3::new(10.0, 0.0, 0.0),
        rotation: Quat::from_rotation_y(0.5 * std::f32::consts::PI),
        ..default()
    };
    let destruct_field = PlateDestructionField { depth: 0.2 };
    let mesh = meshes.add(Mesh::from(Cuboid::new(
        plate.width,
        plate.height,
        plate.depth,
    )));

    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(Color::rgb(0.0, 1.0, 0.0)),
            transform: plate_transform,
            ..Default::default()
        },
        plate_cathode,
        plate,
        destruct_field,
    ));
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
        Velocity(Vec3::new(39.0, 0.0, 0.0)),
    ));
}
