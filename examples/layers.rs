use std::error::Error;

use mgi::prelude::*;

struct MyGame {
    running: bool,
}

impl Drawable for MyGame {
    fn render(
        &mut self,
        renderer: &Renderer,
        texture_manager: &Option<TextureManagerRef>,
        _: &mut Option<TileMapRef>,
    ) {
        renderer
            .draw_texture_layers(texture_manager.as_ref().unwrap())
            .unwrap();
    }
}

impl Updateable for MyGame {
    fn update(&mut self) {}
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

    let bg_layer: TextureLayer = Layer::default().add_obj(
        &"bg",
        Some(Rectangle::new(600., 0., 800., 800.)),
        Rectangle::new(0., 0., 800., 800.),
        0.,
    );
    let player_layer: TextureLayer =
        Layer::init(1).add_obj(&"person", None, Rectangle::new(180., 670., 60., 60.), 0.);

    dbg!(bg_layer.id(), player_layer.id());

    GameBuilder::<MyGame>::init("Layers", (800, 800))
        .add_texture_manager(texture_manager)
        .add_texture_layers(vec![bg_layer, player_layer])
        .run()?;

    Ok(())
}
