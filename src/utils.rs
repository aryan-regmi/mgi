use std::error::Error;

pub type MgiResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Vec2 {
    fn from(v: (i32, i32)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}
