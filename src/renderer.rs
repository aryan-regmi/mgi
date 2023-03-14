use std::cell::RefCell;

use crate::prelude::Context;

pub trait Drawable {
    fn draw(&mut self, ctx: &Context);
}

pub(crate) struct Renderer {
    pub(crate) layers: RefCell<Vec<Vec<Box<dyn Drawable>>>>,
}
