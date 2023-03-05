use mgi::prelude::*;

struct MyGame {
    running: bool,
}

impl Drawable for MyGame {
    fn update(&mut self) {}

    fn render(&mut self, renderer: &mut Renderer) {
        renderer.clear_color([255, 255, 255, 255].into());
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
    GameBuilder::<MyGame>::init("Draw Point")
        .set_size(800, 800)
        .run();
}
