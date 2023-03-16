pub(crate) mod renderer;

pub mod drawable;
pub mod game_builder;
pub mod utils;

pub mod prelude {
    pub use crate::drawable::Rectangle;
    pub use crate::game_builder::*;
    pub use crate::utils::*;

    pub use sdl2::{keyboard::Keycode, pixels::Color};
}
