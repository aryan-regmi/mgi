use mgi::prelude::*;

struct MyGame {
    running: bool,
}

impl Drawable for MyGame {
    fn update(&mut self) {}

    fn render(&mut self, renderer: &mut Renderer) {
        let color = [0xff, 0x00, 0x00, 0xff];

        for mut pixel in renderer.pixels_mut() {
            pixel.set_color(color.into());
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

fn main() {
    GameBuilder::<MyGame>::init("Hello World")
        .set_size(800, 800)
        .run();
}
