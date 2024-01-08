mod engine;
mod font;
mod instruction;
mod engine_execute;

pub mod prelude {
    pub use super::engine::{Engine, EngineError};
}

use font::FONT_DATA;
