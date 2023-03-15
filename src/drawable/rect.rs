use crate::{
    prelude::{renderer::Drawable, Context},
    Color, Point, Rotation, Size,
};

#[derive(Debug)]
pub struct Rect {
    pub(crate) position: Point,
    pub(crate) size: Size,
    pub(crate) color: Color,
    pub(crate) rotation: Rotation,
}

// TODO: Write a `from_center` function that draws at the given center instead of top-left
impl Rect {
    pub fn new(x: i32, y: i32, size: Size, color: Color, rotation: Option<Rotation>) -> Self {
        let rotation = if let Some(rot) = rotation {
            rot
        } else {
            Rotation::Radians(0.)
        };

        Self {
            position: (x, y).into(),
            size,
            color,
            rotation,
        }
    }

    pub fn from_center(
        cx: i32,
        cy: i32,
        size: Size,
        color: Color,
        rotation: Option<Rotation>,
    ) -> Self {
        let rotation = if let Some(rot) = rotation {
            rot
        } else {
            Rotation::Radians(0.)
        };

        let (x, y) = (cx - size.width / 2, cy - size.height / 2);

        Self {
            position: (x, y).into(),
            size,
            color,
            rotation,
        }
    }

    pub fn center(&self) -> Point {
        let (x, y) = self.position.into();
        let (w, h) = self.size.into();

        (x + w / 2, y + h / 2).into()
    }

    pub(crate) fn rotate_point(point: Point, rot: Rotation, center: Point) -> Point {
        let (cx, cy) = (center.x as f32, center.y as f32);
        let (x, y) = (cx - point.x as f32, cy - point.y as f32); // Offset by the center
        let theta = rot.to_radians();

        let rx = x * theta.cos() - y * theta.sin();
        let ry = x * theta.sin() + y * theta.cos();

        let rx = cx + rx;
        let ry = cy + ry;

        (rx as i32, ry as i32).into()
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
            let (x, y) = Rect::rotate_point((x, y).into(), self.rotation, self.center()).into();

            let (xmin, ymin) = self.position.into();
            let (xmax, ymax) = (
                self.position.x + self.size.width,
                self.position.y + self.size.height,
            );

            let xmin_act = i32::min(xmin, xmax);
            let xmax_act = i32::max(xmin, xmax);
            let ymin_act = i32::min(ymin, ymax);
            let ymax_act = i32::max(ymin, ymax);

            if x >= xmin_act && x <= xmax_act && y >= ymin_act && y <= ymax_act {
                pixel.copy_from_slice(self.color.raw());
            }
        }
    }
}
