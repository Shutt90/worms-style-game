use bevy::prelude::*;

#[derive(Resource, Clone, Reflect)]
pub struct Config {
    pub difficulty: String,
    pub volume: i8,
    pub fullscreen: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            difficulty: "Medium".to_string(),
            fullscreen: "Off".to_string(),
            volume: 2
        }
    }
}