use mgi::{drawable::Rect, prelude::*};
use sdl2::{keyboard::Keycode, pixels::Color};

struct MyGame {
    running: bool,
}

impl MyGame {
    fn hello_world() {
        println!("HELLO_WORLD!");
    }
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
        let (w, h) = (400, 400);
        let pos = (ctx.size().x / 2 - w / 2, ctx.size().y / 2 - h / 2).into();
        let mut rect = Rect::new(pos, w as u32, h as u32, Color::RED, None);
        rect.fill();

        ctx.draw(rect, 0);

        Ok(())
    }
}

fn main() -> MgiResult<()> {
    GameBuilder::<MyGame>::init("Hello World", (800, 800))?
        .add_startup_system(MyGame::hello_world)
        .run()?;

    Ok(())
}
