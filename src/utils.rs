use std::{error::Error, f32::consts::PI};

pub type MgiResult<T> = Result<T, Box<dyn Error>>;

pub enum Rotation {
    Degrees(f32),
    Radians(f32),
}

impl Rotation {
    pub fn to_radians(&self) -> f32 {
        match self {
            Rotation::Degrees(d) => d * (PI / 180.),
            Rotation::Radians(r) => *r,
        }
    }

    pub fn to_degrees(&self) -> f32 {
        match self {
            Rotation::Degrees(d) => *d,
            Rotation::Radians(r) => r * (180. / PI),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Vec2 {
    fn from(v: (i32, i32)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

impl Into<(i32, i32)> for Vec2 {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl From<(u32, u32)> for Vec2 {
    fn from(v: (u32, u32)) -> Self {
        Self {
            x: v.0 as i32,
            y: v.1 as i32,
        }
    }
}

impl Into<(u32, u32)> for Vec2 {
    fn into(self) -> (u32, u32) {
        (self.x as u32, self.y as u32)
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from(v: (f32, f32)) -> Self {
        Self {
            x: v.0 as i32,
            y: v.1 as i32,
        }
    }
}

impl Into<(f32, f32)> for Vec2 {
    fn into(self) -> (f32, f32) {
        (self.x as f32, self.y as f32)
    }
}
