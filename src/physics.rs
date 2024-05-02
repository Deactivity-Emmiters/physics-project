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
    let angle_speed_vec = angle_speed_vec.normalize();
    // Rodrigues' rotation formula
    vec * (angle_speed * time_delta).cos()
        + angle_speed_vec.cross(vec) * (angle_speed * time_delta).sin()
        + angle_speed_vec * angle_speed_vec.dot(vec) * (1.0 - (angle_speed * time_delta).cos())
}


pub fn move_by_magnetic_fields(
    time: Res<Time>,
    fields: Query<&MagneticField>,
    mut electorns: Query<(&mut Transform, &mut Velocity), With<Electron>>,
) {
    for (mut transform, mut velocity) in electorns.iter_mut() {
        for field in fields.iter() {
            let acceleration = velocity.0.cross(field.0);

            // составляющая ортогональная магнитному полю
            let mut vel_ort = velocity.0 - velocity.0.dot(field.0.normalize()) * field.0.normalize();

            // составляющая параллельная магнитному полю, на нее не влияет сила Лоренца
            let vel_ = velocity.0 - vel_ort;

            // радиус-вектор движения по дуге (рассматриваем плоскость перпендикулярную магнитному полю)
            let r = -vel_ort.dot(vel_ort) / acceleration.dot(acceleration) * acceleration;

            // угловая скорость
            let angle_speed = r.cross(vel_ort) / r.dot(r);

            // перемещение по известным радиус-вектору, угловой скорости и времени
            // transform.translation += rotate(r, angle_speed, time.delta_seconds()) - r;

            // обновление ортогональной составляющей (тело движется по окружности и меняет свой вектор скорости)
            vel_ort = rotate(vel_ort, angle_speed, time.delta_seconds());

            // передвижение задаваемое ею
            // transform.translation += vel_ * time.delta_seconds();

            // возвращаем актуальную скорость
            velocity.0 = vel_ + vel_ort;
        }
    }
}
