use crate::constants;
use crate::structs::{CameraAngles, MagnetFieldArrow, MagneticField, UiState};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_egui::egui::{Id, Sense};
use bevy_egui::{egui, EguiContexts};

pub fn camera_controls(
    time: Res<Time>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraAngles), With<Camera3d>>,
    ui_state: ResMut<UiState>,
) {
    if ui_state.is_window_focused {
        return;
    }

    let mut horizontal_axis = Vec3::new(0.0, 0.0, 0.0);
    let mut vertical_axis = Vec3::new(0.0, 0.0, 0.0);
    let mut depth_axis = Vec3::new(0.0, 0.0, 0.0);

    if keyboard_buttons.pressed(KeyCode::KeyW) {
        vertical_axis.z += -1.0;
    }

    if keyboard_buttons.pressed(KeyCode::KeyS) {
        vertical_axis.z += 1.0;
    }

    if keyboard_buttons.pressed(KeyCode::KeyA) {
        horizontal_axis.x += -1.0;
    }

    if keyboard_buttons.pressed(KeyCode::KeyD) {
        horizontal_axis.x += 1.0;
    }

    if keyboard_buttons.pressed(KeyCode::ShiftLeft) {
        depth_axis.y += -1.0;
    }

    if keyboard_buttons.pressed(KeyCode::Space) {
        depth_axis.y += 1.0;
    }

    let mut movement = horizontal_axis + vertical_axis + depth_axis;
    if movement.length() > 0.0 {
        movement = movement.normalize();
    }

    for (mut transform, angles) in query.iter_mut() {
        let rotation = angles.horizontal;
        let movement = rotation.mul_vec3(movement);
        transform.translation += movement * constants::CAMERA_SPEED * time.delta_seconds();
    }

    if !mouse_buttons.pressed(MouseButton::Left) {
        return;
    }
    let (mut transform, mut angles) = query.single_mut();
    for event in mouse_motion_events.read() {
        angles.vertical *= Quat::from_rotation_x(-event.delta.y * 0.002);
        angles.horizontal *= Quat::from_rotation_y(-event.delta.x * 0.002);
    }
    transform.rotation = angles.horizontal * angles.vertical;
}

pub fn ui_setup(
    mut ui_state: ResMut<UiState>,
    mut ctx: EguiContexts,
    mut clear_color: ResMut<ClearColor>,
) {
    ui_state.is_window_focused = false;

    let window_response = egui::Window::new("Settings")
        .max_width(constants::SETTINGS_WINDOW_WIDTH)
        .default_width(constants::SETTINGS_WINDOW_WIDTH)
        .max_height(constants::SETTINGS_WINDOW_HEIGHT)
        .default_height(constants::SETTINGS_WINDOW_HEIGHT)
        .resizable(false)
        .show(ctx.ctx_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Здесь размещаете содержимое, которое может растягиваться вниз
                let e_slider = ui.add(
                    egui::Slider::new(&mut ui_state.e_value, 0.0001..=constants::E_MAX_VALUE)
                        .text("E"),
                );
                let b_slider = ui.add(
                    egui::Slider::new(&mut ui_state.b_value, 0.0001..=constants::B_MAX_VALUE)
                        .text("B"),
                );

                // ui.horizontal(|ui| {
                // ui.label("φ: ");
                // ui.text_edit_singleline(&mut ui_state.phi_label);
                // });
                let phi_slider =
                    ui.add(egui::Slider::new(&mut ui_state.phi_value, -90.0..=90.0).text("φ"));

                // ui.horizontal(|ui| {
                // ui.label("θ: ");
                // ui.text_edit_singleline(&mut ui_state.theta_label);
                // });
                let theta_slider =
                    ui.add(egui::Slider::new(&mut ui_state.theta_value, -180.0..=180.0).text("θ"));

                if ui
                    .interact(ui.max_rect(), Id::new("CUM"), Sense::click())
                    .clicked()
                    || e_slider.dragged()
                    || b_slider.dragged()
                    || phi_slider.dragged()
                    || theta_slider.dragged()
                {
                    ui_state.is_window_focused = true;
                }

                if ui.button("Change colour").clicked() {
                    clear_color.0 = match clear_color.0 {
                        Color::DARK_GRAY => Color::ANTIQUE_WHITE,
                        Color::ANTIQUE_WHITE => Color::WHITE,
                        Color::WHITE => Color::DARK_GRAY,
                        _ => Color::WHITE,
                    };
                }
            });
        })
        .unwrap()
        .response;

    if window_response.dragged() {
        ui_state.is_window_focused = true;
    }
}

pub fn change_background_color(
    input: Res<ButtonInput<KeyCode>>,
    mut clear_color: ResMut<ClearColor>,
) {
    if input.just_pressed(KeyCode::F1) {
        clear_color.0 = Color::DARK_GRAY;
    }
    if input.just_pressed(KeyCode::F2) {
        clear_color.0 = Color::ANTIQUE_WHITE;
    }
}

pub fn update_magnet_arrow(
    mut arrow: Query<&mut Transform, With<MagnetFieldArrow>>,
    ui_state: Res<UiState>,
    camera: Query<&Transform, (With<Camera3d>, Without<MagnetFieldArrow>)>,
) {
    let camera_transform = camera.single();
    let camera_position = camera_transform.translation;
    let offset = Vec3::new(1.0, -0.5, -2.0);
    let offset = camera_transform.rotation * offset;

    let mut arrow = arrow.single_mut();

    arrow.translation = camera_position + offset;
    arrow.rotation = Quat::from_rotation_y(ui_state.theta_value.to_radians())
        * Quat::from_rotation_x(ui_state.phi_value.to_radians());
}
