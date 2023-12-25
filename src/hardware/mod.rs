mod engine;
mod font;
mod instruction;

pub mod prelude {
    pub use super::engine::{Engine, EngineError};
}

use font::FONT_DATA;
