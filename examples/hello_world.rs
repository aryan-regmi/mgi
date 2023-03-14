use std::error::Error;

use mgi::{Game, GameBuilder};
use winit::event::VirtualKeyCode;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const WHITE: [u8; 4] = [255, 255, 255, 255];
const RED: [u8; 4] = [255, 0, 0, 255];

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

    fn draw(&mut self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            // Draw a box
            let x = (i % WIDTH as usize) as i32;
            let y = (i / WIDTH as usize) as i32;

            let color;
            if x >= 200 && x <= 600 && y >= 200 && y <= 600 {
                color = RED;
            } else {
                color = WHITE;
            }

            pixel.copy_from_slice(&color);
        }
    }

    fn update(&mut self, inputs: &winit_input_helper::WinitInputHelper) {
        if inputs.key_pressed(VirtualKeyCode::Escape) {
            self.stop();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    GameBuilder::<World>::init("Hello World", (WIDTH as i32, HEIGHT as i32)).run()?;

    Ok(())
}

