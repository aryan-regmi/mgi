use sdl2::{pixels::Color, rect::Point};

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
        let canvas = ctx.canvas();

        // Set color of rectangle
        canvas.borrow_mut().set_draw_color(self.color);

        // TODO: Set rotation too!
        if self.fill {
            let xmin = self.position.x;
            let xmax = self.position.x + self.width as i32;
            let ymin = self.position.y;
            let ymax = self.position.y + self.height as i32;

            let mut points: Vec<Point> = Vec::new();
            for x in xmin..xmax {
                for y in ymin..ymax {
                    points.push((x, y).into());
                }
            }

            canvas.borrow_mut().draw_points(points.as_slice())?;
        } else {
            let tl = Point::new(self.position.x, self.position.y);
            let tr = Point::new(self.position.x + self.width as i32, self.position.y);
            let bl = Point::new(self.position.x, self.position.y + self.height as i32);
            let br = Point::new(
                self.position.x + self.width as i32,
                self.position.y + self.height as i32,
            );
            let points = vec![tl, bl, br, tr, tl];

            canvas.borrow_mut().draw_lines(points.as_slice())?;
        }

        // Reset to clear color
        canvas.borrow_mut().set_draw_color(ctx.clear_color);

        Ok(())
    }
}
