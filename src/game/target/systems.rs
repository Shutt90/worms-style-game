use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;
use super::resources::*;
use crate::game::player::components::*;

use crate::constants::*;

pub fn spawn_target(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
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
                        custom_size: Some(Vec2::new(86., 68.)),
                        ..default()
                    },
                    texture: asset_server.load("sprites/tanks_tankDesert1.png"),
                    transform: Transform::from_xyz(random_x, random_y, 0.),
                    ..default()
                },
                Target{},
            )
        )
        .insert(Collider::cuboid(SPRITE_SIZE.w / 2., SPRITE_SIZE.h / 2.))
        .insert(ActiveEvents::COLLISION_EVENTS);
    }
}

pub fn destroy_target(
    mut commands: Commands,
    mut target_query: Query<Entity, With<Target>>,
    missile_query: Query<Entity, With<Missile>>,
    rapier_context: Res<RapierContext>,
) {
    for missile_entity in missile_query.iter()  {
        for target_entity in target_query.iter_mut()  {
            if let Some(_contact_pair) = rapier_context.contact_pair(missile_entity, target_entity) {
                commands.entity(target_entity).despawn();
                commands.entity(missile_entity).despawn();
            }
        }
    }
}