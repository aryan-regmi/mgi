use mgi::prelude::{Color, Game, GameBuilder, KeyboardKey, Rect};

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
        ctx.clear_background(Color::WHITE);
        ctx.draw_rect(Rect::new(0, 0, 20, 20, Color::RED), 1);
        ctx.fill_rect(Rect::new(0, 0, 20, 20, Color::BLUE), 0);
    }
}

fn main() {
    GameBuilder::<MyGame>::init()
        .add_startup_system(MyGame::hello_world)
        .run();
}
