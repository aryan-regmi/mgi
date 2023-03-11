use std::error::Error;

use mgi::prelude::*;

struct MyGame {
    running: bool,
}

impl Drawable for MyGame {
    fn update(&mut self) {}

    fn render(&mut self, renderer: &Renderer, texture_manager: &Option<TextureManagerRef>) {
        let mut rl = renderer.rl();
        let mut d = rl.begin_drawing(renderer.rt());

        renderer
            .draw_texture_layers(&mut d, texture_manager.as_ref().unwrap())
            .unwrap();
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
    let mut texture_manager = TextureManager::new();
    texture_manager.add_texture("bg", "examples/assets/bg.png");
    texture_manager.add_texture("person", "examples/assets/person.png");

    let bg_layer: TextureLayer = Layer::init().add_obj(
        &"bg",
        Some(Rectangle::new(600., 0., 800., 800.)),
        Rectangle::new(0., 0., 800., 800.),
        0.,
    );
    let player_layer: TextureLayer =
        Layer::init().add_obj(&"person", None, Rectangle::new(180., 670., 60., 60.), 0.);

    GameBuilder::<MyGame>::init("Layers", (800, 800))
        .add_texture_manager(texture_manager)
        .add_texture_layers(vec![bg_layer, player_layer])
        .run()?;

    Ok(())
}
