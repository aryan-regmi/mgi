use std::rc::Rc;

use sdl2::{
    image::LoadTexture,
    render::{Texture as TextureRaw, TextureCreator},
    video::WindowContext,
};

use crate::{
    drawable::{Drawable, Rectangle},
    prelude::{Context, MgiResult, Rotation},
};

// TODO: Add abitlity to change opacity of texture
pub(crate) struct Texture {
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) raw: Option<Rc<TextureRaw>>,
    pub(crate) src: Option<Rectangle>,
    pub(crate) dest: Option<Rectangle>,
    pub(crate) rotation: Rotation,
}

impl Drawable for Texture {
    fn draw(&mut self, ctx: &Context) -> MgiResult<()> {
        let canvas = ctx.canvas();

        if self.raw.is_none() {
            return Err(format!(
                "The associated raw texture was not loaded successfully for `{}`",
                self.name
            )
            .into());
        }

        // Get raw texture
        let raw = self.raw.as_ref().unwrap();

        // Get source if it exists
        let src = if let Some(src) = &self.src {
            let src: sdl2::rect::Rect = src.into();
            Some(src)
        } else {
            None
        };

        // Get destination if it exists
        let dest = if let Some(src) = &self.dest {
            let src: sdl2::rect::Rect = src.into();
            Some(src)
        } else {
            None
        };

        canvas.borrow_mut().copy_ex(
            raw,
            src,
            dest,
            self.rotation.to_degrees() as f64,
            None,
            false,
            false,
        )?;

        Ok(())
    }
}

pub struct TextureManager {
    pub(crate) textures: Vec<Texture>,

    // Used to create the texture
    pub(crate) texture_creator: Option<TextureCreator<WindowContext>>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
            texture_creator: None,
        }
    }

    pub fn add_texture(&mut self, name: &str, path: &str) {
        self.textures.push(Texture {
            name: name.into(),
            path: path.into(),
            raw: None,
            src: None,
            dest: None,
            rotation: Rotation::Radians(0.0),
        });
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
