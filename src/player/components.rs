use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}


#[derive(Component)]
pub struct Aim {}

pub const SPRITE_SIZE: BlockSize = BlockSize{w:25., h:100.};

pub struct BlockSize {
    pub w: f32,
    pub h: f32,
}
