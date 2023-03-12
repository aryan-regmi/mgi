use crate::prelude::{TextureManagerRef, TileMap, TileMapRef};
use std::{cell::RefCell, error::Error, rc::Rc};

use raylib::RaylibHandle;

use crate::{prelude::TextureManager, renderer::Renderer, utils::Vec2};

pub struct ResourceManager {
    texture_manager: Option<Rc<RefCell<TextureManager>>>,
    tilemap: Option<Rc<RefCell<TileMap>>>,
}

impl ResourceManager {
    pub fn texture_manager(&self) -> Option<TextureManagerRef> {
        if let Some(tm) = &self.texture_manager {
            Some(TextureManagerRef(tm.as_ref().borrow()))
        } else {
            None
        }
    }

    pub fn tilemap(&self) -> Option<TileMapRef> {
        if let Some(tm) = &self.tilemap {
            Some(TileMapRef(tm.as_ref().borrow_mut()))
        } else {
            None
        }
    }
}

pub trait Drawable {
    fn render(&mut self, renderer: &Renderer, resources: &ResourceManager);
}

pub trait Updateable {
    fn update(&mut self);
}

pub trait Game: Drawable + Updateable {
    fn setup() -> Self;
    fn is_running(&self) -> bool;
    fn stop(&mut self);
    fn handle_events(&mut self, rl: &RaylibHandle);
}

pub struct GameBuilder<'g, T: Game> {
    // Game Window Configs
    size: Vec2,
    resizeable: bool,
    fullscreen: bool,

    // Internal Configs
    renderer: Renderer<'g>,
    resources: ResourceManager,
    game_obj: T,
}

impl<'g, T: Game> GameBuilder<'g, T> {
    pub fn init(title: &str, size: (i32, i32)) -> Self {
        // Initalize raylib
        let (mut rl, rt) = raylib::init().title(title).size(size.0, size.1).build();
        rl.set_exit_key(None);

        let rc_rl = RefCell::new(rl);
        let renderer = Renderer::new(Rc::new(rc_rl), Rc::new(rt));

        Self {
            size: size.into(),
            resizeable: false,
            fullscreen: false,

            renderer,
            resources: ResourceManager {
                texture_manager: None,
                tilemap: None,
            },
            game_obj: T::setup(),
        }
    }

    pub fn set_size(mut self, width: u32, height: u32) -> Self {
        self.size = (width, height).into();
        self
    }

    pub fn set_resizeable(mut self) -> Self {
        self.resizeable = true;
        self
    }

    pub fn set_fullscreen(mut self) -> Self {
        self.fullscreen = true;
        self
    }

    pub fn add_texture_manager(mut self, texture_manager: TextureManager) -> Self {
        self.resources.texture_manager = Some(Rc::new(RefCell::new(texture_manager)));
        self
    }

    pub fn add_tilemap(mut self, tilemap: TileMap) -> Self {
        self.resources.tilemap = Some(Rc::new(RefCell::new(tilemap)));
        self
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        if let Some(tm) = &self.resources.texture_manager {
            tm.as_ref()
                .borrow_mut()
                .load_textures(&mut *self.renderer.rl(), &self.renderer.rt())?;
        }

        while self.game_obj.is_running() {
            self.game_obj.handle_events(&self.renderer.rl());
            self.game_obj.update();
            self.game_obj.render(&mut self.renderer, &self.resources);
        }

        Ok(())
    }
}
