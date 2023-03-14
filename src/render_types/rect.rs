use crate::{
    game_builder::Context,
    prelude::{Rotation, Shape},
    render_types::Drawable,
    utils::Vec2,
};
use raylib::{
    drawing::RaylibDrawHandle,
    prelude::{Color, RaylibDraw, Rectangle, Vector2},
};

#[derive(Debug)]
pub struct Rect {
    position: Vec2,
    rotation: Rotation,
    pub(crate) size: Vec2,
    color: Color,
    pub(crate) fill: bool,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32, color: Color) -> Self {
        Self {
            position: (x, y).into(),
            rotation: Rotation::Degrees(0.),
            size: (width, height).into(),
            color,
            fill: true,
        }
    }

    pub fn from_center(x: i32, y: i32, width: i32, height: i32, color: Color) -> Self {
        let top_left_x = x - width / 2;
        let top_left_y = y - height / 2;
        let position = (top_left_x, top_left_y).into();

        Self {
            position,
            rotation: Rotation::Degrees(0.),
            size: (width, height).into(),
            color,
            fill: false,
        }
    }

    pub fn rotation(&self) -> &Rotation {
        &self.rotation
    }

    pub fn center(&self) -> Vec2 {
        let (x, y) = (self.position.x, self.position.y);
        let (w, h) = (self.size.x, self.size.y);

        let cx = x + w / 2;
        let cy = y + h / 2;

        (cx, cy).into()
    }

    pub fn position(&self) -> &Vec2 {
        &self.position
    }

    pub fn size(&self) -> &Vec2 {
        &self.size
    }
}

impl From<Rect> for raylib::ffi::Rectangle {
    fn from(r: Rect) -> Self {
        Self {
            x: r.position.x as f32,
            y: r.position.y as f32,
            width: r.size.x as f32,
            height: r.size.y as f32,
        }
    }
}

impl From<&Rect> for raylib::ffi::Rectangle {
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
    fn draw(&mut self, pen: &mut RaylibDrawHandle) {
        if self.fill {
            pen.draw_rectangle_pro(
                Rectangle::new(
                    self.position.x as f32,
                    self.position.y as f32,
                    self.size.x as f32,
                    self.size.y as f32,
                ),
                Vector2::new(0., 0.),
                self.rotation.as_degrees(),
                self.color,
            )
        } else {
            // FIXME: calculate rotated position and draw each line!

            pen.draw_rectangle_lines(
                self.position.x,
                self.position.y,
                self.size.x,
                self.size.y,
                self.color,
            );
        }
    }
}

impl Shape for Rect {
    fn translate(&mut self, x: i32, y: i32) {
        self.position = (x, y).into();
    }

    fn rotate(&mut self, rotation: crate::prelude::Rotation) {
        self.rotation = rotation;
    }
}
