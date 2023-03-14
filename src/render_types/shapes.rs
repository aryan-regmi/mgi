use std::f32::consts::PI;

#[derive(Debug)]
pub enum Rotation {
    Degrees(f32),
    Radians(f32),
}

impl Rotation {
    pub fn as_radians(&self) -> f32 {
        match self {
            Rotation::Degrees(d) => d * (PI / 180.),
            Rotation::Radians(r) => *r,
        }
    }

    pub fn as_degrees(&self) -> f32 {
        match self {
            Rotation::Degrees(d) => *d,
            Rotation::Radians(r) => r * (180. / PI),
        }
    }
}

pub trait Shape {
    fn translate(&mut self, x: i32, y: i32);
    fn rotate(&mut self, rotation: Rotation);
}
