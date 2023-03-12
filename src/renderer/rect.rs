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
}

impl Context {
    pub fn draw_rect(&mut self, rect: Rect, layer: usize) {
        // If the layer already exists, just add to it
        if self.renderer.layers.len() > layer {
            self.renderer.layers[layer].push(Box::new(rect));
            return;
        }

        // Create new layer if the corresponding layer doesn't exist
        self.renderer.layers.push(vec![Box::new(rect)])
    }

    pub fn fill_rect(&mut self, mut rect: Rect, layer: usize) {
        rect.fill = true;

        // If the layer already exists, just add to it
        if self.renderer.layers.len() > layer {
            self.renderer.layers[layer].push(Box::new(rect));
            return;
        }

        // Create new layer if the corresponding layer doesn't exist
        self.renderer.layers.push(vec![Box::new(rect)])
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
