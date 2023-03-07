use crate::prelude::{Position, Rotation, Size};

pub struct Rect {
    pub(crate) position: Position,
    pub(crate) size: Size,
    pub(crate) rotation: Rotation,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            position: (x, y).into(),
            size: (width, height).into(),
            rotation: Rotation::Degrees(0.0),
        }
    }

    pub fn rotate(&mut self, angle: Rotation) {
        self.rotation = angle;
    }

    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        let (w, h) = self.size.into();
        let position = self.position;

        let top_right = (position.x + w, position.y);
        let bottom_left = (position.x, position.y + h);

        if x <= top_right.0 && x >= bottom_left.0 && y <= bottom_left.1 && y >= top_right.1 {
            true
        } else {
            false
        }
    }
}
