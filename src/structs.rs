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

#[derive(Component)]
pub struct PlateCathode {
    pub e_field: f32,
    pub emmisivness: u32,
}

#[derive(Component)]
pub struct Plate {
    pub height: f32,
    pub width: f32,
    pub depth: f32,
}

#[derive(Component)]
pub struct PlateDestructionField {
    pub depth: f32,
}


#[derive(Resource)]
pub struct UiState {
    pub phi_value: f32,
    pub theta_value: f32,
    pub e_value: f32,
    pub b_value: f32,
    pub is_window_focused: bool
}

#[derive(Component)]
pub struct MagnetFieldArrow;

