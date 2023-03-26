use bevy::{prelude::*, input::keyboard::KeyboardInput};
use super::resources::*;

pub fn update_config(
    config: Res<Config>,
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