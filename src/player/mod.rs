use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod resources;

use systems::*;
use resources::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build (&self, app: &mut App) {
        app.init_resource::<Power>()
            .add_startup_system(spawn_player)
            .add_system(control_aim.after(spawn_player))
            .add_system(add_power);
    }
}