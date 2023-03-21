use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use super::components::*;
use crate::player::Power;
use bevy_rapier2d::prelude::*;

use super::super::constants::*;

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
    window_query: Query<&Window, With<PrimaryWindow>>,
    aim_query: Query<&Transform, With<Aim>>,
) {
    let window = window_query.get_single().unwrap();
    if keyboard_input.pressed(KeyCode::Space) {
        if power.reverse == false {
            power.total += MISSILE_POWER_VELOCITY;
            if power.total == MISSILE_POWER_VELOCITY * MISSILE_POWER_VELOCITY_MAX_MULTIPLIER {
                power.reverse = true;
            }
        } else if power.reverse == true {
            power.total -= MISSILE_POWER_VELOCITY;
            if power.total == 0. {
                power.reverse = false;
            }
        }
    }

    if keyboard_input.just_released(KeyCode::Space) {
        let position = calculate_position_of_crosshair(aim_query, window);
        fire_projectile(commands, power, window, position)
    }
}

pub fn fire_projectile(
    mut commands: Commands,
    mut power: ResMut<Power>,
    window: &Window,
    position: Position
) {
    commands.spawn(
        RigidBody::Dynamic
    )
    .insert(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(MISSLE_SIZE, MISSLE_SIZE)),
            ..default()
        },
        ..default()
    })
    .insert(Velocity {
        linvel: Vec2::new(position.x * power.total, position.y* power.total),
        angvel: 0.6
    })
    .insert(TransformBundle::from(Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0)))
    .insert(GravityScale(1.))
    .insert(AdditionalMassProperties::Mass(MISSILE_MASS))
    .insert(Collider::cuboid(MISSLE_SIZE / 2., MISSLE_SIZE / 2.));

    println!("fired_projectile: {:?} distance", power.total);

    power.total = 0.;
    power.reverse = false;
}

pub fn calculate_position_of_crosshair(
    aim_query: Query<&Transform, With<Aim>>,
    window: &Window
) -> Position{
    let mut pos: Position =  Position { x: 0., y: 0.};

    if let Ok(aim) = aim_query.get_single()  {
        pos.x = aim.translation.x - window.width() / 2.;
        pos.y = aim.translation.y - window.height() / 2.;
    }

    return pos
}