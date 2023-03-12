use std::{borrow::Borrow, cell::RefMut, collections::HashMap};

use raylib::{prelude::RaylibDrawHandle, RaylibHandle, RaylibThread};

use crate::{
    layers::Layer,
    utils::{RenderContext, RenderThread},
};

pub struct Renderer<'l> {
    rl: RenderContext,
    rt: RenderThread,
    layers: HashMap<usize, Vec<&'l dyn Layer>>,
}

impl<'l> Renderer<'l> {
    pub(crate) fn new(rl: RenderContext, rt: RenderThread) -> Self {
        Self {
            rl,
            rt,
            layers: HashMap::new(),
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

    pub fn layers(&self) -> &HashMap<usize, Vec<&'l dyn Layer>> {
        &self.layers
    }
}
