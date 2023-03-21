use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::components::*;
use super::resources::*;
use crate::player::components::*;

use super::super::constants::*;

pub fn spawn_target(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let mut random_x = random::<f32>() * window.width();
        let mut random_y = random::<f32>() * window.height();

        // TODO: Change to player position > window
        while random_x <= window.width() / 2. + MIN_ENEMY_PIXELS_FROM_PLAYER && random_x >= window.width() / 2. - MIN_ENEMY_PIXELS_FROM_PLAYER {
            random_x = random::<f32>() * window.width();
        }

        while random_y <= window.height() / 2. + MIN_ENEMY_PIXELS_FROM_PLAYER && random_x >= window.height() / 2. - MIN_ENEMY_PIXELS_FROM_PLAYER {
            random_y = random::<f32>() * window.height();
        }

        commands.spawn(
            (
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::new(SPRITE_SIZE.w, SPRITE_SIZE.h)),
                        ..default()
                    },
                    transform: Transform::from_xyz(random_x, random_y, 0.),
                    ..default()
                },
                Target{},
            )
        );
    }
}