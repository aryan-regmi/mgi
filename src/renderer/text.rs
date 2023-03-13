use raylib::prelude::{Color, RaylibDraw};

use crate::utils::Vec2;

use super::Drawable;

// TODO: Add font type as field
pub struct Text {
    text: String,
    position: Vec2,
    color: Color,
    font_size: i32,
}

impl Text {
    pub fn new(text: &str, position: Vec2, color: Color, font_size: i32) -> Self {
        Self {
            text: text.into(),
            position,
            color,
            font_size,
        }
    }
}

impl Drawable for Text {
    fn draw(&mut self, pen: &mut raylib::prelude::RaylibDrawHandle, position: Vec2) {
        pen.draw_text(
            &self.text,
            position.x,
            position.y,
            self.font_size,
            self.color,
        );

        self.position = position;
    }

    fn position(&self) -> Vec2 {
        self.position.clone()
    }
}
