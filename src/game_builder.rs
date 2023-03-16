use crate::context::{Context, Renderer};
use crate::texture_manager::TextureManager;
use crate::{prelude::MgiResult, utils::Vec2};
use sdl2::{event::Event, pixels::Color, Sdl, VideoSubsystem};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub trait Game {
    fn init() -> Self;
    fn is_running(&self) -> bool;
    fn update(&mut self, ctx: &mut Context) -> MgiResult<()>;
    fn render(&mut self, ctx: &mut Context) -> MgiResult<()>;
}

pub struct GameBuilder<T: Game> {
    title: String,
    size: Vec2,

    sdl_ctx: Sdl,
    video_sys: VideoSubsystem,

    startup_systems: Vec<fn()>,
    texture_manager: Option<Rc<RefCell<TextureManager>>>,
    game: T,
}

impl<T: Game> GameBuilder<T> {
    pub fn init(title: &str, size: (u32, u32)) -> MgiResult<Self> {
        let sdl_ctx = sdl2::init()?;
        let video_sys = sdl_ctx.video()?;

        Ok(Self {
            title: title.into(),
            size: size.into(),
            sdl_ctx,
            video_sys,
            startup_systems: Vec::new(),
            texture_manager: None,
            game: T::init(),
        })
    }

    pub fn fullscreen(self) -> Self {
        self
    }

    pub fn resizeable(self) -> Self {
        self
    }

    pub fn add_startup_system(mut self, system: fn()) -> Self {
        self.startup_systems.push(system);
        self
    }

    pub fn add_texture_manager(mut self, texture_manager: TextureManager) -> Self {
        self.texture_manager = Some(Rc::new(RefCell::new(texture_manager)));
        self
    }

    pub fn run(mut self) -> MgiResult<()> {
        // Create window
        let window = self
            .video_sys
            .window(&self.title, self.size.x as u32, self.size.y as u32)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        // Run startup systems
        for system in self.startup_systems {
            system()
        }

        let texture_manager = if let Some(tm) = &self.texture_manager {
            Some(Rc::clone(tm))
        } else {
            None
        };

        let mut ctx = Context {
            size: self.size,
            clear_color: Color::WHITE,
            key_down: Vec::new(),
            renderer: Renderer {
                canvas: Rc::new(RefCell::new(canvas)),
                layers: Rc::new(RefCell::new(Vec::new())),
            },
            texture_manager,
        };

        // Load textures
        if let Some(tm) = &self.texture_manager {
            tm.borrow_mut().texture_creator = Some(ctx.canvas().borrow().texture_creator());
            tm.borrow_mut().load_textures()?;
        }

        ctx.canvas().borrow_mut().set_draw_color(ctx.clear_color);
        ctx.canvas().borrow_mut().clear();
        ctx.canvas().borrow_mut().present();
        let mut event_pump = self.sdl_ctx.event_pump()?;
        'gameloop: while self.game.is_running() {
            // Handle events
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { timestamp: _ } => {
                        break 'gameloop;
                    }

                    Event::KeyDown { keycode, .. } => {
                        if let Some(key) = keycode {
                            ctx.key_down.push(key);
                        }
                    }

                    _ => {}
                }
            }

            self.game.update(&mut ctx)?;

            // The render function doesnt actually render: it just determines the layers to render
            // stuff in, their textures, and their, displayed positions
            self.game.render(&mut ctx)?;

            for layer in ctx.layers().borrow_mut().iter_mut() {
                for drawable in layer.iter_mut() {
                    drawable.draw(&ctx)?;
                }
            }

            // // canvas.copy(&texture, None, None)?;
            // canvas.set_draw_color(Color::RED);
            // canvas.fill_rect(Rect::new(0, 0, 200, 200))?;
            // canvas.set_draw_color(Color::WHITE);

            ctx.canvas().borrow_mut().present();
            ctx.key_down = vec![]; // Reset keys pressed
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // 60fps
        }

        Ok(())
    }
}
