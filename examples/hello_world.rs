use mgi::prelude::*;

struct MyGame {
    running: bool,
}

impl Drawable for MyGame {
    fn update(&mut self) {}

    fn render(&mut self, pixels: &mut pixels::Pixels) {
        let frame = pixels.get_frame_mut();

        for pixel in frame.chunks_exact_mut(4) {
            let color = [0xff, 0xff, 0xff, 0x00];

            pixel.copy_from_slice(&color);
        }
    }
}

impl EventHandler for MyGame {
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
}

fn main() {
    GameBuilder::<MyGame>::init("Hello World")
        .set_size(800, 800)
        .run();
}
