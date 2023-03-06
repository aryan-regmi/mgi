use crate::prelude::{Position, Rotation, Size};

pub struct Rect {
    pub(crate) position: Position,
    pub(crate) size: Size,
    pub(crate) rotation: Rotation,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
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
