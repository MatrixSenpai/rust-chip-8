use bevy::prelude::*;

enum MenuState {
    Main,
    GameSelect,
    Settings,
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {

    }
}