use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

use crate::AppState;

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build (&self, app: &mut App) {
        app
        .add_system(spawn_timer.in_schedule(OnEnter(AppState::Game)))
        .add_system(restart_timer);
    }
}
