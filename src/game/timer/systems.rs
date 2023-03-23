use bevy::prelude::*;
use super::components::TurnTimer;

pub fn spawn_timer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    commands.spawn((
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "60",
            TextStyle {
                font: asset_server.load("fonts/Roboto-Black.ttf"),
                font_size: 100.0,
                color: Color::BLACK,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.),
                left: Val::Px(5.),
                ..default()
            },
            ..default()
        }
    ),
        TurnTimer{},
    ));
}