use bevy::prelude::*; 
pub mod resources;
pub mod systems;
pub mod components;

use systems::*;
use crate::menu::config::resources::Config;

use super::MenuState;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build (&self, app: &mut App) {
        app.init_resource::<Config>()
            .add_system(spawn_config_menu.in_schedule(OnEnter(MenuState::Config)))
            .add_system(update_config);
    }
}

