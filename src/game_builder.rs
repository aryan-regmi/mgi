use raylib::{
    prelude::{Color, KeyboardKey, RaylibDraw, TraceLogLevel},
    set_trace_log, RaylibHandle, RaylibThread,
};

use std::cell::RefCell;

use crate::renderer::{Drawable, Renderer};

pub trait Game {
    fn init() -> Self;
    fn is_running(&self) -> bool;
    fn update(&mut self, ctx: &mut Context);
    fn render(&mut self, ctx: &mut Context);
}

pub struct Context {
    pub(crate) pressed_key: Option<KeyboardKey>,
    pub(crate) renderer: Renderer,
}

impl Context {
    pub fn pressed_key(&self) -> Option<KeyboardKey> {
        self.pressed_key
    }

    pub fn clear_background(&mut self, color: Color) {
        self.renderer.clear_color = color;
    }

    pub fn draw<T: Drawable + 'static>(&mut self, drawable: T, layer: usize) {
        // If the layer already exists, just add to it
        if self.renderer.layers.len() > layer {
            self.renderer.layers[layer].push(Box::new(drawable));
            return;
        }

        // Create new layer if the corresponding layer doesn't exist
        self.renderer.layers.push(vec![Box::new(drawable)])
    }
}

// TODO: Add a layer manager that can take named layers and convert to vector indices at the end!
pub struct GameBuilder<T: Game> {
    rl: RefCell<RaylibHandle>,
    rt: RaylibThread,
    startup_systems: Vec<fn()>,
    game: T,
}

impl<T: Game> GameBuilder<T> {
    pub fn init(title: &str, size: (i32, i32)) -> Self {
        // TODO: Make this a parameter
        set_trace_log(TraceLogLevel::LOG_ERROR);

        let (mut rl, rt) = raylib::init().size(size.0, size.1).title(title).build();
        rl.set_exit_key(None); // So <Esc> doesn't quit by default

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
        for system in self.startup_systems {
            system()
        }

        let (mut rl, rt) = (self.rl.borrow_mut(), &self.rt);
        let mut ctx = Context {
            pressed_key: None,
            renderer: Renderer {
                clear_color: Color::WHITE,
                layers: Vec::new(),
            },
        };

        while self.game.is_running() {
            ctx.pressed_key = rl.get_key_pressed();

            self.game.update(&mut ctx);

            // The render function doesnt actually render: it just determines the layers to render
            // stuff in, their textures, and their, displayed positions
            self.game.render(&mut ctx);

            // Actually loop through renderer's layers and display stuff
            let mut d = rl.begin_drawing(rt);
            d.clear_background(ctx.renderer.clear_color);

            for layer in ctx.renderer.layers.iter_mut() {
                for drawable in layer.iter_mut() {
                    drawable.draw(&mut d);
                }
            }
        }
    }
}
