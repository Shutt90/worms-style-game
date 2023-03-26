use bevy::prelude::*;

const NODE_LIST: &'static [&'static str] = &[
    "Play Game",
    "Practice Mode",
    "Settings",
    "Game Over"
];

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
        for val in NODE_LIST {
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
            })
            .with_children(|parent| {
                // text
                parent.spawn((
                    TextBundle::from_section(
                        val.to_string(),
                        TextStyle {
                            font: asset_server.load("src/fonts/Roboto-Thin.ttf"),
                            font_size: 20.0,
                            color: Color::BLUE,
                        },
                    )
                    .with_style(Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    }),
                    Label,
                ));
            });
        }
    });
}