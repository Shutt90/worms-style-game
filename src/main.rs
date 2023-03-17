use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod player;
mod target;

use player::PlayerPlugin;
use target::TargetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(TargetPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() /2., 0.),
            ..default()
        }
    );   
}