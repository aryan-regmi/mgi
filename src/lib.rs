pub mod context;
pub mod drawable;
pub mod game_builder;
pub mod texture_manager;
pub mod tilemap;
pub mod utils;

pub(crate) mod resource_manager;

pub mod prelude {
    pub use crate::context::Context;
    pub use crate::drawable::Rectangle;
    pub use crate::game_builder::*;
    pub use crate::texture_manager::*;
    pub use crate::tilemap::*;
    pub use crate::utils::*;

    pub use sdl2::{keyboard::Keycode, pixels::Color};
}
