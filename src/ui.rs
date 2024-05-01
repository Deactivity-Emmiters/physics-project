use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Id, Sense};
use crate::constants;
use crate::structs::{CameraAngles, UiState};


pub fn camera_controls(
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

pub fn ui_setup(
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


pub fn change_background_color(input: Res<ButtonInput<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::F1) {
        clear_color.0 = Color::DARK_GRAY;
    }
    if input.just_pressed(KeyCode::F2) {
        clear_color.0 = Color::ANTIQUE_WHITE;
    }
}
