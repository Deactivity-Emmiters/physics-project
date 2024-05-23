use std::f32::consts::PI;
use bevy::prelude::*;

use crate::structs::{
    Cylinder, CylindricalCathode,
    Electron, MagneticField,
    Plate, PlateCathode,
    DestructionField, Velocity
};

pub mod electrons;

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

/// Placeholder
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
    mut electrons: Query<(&mut Transform, &mut Velocity), With<Electron>>,
) {
    for (mut transform, mut velocity) in electrons.iter_mut() {
        for field in fields.iter() {
            let acceleration = velocity.0.cross(field.0);

            // составляющая ортогональная магнитному полю
            let mut vel_ort =
                velocity.0 - velocity.0.dot(field.0.normalize()) * field.0.normalize();

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


pub fn apply_plate_cathode_electric_field(
    time: Res<Time>,
    plate_cathodes: Query<(&Transform, &PlateCathode, &Plate), Without<Electron>>,
    mut electrons: Query<(&mut Transform, &mut Velocity), With<Electron>>,
) {
    for (plate_transform, plate_cathode, plate) in plate_cathodes.iter() {
        for (mut transform, mut velocity) in electrons.iter_mut() {
            // check if in range
            let rel_electron_pos = transform.translation - plate_transform.translation;
            let rel_electron_pos = plate_transform.rotation.inverse() * rel_electron_pos;
            if rel_electron_pos.x.abs() > plate.width / 2.0
                || rel_electron_pos.y.abs() > plate.height / 2.0
            {
                continue;
            }

            // apply force
            let mut force = Vec3::new(0.0, 0.0, plate_cathode.e_field);

            if rel_electron_pos.z < 0.0 {
                force *= -1.0;
            }

            let force = plate_transform.rotation * force;

            // dbg!(force);

            velocity.0 += force * time.delta_seconds();
            transform.translation += force * time.delta_seconds() * time.delta_seconds() / 2.0;
        }
    }
}

pub fn apply_cylindrical_cathode_electric_field(
    time: Res<Time>,
    cylindrical_cathodes: Query<(&Transform, &CylindricalCathode, &Cylinder), Without<Electron>>,
    mut electrons: Query<(&mut Transform, &mut Velocity), With<Electron>>
){

    for (cylinder_transform, cylindrical_cathode, cylinder) in cylindrical_cathodes.iter() {
        for (mut transform, mut velocity) in electrons.iter_mut() {
            let r = (
                    (transform.translation.x - cylinder_transform.translation.x) *
                        (transform.translation.x - cylinder_transform.translation.x)
                        + transform.translation.z*transform.translation.z
                ).sqrt(); // electron position by radius

            let r2 = cylinder.inner_radius; // radius of anode (big cylinder)
            let ro = cylindrical_cathode.emmisivness as f32; // const: surface charge of the cylinder
            let e_field = 4.0 * PI * ro * (r + r2*r2/r); // resulting value of electric field
            let e_force = e_field * 1.60217663; // 1.60217663 × 10^(-19) - electron charge

            let vec_force = e_force * Vec3::new(transform.translation.x, 0.0, transform.translation.z);

            velocity.0 += vec_force * time.delta_seconds();
            transform.translation += vec_force * time.delta_seconds() * time.delta_seconds() / 2.0;
        }
    }
}
