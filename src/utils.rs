use std::{cell::RefCell, rc::Rc};

use raylib::{RaylibHandle, RaylibThread};

pub type RenderContext = Rc<RefCell<RaylibHandle>>;
pub type RenderThread = Rc<RaylibThread>;

pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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

impl From<(i32, i32)> for Vec2 {
    fn from(v: (i32, i32)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}
