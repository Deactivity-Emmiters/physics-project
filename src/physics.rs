use bevy::prelude::*;



use crate::structs::{Electron, MagneticField, Velocity};

/// Placeholder
pub fn move_by_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

/// Placeholder
pub fn apply_gravity(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        accelerate(
            transform.as_mut(),
            velocity.as_mut(),
            Vec3::new(0.0, -9.81, 0.0),
            time.delta_seconds(),
        );
    }
}

pub fn accelerate(
    transform: &mut Transform,
    velocity: &mut Velocity,
    acceleration: Vec3,
    time_delta: f32,
) {
    velocity.0 += acceleration * time_delta;
    transform.translation += acceleration * time_delta * time_delta / 2.0;
}

pub fn rotate(vec: Vec3, angle_speed_vec: Vec3, time_delta: f32) -> Vec3 {
    let angle_speed = angle_speed_vec.length();
    vec * (angle_speed * time_delta).cos()
        + angle_speed_vec.cross(vec) * (angle_speed * time_delta).sin()
        + angle_speed_vec * angle_speed_vec.dot(vec) * (1.0 - (angle_speed * time_delta).cos())
}

pub fn apply_magnetic_fields(
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
