use bevy::app::PluginGroupBuilder;
use bevy::{
    prelude::*,
    input::common_conditions::input_just_pressed,
    sprite::MaterialMesh2dBundle
};

use crate::vitals::*;
use crate::combat::*;

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    name: crate::vitals::Name,
    vitals: VitalsBundle,
    targetable: Targetable,
    mesh2d: MaterialMesh2dBundle<ColorMaterial>,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            name: Name("Player".to_string()),
            vitals: VitalsBundle::new(100, 100, 0),
            targetable: Targetable,
            mesh2d: MaterialMesh2dBundle::default(),
        }
    }
}

#[derive(Component)]
struct TargetIndicator;

#[derive(Bundle)]
struct TargetIndicatorBundle {
    indicator: TargetIndicator,
    mesh2d: MaterialMesh2dBundle<ColorMaterial>,
}
impl Default for TargetIndicatorBundle {
    fn default() -> Self {
        Self {
            indicator: TargetIndicator,
            mesh2d: MaterialMesh2dBundle::default(),
        }
    }
}

fn player_spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        PlayerBundle {
            mesh2d: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_xyz(-150., 0., 0.),
                ..default()
            },
            ..default()
        }
    );
}

fn player_change_target_system(
    mut player_target_query: Query<(Entity, Option<&mut Target>), With<Player>>,
    targetable_query: Query<Entity, With<Targetable>>,
    mut commands: Commands,
) {
    for (player, maybe_current) in player_target_query.iter_mut() {
        if let Some(mut current_target) = maybe_current {
            println!("Player {:?} current target: {:?}", player, current_target.0);
            
            // "Tab through" each targetable entity
            let mut all_targets = Vec::<Entity>::new();
            let mut current_target_index = 0;
            for (index, targetable) in targetable_query.iter().enumerate() {
                all_targets.push(targetable);
                if targetable == current_target.0 {
                    current_target_index = index;
                }
            }
            
            // Wrap around to the beginning
            let new_target = all_targets[(current_target_index + 1) % all_targets.len()];
            println!("Targeting {:?}", new_target);
            current_target.0 = new_target;
        } else {
            println!("Player {:?} has no target", player);
            for targetable in targetable_query.iter() {
                println!("Targeting {:?}", targetable);
                commands
                    .entity(player)
                    .insert(Target(targetable));
                break;
            }
        }
    }
}

fn player_spawn_target_indicator_system(
    target_query: Query<&Target, With<Player>>,
    transform_query: Query<&Transform>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok(Target(target)) = target_query.get_single() {
        if let Ok(transform) = transform_query.get_component::<Transform>(*target) {
            commands.spawn(TargetIndicatorBundle {
                mesh2d: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y + 80.,
                        transform.translation.z,
                    ),
                    ..default()
                },
                ..default()
            });
        }
    }
}

fn player_despawn_target_indicator_system(
    indicator_query: Query<Entity, With<TargetIndicator>>,
    mut commands: Commands,
) {
    indicator_query.for_each(|indicator| {
        commands.entity(indicator).despawn();
    });
}

fn player_clear_target_system(
    player_query: Query<Entity, (With<Player>, With<Target>)>,
    mut commands: Commands,
) {
    for player in player_query.iter() {
        println!("Player {:?} clearing target", player);
        commands
            .entity(player)
            .remove::<Target>();
    }
}

fn player_attack_target_system(
    player_query: 
        Query<
            (Entity, &Target), 
            (With<Player>, Without<Cooldown<Attack>>)
        >,
    mut commands: Commands,
) {
    for (
        player,
        Target(target),
    ) in player_query.iter() {
        println!("Player {:?} attacking {:?}", player, target);
        if player == *target {
            println!("Can't attack yourself!");
            continue;
        }
        commands
            .entity(player)
            .insert(
                Attack::new(
                    *target,
                    10, 
                    1.0,
                ),
            );
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Startup,
                player_spawn_system,
            )
            .add_systems(
                Update, 
                (
                    (
                        player_change_target_system,
                    )
                        .run_if(
                            input_just_pressed(KeyCode::Tab)
                        ),
                    (
                        player_clear_target_system,
                    )
                        .run_if(
                            input_just_pressed(KeyCode::Escape)
                        ),
                    player_attack_target_system
                        .run_if(
                            input_just_pressed(KeyCode::Space)
                        ),
                ),
            )
        ;
    }
}

pub struct PlayerUIPlugin;
impl Plugin for PlayerUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    player_despawn_target_indicator_system
                        .run_if(
                            input_just_pressed(KeyCode::Escape)
                        ),
                )
            )
            .add_systems(
                PostUpdate,
                (
                    (
                        player_despawn_target_indicator_system,
                        player_spawn_target_indicator_system,
                    )
                        .run_if(
                            input_just_pressed(KeyCode::Tab)
                        ),
                ),
            )
        ;
    }
}

pub struct PlayerPlugins;
impl PluginGroup for PlayerPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(PlayerPlugin)
            .add(PlayerUIPlugin)
        ;
        group
    }
}