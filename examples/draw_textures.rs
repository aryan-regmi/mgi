use std::error::Error;

use mgi::{prelude::*, Rotation};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

struct World {
    running: bool,
}

impl Game for World {
    fn init() -> Self {
        Self { running: true }
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn stop(&mut self) {
        self.running = false;
    }

    fn draw(&mut self, ctx: &Context) {
        ctx.clear_background(Color::WHITE);
    }

    fn update(&mut self, ctx: &Context) {
        if ctx.key_pressed(Keycode::Escape) {
            self.stop();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    GameBuilder::<World>::init("Hello World", (WIDTH as i32, HEIGHT as i32)).run()?;

    Ok(())
}
