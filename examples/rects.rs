use std::error::Error;

use mgi::prelude::*;

struct MyGame {
    running: bool,
}

impl Drawable for MyGame {
    fn render(&mut self, renderer: &Renderer, _: &ResourceManager) -> MgiResult<()> {
        renderer.draw(&|d| {
            d.clear_background(Color::BLACK);

            d.draw_rectangle(0, 0, 200, 200, Color::BLUE);
            d.draw_rectangle(400, 400, 300, 100, Color::RED);
            d.draw_rectangle_lines(0, 600, 400, 200, Color::GREEN);
            d.draw_rectangle(50, 650, 300, 100, Color::GOLD);
        });

        Ok(())
    }
}

impl Updateable for MyGame {
    fn update(&mut self) -> MgiResult<()> {
        Ok(())
    }
}

impl Game for MyGame {
    fn setup() -> Self {
        Self { running: true }
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn stop(&mut self) {
        self.running = false;
    }

    fn handle_events(&mut self, rl: &raylib::RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE)
            || rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
        {
            self.stop();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    GameBuilder::<MyGame>::init("Rects", (800, 800)).run()?;

    Ok(())
}
