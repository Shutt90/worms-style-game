use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use super::components::*;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let player = Player{
        x: window.width() / 2.,
        y: window.height() / 2.,
        z: 0.
    };

    commands.spawn(
        (
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(SPRITE_SIZE.w, SPRITE_SIZE.h)),
                    ..default()
                },
                transform: Transform::from_xyz(player.x, player.x, player.z),
                ..default()
            },
            player,
        )
    );
}

pub fn spawn_aim_for_player(
    player_query: Query<&Player>,
    mut commands: Commands,
) {
    let crosshair: Aim = Aim {
        w: 15.,
        h: 15.
    };

    if let Ok(player) = player_query.get_single() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(10.,10.)),
                    ..default()
                },
                transform: Transform::from_xyz(player.x + 15. - crosshair.w / 2., player.y - crosshair.h / 2., player.z),
                ..default()
            },
            crosshair,
        ));
    } else {
        panic!("error: '{:?}'", player_query.get_single());
    }
}

pub fn check_positons(
    player_query: Query<&Transform, With<Player>>,
    aim_query: Query<&Transform, With<Aim>>
) {
    if let Ok(player)= player_query.get_single()  {
        if let Ok(aim) = aim_query.get_single()  {
            println!("{}, {}", aim.translation.x, player.translation.x); 
        }
    }
}