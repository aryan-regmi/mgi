use std::{cell::RefCell, rc::Rc};

use sdl2::{render::Canvas, video::Window};

use crate::drawable::Drawable;

pub(crate) struct Renderer {
    pub(crate) canvas: Rc<RefCell<Canvas<Window>>>,

    // TODO: Make this a Rc<RefCell<Vec<Box<SOME_TRAIT>>>> every item in inner vec isn't boxed
    pub(crate) layers: Rc<RefCell<Vec<Vec<Box<dyn Drawable>>>>>,
}
