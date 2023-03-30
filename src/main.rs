use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

pub mod constants;
pub mod game;
pub mod menu;

use game::GamePlugin;
use menu::MenuPlugin;

fn main() {
    unsafe { backtrace_on_stack_overflow::enable() };
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_state::<AppState>()
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() /2., 0.),
            ..default()
        }
    );   
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}