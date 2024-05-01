#![allow(dead_code)]

mod constants;
mod structs;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_egui::egui::{Id, Sense};
use structs::{CameraAngles, Electron, MagneticField, SpawnTimer, UiState, Velocity};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .insert_resource(structs::SpawnTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
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
        .run();
}

fn ui_setup(
    mut ui_state: ResMut<UiState>,
    mut ctx: EguiContexts,
) {

    ui_state.is_window_focused = false;

    let window_response = egui::Window::new("Settings")
        .max_width(constants::SETTINGS_WINDOW_WIDTH)
        .default_width(constants::SETTINGS_WINDOW_WIDTH)
        .show(ctx.ctx_mut(), |ui| {
            let e_slider = ui.add(egui::Slider::new(&mut ui_state.e_value, 0.0..=constants::E_MAX_VALUE).text("E"));
            let b_slider = ui.add(egui::Slider::new(&mut ui_state.b_value, 0.0..=constants::B_MAX_VALUE).text("B"));

            ui.horizontal(|ui| {
                ui.label("φ: ");
                ui.text_edit_singleline(&mut ui_state.phi_label);
            });

            ui.horizontal(|ui| {
                ui.label("θ: ");
                ui.text_edit_singleline(&mut ui_state.theta_label);
            });

            if ui.interact(ui.max_rect(), Id::new("CUM"), Sense::drag()).dragged() ||
                e_slider.dragged() ||
                b_slider.dragged() {
                ui_state.is_window_focused = true;
            }


        }).unwrap().response;

    if window_response.dragged() {
        ui_state.is_window_focused = true;
    }

}

fn setup(mut commands: Commands) {
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

/// Placeholder
fn move_by_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

/// Placeholder
fn apply_gravity(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        accelerate(
            transform.as_mut(),
            velocity.as_mut(),
            Vec3::new(0.0, -9.81, 0.0),
            time.delta_seconds(),
        );
    }
}

fn accelerate(
    transform: &mut Transform,
    velocity: &mut Velocity,
    acceleration: Vec3,
    time_delta: f32,
) {
    velocity.0 += acceleration * time_delta;
    transform.translation += acceleration * time_delta * time_delta / 2.0;
}

fn camera_controls(
    mut mouse_motion_events: EventReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &mut CameraAngles), With<Camera3d>>,
    ui_state: ResMut<UiState>
) {
    if !buttons.pressed(MouseButton::Left) || ui_state.is_window_focused {
        return;
    }
    let (mut transform, mut angles) = query.single_mut();
    for event in mouse_motion_events.read() {
        angles.vertical *= Quat::from_rotation_x(-event.delta.y * 0.002);
        angles.horizontal *= Quat::from_rotation_y(-event.delta.x * 0.002);
    }
    transform.rotation = angles.horizontal * angles.vertical;
}

fn rotate(vec: Vec3, angle_speed_vec: Vec3, time_delta: f32) -> Vec3 {
    let angle_speed = angle_speed_vec.length();
    vec * (angle_speed * time_delta).cos()
        + angle_speed_vec.cross(vec) * (angle_speed * time_delta).sin()
        + angle_speed_vec * angle_speed_vec.dot(vec) * (1.0 - (angle_speed * time_delta).cos())
}

fn apply_magnetic_fields(
    time: Res<Time>,
    fields: Query<&MagneticField>,
    mut electorns: Query<(&mut Transform, &mut Velocity), With<Electron>>,
) {
    for (mut transform, mut velocity) in electorns.iter_mut() {
        for field in fields.iter() {
            let acceleration = velocity.0.cross(field.0);
            accelerate(
                transform.as_mut(),
                velocity.as_mut(),
                acceleration,
                time.delta_seconds(),
            );
            // let r = -velocity.0.dot(velocity.0) / acceleration.dot(acceleration) * acceleration;
            // let ortogonal = r.cross(velocity.0);
            // let angle_speed = acceleration / r.dot(r);
            // let rotation = Quat::from_axis_angle(ortogonal.normalize(), angle_speed.length() * time.delta_seconds());
            //
            // // transform.translation += rotate(r, angle_speed, time.delta_seconds()) - r;
            // // velocity.0 = rotate(velocity.0, angle_speed, time.delta_seconds());
            // transform.translation += rotation * r - r;
            // velocity.0 = rotation * velocity.0;
        }
    }
}
