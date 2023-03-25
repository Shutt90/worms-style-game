use bevy::prelude::*;
use super::components::TurnTimer;

pub fn spawn_timer(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let turn_timer = TurnTimer{
        timer: Timer::from_seconds(60., TimerMode::Once)
    };

    commands.spawn((
        TextBundle::from_section(
            "timer goes here",
            TextStyle {
                font: asset_server.load("fonts/Roboto-Black.ttf"),
                font_size: 100.0,
                color: Color::BLACK,
            },
        )
        .with_text_alignment(TextAlignment::Center)
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
        turn_timer,
    ));
}