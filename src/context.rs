use crate::prelude::MgiResult;
use crate::prelude::Rectangle;
use crate::prelude::Rotation;
use crate::resource_manager::ResourceManager;
use crate::texture_manager::Texture;
use std::{cell::RefCell, rc::Rc};

use sdl2::{keyboard::Keycode, pixels::Color, render::Canvas, video::Window};

use crate::{drawable::Drawable, prelude::Vec2};

pub(crate) struct Renderer {
    pub(crate) canvas: Rc<RefCell<Canvas<Window>>>,

    pub(crate) layers: Rc<RefCell<Vec<Vec<Box<dyn Drawable>>>>>,
}

pub struct Context {
    pub(crate) size: Vec2,
    pub(crate) clear_color: Color,
    pub(crate) key_down: Vec<Keycode>,
    pub(crate) renderer: Renderer,
    pub(crate) resource_manager: ResourceManager,
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

    // TODO: Add simpler function with less params for ease of use
    pub fn draw_texture(
        &mut self,
        texture_name: &str,
        src: Option<Rectangle>,
        dest: Option<Rectangle>,
        rotation: Option<Rotation>,
        color_tint: Option<Color>,
        alpha_tint: Option<f32>,
        layer: usize,
    ) -> MgiResult<()> {
        // NOTE: The texture must be set before hand!
        let mut texture_manager = self
            .resource_manager
            .texture_manager
            .as_ref()
            .unwrap()
            .borrow_mut();
        let texture = texture_manager.get_texture_mut(texture_name);

        if let Some(texture) = texture {
            let layers = self.layers();

            let raw = if let Some(r) = &texture.raw {
                if let Some(alpha) = alpha_tint {
                    r.borrow_mut().set_alpha_mod((255. * alpha) as u8);
                }
                if let Some(color) = color_tint {
                    r.borrow_mut().set_color_mod(color.r, color.g, color.b);
                }

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

    // TODO: Add simpler function with less params for ease of use
    // TODO: Choose position to place the tilemap too! (add offset to tile.rect)
    // TODO: Don't render tilemap that is outside the screen
    // TODO: Add scrolling tilemap? (Need to add camera first)
    pub fn draw_tilemap(
        &mut self,
        tilemap_id: usize,
        color_tint: Option<Color>,
        alpha_tint: Option<f32>,
        layer: usize,
    ) {
        // TODO: Proper error handling
        let tilemap_manager = self.resource_manager.tilemap_manager.as_ref().unwrap();
        let texture_manager = self.resource_manager.texture_manager.as_ref().unwrap();
        let tilemap = &tilemap_manager.borrow()[tilemap_id];

        for tile in &tilemap.tiles {
            // NOTE: The texture must be set before hand!
            let texture_name = tilemap.get_texture_name(tile.texture_idx);

            if let Some(texture) = texture_manager.borrow_mut().get_texture_mut(texture_name) {
                let layers = self.layers();

                let raw = if let Some(r) = &texture.raw {
                    if let Some(alpha) = alpha_tint {
                        r.borrow_mut().set_alpha_mod((255. * alpha) as u8);
                    }
                    if let Some(color) = color_tint {
                        r.borrow_mut().set_color_mod(color.r, color.g, color.b);
                    }

                    Some(Rc::clone(r))
                } else {
                    None
                };

                if layers.borrow_mut().len() > layer {
                    layers.borrow_mut()[layer].push(Box::new(Texture {
                        name: texture.name.to_owned(),
                        path: texture.path.to_owned(),
                        raw,
                        src: None,
                        dest: Some(tile.rect.clone()),
                        rotation: tile.rotation,
                    }));
                } else {
                    layers.borrow_mut().push(vec![Box::new(Texture {
                        name: texture.name.to_owned(),
                        path: texture.path.to_owned(),
                        raw,
                        src: None,
                        dest: Some(tile.rect.clone()),
                        rotation: tile.rotation,
                    })])
                }
            }
        }
    }
}
