use bevy::prelude::*;

#[derive(Resource)]
pub struct Config {
    pub controls: Controls,
    pub fullscreen: bool,
}

#[derive(Resource)]
pub struct Controls {
    pub up: KeyCode,
    pub down: KeyCode,
    pub fire: KeyCode
}

impl Default for Config {
    fn default() -> Config {
        Config {
            fullscreen: false,
            controls: Controls::default()
        }
    }
}

impl Default for Controls {
    fn default() -> Controls {
        Controls {
            up: KeyCode::W,
            down: KeyCode::S,
            fire: KeyCode::Space
        }
    }
}
