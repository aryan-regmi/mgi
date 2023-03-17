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
        const BACKGROUND: usize = 0;
        const PERSON: usize = BACKGROUND + 1;

        ctx.draw_texture("bg", None, BACKGROUND)?;

        ctx.draw_texture_pro(
            "person",
            None,
            Some(Rectangle::new((400, 520).into(), 128, 128, Color::WHITE)),
            Some(Rotation::Degrees(30.)),
            None,
            None,
            PERSON,
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
