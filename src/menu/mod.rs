use bevy::prelude::*; 
mod systems;
pub mod components;

use systems::*;

use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build (&self, app: &mut App) {
        app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(click_menu_item);
    }
}

