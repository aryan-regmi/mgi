use std::f32::consts::PI;

pub(crate) fn screen_to_pixel(screen_size: (f32, f32), x: f32, y: f32) -> usize {
    let (w, h) = screen_size;

    let mut idx: usize = (4. * (h * x + y)) as usize;
    if idx >= (4. * h * w) as usize {
        idx = ((4. * h * w) - 4.) as usize;
    }

    idx
}

pub enum Rotation {
    Degrees(f32),
    Radians(f32),
}

impl Rotation {
    pub fn as_degrees(&self) -> f32 {
        match self {
            Rotation::Degrees(d) => *d,
            Rotation::Radians(r) => r * 180. / PI,
        }
    }

    pub fn as_radians(&self) -> f32 {
        match self {
            Rotation::Degrees(d) => d * PI / 180.,
            Rotation::Radians(r) => *r,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl From<(f32, f32)> for Position {
    fn from(tup: (f32, f32)) -> Self {
        Self { x: tup.0, y: tup.1 }
    }
}

impl From<(u32, u32)> for Position {
    fn from(tup: (u32, u32)) -> Self {
        Self {
            x: tup.0 as f32,
            y: tup.1 as f32,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl From<(f32, f32)> for Size {
    fn from(tup: (f32, f32)) -> Self {
        Self {
            width: tup.0,
            height: tup.1,
        }
    }
}

impl From<(u32, u32)> for Size {
    fn from(tup: (u32, u32)) -> Self {
        Self {
            width: tup.0 as f32,
            height: tup.1 as f32,
        }
    }
}

impl Into<(f32, f32)> for Size {
    fn into(self) -> (f32, f32) {
        (self.width, self.height)
    }
}
