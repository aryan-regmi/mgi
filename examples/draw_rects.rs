use mgi::prelude::*;

struct MyGame {
    running: bool,
}

impl Drawable for MyGame {
    fn update(&mut self) {}

    fn render(&mut self, renderer: &mut Renderer) {
        renderer.clear_color([255, 255, 255, 255].into());

        let color = Rgba::new(0, 0, 255, 1.0).unwrap();
        renderer.draw_rect(Rect::new(300., 0., 200., 200.), color);

        let color = Rgba::new(255, 0, 0, 1.0).unwrap();
        let mut rect = Rect::new(300., 350., 200., 100.);
        rect.rotate(Rotation::Degrees(45.));
        renderer.draw_rect(rect, color);
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
    GameBuilder::<MyGame>::init("Draw Rect")
        .set_size(800, 800)
        .run();
}
