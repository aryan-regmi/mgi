use std::{cell::RefCell, rc::Rc};

use crate::prelude::{TextureManager, TileMap};

pub(crate) struct ResourceManager {
    pub(crate) texture_manager: Option<Rc<RefCell<TextureManager>>>,
    pub(crate) tilemap_manager: Option<Rc<RefCell<Vec<TileMap>>>>,
}

impl Clone for ResourceManager {
    fn clone(&self) -> Self {
        let texture_manager = if let Some(tm) = &self.texture_manager {
            Some(Rc::clone(tm))
        } else {
            None
        };

        let tilemap_manager = if let Some(tm) = &self.tilemap_manager {
            Some(Rc::clone(tm))
        } else {
            None
        };

        Self {
            texture_manager,
            tilemap_manager,
        }
    }
}

impl ResourceManager {
    pub(crate) fn new(
        texture_manager: Option<Rc<RefCell<TextureManager>>>,
        tilemap_manager: Option<Rc<RefCell<Vec<TileMap>>>>,
    ) -> Self {
        Self {
            texture_manager,
            tilemap_manager,
        }
    }
}
