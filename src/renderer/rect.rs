use crate::{game_builder::Context, renderer::Drawable, utils::Vec2};
use raylib::{
    drawing::RaylibDrawHandle,
    ffi::Rectangle,
    prelude::{Color, RaylibDraw},
};

#[derive(Debug)]
pub struct Rect {
    position: Vec2,
    size: Vec2,
    color: Color,
    pub(crate) fill: bool,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32, color: Color) -> Self {
        Self {
            position: (x, y).into(),
            size: (width, height).into(),
            color,
            fill: false,
        }
    }

    pub fn from_center(x: i32, y: i32, width: i32, height: i32, color: Color) -> Self {
        let top_left_x = x - width / 2;
        let top_left_y = y - height / 2;
        let position = (top_left_x, top_left_y).into();

        Self {
            position,
            size: (width, height).into(),
            color,
            fill: false,
        }
    }
}

impl From<Rect> for Rectangle {
    fn from(r: Rect) -> Self {
        Self {
            x: r.position.x as f32,
            y: r.position.y as f32,
            width: r.size.x as f32,
            height: r.size.y as f32,
        }
    }
}

impl From<&Rect> for Rectangle {
    fn from(r: &Rect) -> Self {
        Self {
            x: r.position.x as f32,
            y: r.position.y as f32,
            width: r.size.x as f32,
            height: r.size.y as f32,
        }
    }
}

impl Context {
    pub fn fill_rect(&mut self, mut rect: Rect, layer: usize) {
        rect.fill = true;
        self.draw(rect, layer);
    }
}

impl Drawable for Rect {
    fn draw(&mut self, pen: &mut RaylibDrawHandle, position: Vec2) {
        if self.fill {
            pen.draw_rectangle(position.x, position.y, self.size.x, self.size.y, self.color);
        } else {
            pen.draw_rectangle_lines(position.x, position.y, self.size.x, self.size.y, self.color);
        }

        self.position = position;
    }

    fn position(&self) -> Vec2 {
        self.position.clone()
    }
}
