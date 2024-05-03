#![allow(dead_code)]

mod constants;
mod controls;
mod physics;
mod structs;
mod ui;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use controls::{
    apply_destruction_field, cathodes_spawn_electrons,
    update_electric_field, update_magnetic_field,
};
use physics::{
    apply_plate_cathode_electric_field, move_by_magnetic_fields,
    electron_repulsion,
    move_by_velocity
};
use structs::{
    DestructionField,
    Electron, MagneticField, Velocity,
    SpawnTimer, CameraAngles, UiState, MagnetFieldArrow,
    Cylinder, CylindricalCathode, Plate, PlateCathode
};
use ui::{
    ui_setup, change_background_color,
    camera_controls, update_magnet_arrow,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .insert_resource(ClearColor(Color::rgb(255.0, 255.0, 255.0)))
        .insert_resource(structs::SpawnTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )))
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
                electron_repulsion,
                update_magnetic_field,
                update_electric_field,
                update_magnet_arrow,
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
    asset_server: Res<AssetServer>,
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
    let mesh = meshes.add(Mesh::from(Cuboid::new(0.3, 0.3, 1.0)));
    let material = materials.add(Color::rgb(0.0, 0.0, 1.0));
    commands.spawn((
        PbrBundle {
            mesh,
            material,
            ..Default::default()
        },
        MagnetFieldArrow,
    ));

    //setup_plate_diode(commands, meshes, materials);
    setup_cylindrical_diode(commands, meshes, materials, asset_server);
}

fn setup_plate_diode(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    const HEIGHT: f32 = 200.0;
    const WIDTH: f32 = 80.0;
    const CATHODE_POS: Vec3 = Vec3::new(15.0, 0.0, 0.0);
    let cathode_rot: Quat = Quat::from_rotation_y(0.5 * std::f32::consts::PI);
    const ANODE_POS: Vec3 = Vec3::new(-15.0, 0.0, 0.0);
    let anode_rot: Quat = Quat::from_rotation_y(0.5 * std::f32::consts::PI);

    // cathode plate
    let plate_cathode = PlateCathode {
        e_field: 10.0,
        emmisivness: 80,
    };
    let plate = Plate {
        height: HEIGHT,
        width: WIDTH,
        depth: 1.0,
    };
    let plate_transform = Transform {
        translation: CATHODE_POS,
        rotation: cathode_rot,
        ..default()
    };
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
        DestructionField { depth: 0.2 },
    ));

    // anode plate
    let plate = Plate {
        height: HEIGHT,
        width: WIDTH,
        depth: 1.0,
    };
    let plate_transform = Transform {
        translation: ANODE_POS,
        rotation: anode_rot,
        ..default()
    };
    let mesh = meshes.add(Mesh::from(Cuboid::new(
        plate.width,
        plate.height,
        plate.depth,
    )));
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            transform: plate_transform,
            ..Default::default()
        },
        plate,
        DestructionField { depth: 0.8 },
    ));

    // bounding box, destruction panels
    spawn_dp(&mut commands, Vec3::new(0.0, 0.0, WIDTH / 2.0), Quat::default());
    spawn_dp(&mut commands, Vec3::new(0.0, 0.0, -WIDTH / 2.0), Quat::default());
    spawn_dp(&mut commands, Vec3::new(0.0, HEIGHT / 2.0, 0.0), Quat::from_rotation_x(0.5 * std::f32::consts::PI));
    spawn_dp(&mut commands, Vec3::new(0.0, -HEIGHT / 2.0, 0.0), Quat::from_rotation_x(0.5 * std::f32::consts::PI));
    spawn_dp(&mut commands, CATHODE_POS + Vec3::new(1.0, 0.0, 0.0), cathode_rot);
}

fn spawn_dp(commands: &mut Commands, pos: Vec3, rot: Quat){
    let plate = Plate {
        height: 1000000.0,
        width: 1000000.0,
        depth: 1.0,
    };
    let plate_transform = Transform {
        translation: pos,
        rotation: rot,
        ..default()
    };
    commands.spawn((
        plate_transform,
        plate,
        DestructionField { depth: 1.0 },
    ));
}

fn setup_cylindrical_diode(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
){
    const HEIGHT: f32 = 200.0;
    const WIDTH: f32 = 80.0;

    // cathode cylinder
    let mesh = meshes.add(Mesh::from(bevy::prelude::Cylinder {
        radius: 5.0,
        half_height: HEIGHT / 2.0,
        ..default()
    }));
    let cylindrical_cathode = CylindricalCathode{
        e_field: 10.0,
        emmisivness: 80,
    };
    let cylinder = Cylinder {
        inner_radius: 0.0,
        outer_radius: 5.0,
        height: HEIGHT
    };
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(Color::rgb(0.0, 1.0, 0.0)),
            transform: Transform{
                translation: Vec3::new(3.0, 0.0, 0.0),
                ..default()
            },
            ..Default::default()
        },
        cylinder,
        cylindrical_cathode,
        DestructionField { depth: 0.2 },
    ));

    // anode cylinder

    // in this model: height = 20, inner_radius: 9.0, outer_radius: 10.0
    let model = asset_server.load("models/Hole_Cylinder.gltf#Scene0");

    let cylinder = Cylinder {
        inner_radius: 27.0,
        outer_radius: 30.0,
        height: HEIGHT
    };
    commands.spawn((
        SceneBundle {
            scene: model.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, -100.0, 0.0),
                scale: Vec3::new(3.0, 10.0, 3.0),
                ..default()
            },
            ..default()
        },
        cylinder,
        DestructionField { depth: 0.8 },
    ));

    // bounding box, destruction panels
    spawn_dp(&mut commands, Vec3::new(0.0, HEIGHT / 2.0, 0.0), Quat::from_rotation_x(0.5 * std::f32::consts::PI));
    spawn_dp(&mut commands, Vec3::new(0.0, -HEIGHT / 2.0, 0.0), Quat::from_rotation_x(0.5 * std::f32::consts::PI));
}

/// Placeholder
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
