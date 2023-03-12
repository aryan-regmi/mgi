use std::{borrow::Borrow, cell::RefMut};

use raylib::{prelude::RaylibDrawHandle, RaylibHandle, RaylibThread};

use crate::{
    game_builder::Drawable,
    utils::{RenderContext, RenderThread},
};

pub struct Renderer<'l> {
    rl: RenderContext,
    rt: RenderThread,
    pub(crate) layers: Vec<&'l dyn Drawable>,
}

impl<'l> Renderer<'l> {
    pub(crate) fn new(rl: RenderContext, rt: RenderThread) -> Self {
        Self {
            rl,
            rt,
            layers: Vec::new(),
        }
    }

    pub(crate) fn rl(&self) -> RefMut<RaylibHandle> {
        self.rl.borrow_mut()
    }

    pub(crate) fn rt(&self) -> &RaylibThread {
        self.rt.borrow()
    }

    pub fn draw(&self, draw_fn: &dyn Fn(&mut RaylibDrawHandle)) {
        let mut rl = self.rl();
        let mut d = rl.begin_drawing(self.rt());

        draw_fn(&mut d)
    }

    pub fn draw_layers(&self) {
        let mut rl = self.rl();
        let mut d = rl.begin_drawing(self.rt());

        // TODO: Draw all the layers
    }
}
