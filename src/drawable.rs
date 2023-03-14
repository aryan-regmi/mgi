use sdl2::pixels::Color;

use crate::prelude::*;

pub trait Drawable {
    fn draw(&mut self, ctx: &Context) -> MgiResult<()>;
}

pub struct Rect {
    position: Vec2,
    width: u32,
    height: u32,
    rotation: Option<Rotation>,
    color: Color,
    fill: bool,
}

impl Rect {
    pub fn new(
        position: Vec2,
        width: u32,
        height: u32,
        color: Color,
        rotation: Option<Rotation>,
    ) -> Self {
        Self {
            position,
            width,
            height,
            rotation,
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
        let (x, y): (i32, i32) = self.position.into();

        let canvas = ctx.canvas();

        canvas.borrow_mut().set_draw_color(self.color);
        // TODO: Set rotation too!
        if self.fill {
            canvas
                .borrow_mut()
                .fill_rect(sdl2::rect::Rect::new(x, y, self.width, self.height))?;
        } else {
            canvas
                .borrow_mut()
                .draw_rect(sdl2::rect::Rect::new(x, y, self.width, self.height))?;
        }
        canvas.borrow_mut().set_draw_color(ctx.clear_color);

        Ok(())
    }
}
