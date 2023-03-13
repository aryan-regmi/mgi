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
    pub(crate) raw: Option<Texture2D>,
    pub(crate) tint: Color,
    pub(crate) size: Vec2,
    pub(crate) position: Vec2,
}

impl Drawable for Rc<RefCell<Texture>> {
    fn draw(&mut self, pen: &mut raylib::prelude::RaylibDrawHandle, position: Vec2) {
        // FIX: Need to check that `raw` is set before calling this

        let dest = Rectangle::new(
            position.x as f32,
            position.y as f32,
            self.borrow().size.x as f32,
            self.borrow().size.y as f32,
        );

        if let Some(src) = &self.borrow().src {
            pen.draw_texture_pro(
                self.borrow().raw.as_ref().unwrap(),
                src,
                dest,
                Vector2::new(0., 0.),
                0.,
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
                dest,
                Vector2::new(0., 0.),
                0.,
                self.borrow().tint,
            )
        }

        self.borrow_mut().position = position;
    }

    fn position(&self) -> crate::prelude::Vec2 {
        self.borrow().position.clone()
    }
}
