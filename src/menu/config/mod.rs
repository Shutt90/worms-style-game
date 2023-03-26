use bevy::prelude::*; 
pub mod resources;
pub mod systems;

use systems::*;
use crate::menu::config::resources::Config;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build (&self, app: &mut App) {
        app.init_resource::<Config>()
            .add_system(update_config);
    }
}

