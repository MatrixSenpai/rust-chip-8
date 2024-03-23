#![allow(unused, dead_code)]

mod bevy_data;
mod hardware;

#[macro_use]
extern crate log;

use bevy::prelude::*;
use crate::hardware::prelude::*;

fn main() -> Result<(), SystemError> {
    dotenv::dotenv().ok();
    pretty_env_logger::init_timed();

    App::new()
        .add_plugins(DefaultPlugins)
        .run();

    Ok(())
}

#[derive(Debug)]
enum SystemError {
    PathNotFound(std::io::Error),
    EngineError(EngineError),
}