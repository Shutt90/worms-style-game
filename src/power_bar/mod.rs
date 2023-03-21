use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub struct PowerBarPlugin;

impl Plugin for PowerBarPlugin {
    fn build (&self, app: &mut App) {
        app.add_startup_system(spawn_bar)
            .add_system(fill_bar);
    }
}