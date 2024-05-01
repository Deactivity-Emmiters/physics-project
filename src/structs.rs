use bevy::prelude::*;

#[derive(Component)]
pub struct Electron;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

#[derive(Component)]
pub struct CameraAngles {
    pub vertical: Quat,
    pub horizontal: Quat,
}

#[derive(Component)]
pub struct MagneticField(pub Vec3);

#[derive(Default, Resource)]
pub struct UiState {
    pub phi_label: String,
    pub theta_label: String,
    pub e_value: f32,
    pub b_value: f32,
    pub is_window_focused: bool
}