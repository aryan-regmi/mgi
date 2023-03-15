use super::renderer::Drawable;
use crate::MgiResult;
use image::{io::Reader as ImageReader, ImageBuffer, Rgba};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct TextureManager<'tex> {
    textures: Rc<RefCell<HashMap<&'tex str, Texture<'tex>>>>,
}

impl<'tex> TextureManager<'tex> {
    pub fn new() -> Self {
        Self {
            textures: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn add_texture(&mut self, name: &'tex str, path: &'tex str) {
        self.textures
            .borrow_mut()
            .insert(name, Texture { path, img: None });
    }

    pub fn add_textures(&mut self, texture_infos: HashMap<&'tex str, &'tex str>) {
        for (name, path) in texture_infos {
            self.add_texture(name, path);
        }
    }

    fn load_textures(&mut self) -> MgiResult<()> {
        for (_, texture) in self.textures.borrow_mut().iter_mut() {
            let img = ImageReader::open(texture.path)?.decode()?;

            // TODO: Copy only part of image from source rect (during drawing?)

            // TODO: Resize image to fit destination

            texture.img = Some(img.into_rgba8());
        }

        Ok(())
    }
}

struct Texture<'tex> {
    path: &'tex str,
    img: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
}

impl<'tex> Drawable for Texture<'tex> {
    fn draw(&mut self, ctx: &crate::prelude::Context) {
        // NOTE: `img` field must be set by loading textures

        let img = self.img.as_ref().unwrap();
        let texture = img.pixels();
        let (tex_width, tex_height) = (img.width(), img.height());

        let screen_pixels = ctx.pixels();
        let screen = screen_pixels.borrow_mut().get_frame_mut();

        todo!()
    }
}
