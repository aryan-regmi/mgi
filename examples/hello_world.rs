use mgi::{
    prelude::{Color, Game, GameBuilder, KeyboardKey, Rect},
    renderer::Text,
};

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
        ctx.draw(Rect::from_center(400, 400, 400, 400, Color::RED), 1);
        ctx.fill_rect(Rect::new(200, 200, 400, 400, Color::BLUE), 0);
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

fn main() {
    GameBuilder::<MyGame>::init("Hello World", (800, 800))
        .add_startup_system(MyGame::hello_world)
        .run();
}
