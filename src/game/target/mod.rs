use bevy::prelude::*;

pub mod resources;
pub mod components;
pub mod systems;

use systems::*;

use crate::AppState;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build (&self, app: &mut App) {
        app
        .add_system(spawn_target.in_schedule(OnEnter(AppState::Game)))
        .add_system(destroy_target);
    }
}
