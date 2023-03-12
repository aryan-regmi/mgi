pub mod game_builder;
pub mod renderer;
pub mod utils;

pub mod prelude {
    pub use crate::game_builder::*;
    pub use crate::renderer::*;
    pub use crate::utils::*;

    pub use raylib::prelude::{Color, KeyboardKey};
}
