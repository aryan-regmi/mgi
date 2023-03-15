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

        ctx.draw_texture(
            "bg",
            None,
            Some(Rect::new(
                0,
                0,
                (800, 800).into(),
                Color::WHITE,
                // None,
                Some(Rotation::Degrees(15.)),
            )),
            0,
        )
        .unwrap();

        // ctx.draw_texture(
        //     "person",
        //     None,
        //     Some(Rect::from_center(
        //         400,
        //         400,
        //         (256, 256).into(),
        //         Color::BLACK,
        //         None,
        //         // Some(Rotation::Degrees(45.)),
        //     )),
        //     1,
        // )
        // .unwrap();
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
    texture_manager.add_texture("person", "examples/assets/person.png");

    GameBuilder::<World>::init("Hello World", (WIDTH as i32, HEIGHT as i32))
        .add_texture_manager(texture_manager)
        .run()?;

    Ok(())
}
