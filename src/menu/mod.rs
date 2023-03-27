use bevy::prelude::*; 

pub mod main_menu;
pub mod config;

use crate::menu::main_menu::MainMenuPlugin;
use crate::menu::config::ConfigPlugin;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build (&self, app: &mut App) {
        app.add_state::<MenuState>()
        .add_plugin(MainMenuPlugin)
        .add_plugin(ConfigPlugin);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MenuState {
    #[default]
    Main,
    Config,
    Controls,
    Exited
}