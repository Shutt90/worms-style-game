use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct Power {
    pub total: u8,
    pub reverse: bool,
}

impl Default for Power {
    fn default() -> Power {
        Power { total: 0, reverse: false}
    }
}
