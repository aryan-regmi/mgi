use std::error::Error;

use mgi::{prelude::*, textures::TextureManager};

struct MyGame {
    running: bool,
}

impl Drawable for MyGame {
    fn update(&mut self) {}

    fn render(&mut self, renderer: &mut Renderer) {
        renderer.clear_color([255, 255, 255, 255].into());

        let rect = Rect::new(700, 0, 200, 200);

        // renderer.draw_rect(rect, [255, 0, 0, 255].into());
        renderer.draw_texture("bg", None, rect).unwrap();
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
    let mut texture_manager = TextureManager::new();
    texture_manager.add_texture("bg".into(), "examples/assets/bg.png")?;

    GameBuilder::<MyGame>::init("Draw Rect")
        .add_texture_manager(texture_manager)
        .set_size(800, 800)
        .run()?;

    Ok(())
}
