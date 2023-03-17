use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::resources::*;
use super::components::*;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(SPRITE_SIZE.w, SPRITE_SIZE.h)),
                    ..default()
                },
                transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
                ..default()
            },
            Player{},
        )
    );
}