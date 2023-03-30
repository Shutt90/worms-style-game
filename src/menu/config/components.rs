use bevy::prelude::*;

use super::resources::Config;

#[derive(Component, Clone, Reflect)]
pub struct ConfigMenu{
    pub config: Config
}

#[derive(Component)]
pub struct MenuItem {}

impl Default for ConfigMenu {
    fn default() -> ConfigMenu {
        ConfigMenu {
            config: Config::default()
        }
    }
}