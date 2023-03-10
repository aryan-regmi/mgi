use std::{borrow::Borrow, cell::RefMut};

use raylib::{RaylibHandle, RaylibThread};

use crate::utils::{RenderContext, RenderThread};

pub struct Renderer {
    rl: RenderContext,
    rt: RenderThread,
}

impl Renderer {
    pub(crate) fn new(rl: RenderContext, rt: RenderThread) -> Self {
        Self { rl, rt }
    }

    pub fn rl(&self) -> RefMut<RaylibHandle> {
        self.rl.borrow_mut()
    }

    pub fn rt(&self) -> &RaylibThread {
        self.rt.borrow()
    }
}
