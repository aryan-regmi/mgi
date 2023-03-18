use sdl2::{
    image::LoadTexture,
    render::{Texture as RawTexture, TextureCreator},
    video::WindowContext,
};

use crate::MgiResult;

#[allow(unused)]
struct TextureInfo {
    name: String,
    path: String,
}

pub struct TextureManager {
    texture_info: Vec<TextureInfo>,
    raw_textures: Vec<RawTexture>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            texture_info: vec![],
            raw_textures: vec![],
        }
    }

    pub fn add_texture(&mut self, name: &str, path: &str) {
        self.texture_info.push(TextureInfo {
            name: name.into(),
            path: path.into(),
        })
    }

    pub(crate) fn load_textures(
        &mut self,
        texture_creator: TextureCreator<WindowContext>,
    ) -> MgiResult<()> {
        for tex_info in &self.texture_info {
            let raw_texture = texture_creator.load_texture(&tex_info.path)?;
            self.raw_textures.push(raw_texture);
        }

        Ok(())
    }

    #[allow(unused)]
    pub(crate) fn get_texture(&self, name: &str) -> Option<&RawTexture> {
        for (i, tex_info) in self.texture_info.iter().enumerate() {
            if tex_info.name == name {
                return Some(&self.raw_textures[i]);
            }
        }

        None
    }

    pub(crate) fn get_texture_mut(&mut self, name: &str) -> Option<&mut RawTexture> {
        for (i, tex_info) in self.texture_info.iter().enumerate() {
            if tex_info.name == name {
                return Some(&mut self.raw_textures[i]);
            }
        }

        None
    }
}
