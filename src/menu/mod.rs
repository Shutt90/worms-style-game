use bevy::prelude::*; 
mod systems;
pub mod components;
pub mod config;

use systems::*;

use crate::AppState;
use crate::menu::config::ConfigPlugin;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build (&self, app: &mut App) {
        app.add_plugin(ConfigPlugin)
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(click_menu_item.run_if(in_state(AppState::MainMenu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}

