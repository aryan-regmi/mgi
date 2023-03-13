pub mod game_builder;
pub mod renderer;
pub mod texture_manager;
pub mod utils;

pub mod prelude {
    pub use crate::game_builder::*;
    pub use crate::renderer::*;
    pub use crate::texture_manager::*;
    pub use crate::utils::*;

    pub use raylib::prelude::{Color, KeyboardKey};
}
