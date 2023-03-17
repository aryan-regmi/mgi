use sdl2::pixels::Color;

use crate::prelude::*;

pub trait Drawable {
    fn draw(&mut self, ctx: &Context) -> MgiResult<()>;
}

#[derive(Clone)]
pub struct Rectangle {
    pub(crate) position: Vec2,
    pub(crate) width: u32,
    pub(crate) height: u32,
    color: Color,
    fill: bool,
}

impl Rectangle {
    pub fn new(position: Vec2, width: u32, height: u32, color: Color) -> Self {
        Self {
            position,
            width,
            height,
            color,
            fill: true,
        }
    }

    pub fn fill(&mut self, val: bool) {
        self.fill = val;
    }
}

impl Drawable for Rectangle {
    fn draw(&mut self, ctx: &Context) -> MgiResult<()> {
        let canvas = ctx.canvas();

        // Set color of rectangle
        canvas.borrow_mut().set_draw_color(self.color);

        if self.fill {
            canvas.borrow_mut().fill_rect(sdl2::rect::Rect::new(
                self.position.x,
                self.position.y,
                self.width,
                self.height,
            ))?;
        } else {
            canvas.borrow_mut().draw_rect(sdl2::rect::Rect::new(
                self.position.x,
                self.position.y,
                self.width,
                self.height,
            ))?;
        }

        // Reset to clear color
        canvas.borrow_mut().set_draw_color(ctx.clear_color);

        Ok(())
    }
}

impl From<&Rectangle> for sdl2::rect::Rect {
    fn from(r: &Rectangle) -> Self {
        sdl2::rect::Rect::new(r.position.x, r.position.y, r.width, r.height)
    }
}
