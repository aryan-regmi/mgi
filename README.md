MGI

A game framework built using SDL2 and OpenGL (uses the `sdl2` crate) for Rust.

# Usage

```rust

use super::{BaseGame, Canvas, Color, Event, GameBuilder, Keycode, Window, WindowConfig};
use std::error::Error;

struct TestGame {
    running: bool,
}

impl TestGame {
    pub fn init() -> Self {
        Self { running: true }
    }
}

impl BaseGame for TestGame {
    fn update(&mut self) {}

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        canvas.present();
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn handle_events(&mut self, event: sdl2::event::Event) {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => self.running = false,
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    const WIDTH: u32 = 1280;
    const HEIGHT: u32 = 720;

    GameBuilder::init(
        "TEST",
        WIDTH,
        HEIGHT,
        &mut TestGame::init(),
        WindowConfig {
            position: None,
            fullscreen: false,
            borderless: false,
            resizeable: false,
            centered: true,
        },
    )
    .run();

    Ok(())
}

```
