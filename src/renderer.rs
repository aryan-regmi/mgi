use std::{borrow::Borrow, cell::RefMut, error::Error};

use raylib::{
    prelude::{Color, RaylibDraw, Rectangle, Vector2},
    RaylibHandle, RaylibThread,
};

use crate::{
    game_builder::{Drawable, ResourceManager},
    layers::Layer,
    prelude::TextureManagerRef,
    utils::{RenderContext, RenderThread},
};

pub struct Renderer<'l> {
    rl: RenderContext,
    rt: RenderThread,
    layers: Vec<&'l dyn Layer>,
}

impl<'l> Renderer<'l> {
    pub(crate) fn new(rl: RenderContext, rt: RenderThread) -> Self {
        Self {
            rl,
            rt,
            layers: Vec::new(),
        }
    }

    pub fn rl(&self) -> RefMut<RaylibHandle> {
        self.rl.borrow_mut()
    }

    pub fn rt(&self) -> &RaylibThread {
        self.rt.borrow()
    }

    // // TODO: Make sure tilemap exists first
    // pub fn draw_tilemap(&self, resources: &ResourceManager) {
    //     // TODO: Proper error handlin
    //     std::borrow::BorrowMut::borrow_mut(&mut resources.tilemap().as_mut().unwrap().0)
    //         .render(self, resources)
    // }
}
