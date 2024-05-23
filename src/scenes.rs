use bevy::prelude::*;

use crate::structs::{
    Cylinder, CylindricalCathode, DestructionField, Electron, Plate, PlateCathode,
};

#[derive(Component)]
struct CylindricalDiodeSceneEntity;

#[derive(Component)]
struct PlateDiodeSceneEntity;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy, States)]
pub enum SelectedScene {
    #[default]
    CylindricalDiode,
    PlateDiode,
}

pub fn scenes_plugin(app: &mut App) {
    app.init_state::<SelectedScene>()
        .add_systems(
            OnEnter(SelectedScene::CylindricalDiode),
            setup_cylindrical_diode,
        )
        .add_systems(
            OnExit(SelectedScene::CylindricalDiode),
            despawn_scene::<CylindricalDiodeSceneEntity>,
        )
        .add_systems(OnEnter(SelectedScene::PlateDiode), setup_plate_diode)
        .add_systems(
            OnExit(SelectedScene::PlateDiode),
            despawn_scene::<PlateDiodeSceneEntity>,
        );
}

fn despawn_scene<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
    electrons: Query<Entity, With<Electron>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in electrons.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_plate_diode(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const HEIGHT: f32 = 200.0;
    const WIDTH: f32 = 80.0;
    const CATHODE_POS: Vec3 = Vec3::new(15.0, 0.0, 0.0);
    let cathode_rot: Quat = Quat::from_rotation_y(0.5 * std::f32::consts::PI);
    const ANODE_POS: Vec3 = Vec3::new(-15.0, 0.0, 0.0);
    let anode_rot: Quat = Quat::from_rotation_y(0.5 * std::f32::consts::PI);

    // cathode plate
    let plate_cathode = PlateCathode {
        e_field: 10.0,
        emmisivness: 80,
    };
    let plate = Plate {
        height: HEIGHT,
        width: WIDTH,
        depth: 1.0,
    };
    let plate_transform = Transform {
        translation: CATHODE_POS,
        rotation: cathode_rot,
        ..default()
    };
    let mesh = meshes.add(Mesh::from(Cuboid::new(
        plate.width,
        plate.height,
        plate.depth,
    )));
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(Color::rgb(0.0, 1.0, 0.0)),
            transform: plate_transform,
            ..Default::default()
        },
        plate_cathode,
        plate,
        DestructionField { depth: 0.2 },
        PlateDiodeSceneEntity,
    ));

    // anode plate
    let plate = Plate {
        height: HEIGHT,
        width: WIDTH,
        depth: 1.0,
    };
    let plate_transform = Transform {
        translation: ANODE_POS,
        rotation: anode_rot,
        ..default()
    };
    let mesh = meshes.add(Mesh::from(Cuboid::new(
        plate.width,
        plate.height,
        plate.depth,
    )));
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            transform: plate_transform,
            ..Default::default()
        },
        plate,
        DestructionField { depth: 0.8 },
        PlateDiodeSceneEntity,
    ));

    // bounding box, destruction panels
    spawn_dp(
        &mut commands,
        Vec3::new(0.0, 0.0, WIDTH / 2.0),
        Quat::default(),
        PlateDiodeSceneEntity,
    );
    spawn_dp(
        &mut commands,
        Vec3::new(0.0, 0.0, -WIDTH / 2.0),
        Quat::default(),
        PlateDiodeSceneEntity,
    );
    spawn_dp(
        &mut commands,
        Vec3::new(0.0, HEIGHT / 2.0, 0.0),
        Quat::from_rotation_x(0.5 * std::f32::consts::PI),
        PlateDiodeSceneEntity,
    );
    spawn_dp(
        &mut commands,
        Vec3::new(0.0, -HEIGHT / 2.0, 0.0),
        Quat::from_rotation_x(0.5 * std::f32::consts::PI),
        PlateDiodeSceneEntity,
    );
    spawn_dp(
        &mut commands,
        CATHODE_POS + Vec3::new(1.0, 0.0, 0.0),
        cathode_rot,
        PlateDiodeSceneEntity,
    );
}

fn spawn_dp(commands: &mut Commands, pos: Vec3, rot: Quat, scene_component: impl Component) {
    let plate = Plate {
        height: 1000000.0,
        width: 1000000.0,
        depth: 1.0,
    };
    let plate_transform = Transform {
        translation: pos,
        rotation: rot,
        ..default()
    };
    commands.spawn((
        plate_transform,
        plate,
        DestructionField { depth: 1.0 },
        scene_component,
    ));
}

fn setup_cylindrical_diode(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    const HEIGHT: f32 = 200.0;
    const WIDTH: f32 = 80.0;

    // cathode cylinder
    let mesh = meshes.add(Mesh::from(bevy::prelude::Cylinder {
        radius: 5.0,
        half_height: HEIGHT / 2.0,
    }));
    let cylindrical_cathode = CylindricalCathode {
        e_field: 10.0,
        emmisivness: 80,
    };
    let cylinder = Cylinder {
        inner_radius: 0.0,
        outer_radius: 5.0,
        height: HEIGHT,
    };
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(Color::rgb(0.0, 1.0, 0.0)),
            transform: Transform {
                translation: Vec3::new(3.0, 0.0, 0.0),
                ..default()
            },
            ..Default::default()
        },
        cylinder,
        cylindrical_cathode,
        DestructionField { depth: 0.2 },
        CylindricalDiodeSceneEntity,
    ));

    // anode cylinder

    // in this model: height = 20, inner_radius: 9.0, outer_radius: 10.0
    let model = asset_server.load("models/Hole_Cylinder.gltf#Scene0");

    let cylinder = Cylinder {
        inner_radius: 27.0,
        outer_radius: 30.0,
        height: HEIGHT,
    };
    commands.spawn((
        SceneBundle {
            scene: model.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, -100.0, 0.0),
                scale: Vec3::new(3.0, 10.0, 3.0),
                ..default()
            },
            ..default()
        },
        cylinder,
        DestructionField { depth: 0.8 },
        CylindricalDiodeSceneEntity,
    ));

    // bounding box, destruction panels
    spawn_dp(
        &mut commands,
        Vec3::new(0.0, HEIGHT / 2.0, 0.0),
        Quat::from_rotation_x(0.5 * std::f32::consts::PI),
        CylindricalDiodeSceneEntity,
    );
    spawn_dp(
        &mut commands,
        Vec3::new(0.0, -HEIGHT / 2.0, 0.0),
        Quat::from_rotation_x(0.5 * std::f32::consts::PI),
        CylindricalDiodeSceneEntity,
    );
}
