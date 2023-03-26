use bevy::input::keyboard::KeyCode;

pub struct BlockSize {
    pub w: f32,
    pub h: f32,
}

pub const AIM_SPEED: f32 = 10.;
pub const CROSSHAIR_DISTANCE_FROM_PLAYER: f32 = 70.;
pub const MISSILE_POWER_VELOCITY: f32 = 0.25;
pub const MISSILE_MASS: f32 = 25.;
pub const MISSILE_POWER_VELOCITY_MAX_MULTIPLIER: f32 = 25.;
pub const MIN_ENEMY_PIXELS_FROM_PLAYER: f32 = 30.;
pub const MISSLE_SIZE: f32 = 10.;

pub const SPRITE_SIZE: BlockSize = BlockSize{w:32., h:32.};

pub const MENU_ITEM_SCALING: f32 = 20.;