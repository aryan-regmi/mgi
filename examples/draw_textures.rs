use std::rc::Rc;

use mgi::{
    prelude::{Color, Game, GameBuilder, KeyboardKey},
    render_types::{Rect, Rotation, Shape},
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
        let mut dest = Rect::new(400 - 64, 400 - 64, 128, 128, Color::WHITE);
        dest.rotate(Rotation::Degrees(45.));
        dest.translate(
            400 - 64 * (45f32.to_radians().cos() as i32),
            400 + 64 * (45),
        );
        texture.borrow_mut().set_dest(dest);
        ctx.draw(texture, 1);

        let texture = texture_manager.get_texture("person2").unwrap();
        let dest = Rect::new(400 - 64, 400 - 64, 128, 128, Color::WHITE);
        texture.borrow_mut().set_dest(dest);
        ctx.draw(texture, 1);
    }
}

fn main() -> MgiResult<()> {
    let texture_manager = TextureManager::init()
        .add_texture(
            "bg",
            "examples/assets/bg.png",
            None,
            Rect::new(0, 0, 800, 800, Color::WHITE),
        )
        .add_texture(
            "person",
            "examples/assets/person.png",
            None,
            Rect::new(0, 0, 32, 32, Color::WHITE),
        )
        .add_texture(
            "person2",
            "examples/assets/person.png",
            None,
            Rect::new(0, 0, 32, 32, Color::WHITE),
        );

    GameBuilder::<MyGame>::init("Textures", (800, 800))
        .add_texture_manager(texture_manager)?
        .run()?;

    Ok(())
}
