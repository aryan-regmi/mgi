use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use pixels::Pixels;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{
    prelude::{
        renderer::{Drawable, Renderer},
        LineSegment, Rect, TextureManager,
    },
    Color, MgiResult, Point, Size,
};

pub struct Context {
    pub(crate) size: Size,
    pub(crate) renderer: Renderer,
    pub(crate) pixels: Rc<RefCell<Pixels>>,
    pub(crate) inputs: Rc<RefCell<WinitInputHelper>>,
    pub(crate) texture_manager: Option<Rc<RefCell<TextureManager>>>,
}

impl Context {
    pub(crate) fn layers(&self) -> RefMut<Vec<Vec<Box<dyn Drawable>>>> {
        self.renderer.layers.borrow_mut()
    }

    pub(crate) fn pixels(&self) -> Rc<RefCell<Pixels>> {
        Rc::clone(&self.pixels)
    }

    pub(crate) fn set_pixel(&self, x: i32, y: i32, color: &[u8]) {
        // Don't wrap around screen
        if x > self.size.width || x < 0 || y > self.size.height || y < 0 {
            return;
        }

        let idx = 4 * (x * self.size.height + y) as usize;

        let frame = &mut self.pixels.borrow_mut();
        let pixels = frame.get_frame_mut();

        if idx < pixels.len() {
            pixels[idx] = color[0];
            pixels[idx + 1] = color[1];
            pixels[idx + 2] = color[2];
            pixels[idx + 3] = color[3];
        }
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

    pub fn draw_texture(
        &self,
        texture_name: &str,
        src: Option<Rect>,
        dest: Option<Rect>,
        layer: usize,
    ) -> MgiResult<()> {
        // Grab correct texture from texture_manager
        let texture_manager = self.texture_manager.as_ref().unwrap();
        let texture_manager = texture_manager.borrow_mut();
        let texture = Rc::clone(&texture_manager.textures[texture_name]);

        texture.borrow_mut().src = src;
        texture.borrow_mut().dest = dest;
        self.draw(texture, layer);

        Ok(())
    }

    // TODO: Implement rotation!
    pub fn draw_rect_outline(&self, rect: Rect, layer: usize) {
        let (h, w) = (rect.size.width, rect.size.height);

        let tl = Point::new(rect.position.y, rect.position.x);
        let bl = Point::new(tl.x, tl.y + h);
        let br = Point::new(bl.x + w, bl.y);
        let tr = Point::new(br.x, tl.y);

        let left_edge = LineSegment::new(tl, bl, rect.color.clone());
        let bottom_edge = LineSegment::new(bl, br, rect.color.clone());
        let right_edge = LineSegment::new(br, tr, rect.color.clone());
        let top_edge = LineSegment::new(tr, tl, rect.color.clone());

        self.draw(left_edge, layer);
        self.draw(bottom_edge, layer);
        self.draw(right_edge, layer);
        self.draw(top_edge, layer);
    }
}
