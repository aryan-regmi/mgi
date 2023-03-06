use mgi::prelude::*;

struct MyGame {
    running: bool,
}

impl Drawable for MyGame {
    fn update(&mut self) {}

    fn render(&mut self, renderer: &mut Renderer) {
        renderer.clear_color(Rgba::new(255, 255, 255, 1.0).unwrap());

        let color = Rgba::new(255, 0, 0, 1.0).unwrap();
        renderer.draw_line((0., 0.).into(), (400., 400.).into(), color);

        let color = Rgba::new(0, 0, 255, 1.0).unwrap();
        renderer.draw_line((0., 800.).into(), (400., 400.).into(), color);
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
