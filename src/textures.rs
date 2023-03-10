use std::{cell::Ref, collections::HashMap};

use raylib::{texture::Texture2D, RaylibHandle, RaylibThread};

// TODO: Make Texture its own struct (with path and raw_texture as fields)
pub struct TextureManager {
    textures: HashMap<String, (String, Option<Texture2D>)>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn add_texture(&mut self, name: &str, path: &str) {
        self.textures.insert(name.into(), (path.into(), None));
    }

    /// Takes a map of texture names and paths.
    pub fn add_textures(&mut self, textures: HashMap<&str, &str>) {
        for (name, path) in textures {
            self.textures.insert(name.into(), (path.into(), None));
        }
    }

    pub(crate) fn load_textures(
        &mut self,
        rl: &mut RaylibHandle,
        rt: &RaylibThread,
    ) -> Result<(), String> {
        for (path, tex) in self.textures.values_mut() {
            let texture = rl.load_texture(rt, path)?;

            if let None = tex {
                *tex = Some(texture);
            }
        }

        Ok(())
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        let texture = self.textures.get(name)?;
        texture.1.as_ref()
    }
}

pub struct TextureManagerRef<'a>(pub(crate) Ref<'a, TextureManager>);

impl<'a> TextureManagerRef<'a> {
    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.0.get_texture(name)
    }
}
