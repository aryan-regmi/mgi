use crate::prelude::Vec2;
use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;

pub trait Drawable {
    fn draw(&mut self, pen: &mut RaylibDrawHandle, position: Vec2);
    fn position(&self) -> Vec2;
}

pub(crate) struct Renderer {
    pub(crate) clear_color: Color,
    // TODO: abstrct into a trait so that layer: Vec<Box<dyn Layerable>> (so only vec is boxed, not
    // each element in the vec)
    pub(crate) layers: Vec<Vec<Box<dyn Drawable>>>,
}
