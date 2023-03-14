use sdl2::pixels::Color;

use crate::prelude::*;

pub trait Drawable {
    fn draw(&mut self, ctx: &Context) -> MgiResult<()>;
}

pub struct Rect {
    position: Vec2,
    width: u32,
    height: u32,
    color: Color,
    fill: bool,
}

impl Rect {
    pub fn new(position: Vec2, width: u32, height: u32, color: Color) -> Self {
        Self {
            position,
            width,
            height,
            color,
            fill: false,
        }
    }

    pub fn fill(&mut self) {
        self.fill = true;
    }
}

impl Drawable for Rect {
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
