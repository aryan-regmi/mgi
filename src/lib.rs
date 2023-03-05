pub mod game_builder;
pub mod renderer;

pub mod prelude {
    pub use crate::game_builder::*;
    pub use crate::renderer::*;
    pub use winit::event::VirtualKeyCode;
    pub use winit::event_loop::ControlFlow;
}
