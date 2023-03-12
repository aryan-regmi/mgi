use crate::{game_builder::Context, renderer::Drawable, utils::Vec2};
use raylib::{
    drawing::RaylibDrawHandle,
    prelude::{Color, RaylibDraw},
};

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

impl Context {
    pub fn fill_rect(&mut self, mut rect: Rect, layer: usize) {
        rect.fill = true;
        self.draw(rect, layer);
    }
}

impl Drawable for Rect {
    fn draw(&mut self, pen: &mut RaylibDrawHandle) {
        if self.fill {
            pen.draw_rectangle(
                self.position.x,
                self.position.y,
                self.size.x,
                self.size.y,
                self.color,
            );
            return;
        }

        pen.draw_rectangle_lines(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            self.color,
        );
    }
}
