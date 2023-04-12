use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod resources;

use systems::*;
pub use resources::*;

use crate::AppState;
use super::TurnState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build (&self, app: &mut App) {
        app.init_resource::<Power>()
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_system(spawn_aim_for_character.in_schedule(OnEnter(TurnState::Player)))
            .add_system(control_aim.after(spawn_player))
            .add_system(add_power);
    }
}