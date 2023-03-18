use crate::{prelude::TextureManager, LayerManager};
use sdl2::{keyboard::Keycode, pixels::Color, render::Canvas, video::Window};
use std::{cell::RefCell, rc::Rc};

pub struct MgiContext {
    pub(crate) inner: Rc<RefCell<MgiInnerContext>>,
    pub(crate) clear_color: Color,
    pub(crate) texture_manager: Option<RefCell<TextureManager>>,
    pub(crate) layer_manager: Option<LayerManager>,
}

pub(crate) struct Inputs {
    pub(crate) key_down: Vec<Keycode>,
    pub(crate) key_up: Vec<Keycode>,
    pub(crate) mouse_pos: (i32, i32),
    pub(crate) left_click: bool,
    pub(crate) right_click: bool,
    pub(crate) middle_click: bool,
}

pub(crate) struct MgiInnerContext {
    pub(crate) canvas: Option<Canvas<Window>>,
    pub(crate) inputs: Inputs,
}

impl MgiContext {
    pub fn key_down(&self, key: Keycode) -> bool {
        self.inner.borrow().inputs.key_down.contains(&key)
    }

    pub fn key_up(&self, key: Keycode) -> bool {
        self.inner.borrow().inputs.key_up.contains(&key)
    }

    pub fn mouse_pos(&self) -> (i32, i32) {
        self.inner.borrow().inputs.mouse_pos
    }

    pub fn left_click(&self) -> bool {
        self.inner.borrow().inputs.left_click
    }

    pub fn right_click(&self) -> bool {
        self.inner.borrow().inputs.right_click
    }

    pub fn middle_click(&self) -> bool {
        self.inner.borrow().inputs.middle_click
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }
}
