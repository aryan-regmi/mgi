use std::error::Error;

use mgi::prelude::*;

struct MyGame {
    running: bool,
}

impl Drawable for MyGame {
    fn update(&mut self) {}

    fn render(&mut self, renderer: &Renderer, _tm: &Option<TextureManagerRef>) {
        let mut rl = renderer.rl();
        let mut d = rl.begin_drawing(renderer.rt());

        d.clear_background(Color::BLACK);

        d.draw_line(0, 0, 400, 400, Color::RED);
        d.draw_line(800, 0, 400, 400, Color::BLUE);
        d.draw_line(0, 800, 400, 400, Color::GREEN);
        d.draw_line(800, 800, 400, 400, Color::WHITE);
        d.draw_line(0, 400, 400, 400, Color::YELLOW);
        d.draw_line(800, 400, 400, 400, Color::MAGENTA);
        d.draw_line(400, 0, 400, 400, Color::GOLD);
        d.draw_line(400, 800, 400, 400, Color::MAROON);
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
    GameBuilder::<MyGame>::init("Lines", (800, 800)).run()?;

    Ok(())
}
