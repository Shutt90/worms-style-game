use bevy::prelude::*;

pub fn spawn_main_menu(
    mut commands: Commands,
) {
    commands.spawn(NodeBundle {
        style: Style {
            size: Size {
                width: Val::Percent(100.),
                height: Val::Percent(100.)
            },
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }).with_children(| parent | {
        parent.spawn(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(40.),
                    height: Val::Percent(20.),
                },
                margin: UiRect {
                    top: Val::Percent(0.75),
                    bottom: Val::Px(0.75),
                    ..default()
                },
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::WHITE.into(),
            ..default()
        });
        parent.spawn(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(40.),
                    height: Val::Percent(20.),
                },
                margin: UiRect {
                    top: Val::Percent(0.75),
                    bottom: Val::Px(0.75),
                    ..default()
                },
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        });
        parent.spawn(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(40.),
                    height: Val::Percent(20.),
                },
                margin: UiRect {
                    top: Val::Percent(0.75),
                    bottom: Val::Px(0.75),
                    ..default()
                },
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::ORANGE_RED.into(),
            ..default()
        });
    });
}