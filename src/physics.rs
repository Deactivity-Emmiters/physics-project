use bevy::prelude::*;

use crate::structs::{
    Electron, MagneticField, Plate, PlateCathode, PlateDestructionField, Velocity,
};

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

pub fn apply_cathode_electric_field(
    time: Res<Time>,
    plate_cathodes: Query<(&Transform, &PlateCathode, &Plate), Without<Electron>>,
    mut electorns: Query<(&mut Transform, &mut Velocity), With<Electron>>,
) {
    for (plate_transform, plate_cathode, plate) in plate_cathodes.iter() {
        for (mut transform, mut velocity) in electorns.iter_mut() {
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

pub fn apply_desruction_field(
    mut commands: Commands,
    plate_fields: Query<(&Transform, &PlateDestructionField, &Plate), Without<Electron>>,
    electorns: Query<(Entity, &Transform), With<Electron>>,
) {
    for (plate_transform, plate_destruction_field, plate) in plate_fields.iter() {
        for (entity, transform) in electorns.iter() {
            // check if in range
            let rel_electron_pos = transform.translation - plate_transform.translation;
            let rel_electron_pos = plate_transform.rotation.inverse() * rel_electron_pos;
            if rel_electron_pos.x.abs() > plate.width / 2.0
                || rel_electron_pos.y.abs() > plate.height / 2.0
                || rel_electron_pos.z.abs() > plate_destruction_field.depth
            {
                continue;
            }

            // destroy
            commands.entity(entity).despawn();
        }
    }
}

pub fn cathodes_spawn_electrons(
    time: Res<Time>,
    mut spawn_timer: ResMut<crate::structs::SpawnTimer>,
    plate_cathodes: Query<(&Transform, &PlateCathode, &Plate)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut spawn = |position: Vec3, velocity: Vec3| {
        let mesh = meshes.add(Sphere::new(1.0).mesh().ico(3).unwrap());

        commands.spawn((
            PbrBundle {
                mesh,
                material: materials.add(Color::rgb(0.0, 0.0, 1.0)),
                transform: Transform::from_translation(position),
                ..Default::default()
            },
            Electron,
            Velocity(velocity),
        ));
    };

    if !spawn_timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for (plate_transform, plate_cathode, plate) in plate_cathodes.iter() {
        for _ in 0..plate_cathode.emmisivness {
            let position = plate_transform.translation
                + plate_transform.rotation
                    * Vec3::new(
                        (rand::random::<f32>() - 0.5) * plate.width,
                        (rand::random::<f32>() - 0.5) * plate.height,
                        (rand::random::<f32>() - 0.5) * plate.depth,
                    );
            let velocity = Vec3::new(
                (rand::random::<f32>() - 0.5) * 10.0,
                (rand::random::<f32>() - 0.5) * 10.0,
                (rand::random::<f32>() - 0.5) * 10.0,
            );

            spawn(position, velocity);
        }
    }
}

pub fn electon_repulsion(
    time: Res<Time>,
    mut electorns: Query<(&Transform, &mut Velocity), With<Electron>>,
) {
    let mut iter = electorns.iter_combinations_mut();
    while let Some([el1, el2]) = iter.fetch_next() {
        let (transform1, mut velocity1) = el1;
        let (transform2, mut velocity2) = el2;

        let rel_pos = transform1.translation - transform2.translation;

        let force = 100.0 / (rel_pos.length_squared()) * rel_pos.normalize();

        velocity1.0 += force * time.delta_seconds();
        velocity2.0 -= force * time.delta_seconds();
    }
}

pub fn update_magnetic_field(
    ui_input: Res<crate::structs::UiState>,
    mut magnetic_fields: Query<&mut MagneticField>,
) {
    let phi = ui_input.phi_value.to_radians();
    let theta = ui_input.theta_value.to_radians();
    let b_value = ui_input.b_value;

    for mut field in magnetic_fields.iter_mut() {
        field.0 = Vec3::new(
            b_value * theta.sin() * phi.cos(),
            b_value * theta.sin() * phi.sin(),
            b_value * theta.cos(),
        );
    }
}

pub fn update_electric_field(
    ui_input: Res<crate::structs::UiState>,
    mut plate_cathodes: Query<&mut PlateCathode>,
) {
    let e_value = ui_input.e_value;

    for mut cathode in plate_cathodes.iter_mut() {
        cathode.e_field = e_value;
    }
}
