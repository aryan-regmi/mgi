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

        ctx.draw(Rect::new(100, 100, (400, 400).into(), Color::RED, None), 1);
        ctx.draw(
            Rect::from_center(
                400,
                400,
                (400, 400).into(),
                Color::GREEN,
                Some(Rotation::Degrees(45.)),
            ),
            2,
        );
        ctx.draw(Rect::new(300, 300, (400, 400).into(), Color::BLUE, None), 3);
        ctx.draw(
            Rect::new(
                0,
                0,
                (800, 800).into(),
                Color::rgba(255, 200, 150, 1.0),
                None,
            ),
            0,
        );

        ctx.draw_rect_outline(
            Rect::from_center(000, 800, (600, 400).into(), Color::BLACK, None),
            4,
        );
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
