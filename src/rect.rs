use crate::{prelude::Context, renderer::Drawable, Color, Point, Size};

#[derive(Debug)]
pub struct Rect {
    position: Point,
    size: Size,
    color: Color,
    pub(crate) fill: bool,
}

impl Rect {
    pub fn new(x: i32, y: i32, size: Size, color: Color) -> Self {
        Self {
            position: (x, y).into(),
            size,
            color,
            fill: true,
        }
    }
}

impl Drawable for Rect {
    fn draw(&mut self, ctx: &Context) {
        for (i, pixel) in ctx
            .pixels()
            .borrow_mut()
            .get_frame_mut()
            .chunks_exact_mut(4)
            .enumerate()
        {
            let x = (i % ctx.size().width as usize) as i32;
            let y = (i / ctx.size().width as usize) as i32;

            if self.fill {
                let (xmin, ymin) = self.position.into();
                let (xmax, ymax) = (
                    self.position.x + self.size.width,
                    self.position.y + self.size.height,
                );

                if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
                    pixel.copy_from_slice(self.color.raw());
                }
            }

            // TODO: Implement func when self.fill is false (just draw the outlines)

            // TODO: Implement rotation
        }
    }
}
