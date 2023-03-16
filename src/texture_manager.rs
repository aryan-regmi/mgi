use std::rc::Rc;

use sdl2::{
    image::LoadTexture,
    render::{Texture as TextureRaw, TextureCreator},
    video::WindowContext,
};

use crate::{
    drawable::Rectangle,
    prelude::{MgiResult, Rotation},
};

pub(crate) struct Texture {
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) raw: Option<Rc<TextureRaw>>,
    pub(crate) src: Option<Rectangle>,
    pub(crate) dest: Option<Rectangle>,
    pub(crate) rotation: Rotation,
}

pub(crate) struct TextureManager {
    pub(crate) textures: Vec<Texture>,

    // Used to create the texture
    pub(crate) texture_creator: Option<TextureCreator<WindowContext>>,
}

impl TextureManager {
    pub(crate) fn new() -> Self {
        Self {
            textures: Vec::new(),
            texture_creator: None,
        }
    }

    pub(crate) fn load_textures(&mut self) -> MgiResult<()> {
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

    pub(crate) fn get_texture_mut(&mut self, name: &str) -> Option<&mut Texture> {
        for texture in &mut self.textures {
            if texture.name == name {
                return Some(texture);
            }
        }

        None
    }
}
