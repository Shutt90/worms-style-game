use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::components::*;
use super::resources::*;
use crate::player::resources::SPRITE_SIZE;

pub fn spawn_target(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

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