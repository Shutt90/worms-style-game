use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::player::Power;

use super::components::*;

use super::super::constants::*;


pub fn spawn_bar(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    const POWER_BAR: PowerBar = PowerBar{w: 25.};

    commands.spawn(
        (SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(POWER_BAR.w, window.height() / 6.)),
                ..default()
            },
            transform: Transform::from_xyz(window.width() - POWER_BAR.w, window.height() / 2., 0.),
            ..default()
        },
        POWER_BAR
    ));
}

pub fn fill_bar(
    power: Res<Power>,
    mut power_bar_query: Query<(&mut Transform, &mut Sprite), With<PowerBar>>
) {
    if let Ok((mut transform, mut sprite)) = power_bar_query.get_single_mut()  {
        transform.scale.y = power.total;
        if power.total > MISSILE_POWER_VELOCITY * MISSILE_POWER_VELOCITY_MAX_MULTIPLIER / 2. {
            sprite.color = Color::GREEN;
        } else if power.total < MISSILE_POWER_VELOCITY * MISSILE_POWER_VELOCITY_MAX_MULTIPLIER && power.total < MISSILE_POWER_VELOCITY * MISSILE_POWER_VELOCITY_MAX_MULTIPLIER / 2.{
            sprite.color = Color::RED;
        }
    }
}