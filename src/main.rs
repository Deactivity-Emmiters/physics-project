use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

// TODO: move constant to separate file
const SETTINGS_WINDOW_WIDTH: f32 = 180.;
const E_MAX_VALUE: f32 = 10.0;
const B_MAX_VALUE: f32 = 10.0;

#[derive(Component)]
struct Electron;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Component)]
struct CameraAngles {
    vertical: Quat,
    horizontal: Quat,
}

#[derive(Default, Resource)]
struct UiState {
    phi_label: String,
    theta_label: String,
    e_value: f32,
    b_value: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .insert_resource(SpawnTimer(Timer::from_seconds(0.3, TimerMode::Repeating)))
        .init_resource::<UiState>()
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (spawn_electrons, move_by_velocity, apply_gravity),
        )
        .add_systems(Update, camera_controls)
        .add_systems(Update, ui_setup)
        .add_systems(FixedUpdate, (spawn_electrons, move_by_velocity, apply_gravity))
        .run();
}

fn ui_setup(
    mut ui_state: ResMut<UiState>,
    mut ctx: EguiContexts,
) {
    egui::Window::new("Settings")
        .max_width(SETTINGS_WINDOW_WIDTH)
        .default_width(SETTINGS_WINDOW_WIDTH)
        .show(ctx.ctx_mut(), |ui| {
            ui.add(egui::Slider::new(&mut ui_state.e_value, 0.0..=E_MAX_VALUE).text("E"));
            ui.add(egui::Slider::new(&mut ui_state.b_value, 0.0..=B_MAX_VALUE).text("B"));

            ui.horizontal(|ui| {
                ui.label("φ: ");
                ui.text_edit_singleline(&mut ui_state.phi_label);
            });

            ui.horizontal(|ui| {
                ui.label("θ: ");
                ui.text_edit_singleline(&mut ui_state.theta_label);
            });

        });
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..Default::default()
        },
        CameraAngles {
            vertical: Quat::from_rotation_x(0.0),
            horizontal: Quat::from_rotation_y(0.0),
        },
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
        Velocity(Vec3::new(0.0, 0.0, 0.0)),
    ));
}

/// Placeholder
fn move_by_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

/// Placeholder
fn apply_gravity(time: Res<Time>, mut query: Query<&mut Velocity>) {
    for mut velocity in query.iter_mut() {
        velocity.0.y -= 9.81 * time.delta_seconds();
    }
}

fn camera_controls(
    mut mouse_motion_events: EventReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &mut CameraAngles), With<Camera3d>>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }
    let (mut transform, mut angles) = query.single_mut();
    for event in mouse_motion_events.read() {
        angles.vertical *= Quat::from_rotation_x(-event.delta.y * 0.002);
        angles.horizontal *= Quat::from_rotation_y(-event.delta.x * 0.002);
    }
    transform.rotation = angles.vertical * angles.horizontal;
}
