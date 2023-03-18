use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build (&self, app: &mut App) {
        app.add_startup_system(spawn_player.before(spawn_aim_for_player))
            .add_startup_system(spawn_aim_for_player)
            .add_system(check_positons);
    }
}