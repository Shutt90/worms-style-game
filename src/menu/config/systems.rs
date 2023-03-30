use bevy::{prelude::*, window::PrimaryWindow, input::keyboard::KeyboardInput};
use super::resources::*;
use super::components::*;
use crate::constants::*;
use crate::menu::MenuState;

pub fn spawn_config_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    let main_menu: ConfigMenu = ConfigMenu::default();

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
        main_menu.clone(),
    ))
    .with_children(| parent | {
        for (i, val) in config.iter_fields().enumerate() {
            parent.spawn((NodeBundle{
                style: Style {
                    size: Size {
                        width: Val::Percent(CONFIG_MENU_ITEM_SCALING * 3.),
                        height: Val::Percent(CONFIG_MENU_ITEM_SCALING * 2.),
                    },
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(CONFIG_MENU_ITEM_SCALING / 100.)),
                    ..default()
                },
                ..default()
            },
            MenuItem{},
        ))
        .with_children(|parent| {
            parent.spawn((NodeBundle{
                style: Style {
                    size: Size {
                        width: Val::Percent(50.),
                        height: Val::Percent(CONFIG_MENU_ITEM_SCALING * 4.),
                    },
                    margin: UiRect::all(Val::Px(CONFIG_MENU_ITEM_SCALING / 100. * 3.5)),
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
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/Roboto-Thin.ttf"),
                        font_size: CONFIG_MENU_ITEM_SCALING * 2.5,
                        color: Color::WHITE,
                    },
                ),
                Label,
            ));
        });
        parent.spawn((NodeBundle{
            style: Style {
                size: Size {
                    width: Val::Percent(50.),
                    height: Val::Percent(CONFIG_MENU_ITEM_SCALING * 4.),
                },
                margin: UiRect::all(Val::Px(CONFIG_MENU_ITEM_SCALING / 100. * 3.5)),
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
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/Roboto-Thin.ttf"),
                        font_size: CONFIG_MENU_ITEM_SCALING * 2.5,
                        color: Color::WHITE,
                    },
                ),
                Label,
            ));
        });
    });
}
});
}

pub fn despawn_config_menu(
    mut commands: Commands,
    query_menu: Query<Entity, With<ConfigMenu>>,
    query_menu_items: Query<Entity, With<MenuItem>>,
    query_text_labels: Query<Entity, With<Label>>,    

) {
    // TODO: CLEAN THIS UP
    if let Ok(menu_entity) = query_menu.get_single() {
        commands.entity(menu_entity).despawn();
        for item in query_menu_items.iter() {
            commands.entity(item).despawn()
        }
        for label in query_text_labels.iter() {
            commands.entity(label).despawn()
        }
    }
}

pub fn click_menu_item(
    mut commands: Commands,
    mouse_click_events: Res<Input<MouseButton>>,
    query_list: Query<&GlobalTransform, With<MenuItem>>,
    window: Query<&Window, With<PrimaryWindow>>,
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
                            commands.insert_resource(NextState(Some(MenuState::Main)));
                        }
                    },
                    1=> {
                        if 
                            position.y >= menu_item.translation().y - calculated_menu_item_height && position.y <= menu_item.translation().y + calculated_menu_item_height &&
                            position.x >= menu_item.translation().x - calculated_menu_item_width && position.x <= menu_item.translation().x + calculated_menu_item_width
                        {
                            println!("pressed menu 3")
                        }
                    },
                    2=> {
                        if 
                            position.y >= menu_item.translation().y - calculated_menu_item_height && position.y <= menu_item.translation().y + calculated_menu_item_height &&
                            position.x >= menu_item.translation().x - calculated_menu_item_width && position.x <= menu_item.translation().x + calculated_menu_item_width
                        {
                            println!("pressed menu 2")
                        }
                    },
                    3=> {
                        if 
                            position.y >= menu_item.translation().y - calculated_menu_item_height && position.y <= menu_item.translation().y + calculated_menu_item_height &&
                            position.x >= menu_item.translation().x - calculated_menu_item_width && position.x <= menu_item.translation().x + calculated_menu_item_width
                        {
                            println!("pressed menu 1")
                        }
                    },
                    _=>println!("No menu item for this implemented")
                }
            }
        }
    }
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