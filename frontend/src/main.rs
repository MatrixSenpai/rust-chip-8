use bevy::prelude::*;

mod ch8_plugin;
mod gui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(gui::gui_plugin)
        .add_plugins(ch8_plugin::chip8_emulator_plugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}
