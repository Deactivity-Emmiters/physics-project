use std::f32::consts::PI;
use bevy::prelude::*;
use crate::structs::{
    Electron, MagneticField,
    Plate, PlateCathode, Cylinder, CylindricalCathode,
    DestructionField, Velocity
};


pub fn apply_destruction_field(
    mut commands: Commands,
    plate_fields: Query<(&Transform, &DestructionField, &Plate), Without<Electron>>,
    cylindrical_fields: Query<(&Transform, &DestructionField, &Cylinder), Without<Electron>>,
    electrons: Query<(Entity, &Transform), With<Electron>>,
) {
    for (plate_transform, destruction_field, plate) in plate_fields.iter() {
        for (entity, transform) in electrons.iter() {
            // check if in range
            let rel_electron_pos = transform.translation - plate_transform.translation;
            let rel_electron_pos = plate_transform.rotation.inverse() * rel_electron_pos;
            if rel_electron_pos.x.abs() > plate.width / 2.0
                || rel_electron_pos.y.abs() > plate.height / 2.0
                || rel_electron_pos.z.abs() > destruction_field.depth
            {
                continue;
            }

            // destroy
            commands.entity(entity).despawn();
        }
    }

    for (cylinder_transform, destruction_field, cylinder) in cylindrical_fields.iter() {
        for (entity, transform) in electrons.iter() {
            // bad code. but we know that both centers of cylinders in position x=3 z=0
            let rel_electron_pos = (
                (transform.translation.x - 3.0)*(transform.translation.x-3.0)
                    + transform.translation.z*transform.translation.z
            ).sqrt();
            if rel_electron_pos > cylinder.inner_radius  &&
                rel_electron_pos < cylinder.outer_radius
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn cathodes_spawn_electrons(
    time: Res<Time>,
    mut spawn_timer: ResMut<crate::structs::SpawnTimer>,
    plate_cathodes: Query<(&Transform, &PlateCathode, &Plate)>,
    cylindrical_cathodes:  Query<(&Transform, &CylindricalCathode, &Cylinder)>,
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

    for (cylinder_transform, cylinder_cathode, cylinder) in cylindrical_cathodes.iter() {
        for _ in 0..cylinder_cathode.emmisivness {
            let phi = rand::random::<f32>() * 2.0 * PI - PI;
            let position = cylinder_transform.translation
                + cylinder_transform.rotation
                    * Vec3::new(
                        cylinder.outer_radius * phi.cos(),
                        (rand::random::<f32>() - 0.5) * cylinder.height,
                        cylinder.outer_radius * phi.sin(),
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


pub fn update_magnetic_field(
    ui_input: Res<crate::structs::UiState>,
    mut magnetic_fields: Query<&mut MagneticField>,
) {
    let phi = ui_input.phi_value.to_radians();
    let theta = ui_input.theta_value.to_radians();
    let b_value = ui_input.b_value;

    for mut field in magnetic_fields.iter_mut() {
        field.0 = Vec3::new(
            b_value * theta.sin() * phi.sin(),
            b_value * theta.cos(),
            b_value * theta.sin() * phi.cos(),
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