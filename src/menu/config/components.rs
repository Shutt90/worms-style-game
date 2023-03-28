use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Component, Clone)]
pub struct ConfigMenu{
    pub node_list: HashMap<String, String>
}

#[derive(Component)]
pub struct MenuItem {}

impl Default for ConfigMenu {
    fn default() -> ConfigMenu {
        ConfigMenu {
            node_list: HashMap::new()
        }
    }
}