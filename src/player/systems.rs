use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use super::components::{Player, Aim, SPRITE_SIZE};

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
            Player {}
        )
    );
}

pub fn spawn_aim_for_player(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>
) {
    if let Ok(transform) = player_query.get_single() {
        println!("ola");
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(10.,10.)),
                    ..default()
                },
                transform: Transform::from_xyz(transform.translation.x + 15., transform.translation.y, 0.),
                ..default()
            },
            Aim {},
        ));
    } else {
        println!("no comprende")
    }
}
