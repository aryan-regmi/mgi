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

        ctx.draw_texture("bg", 0).unwrap();
    }

    fn update(&mut self, ctx: &Context) {
        if ctx.key_pressed(Keycode::Escape) {
            self.stop();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut texture_manager = TextureManager::new();
    texture_manager.add_texture("bg", "examples/assets/bg.png");

    GameBuilder::<World>::init("Hello World", (WIDTH as i32, HEIGHT as i32))
        .add_texture_manager(texture_manager)
        .run()?;

    Ok(())
}
