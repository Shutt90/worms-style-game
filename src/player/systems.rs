use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::resources::*;
use super::components::{Player, Aim};

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

pub fn spawn_aim_for_player(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
) {
    for mut player_transform in player_query.iter() {
        println!("players")
    }

    if let Ok(player_transform) = player_query.get_single() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(10.,10.)),
                    ..default()
                },
                transform: Transform::from_xyz(player_transform.translation.x + 15., player_transform.translation.y, 0.),
                ..default()
            },
            Aim {},
        ));
    } else {
        println!("failing")
    }
}
