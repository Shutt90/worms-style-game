use bevy::prelude::*;

pub mod resources;
pub mod components;
pub mod systems;

use systems::*;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build (&self, app: &mut App) {
        app.add_startup_system(spawn_target);
    }
}
