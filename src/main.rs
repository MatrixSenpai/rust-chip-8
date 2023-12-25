mod hardware;

#[macro_use]
extern crate log;

use std::cell::RefCell;
use std::path::PathBuf;
use clap::Parser;
use crate::hardware::prelude::*;

fn main() -> Result<(), SystemError> {
    dotenv::dotenv().ok();
    pretty_env_logger::init_timed();

    let engine = Engine::new()
        .map_err(|e| SystemError::EngineError(e))?;
    let engine = RefCell::new(engine);

    let flags = Arg::parse();
    let load_game = flags.game;

    let import = std::fs::read(load_game)
        .map_err(|e| SystemError::PathNotFound(e))?;

    engine.borrow_mut()
        .load_game(import)
        .map_err(|e| SystemError::EngineError(e))?;

    engine.borrow_mut().tick();

    Ok(())
}

#[derive(Debug, Parser)]
struct Arg {
    #[arg(short, long)]
    game: PathBuf
}

#[derive(Debug)]
enum SystemError {
    PathNotFound(std::io::Error),
    EngineError(EngineError),
}