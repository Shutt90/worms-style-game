use bevy::prelude::*; 


mod systems;

use systems::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build (&self, app: &mut App) {
        app.add_startup_system(say_hi);
    }
}

