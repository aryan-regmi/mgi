use mgi::prelude::*;

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

    fn update(&mut self, ctx: &mut mgi::prelude::Context) -> MgiResult<()> {
        if ctx.is_keydown(Keycode::Escape) || ctx.is_keydown(Keycode::Backspace) {
            self.running = false;
        }

        Ok(())
    }

    fn render(&mut self, ctx: &mut mgi::prelude::Context) -> MgiResult<()> {
        ctx.draw_texture("bg", None, None, None, 0)?;

        ctx.draw_texture(
            "person",
            None,
            Some(Rectangle::new((400, 520).into(), 128, 128, Color::WHITE)),
            Some(Rotation::Degrees(30.)),
            1,
        )?;

        Ok(())
    }
}

fn main() -> MgiResult<()> {
    let mut texture_manager = TextureManager::new();
    texture_manager.add_texture("bg", "./examples/assets/bg.png");
    texture_manager.add_texture("person", "./examples/assets/person.png");

    GameBuilder::<MyGame>::init("Textures", (800, 800))?
        .add_texture_manager(texture_manager)
        .run()?;

    Ok(())
}
