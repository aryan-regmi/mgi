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
        ctx.set_clear_color(Color::BLACK);

        ctx.draw_texture(
            "bg",
            Some(Rect::new(0, 0, 600, 800)),
            Rect::new(0, 0, 800, 800),
            0.,
            0.8,
        )?;
        ctx.draw_texture("person", None, Rect::new(400, 620, 128, 128), 10., 1.0)?;

        Ok(())
    }
}

fn main() -> MgiResult<()> {
    let mut texture_manager = TextureManager::new();
    texture_manager.add_texture("bg", "./examples/assets/bg.png");
    texture_manager.add_texture("person", "./examples/assets/person.png");

    GameBuilder::<TestGame>::init("Textures", 800, 800)?
        .add_texture_manager(texture_manager)?
        .run()?;

    Ok(())
}
