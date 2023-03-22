use bevy::prelude::*;

#[derive(Component, Debug, Copy, Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component)]
pub struct Aim {
    pub w: f32,
    pub h: f32,
}

pub const SPRITE_SIZE: BlockSize = BlockSize{w:25., h:100.};

pub struct BlockSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Missile {}
