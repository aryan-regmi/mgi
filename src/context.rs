use crate::{LayerManager, MgiResult, TextureManager};
use sdl2::{keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window};
use std::{cell::RefCell, rc::Rc};

pub struct MgiContext {
    pub(crate) inner: Rc<RefCell<MgiInnerContext>>,
    pub(crate) clear_color: Color,
}

pub(crate) struct Inputs {
    pub(crate) key_down: Vec<Keycode>,
    pub(crate) key_up: Vec<Keycode>,
}

pub(crate) struct MgiInnerContext {
    pub(crate) canvas: Option<Canvas<Window>>,
    pub(crate) texture_manager: Option<TextureManager>,
    pub(crate) layer_manager: Option<LayerManager>,
    pub(crate) inputs: Inputs,
}

impl MgiContext {
    pub fn key_down(&self, key: Keycode) -> bool {
        self.inner.borrow().inputs.key_down.contains(&key)
    }

    pub fn key_up(&self, key: Keycode) -> bool {
        self.inner.borrow().inputs.key_up.contains(&key)
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    pub fn draw_rect(
        &self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        mut color: Color,
        alpha: f32,
    ) -> MgiResult<()> {
        color.a = (255. * alpha) as u8;
        let rect = Rect::new(x, y, width, height);

        let mut ctx = self.inner.borrow_mut();
        let canvas = ctx
            .canvas
            .as_mut()
            .ok_or("Canvas was not initalized properly in GameBuilder::run()")?;

        canvas.set_draw_color(color);
        canvas.draw_rect(rect)?;
        canvas.set_draw_color(self.clear_color);

        Ok(())
    }

    pub fn fill_rect(
        &self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        mut color: Color,
        alpha: f32,
    ) -> MgiResult<()> {
        color.a = (255. * alpha) as u8;
        let rect = Rect::new(x, y, width, height);

        let mut ctx = self.inner.borrow_mut();
        let canvas = ctx
            .canvas
            .as_mut()
            .ok_or("Canvas was not initalized properly in GameBuilder::run()")?;

        canvas.set_draw_color(color);
        canvas.fill_rect(rect)?;
        canvas.set_draw_color(self.clear_color);

        Ok(())
    }

    pub fn draw_line(
        &self,
        start: (i32, i32),
        end: (i32, i32),
        mut color: Color,
        alpha: f32,
    ) -> MgiResult<()> {
        color.a = (255. * alpha) as u8;

        let mut ctx = self.inner.borrow_mut();
        let canvas = ctx
            .canvas
            .as_mut()
            .ok_or("Canvas was not initalized properly in GameBuilder::run()")?;

        canvas.set_draw_color(color);
        canvas.draw_line(start, end)?;
        canvas.set_draw_color(self.clear_color);

        Ok(())
    }
}
