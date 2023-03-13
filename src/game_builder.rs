use raylib::{
    prelude::{Color, KeyboardKey, RaylibDraw, TraceLogLevel},
    set_trace_log, RaylibHandle, RaylibThread,
};

use std::{cell::RefCell, rc::Rc};

use crate::{
    prelude::MgiResult,
    renderer::{Drawable, Renderer},
    texture_manager::TextureManager,
};

pub trait Game {
    fn init() -> Self;
    fn is_running(&self) -> bool;
    fn update(&mut self, ctx: &mut Context);
    fn render(&mut self, ctx: &mut Context);
}

// TODO: Create a resource manager and store texture_manager in there
pub struct Context {
    pub(crate) pressed_key: Option<KeyboardKey>,
    pub(crate) renderer: Renderer,
    pub(crate) texture_manager: Option<Rc<RefCell<TextureManager>>>,
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

    pub fn texture_manager(&self) -> Option<&Rc<RefCell<TextureManager>>> {
        self.texture_manager.as_ref()
    }
}

// TODO: Add a layer manager that can take named layers and convert to vector indices at the end!
pub struct GameBuilder<T: Game> {
    pub(crate) rl: RefCell<RaylibHandle>,
    pub(crate) rt: RaylibThread,
    startup_systems: Vec<fn()>,
    game: T,
    texture_manager: Option<Rc<RefCell<TextureManager>>>,
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
            texture_manager: None,
        }
    }

    pub fn fullscreen(self) -> Self {
        self.rl.borrow_mut().toggle_fullscreen();
        self
    }

    pub fn add_startup_system(mut self, system: fn()) -> Self {
        self.startup_systems.push(system);
        self
    }

    pub fn add_texture_manager(mut self, texture_manager: TextureManager) -> MgiResult<Self> {
        if self.texture_manager.is_some() {
            return Err(format!("Only one texture manager can be added to a game").into());
        }

        self.texture_manager = Some(Rc::new(RefCell::new(texture_manager)));
        Ok(self)
    }

    fn load_textures(&mut self) -> MgiResult<()> {
        if let Some(tm) = &mut self.texture_manager {
            let textures = &mut tm.borrow_mut().textures;

            for (_, texture) in textures.iter_mut() {
                let raw = self
                    .rl
                    .borrow_mut()
                    .load_texture(&self.rt, &texture.borrow_mut().path)?;

                texture.borrow_mut().raw = Some(raw);
            }
        }

        Ok(())
    }

    pub fn run(mut self) -> MgiResult<()> {
        self.load_textures()?;

        // Run startup systems
        for system in self.startup_systems {
            system()
        }

        let (mut rl, rt) = (self.rl.borrow_mut(), &self.rt);

        let texture_manager = if let Some(tm) = self.texture_manager {
            Some(Rc::clone(&tm))
        } else {
            None
        };
        let mut ctx = Context {
            pressed_key: None,
            renderer: Renderer {
                clear_color: Color::WHITE,
                layers: Vec::new(),
            },
            texture_manager,
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
                    drawable.draw(&mut d, drawable.position());
                }
            }
        }

        Ok(())
    }
}
