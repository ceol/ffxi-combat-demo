use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate::{
    vitals::*,
    combat::*,
};

#[derive(Bundle)]
struct MonsterBundle {
    name: crate::vitals::Name,
    vitals: VitalsBundle,
    targetable: Targetable,
    mesh2d: MaterialMesh2dBundle<ColorMaterial>,
}
impl MonsterBundle {
    fn new(
        name: String,
        health: HealthPoints,
        mana: ManaPoints,
        mesh2d: MaterialMesh2dBundle<ColorMaterial>
    ) -> Self {
        Self {
            name: Name(name),
            vitals: VitalsBundle::new(health, mana, 0),
            mesh2d,
            ..default()
        }
    }
}
impl Default for MonsterBundle {
    fn default() -> Self {
        Self {
            name: Name("Monster".to_string()),
            vitals: VitalsBundle::new(25, 25, 0),
            targetable: Targetable,
            mesh2d: MaterialMesh2dBundle::default(),
        }
    }
}

fn monster_spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_batch(vec![
        MonsterBundle::new(
            "Tunnel Worm".to_string(),
            25, 25,
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(50., 100.)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::BEIGE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            },
        ),
        MonsterBundle::new(
            "Tunnel Worm".to_string(),
            25, 25,
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(50., 100.)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::BEIGE)),
                transform: Transform::from_translation(Vec3::new(60., 0., 0.)),
                ..default()
            },
        ),
        MonsterBundle::new(
            "Huge Hornet".to_string(),
            30, 30,
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(50., 50.)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::YELLOW)),
                transform: Transform::from_translation(Vec3::new(120., 25., 0.)),
                ..default()
            },
        ),
        MonsterBundle::new(
            "Huge Hornet".to_string(),
            30, 30,
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(50., 50.)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::YELLOW)),
                transform: Transform::from_translation(Vec3::new(180., 25., 0.)),
                ..default()
            },
        ),
    ]);
}

pub struct MonsterPlugin;
impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Startup,
                monster_spawn_system,
            );
    }
}