use bevy::prelude::*;

pub mod player;
mod power_bar;
pub mod target;

use player::*;
use power_bar::*;
use target::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build (&self, app: &mut App) {
        app
        .add_plugin(PlayerPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(PowerBarPlugin);
    }
}

