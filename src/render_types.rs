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
}
