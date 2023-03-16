use crate::prelude::MgiResult;
use crate::prelude::Rectangle;
use crate::prelude::Rotation;
use crate::texture_manager::Texture;
use std::{cell::RefCell, rc::Rc};

use sdl2::{keyboard::Keycode, pixels::Color, render::Canvas, video::Window};

use crate::{drawable::Drawable, prelude::Vec2, texture_manager::TextureManager};

pub(crate) struct Renderer {
    pub(crate) canvas: Rc<RefCell<Canvas<Window>>>,

    // TODO: Make this a Rc<RefCell<Vec<Box<SOME_TRAIT>>>> every item in inner vec isn't boxed
    pub(crate) layers: Rc<RefCell<Vec<Vec<Box<dyn Drawable>>>>>,
}

pub struct Context {
    pub(crate) size: Vec2,
    pub(crate) clear_color: Color,
    pub(crate) key_down: Vec<Keycode>,
    pub(crate) renderer: Renderer,
    pub(crate) texture_manager: Option<Rc<RefCell<TextureManager>>>,
}

impl Context {
    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn is_keydown(&self, key: Keycode) -> bool {
        self.key_down.contains(&key)
    }

    pub(crate) fn canvas(&self) -> Rc<RefCell<Canvas<Window>>> {
        Rc::clone(&self.renderer.canvas)
    }

    pub(crate) fn layers(&self) -> Rc<RefCell<Vec<Vec<Box<dyn Drawable>>>>> {
        Rc::clone(&self.renderer.layers)
    }

    pub fn draw<T: Drawable + 'static>(&mut self, drawable: T, layer: usize) {
        let layers = self.layers();

        if layers.borrow_mut().len() > layer {
            layers.borrow_mut()[layer].push(Box::new(drawable));
        } else {
            layers.borrow_mut().push(vec![Box::new(drawable)])
        }
    }

    // TODO: Add rotation
    pub fn draw_texture(
        &mut self,
        texture_name: &str,
        src: Option<Rectangle>,
        dest: Option<Rectangle>,
        rotation: Option<Rotation>,
        layer: usize,
    ) -> MgiResult<()> {
        // NOTE: The texture must be set before hand!
        let mut texture_manager = self.texture_manager.as_ref().unwrap().borrow_mut();
        let texture = texture_manager.get_texture_mut(texture_name);

        if let Some(texture) = texture {
            let layers = self.layers();

            let raw = if let Some(r) = &texture.raw {
                Some(Rc::clone(r))
            } else {
                None
            };

            let rotation = if let Some(rot) = rotation {
                rot
            } else {
                Rotation::Radians(0.0)
            };

            if layers.borrow_mut().len() > layer {
                layers.borrow_mut()[layer].push(Box::new(Texture {
                    name: texture.name.to_owned(),
                    path: texture.path.to_owned(),
                    raw,
                    src,
                    rotation,
                    dest,
                }));
            } else {
                layers.borrow_mut().push(vec![Box::new(Texture {
                    name: texture.name.to_owned(),
                    path: texture.path.to_owned(),
                    raw,
                    src,
                    dest,
                    rotation,
                })])
            }
        }

        Ok(())
    }
}
