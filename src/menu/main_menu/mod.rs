use bevy::prelude::*; 
mod systems;
pub mod components;

use systems::*;

use crate::AppState;
use crate::menu::MenuState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build (&self, app: &mut App) {
        app.add_system(spawn_main_menu.in_schedule(OnEnter(MenuState::Main)))
            .add_system(click_menu_item.run_if(in_state(AppState::MainMenu)).run_if(in_state(MenuState::Main)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}

