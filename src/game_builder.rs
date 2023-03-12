use raylib::{RaylibHandle, RaylibThread};
use std::cell::{RefCell, RefMut};

pub trait Game {
    fn init() -> Self;
    fn is_running(&self) -> bool;
    fn handle_events(&mut self, rl: RefMut<RaylibHandle>);
}

pub struct GameBuilder<T: Game> {
    rl: RefCell<RaylibHandle>,
    rt: RaylibThread,
    startup_systems: Vec<fn()>,
    game: T,
}

impl<T: Game> GameBuilder<T> {
    pub fn init() -> Self {
        let (mut rl, rt) = raylib::init().build();
        // rl.set_exit_key(None); // So <Esc> doesn't quit by default

        Self {
            rl: RefCell::new(rl),
            rt,
            startup_systems: Vec::new(),
            game: T::init(),
        }
    }

    pub fn add_startup_system(mut self, system: fn()) -> Self {
        self.startup_systems.push(system);
        self
    }

    pub fn run(mut self) {
        // for system in self.startup_systems {
        //     system()
        // }

        while self.game.is_running() {
            self.game.handle_events(self.rl.borrow_mut());
        }
    }
}
