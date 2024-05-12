use bevy::{prelude::*, utils::HashMap};

use crate::structs::{Electron, Velocity};

const CELL_SIZE: f32 = 10.0;

const ELECTRON_REPULSION_FORCE: f32 = 100.0;

#[derive(Resource, Default)]
pub struct ElectronChunks(HashMap<IVec3, Vec<ElectronRepr>>);

pub struct ElectronRepr {
    pub position: Vec3,
    pub id: Entity,
}

pub fn world_pos_to_chunk_pos(pos: Vec3) -> IVec3 {
    IVec3 {
        x: pos.x.div_euclid(CELL_SIZE) as i32,
        y: pos.y.div_euclid(CELL_SIZE) as i32,
        z: pos.z.div_euclid(CELL_SIZE) as i32,
    }
}

pub fn update_electron_chunks(
    mut chunks: ResMut<ElectronChunks>,
    electrons: Query<(Entity, &Transform), With<Electron>>,
) {
    let chunks = &mut chunks.0;
    chunks.clear();

    for (id, transform) in electrons.iter() {
        let pos = world_pos_to_chunk_pos(transform.translation);

        chunks.entry(pos).or_default().push(ElectronRepr {
            position: transform.translation,
            id,
        });
    }
}

pub fn electron_repulsion(
    time: Res<Time>,
    chunks: Res<ElectronChunks>,
    mut electrons: Query<&mut Velocity, With<Electron>>,
) {
    let chunks = &chunks.0;
    for chunk in chunks.iter() {
        let neighbor_chunks = (-1..=1)
            .flat_map(|x| (-1..=1).flat_map(move |y| (-1..=1).map(move |z| IVec3::new(x, y, z))))
            .filter(|v| *v != IVec3::new(0, 0, 0))
            .map(|v| v + *chunk.0)
            .filter_map(|v| chunks.get(&v))
            .collect::<Vec<_>>();
        for electron in chunk.1 {
            let mut electron_velocity = match electrons.get_mut(electron.id) {
                Ok(v) => v,
                Err(e) => {
                    warn!("Dead electron: {}", e);
                    continue;
                }
            };
            for other in neighbor_chunks
                .iter()
                .flat_map(|v| *v)
                .chain(chunk.1.iter().filter(|e| e.id != electron.id))
            {
                let rel_pos = other.position - electron.position;
                let force =
                    -ELECTRON_REPULSION_FORCE / (rel_pos.length_squared()) * rel_pos.normalize();
                electron_velocity.0 += force * time.delta_seconds();
            }
        }
    }
}
