use bevy::prelude::*;

#[derive(Component)]
pub struct Name(pub String);

pub type HealthPoints = usize;

#[derive(Component)]
pub struct Health(pub HealthPoints);

pub type ManaPoints = usize;

#[derive(Component)]
pub struct Mana(pub ManaPoints);

pub type TacticalPoints = usize;

#[derive(Component)]
pub struct Tactical(pub TacticalPoints);

#[derive(Bundle)]
pub struct VitalsBundle {
    health: Health,
    mana: Mana,
    tactical: Tactical,
}
impl VitalsBundle {
    pub fn new(health: HealthPoints, mana: ManaPoints, tactical: TacticalPoints) -> Self {
        Self {
            health: Health(health),
            mana: Mana(mana),
            tactical: Tactical(tactical),
        }
    }
}