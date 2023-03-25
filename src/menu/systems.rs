use bevy::prelude::*;

pub fn say_hi(
    mut commands: Commands,
) {
    commands.spawn(NodeBundle {
        style: Style {
            size: Size {
                width: Val::Px(150.),
                height: Val::Px(25.)
            },
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    }).with_children(| parent | {
        parent.spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(50.)),
                border: UiRect::all(Val::Percent(100.))
                ,..default()
            },
            background_color: Color::ALICE_BLUE.into(),
            ..default()
        });
    });
}