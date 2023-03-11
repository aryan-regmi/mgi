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
        let mut rl = renderer.rl();
        let mut d = rl.begin_drawing(renderer.rt());

        d.clear_background(Color::BLACK);

        let tm = texture_manager.as_ref().unwrap();
        let bg = tm.get_texture("bg").unwrap();
        let person = tm.get_texture("person").unwrap();
        d.draw_texture_rec(
            bg.raw_texture().unwrap(),
            Rectangle::new(600., 0., 800., 800.),
            Vector2::new(0., 0.),
            Color::WHITE,
        );
        d.draw_texture_pro(
            person.raw_texture().unwrap(),
            Rectangle::new(0., 0., 32., 32.),
            Rectangle::new(180., 670., 60., 60.),
            Vector2::new(0., 0.),
            -20.,
            Color::WHITE,
        )
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

    GameBuilder::<MyGame>::init("Textures", (800, 800))
        .add_texture_manager(texture_manager)
        .run()?;

    Ok(())
}
