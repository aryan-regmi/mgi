use std::rc::Rc;

use mgi::{
    prelude::{Color, Game, GameBuilder, KeyboardKey},
    texture_manager::TextureManager,
    utils::MgiResult,
};

struct MyGame {
    running: bool,
}

impl Game for MyGame {
    fn init() -> Self {
        Self { running: true }
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn update(&mut self, ctx: &mut mgi::prelude::Context) {
        if let Some(KeyboardKey::KEY_ESCAPE) = ctx.pressed_key() {
            self.running = false;
        }
    }

    fn render(&mut self, ctx: &mut mgi::prelude::Context) {
        ctx.clear_background(Color::WHITE);

        let tm = Rc::clone(ctx.texture_manager().as_ref().unwrap());
        let texture_manager = tm.borrow_mut();

        let texture = texture_manager.get_texture("bg").unwrap();
        ctx.draw(texture, 0);

        let texture = texture_manager.get_texture("person").unwrap();
        texture.borrow_mut().set_size(128, 128);
        ctx.draw(texture, 1);
    }
}

fn main() -> MgiResult<()> {
    let texture_manager = TextureManager::init()
        .add_texture("bg", "examples/assets/bg.png", (800, 800).into(), None)
        .add_texture(
            "person",
            "examples/assets/person.png",
            (32, 32).into(),
            None,
        );

    GameBuilder::<MyGame>::init("Textures", (800, 800))
        .add_texture_manager(texture_manager)?
        .run()?;

    Ok(())
}
