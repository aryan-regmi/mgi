use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use pixels::Pixels;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{
    prelude::Rect,
    renderer::{Drawable, Renderer},
    Color, Size,
};

pub struct Context {
    pub(crate) size: Size,
    pub(crate) renderer: Renderer,
    pub(crate) pixels: Rc<RefCell<Pixels>>,
    pub(crate) inputs: Rc<RefCell<WinitInputHelper>>,
}

impl Context {
    pub(crate) fn layers(&self) -> RefMut<Vec<Vec<Box<dyn Drawable>>>> {
        self.renderer.layers.borrow_mut()
    }

    pub(crate) fn pixels(&self) -> Rc<RefCell<Pixels>> {
        Rc::clone(&self.pixels)
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn key_pressed(&self, keycode: VirtualKeyCode) -> bool {
        self.inputs.borrow().key_pressed(keycode)
    }

    pub fn clear_background(&self, color: Color) {
        self.pixels().borrow_mut().set_clear_color(color.into());
    }

    pub fn draw<T: Drawable + 'static>(&self, drawable: T, layer: usize) {
        // If layer already exists, add the drawable to it
        if self.layers().len() > layer {
            self.layers()[layer].push(Box::new(drawable));
            return;
        }

        // Create new layer if necessary
        self.layers().push(vec![Box::new(drawable)]);
    }

    pub fn draw_rect_outline(&self, mut rect: Rect, layer: usize) {
        rect.fill = false;
        self.draw(rect, layer);
    }
}
