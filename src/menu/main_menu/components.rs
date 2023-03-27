use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub struct MainMenu{
    pub node_list: &'static [&'static str]
}

#[derive(Component)]
pub struct MenuItem {}

