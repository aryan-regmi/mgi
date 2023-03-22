use std::error::Error;

pub mod game_builder;

pub mod prelude {
    pub use crate::game_builder::*;
    pub use crate::*;

    pub use mgics::prelude::SystemType::*;
    pub use mgics::prelude::*;
}

pub type MgiResult<T> = Result<T, Box<dyn Error>>;
