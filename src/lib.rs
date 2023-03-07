pub mod colors;
pub mod game_builder;
pub mod render_types;
pub mod renderer;
pub mod textures;
pub mod utils;

pub mod prelude {
    pub use crate::colors::*;
    pub use crate::game_builder::*;
    pub use crate::render_types::*;
    pub use crate::renderer::*;
    pub use crate::utils::*;
    pub use winit::event::VirtualKeyCode;
    pub use winit::event_loop::ControlFlow;
}
