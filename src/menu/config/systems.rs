use bevy::{prelude::*, input::keyboard::KeyboardInput};

use super::resources::*;
use super::components::*;

use crate::constants::*;

pub fn spawn_config_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let main_menu: ConfigMenu = ConfigMenu {
        node_list: &[
            "Difficulty",
            "Volume",
            "Fullscreen",
            "Controls",
            "Back"
        ]
    };

    commands.spawn((
        NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.)
                },
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        main_menu,
    ))
    .with_children(| parent | {
        for text in main_menu.node_list.iter() {
            parent.spawn((NodeBundle{
                style: Style {
                    size: Size {
                        width: Val::Percent(CONFIG_MENU_ITEM_SCALING * 2.),
                        height: Val::Percent(CONFIG_MENU_ITEM_SCALING),
                    },
                    margin: UiRect {
                        top: Val::Percent(CONFIG_MENU_ITEM_SCALING / 100. * 3.5),
                        bottom: Val::Px(CONFIG_MENU_ITEM_SCALING / 100. * 3.5),
                        ..default()
                    },
                    border: UiRect::all(Val::Px(CONFIG_MENU_ITEM_SCALING / 10.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            MenuItem{},
        ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        text.to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/Roboto-Thin.ttf"),
                            font_size: CONFIG_MENU_ITEM_SCALING * 2.5,
                            color: Color::WHITE,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    }),
                    Label,
                ));
            });
        }
    });
}

pub fn update_config(
    _config: Res<Config>,
    mut key_evr: EventReader<KeyboardInput>,
){
    use bevy::input::ButtonState;

    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Key press: {:?} ({})", ev.key_code, ev.scan_code);
            }
            ButtonState::Released => {
                println!("Key release: {:?} ({})", ev.key_code, ev.scan_code);
            }
        }
    }
}