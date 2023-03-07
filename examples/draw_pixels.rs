use std::error::Error;

use mgi::prelude::*;
use rand::{thread_rng, Rng};

struct MyGame {
    running: bool,
}

impl Drawable for MyGame {
    fn update(&mut self) {}

    fn render(&mut self, renderer: &mut Renderer) {
        let mut rng = thread_rng();

        for i in 200..600 {
            for j in 200..600 {
                let color: [u8; 4] = rng.gen();

                renderer.draw_pixel(i, j, color.into());
            }
        }
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

    fn handle_events(
        &mut self,
        event: &winit_input_helper::WinitInputHelper,
        cf: &mut winit::event_loop::ControlFlow,
    ) {
        if event.key_pressed(VirtualKeyCode::Escape) {
            *cf = ControlFlow::Exit
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    GameBuilder::<MyGame>::init("Draw Point")
        .set_size(800, 800)
        .run()?;

    Ok(())
}
