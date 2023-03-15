use super::renderer::Drawable;
use crate::{prelude::Rect, Color, MgiResult};
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
                src: None,
                dest: None,
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

// TODO: Add tint color to tint the texture by
// TODO: Add rotation!
pub struct Texture {
    path: String,
    img: Option<DynamicImage>,
    pub(crate) src: Option<Rect>,
    pub(crate) dest: Option<Rect>,
}

impl Drawable for Rc<RefCell<Texture>> {
    fn draw(&mut self, ctx: &crate::prelude::Context) {
        // NOTE: `img` field must be set by loading textures

        let mut tex_ref = self.borrow_mut();
        let mut adjusted_img = tex_ref.img.as_mut().unwrap().clone();

        // Copy only `src` from image instead of the whole image
        if let Some(src) = &tex_ref.src {
            adjusted_img = adjusted_img.crop(
                src.position.x as u32,
                src.position.y as u32,
                src.size.width as u32,
                src.size.height as u32,
            );
        }

        // Set destination for texture
        let dest = if let Some(dest) = tex_ref.dest.as_ref() {
            dest.clone()
        } else {
            let dest = Rect::new(0, 0, ctx.size, Color::WHITE, None);
            dest
        };

        // Resize image to `dest` size
        if adjusted_img.width() != dest.size.width as u32
            || adjusted_img.height() != dest.size.height as u32
        {
            adjusted_img = adjusted_img.resize_exact(
                dest.size.width as u32,
                dest.size.height as u32,
                image::imageops::FilterType::Nearest,
            );
        }

        let pixels_ref = ctx.pixels();
        let mut pixels = pixels_ref.borrow_mut();
        let screen = pixels.get_frame_mut();

        let (dest_width, dest_height) = dest.size.into();
        let (xmin, xmax) = (dest.position.x, dest.position.x + dest_width);
        let (ymin, ymax) = (dest.position.y, dest.position.y + dest_height);

        let (xmin, ymin) =
            Rect::rotate_point((xmin, ymin).into(), dest.rotation, dest.center()).into();
        let (xmax, ymax) =
            Rect::rotate_point((xmax, ymax).into(), dest.rotation, dest.center()).into();

        // TODO: Implement anit-aliasing
        for x in xmin..xmax {
            for y in ymin..ymax {
                // let (x, y) = Rect::rotate_point((x, y).into(), dest.rotation, dest.center()).into();

                let mut xoffset = (x - xmin) as u32;
                let mut yoffset = (y - ymin) as u32;
                if xoffset >= dest_width as u32 {
                    xoffset = (dest_width - 1) as u32;
                }
                if yoffset >= dest_height as u32 {
                    yoffset = (dest_height - 1) as u32;
                }

                let tpx = adjusted_img.get_pixel(xoffset, yoffset);
                let color = tpx.channels();

                let mut idx = 4 * (x + y * ctx.size.width) as usize;
                if idx >= (4 * ctx.size.height * ctx.size.width) as usize {
                    idx = ((4 * ctx.size.height * ctx.size.width) - 4) as usize;
                }

                // Don't wrap texture around the screen
                if !(x > ctx.size.width || y > ctx.size.height || x < 0 || y < 0) {
                    for i in 0..4 {
                        if color[i] > 0 {
                            screen[idx + i] = color[i];
                        }
                    }
                }
            }
        }
    }
}
