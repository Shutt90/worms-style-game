use bevy::prelude::*;
use super::components::TurnTimer;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};


pub fn spawn_timer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let turn_timer = TurnTimer {
        timer: Timer::from_seconds(60., TimerMode::Once)
    };

    commands.spawn((
        TextBundle::from_section(
            "60",
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

pub fn restart_timer(
    mut timer_query: Query<&mut TurnTimer>,
    mut text_query: Query<&mut Text, With<TurnTimer>>,
    time: Res<Time>
) {
    if let Ok(mut turn_timer) = timer_query.get_single_mut() {
        turn_timer.timer.tick(time.delta());

        if let Ok(mut timer_text) = text_query.get_single_mut() {
            timer_text.sections[0].value = (60. - turn_timer.timer.elapsed_secs()).round().to_string();
        }
    }
}