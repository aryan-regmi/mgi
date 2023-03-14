use std::{cell::RefCell, rc::Rc};

use raylib::{
    prelude::{Color, RaylibDraw, Rectangle, Vector2},
    texture::Texture2D,
};

use crate::prelude::Vec2;

use super::{Drawable, Rect};

#[derive(Debug)]
pub struct Texture {
    pub(crate) path: String,
    pub(crate) src: Option<Rect>,
    pub(crate) dest: Rect,
    pub(crate) raw: Option<Texture2D>,
    pub(crate) tint: Color,
}

impl Texture {
    pub fn set_tint(&mut self, tint: Color) {
        self.tint = tint;
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.dest.size = (width, height).into();
    }

    pub fn set_dest(&mut self, dest: Rect) {
        self.dest = dest;
    }

    pub fn position(&self) -> &Vec2 {
        self.dest.position()
    }
}

impl Drawable for Rc<RefCell<Texture>> {
    fn draw(&mut self, pen: &mut raylib::prelude::RaylibDrawHandle) {
        if self.borrow_mut().raw.is_none() {
            panic!(
                "The texture defined in `{}` was not loaded properly by the TextureManager",
                self.borrow().path
            );
        }

        if let Some(src) = &self.borrow().src {
            pen.draw_texture_pro(
                self.borrow().raw.as_ref().unwrap(),
                src,
                &self.borrow().dest,
                Vector2::new(0., 0.),
                360. - self.borrow().dest.rotation().as_degrees(),
                self.borrow().tint,
            )
        } else {
            let src = Rectangle::new(
                0.,
                0.,
                self.borrow().raw.as_ref().unwrap().width as f32,
                self.borrow().raw.as_ref().unwrap().height as f32,
            );

            pen.draw_texture_pro(
                self.borrow().raw.as_ref().unwrap(),
                src,
                &self.borrow().dest,
                Vector2::new(0., 0.),
                360. - self.borrow().dest.rotation().as_degrees(),
                self.borrow().tint,
            )
        }
    }
}
