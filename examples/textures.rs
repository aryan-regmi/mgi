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
        ctx.draw_texture(
            "bg",
            None,
            Some(Rectangle::new((100, 100).into(), 600, 600, Color::WHITE)),
            Some(Rotation::Degrees(180.)),
            0,
        )?;

        Ok(())
    }
}

fn main() -> MgiResult<()> {
    GameBuilder::<MyGame>::init("Textures", (800, 800))?
        .add_texture("bg", "examples/assets/bg.png")
        .run()?;

    Ok(())
}
