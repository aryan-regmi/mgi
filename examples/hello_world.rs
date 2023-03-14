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

    fn update(&mut self, ctx: &mut mgi::prelude::Context) {
        if let Some(KeyboardKey::KEY_ESCAPE) = ctx.pressed_key() {
            self.running = false;
        }
    }

    fn render(&mut self, ctx: &mut mgi::prelude::Context) {
        ctx.clear_background(Color::GRAY);

        let mut r1 = Rect::from_center(400, 400, 400, 400, Color::RED);
        r1.rotate(Rotation::Degrees(45.));
        ctx.draw(r1, 1);

        let mut r2 = Rect::new(200, 200, 400, 400, Color::BLUE);
        r2.rotate(Rotation::Degrees(45.));
        r2.translate(400, 120);
        ctx.fill_rect(r2, 0);

        let txt = "HELLO  WORLD";
        ctx.draw(
            Text::new(
                txt,
                (300 + (2.5 * txt.len() as f32) as i32, 400).into(),
                Color::YELLOW,
                20,
            ),
            2,
        );
    }
}

fn main() -> MgiResult<()> {
    GameBuilder::<MyGame>::init("Hello World", (800, 800))
        .add_startup_system(MyGame::hello_world)
        .run()?;

    Ok(())
}
