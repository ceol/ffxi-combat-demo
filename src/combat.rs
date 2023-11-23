use std::marker::PhantomData;

use bevy::{
    prelude::*,
    app::PluginGroupBuilder,
};

use crate::vitals::*;

#[derive(Component)]
pub struct Target(pub Entity);

#[derive(Component)]
pub struct Targetable;

///
/// Damage
///

#[derive(Event)]
pub struct DamageEvent {
    pub source: Entity,
    pub target: Entity,
    pub amount: HealthPoints,
}
impl DamageEvent {
    pub fn new(source: Entity, target: Entity, amount: HealthPoints) -> Self {
        Self {
            source,
            target,
            amount,
        }
    }
}

fn damage_system(
    mut events: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health, Without<Downed>>,
    mut commands: Commands,
) {
    for event in events.iter() {
        let DamageEvent { source, target, amount } = event;
        if let Ok(mut health) = health_query.get_mut(*target) {
            if amount > &health.0 {
                println!("{:?} downing {:?}", source, target);
                health.0 = 0;
                commands
                    .entity(*target)
                    .remove::<Targetable>()
                    .insert(Downed::new(5.0));
            } else {
                health.0 = health.0 - amount;
            }
            println!(
                "{:?} damaged {:?} for {} points (now {})",
                source, target, amount, health.0,
            );
        } 
    }
}

struct DamagePlugin;
impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DamageEvent>()
            .add_systems(
                Update,
                (
                    damage_system,
                )
            );
    }
}

///
/// Downing
/// 

#[derive(Component)]
struct Downed {
    despawn_timer: Timer,
}
impl Downed {
    fn new(despawn: f32) -> Self {
        Self {
            despawn_timer: Timer::from_seconds(despawn, TimerMode::Once),
        }
    }
}
impl Default for Downed {
    fn default() -> Self {
        Self::new(5.0)
    }
}

fn despawn_downed_system(
    time: Res<Time>,
    mut downed_query: Query<(Entity, &mut Downed)>,
    mut commands: Commands,
) {
    for (entity, mut downed) in downed_query.iter_mut() {
        downed.despawn_timer.tick(time.delta());
        if downed.despawn_timer.just_finished() {
            println!("Despawning {:?}", entity);
            commands
                .entity(entity)
                .despawn();
        }
    }
}

struct DowningPlugin;
impl Plugin for DowningPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    despawn_downed_system,
                )
            );
    }
}

///
/// Attack
///

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Attack {
    target: Entity,
    damage: HealthPoints,
    cooldown: f32,
}
impl Attack {
    pub fn new(target: Entity, damage: HealthPoints, cooldown: f32) -> Self {
        Self {
            target,
            damage,
            cooldown,
        }
    }
}

fn attack_system(
    query: Query<(Entity, &Attack), Without<Cooldown<Attack>>>,
    mut events: EventWriter<DamageEvent>,
    mut commands: Commands,
) {
    for (source, attack) in query.iter() {
        let Attack { target, damage, cooldown } = *attack;
        println!("{:?} attacking {:?}", source, target);
        commands
            .entity(source)
            .remove::<Attack>()
            .insert(Cooldown::<Attack>::new(cooldown));
        events.send(
            DamageEvent::new(
                source,
                target, 
                damage,
            )
        );
    }
}

struct AttackPlugin;
impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                attack_system,
            );
    }
}

///
/// Cooldown
///

#[derive(Component)]
pub struct Cooldown<T: Component> {
    pub timer: Timer,
    _marker: PhantomData<T>,
}
impl<T: Component> Cooldown<T> {
    fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
            _marker: PhantomData,
        }
    }
}

fn cooldown_system<T: Component>(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Cooldown<T>)>,
    mut commands: Commands,
) {
    for (entity, mut cooldown) in query.iter_mut() {
        cooldown.timer.tick(time.delta());
        if cooldown.timer.finished() {
            commands.entity(entity).remove::<Cooldown<T>>();
        }
    }
}

struct CooldownPlugin;
impl Plugin for CooldownPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    cooldown_system::<Attack>,
                ),
            );
    }
}

pub struct CombatPlugins;
impl PluginGroup for CombatPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(DamagePlugin)
            .add(DowningPlugin)
            .add(CooldownPlugin)
            .add(AttackPlugin);
        group
    }
}