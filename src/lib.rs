use std::error::Error;

mod drawable;

pub mod context;
pub mod game_builder;
pub mod texture_manager;

pub mod prelude {
    pub use crate::context::*;
    pub use crate::game_builder::*;
    pub use crate::texture_manager::*;
    pub use crate::MgiResult;

    pub use sdl2::keyboard::Keycode;
    pub use sdl2::mouse::MouseButton;
    pub use sdl2::pixels::Color;
    pub use sdl2::rect::Rect;
}

pub type MgiResult<T> = Result<T, Box<dyn Error>>;

pub struct LayerManager {}
