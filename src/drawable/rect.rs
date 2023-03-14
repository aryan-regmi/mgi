use crate::{
    prelude::{renderer::Drawable, Context, LineSegment},
    Color, Point, Size,
};

#[derive(Debug)]
pub struct Rect {
    pub(crate) position: Point,
    pub(crate) size: Size,
    pub(crate) color: Color,
}

// TODO: Write a `from_center` function that draws at the given center instead of top-left
impl Rect {
    pub fn new(x: i32, y: i32, size: Size, color: Color) -> Self {
        Self {
            position: (x, y).into(),
            size,
            color,
        }
    }
}

impl Drawable for Rect {
    // TODO: Implement rotation
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

            let (xmin, ymin) = self.position.into();
            let (xmax, ymax) = (
                self.position.x + self.size.width,
                self.position.y + self.size.height,
            );

            if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
                pixel.copy_from_slice(self.color.raw());
            }
        }
    }
}
