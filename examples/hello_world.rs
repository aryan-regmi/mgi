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

    fn handle_input(&mut self, ctx: &mut MgiContext) -> mgi::MgiResult<()> {
        if ctx.key_down(Keycode::Escape) {
            self.running = false;
        }

        Ok(())
    }

    fn update(&mut self, _ctx: &mut MgiContext) -> mgi::MgiResult<()> {
        Ok(())
    }

    fn render(&mut self, ctx: &mut MgiContext) -> mgi::MgiResult<()> {
        ctx.set_clear_color(Color::RGB(100, 100, 100));

        ctx.draw_rect(200, 200, 400, 400, Color::RED, 1.0)?;

        // ctx.fill_rect(300, 300, 200, 200, Color::BLUE, 0.5)?;
        ctx.fill_rect(300, 300, 200, 200, Color::BLUE, 1.0)?;

        ctx.draw_line((0, 0), (800, 800), Color::GREEN, 1.0)?;

        Ok(())
    }
}

fn main() -> MgiResult<()> {
    GameBuilder::<TestGame>::init("Hello World", 800, 800)?.run()?;

    Ok(())
}
