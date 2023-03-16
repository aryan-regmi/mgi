use crate::{
    drawable::{Drawable, Rectangle},
    prelude::Rotation,
    renderer::Renderer,
};
use crate::{prelude::MgiResult, utils::Vec2};
use sdl2::{
    event::Event,
    image::LoadTexture,
    keyboard::Keycode,
    pixels::Color,
    render::{Canvas, Texture as TextureRaw, TextureCreator},
    video::{Window, WindowContext},
    Sdl, VideoSubsystem,
};
use std::{cell::RefCell, rc::Rc, time::Duration};

pub trait Game {
    fn init() -> Self;
    fn is_running(&self) -> bool;
    fn update(&mut self, ctx: &mut Context) -> MgiResult<()>;
    fn render(&mut self, ctx: &mut Context) -> MgiResult<()>;
}

pub struct Context {
    size: Vec2,
    pub(crate) clear_color: Color,
    key_down: Vec<Keycode>,
    renderer: Renderer,
    texture_manager: Rc<RefCell<TextureManager>>,
}

impl Context {
    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn is_keydown(&self, key: Keycode) -> bool {
        self.key_down.contains(&key)
    }

    pub(crate) fn canvas(&self) -> Rc<RefCell<Canvas<Window>>> {
        Rc::clone(&self.renderer.canvas)
    }

    pub(crate) fn layers(&self) -> Rc<RefCell<Vec<Vec<Box<dyn Drawable>>>>> {
        Rc::clone(&self.renderer.layers)
    }

    pub fn draw<T: Drawable + 'static>(&mut self, drawable: T, layer: usize) {
        let layers = self.layers();

        if layers.borrow_mut().len() > layer {
            layers.borrow_mut()[layer].push(Box::new(drawable));
        } else {
            layers.borrow_mut().push(vec![Box::new(drawable)])
        }
    }

    // TODO: Add rotation
    pub fn draw_texture(
        &mut self,
        texture_name: &str,
        src: Option<Rectangle>,
        dest: Option<Rectangle>,
        rotation: Option<Rotation>,
        layer: usize,
    ) -> MgiResult<()> {
        // NOTE: The texture must be set before hand!
        let mut texture_manager = self.texture_manager.borrow_mut();
        let texture = texture_manager.get_texture_mut(texture_name);

        if let Some(texture) = texture {
            let layers = self.layers();

            let raw = if let Some(r) = &texture.raw {
                Some(Rc::clone(r))
            } else {
                None
            };

            let rotation = if let Some(rot) = rotation {
                rot
            } else {
                Rotation::Radians(0.0)
            };

            if layers.borrow_mut().len() > layer {
                layers.borrow_mut()[layer].push(Box::new(Texture {
                    name: texture.name.to_owned(),
                    path: texture.path.to_owned(),
                    raw,
                    src,
                    rotation,
                    dest,
                }));
            } else {
                layers.borrow_mut().push(vec![Box::new(Texture {
                    name: texture.name.to_owned(),
                    path: texture.path.to_owned(),
                    raw,
                    src,
                    dest,
                    rotation,
                })])
            }
        }

        Ok(())
    }
}

pub(crate) struct Texture {
    pub(crate) name: String,
    path: String,
    pub(crate) raw: Option<Rc<TextureRaw>>,
    pub(crate) src: Option<Rectangle>,
    pub(crate) dest: Option<Rectangle>,
    pub(crate) rotation: Rotation,
}

struct TextureManager {
    textures: Vec<Texture>,

    // Used to create the texture
    texture_creator: Option<TextureCreator<WindowContext>>,
}

impl TextureManager {
    fn new() -> Self {
        Self {
            textures: Vec::new(),
            texture_creator: None,
        }
    }

    fn load_textures(&mut self) -> MgiResult<()> {
        for texture in self.textures.iter_mut() {
            texture.raw = Some(Rc::new(
                self.texture_creator
                    .as_ref()
                    .unwrap()
                    .load_texture(&texture.path)?,
            ));
        }

        Ok(())
    }

    fn get_texture_mut(&mut self, name: &str) -> Option<&mut Texture> {
        for texture in &mut self.textures {
            if texture.name == name {
                return Some(texture);
            }
        }

        None
    }
}

pub struct GameBuilder<T: Game> {
    title: String,
    size: Vec2,

    sdl_ctx: Sdl,
    video_sys: VideoSubsystem,

    startup_systems: Vec<fn()>,
    texture_manager: Rc<RefCell<TextureManager>>,
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
            texture_manager: Rc::new(RefCell::new(TextureManager::new())),
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

    pub fn add_texture(self, name: &str, path: &str) -> Self {
        self.texture_manager.borrow_mut().textures.push(Texture {
            name: name.into(),
            path: path.into(),
            raw: None,
            src: None,
            dest: None,
            rotation: Rotation::Radians(0.0),
        });
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

        let mut ctx = Context {
            size: self.size,
            clear_color: Color::WHITE,
            key_down: Vec::new(),
            renderer: Renderer {
                canvas: Rc::new(RefCell::new(canvas)),
                layers: Rc::new(RefCell::new(Vec::new())),
            },
            texture_manager: Rc::clone(&self.texture_manager),
        };

        // Load textures
        self.texture_manager.borrow_mut().texture_creator =
            Some(ctx.canvas().borrow().texture_creator());
        self.texture_manager.borrow_mut().load_textures()?;

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
