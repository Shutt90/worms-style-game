use bevy::{prelude::*, window::PrimaryWindow};
use bevy::app::AppExit;

use crate::AppState;

use super::components::*;
use crate::constants::*;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let main_menu: MainMenu = MainMenu {
        node_list: &[
            "Play Game",
            "Practice Mode",
            "Settings",
            "Exit Game"
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
                        width: Val::Percent(MENU_ITEM_SCALING * 2.),
                        height: Val::Percent(MENU_ITEM_SCALING),
                    },
                    margin: UiRect {
                        top: Val::Percent(MENU_ITEM_SCALING / 100. * 3.5),
                        bottom: Val::Px(MENU_ITEM_SCALING / 100. * 3.5),
                        ..default()
                    },
                    border: UiRect::all(Val::Px(MENU_ITEM_SCALING / 10.)),
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
                // text
                parent.spawn((
                    TextBundle::from_section(
                        text.to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/Roboto-Thin.ttf"),
                            font_size: MENU_ITEM_SCALING * 2.5,
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

pub fn click_menu_item(
    mut commands: Commands,
    mouse_click_events: Res<Input<MouseButton>>,
    query_list: Query<&GlobalTransform, With<MenuItem>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    let window = window.get_single().unwrap();
    
    if mouse_click_events.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            let calculated_menu_item_height = window.height() / MENU_ITEM_SCALING * 2.;
            let calculated_menu_item_width = window.width() / MENU_ITEM_SCALING * 4.;

            for (i, menu_item) in query_list.iter().enumerate() {
                match i{
                    0=> {
                        if 
                            position.y >= menu_item.translation().y - calculated_menu_item_height && position.y <= menu_item.translation().y + calculated_menu_item_height &&
                            position.x >= menu_item.translation().x - calculated_menu_item_width && position.x <= menu_item.translation().x + calculated_menu_item_width
                        {
                            println!("{:?}, {:?}. {:?}", position.x, menu_item.translation().x - calculated_menu_item_width, menu_item.translation().x + calculated_menu_item_width);
                            app_exit_event_writer.send(AppExit)
                        }
                    },
                    1=> {
                        if 
                            position.y >= menu_item.translation().y - calculated_menu_item_height && position.y <= menu_item.translation().y + calculated_menu_item_height &&
                            position.x >= menu_item.translation().x - calculated_menu_item_width && position.x <= menu_item.translation().x + calculated_menu_item_width
                        {
                            println!("menu item 3")
                        }
                    },
                    2=> {
                        if 
                            position.y >= menu_item.translation().y - calculated_menu_item_height && position.y <= menu_item.translation().y + calculated_menu_item_height &&
                            position.x >= menu_item.translation().x - calculated_menu_item_width && position.x <= menu_item.translation().x + calculated_menu_item_width
                        {
                            commands.insert_resource(NextState(Some(AppState::Game)));
                        }
                    },
                    3=> {
                        if 
                            position.y >= menu_item.translation().y - calculated_menu_item_height && position.y <= menu_item.translation().y + calculated_menu_item_height &&
                            position.x >= menu_item.translation().x - calculated_menu_item_width && position.x <= menu_item.translation().x + calculated_menu_item_width
                        {
                            commands.insert_resource(NextState(Some(AppState::Game)));
                        }
                    },
                    _=>println!("No menu item for this implemented")
                }
            }
        }
    }
}