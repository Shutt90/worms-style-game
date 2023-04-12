use bevy::prelude::*;

pub mod player;
mod power_bar;
pub mod target;
pub mod timer;

use player::*;
use power_bar::*;
use target::*;
use timer::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build (&self, app: &mut App) {
        app
        .add_state::<PlayingState>()
        .add_state::<TurnState>()
        .add_plugin(PlayerPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(PowerBarPlugin)
        .add_plugin(TimerPlugin);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PlayingState {
    Running,
    #[default]
    Paused,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum TurnState {
    #[default]
    Player,
    Enemy,
}