use std::cell::RefMut;

use mgi::prelude::{Game, GameBuilder};
use raylib::RaylibHandle;

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

    fn handle_events(&mut self, rl: RefMut<RaylibHandle>) {
        if rl.is_key_pressed(raylib::prelude::KeyboardKey::KEY_ESCAPE) {
            dbg!("Stopping");
            self.running = false;
        }
    }
}

fn main() {
    GameBuilder::<MyGame>::init()
        .add_startup_system(MyGame::hello_world)
        .run();
}
