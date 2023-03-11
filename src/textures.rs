use std::{cell::Ref, collections::HashMap};

use raylib::{texture::Texture2D, RaylibHandle, RaylibThread};

pub struct Texture {
    name: String,
    path: String,
    raw_texture: Option<Texture2D>,
}

// TODO: Make Texture its own struct (with path and raw_texture as fields)
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
        });
    }

    /// Takes a map of texture names and paths.
    pub fn add_textures(&mut self, textures: HashMap<&str, &str>) {
        for (name, path) in textures {
            self.textures.push(Texture {
                name: name.into(),
                path: path.into(),
                raw_texture: None,
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

    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        for texture in &self.textures {
            if texture.name == name {
                return texture.raw_texture.as_ref();
            }
        }

        None
    }
}

pub struct TextureManagerRef<'a>(pub(crate) Ref<'a, TextureManager>);

impl<'a> TextureManagerRef<'a> {
    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.0.get_texture(name)
    }
}
