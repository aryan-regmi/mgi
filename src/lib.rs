pub mod context;
pub mod drawable;
pub mod game_builder;

pub mod prelude {
    pub use crate::context::*;
    pub use crate::drawable::*;
    pub use crate::game_builder::*;
    pub use crate::{Color, Point, Size};

    pub use winit::event::VirtualKeyCode as Keycode;
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl From<(i32, i32)> for Size {
    fn from(v: (i32, i32)) -> Self {
        Self {
            width: v.0,
            height: v.1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<(i32, i32)> for Point {
    fn from(v: (i32, i32)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

impl Into<(i32, i32)> for Point {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub type PixelBuffer<'b> = &'b mut [u8];

#[derive(Debug, Clone)]
pub struct Color {
    raw: [u8; 4],
}

impl Color {
    pub const BLACK: Color = Color {
        raw: [0, 0, 0, 255],
    };
    pub const WHITE: Color = Color {
        raw: [255, 255, 255, 255],
    };
    pub const RED: Color = Color {
        raw: [255, 0, 0, 255],
    };
    pub const GREEN: Color = Color {
        raw: [0, 255, 0, 255],
    };
    pub const BLUE: Color = Color {
        raw: [0, 0, 255, 255],
    };

    pub fn rgba(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
        Self {
            raw: [red, green, blue, (alpha * 255.) as u8],
        }
    }

    pub fn red(&self) -> u8 {
        self.raw[0]
    }

    pub fn green(&self) -> u8 {
        self.raw[1]
    }

    pub fn blue(&self) -> u8 {
        self.raw[2]
    }

    pub fn alpha(&self) -> f32 {
        self.raw[3] as f32 / 255.
    }

    pub fn raw(&self) -> &[u8] {
        &self.raw
    }
}

impl Into<pixels::wgpu::Color> for Color {
    fn into(self) -> pixels::wgpu::Color {
        pixels::wgpu::Color {
            r: self.red() as f64,
            g: self.green() as f64,
            b: self.blue() as f64,
            a: self.alpha() as f64,
        }
    }
}
