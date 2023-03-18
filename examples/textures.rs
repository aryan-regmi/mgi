use mgi::prelude::*;

struct TestGame {
    running: bool,
}

impl Game for TestGame {
    fn setup() -> Self {
        Self { running: true }
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn update(&mut self, ctx: &mut MgiContext) -> mgi::MgiResult<()> {
        if ctx.key_down(Keycode::Escape) {
            self.running = false;
        }

        Ok(())
    }

    fn render(&mut self, ctx: &mut MgiContext) -> mgi::MgiResult<()> {
        let _ = ctx;

        // TODO: Draw the texture!
        ctx.draw_texture("bg", None, Rect::new(0, 0, 800, 800), 0., 1.)?;

        Ok(())
    }
}

fn main() -> MgiResult<()> {
    let mut texture_manager = TextureManager::new();
    texture_manager.add_texture("bg", "./examples/assets/bg.png");

    GameBuilder::<TestGame>::init("Hello World", 800, 800)?
        .add_texture_manager(texture_manager)?
        .run()?;

    Ok(())
}
