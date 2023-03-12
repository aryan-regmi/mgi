use raylib::{
    prelude::{Color, KeyboardKey, RaylibDraw, RaylibDrawHandle},
    RaylibHandle, RaylibThread,
};
use std::cell::RefCell;

use crate::utils::Vec2;

pub trait Game {
    fn init() -> Self;
    fn is_running(&self) -> bool;
    fn update(&mut self, ctx: &mut MgiContext);
    fn render(&mut self, ctx: &mut MgiContext);
}

trait Drawable {
    fn draw(&mut self, pen: &mut RaylibDrawHandle);
}

struct Renderer {
    clear_color: Color,
    // TODO: abstrct into a trait so that layer: Vec<Box<dyn Layerable>> (so only vec is boxed, not
    // each element in the vec)
    layers: Vec<Vec<Box<dyn Drawable>>>,
}

pub struct MgiContext {
    pressed_key: Option<KeyboardKey>,
    renderer: Renderer,
}

impl MgiContext {
    pub fn pressed_key(&self) -> Option<KeyboardKey> {
        self.pressed_key
    }

    pub fn clear_background(&mut self, color: Color) {
        self.renderer.clear_color = color;
    }

    pub fn draw_rect(&mut self, rect: Rect, layer: usize) {
        // If the layer already exists, just add to it
        if self.renderer.layers.len() > layer {
            self.renderer.layers[layer].push(Box::new(rect));
            return;
        }

        // Create new layer if the corresponding layer doesn't exist
        self.renderer.layers.push(vec![Box::new(rect)])
    }

    pub fn fill_rect(&mut self, mut rect: Rect, layer: usize) {
        rect.fill = true;

        // If the layer already exists, just add to it
        if self.renderer.layers.len() > layer {
            self.renderer.layers[layer].push(Box::new(rect));
            return;
        }

        // Create new layer if the corresponding layer doesn't exist
        self.renderer.layers.push(vec![Box::new(rect)])
    }
}

pub struct Rect {
    position: Vec2,
    size: Vec2,
    color: Color,
    pub(crate) fill: bool,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32, color: Color) -> Self {
        Self {
            position: (x, y).into(),
            size: (width, height).into(),
            color,
            fill: false,
        }
    }
}

impl Drawable for Rect {
    fn draw(&mut self, pen: &mut RaylibDrawHandle) {
        if self.fill {
            pen.draw_rectangle(
                self.position.x,
                self.position.y,
                self.size.x,
                self.size.y,
                self.color,
            );
            return;
        }

        pen.draw_rectangle_lines(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            self.color,
        );
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
    pub fn init() -> Self {
        let (mut rl, rt) = raylib::init().build();
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
        let mut ctx = MgiContext {
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
