use mgi::prelude::*;

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
        const BACKGROUND: usize = 0;
        const FOREGROUND: usize = 1;

        let (w, h) = (400, 400);

        let pos = (300, 200).into();
        let mut rect = Rectangle::new(pos, w as u32, h as u32, Color::BLUE);
        rect.fill(false);
        ctx.draw(rect, FOREGROUND);

        let pos = (ctx.size().x / 2 - w / 2, ctx.size().y / 2 - h / 2).into();
        let rect = Rectangle::new(pos, w as u32, h as u32, Color::RED);
        ctx.draw(rect, BACKGROUND);

        Ok(())
    }
}

fn main() -> MgiResult<()> {
    GameBuilder::<MyGame>::init("Hello World", (800, 800))?
        .add_startup_system(MyGame::hello_world)
        .run()?;

    Ok(())
}
