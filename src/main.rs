use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemy)
        .add_startup_system(spawn_camera)

        .run();
}

pub struct BlockSize {
    w: f32,
    h: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Target;

const NUMBER_OF_ENEMIES: usize = 3;

const SPRITE_SIZE: BlockSize = BlockSize{w:25., h:100.};

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

fn spawn_player(
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

fn spawn_enemy(
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