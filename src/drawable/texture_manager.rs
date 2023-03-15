use super::renderer::Drawable;
use crate::MgiResult;
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, Pixel};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct TextureManager {
    pub(crate) textures: HashMap<String, Rc<RefCell<Texture>>>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn add_texture(&mut self, name: &str, path: &str) {
        self.textures.insert(
            name.into(),
            Rc::new(RefCell::new(Texture {
                path: path.into(),
                img: None,
            })),
        );
    }

    pub fn add_textures(&mut self, texture_infos: HashMap<&str, &str>) {
        for (name, path) in texture_infos {
            self.add_texture(name, path);
        }
    }

    pub(crate) fn load_textures(&mut self) -> MgiResult<()> {
        for (_, texture) in self.textures.iter_mut() {
            let img = ImageReader::open(&texture.borrow().path)?.decode()?;

            texture.borrow_mut().img = Some(img);
        }

        Ok(())
    }
}

pub struct Texture {
    path: String,
    img: Option<DynamicImage>,
}

impl Drawable for Rc<RefCell<Texture>> {
    fn draw(&mut self, ctx: &crate::prelude::Context) {
        // NOTE: `img` field must be set by loading textures

        // TODO: Copy only `src` from image instead of the whole image

        // TODO: Resize image to `dest` size

        let mut img = self.borrow_mut();
        let img = img.img.as_mut().unwrap();
        let img = img.resize_exact(
            ctx.size.width as u32,
            ctx.size.height as u32,
            image::imageops::FilterType::Nearest,
        );

        let texture = img.pixels();

        for (i, (_, _, tpx)) in texture.enumerate() {
            let y = (i % ctx.size().width as usize) as i32;
            let x = (i / ctx.size().width as usize) as i32;

            let color = tpx.channels();

            ctx.set_pixel(x, y, color);
        }
    }
}
