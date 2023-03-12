pub mod game_builder;
pub mod renderer;
pub mod textures;
pub mod tilemap;
pub mod utils;

pub mod prelude {
    pub use crate::game_builder::*;
    pub use crate::renderer::*;
    pub use crate::textures::*;
    pub use crate::tilemap::*;

    pub use raylib::prelude::{Color, KeyboardKey, RaylibDraw, Rectangle, Vector2};
}
