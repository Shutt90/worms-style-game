use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::components::*;
use super::resources::*;
use crate::player::resources::SPRITE_SIZE;
use crate::player::components::*;

pub fn spawn_target(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>
) {
    let window = window_query.get_single().unwrap();

    if let Ok(player_transform) = player_query.get_single() {
        for _ in 0..NUMBER_OF_ENEMIES {
            let mut random_x = random::<f32>() * window.width();
            let mut random_y = random::<f32>() * window.height();

            while random_x <= player_transform.translation.x + 10. || random_x >= player_transform.translation.x - 10. {
                random_x = random::<f32>() * window.width();
            }

            while random_x <= player_transform.translation.x + 10. || random_x >= player_transform.translation.x - 10. {
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
    } else {
        println!("wtf")
    }
}