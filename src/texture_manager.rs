use std::{cell::RefCell, collections::HashMap, rc::Rc};

use raylib::prelude::Color;

use crate::render_types::{Rect, Texture};

pub struct TextureManager {
    pub(crate) textures: HashMap<String, Rc<RefCell<Texture>>>,
}

impl TextureManager {
    pub fn init() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn add_texture(mut self, name: &str, path: &str, src: Option<Rect>, dest: Rect) -> Self {
        self.textures.insert(
            name.into(),
            Rc::new(RefCell::new(Texture {
                path: path.into(),
                src,
                dest,
                raw: None,
                tint: Color::WHITE,
            })),
        );
        self
    }

    pub fn get_texture(&self, name: &str) -> Option<Rc<RefCell<Texture>>> {
        if !self.textures.contains_key(name) {
            return None;
        }

        Some(Rc::clone(self.textures.get(name).unwrap()))
    }
}
