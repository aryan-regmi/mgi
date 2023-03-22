use mgics::prelude::*;

use crate::MgiResult;

pub struct GameBuilder {
    app: App,
}

// Game resources
pub struct GameTitle(pub String);
pub struct GameSize(u32, u32);
impl GameSize {
    pub fn width(&self) -> u32 {
        self.0
    }

    pub fn height(&self) -> u32 {
        self.1
    }
}

impl GameBuilder {
    pub fn new(title: &str, size: (u32, u32)) -> Self {
        let app = App::new()
            .add_resource(GameTitle(title.into()))
            .add_resource(GameSize(size.0, size.1));

        Self { app }
    }

    pub fn add_system<F: Fn(&mut World) -> MgiResult<()> + 'static>(
        mut self,
        system_type: SystemType,
        system: F,
    ) -> Self {
        self.app = self.app.add_system(system_type, system);
        self
    }

    pub fn run(self) -> MgiResult<()> {
        self.app.run()
    }
}
