use bevy::prelude::*;

mod vitals;
mod combat;
mod player;
mod monster;

#[derive(Resource, Default)]
struct GameState {
    
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            combat::CombatPlugins,
            player::PlayerPlugins,
            monster::MonsterPlugin,
        ))
        .init_resource::<GameState>()
        .run();
}
