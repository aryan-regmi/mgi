use std::{cell::Ref, collections::HashMap};

use raylib::{texture::Texture2D, RaylibHandle, RaylibThread};

use crate::layers::{Layer, LayerInfo};

pub struct Texture {
    pub(crate) name: String,
    path: String,
    raw_texture: Option<Texture2D>,
    layer_info: LayerInfo,
}

impl Texture {
    pub fn raw_texture(&self) -> Option<&Texture2D> {
        self.raw_texture.as_ref()
    }
}

impl Layer for Texture {
    fn set_layer(&mut self, layer: usize) {
        self.layer_info.layer_id = layer;
    }

    fn set_visible(&mut self, visible: bool) {
        self.layer_info.visible = visible;
    }

    fn tint(&self) -> raylib::prelude::Color {
        self.layer_info.tint
    }

    fn set_tint(&mut self, tint: raylib::prelude::Color) {
        self.layer_info.tint = tint;
    }
}

pub struct TextureManager {
    textures: Vec<Texture>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
        }
    }

    pub fn add_texture(&mut self, name: &str, path: &str) {
        self.textures.push(Texture {
            name: name.into(),
            path: path.into(),
            raw_texture: None,
            layer_info: LayerInfo::default(),
        });
    }

    /// Takes a map of texture names and paths.
    pub fn add_textures(&mut self, textures: HashMap<&str, &str>) {
        for (name, path) in textures {
            self.textures.push(Texture {
                name: name.into(),
                path: path.into(),
                raw_texture: None,
                layer_info: LayerInfo::default(),
            });
        }
    }

    pub(crate) fn load_textures(
        &mut self,
        rl: &mut RaylibHandle,
        rt: &RaylibThread,
    ) -> Result<(), String> {
        for texture in self.textures.iter_mut() {
            let raw_texture = rl.load_texture(rt, &texture.path)?;
            texture.raw_texture = Some(raw_texture);
        }

        Ok(())
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture> {
        for texture in &self.textures {
            if texture.name == name {
                return Some(texture);
            }
        }

        None
    }
}

pub struct TextureManagerRef<'a>(pub(crate) Ref<'a, TextureManager>);

impl<'a> TextureManagerRef<'a> {
    pub fn get_texture(&self, name: &str) -> Option<&Texture> {
        self.0.get_texture(name)
    }
}
