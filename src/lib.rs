use std::error::Error;

pub mod context;
pub mod game_builder;

pub mod prelude {
    pub use crate::context::*;
    pub use crate::game_builder::*;
    pub use crate::MgiResult;

    pub use sdl2::keyboard::Keycode;
    pub use sdl2::pixels::Color;
}

pub type MgiResult<T> = Result<T, Box<dyn Error>>;

pub struct TextureManager {}

pub struct LayerManager {}
