use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use super::components::*;
use crate::player::Power;
use bevy_rapier2d::prelude::*;

const AIM_SPEED: f32 = 10.;
const CROSSHAIR_DISTANCE_FROM_PLAYER: f32 = 70.;

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
                transform: Transform::from_xyz(player.x, player.y, player.z),
                ..default()
            },
            player,
        )
    );

    spawn_aim_for_player(commands, player.clone())
}

pub fn spawn_aim_for_player(
    mut commands: Commands,
    player: Player,
) {
    let crosshair: Aim = Aim {
        w: 5.,
        h: 5.
    };

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(crosshair.w, crosshair.h)),
                ..default()
            },
            transform: Transform::from_xyz(player.x + CROSSHAIR_DISTANCE_FROM_PLAYER, player.y, 0.),
            ..default()
        },
        crosshair,
    ));
    
}

pub fn control_aim(
    mut aim_query: Query<&mut Transform, With<Aim>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok(mut aim) = aim_query.get_single_mut()  {
        if keyboard_input.pressed(KeyCode::W) {
            aim.rotate_around(Vec3::new(window.width() / 2., window.height() / 2., 0.), Quat::from_rotation_z(1. * time.delta_seconds() * AIM_SPEED));
        }
        if keyboard_input.pressed(KeyCode::S) {
            aim.rotate_around(Vec3::new(window.width() / 2., window.height() / 2., 0.), Quat::from_rotation_z(-1. * -time.delta_seconds() * -AIM_SPEED));
        }
    }
}

pub fn add_power(
    commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut power: ResMut<Power>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        if power.reverse == false {
            power.total +=1.;
            if power.total == 100. {
                power.reverse = true;
            }
        } else if power.reverse == true {
            power.total -= 1.;
            if power.total == 0. {
                power.reverse = false;
            }
        }
    }

    println!("{:?}", power.total);

    if keyboard_input.just_released(KeyCode::Space) {
        fire_projectile(commands, power)
    }
}

pub fn fire_projectile(
    mut commands: Commands,
    mut power: ResMut<Power>,
) {
    commands.spawn(
        RigidBody::Dynamic
    ).insert(TransformBundle::from(Transform::from_xyz(0.0, 5.0, 0.0)))
    .insert(Velocity {
        linvel: Vec2::new(power.total, 0.),
        angvel: 0.2
    })
    .insert(GravityScale(0.5));

    println!("fired_projectile: {:?} distance", power.total);

    power.total = 0.;
    power.reverse = false;
}