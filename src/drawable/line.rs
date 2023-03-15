use crate::{Color, Point};

use super::renderer::Drawable;

// TODO: Add anti-aliasing
pub struct LineSegment {
    start: Point,
    end: Point,
    color: Color,
}

impl LineSegment {
    pub fn new(start: Point, end: Point, color: Color) -> Self {
        Self { start, end, color }
    }
}

impl Drawable for LineSegment {
    // https://stackoverflow.com/questions/34440429/draw-a-line-in-a-bitmap-possibly-with-piston
    fn draw(&mut self, ctx: &crate::prelude::Context) {
        let (mut x0, mut y0) = self.start.into();
        let (x1, y1) = self.end.into();

        // Get absolute x/y offset
        let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
        let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };

        // Get slopes
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        // Initialize error
        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut err2;

        loop {
            // Check end condition
            if x0 == x1 && y0 == y1 {
                break;
            };

            // TODO: Dont wrap around screen

            // Set pixel
            ctx.set_pixel(x0, y0, self.color.raw());

            // Store old error
            err2 = 2 * err;

            // Adjust error and start position
            if err2 > -dx {
                err -= dy;
                x0 += sx;
            }
            if err2 < dy {
                err += dx;
                y0 += sy;
            }
        }
    }
}
