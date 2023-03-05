use std::slice::{ChunksExact, ChunksExactMut};

use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

type Rgba = [u8; 4];

pub trait Colorable {
    fn set_color(&mut self, color: Rgba);
}

impl Colorable for &mut [u8] {
    fn set_color(&mut self, color: Rgba) {
        self.copy_from_slice(&color);
    }
}

pub struct Renderer {
    window_size: (u32, u32),
    pub(crate) pixels: Pixels,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);

        // TODO: Proper error handling
        let pixels = Pixels::new(size.width, size.height, surface_texture).unwrap();

        Self {
            window_size: (size.width, size.height),
            pixels,
        }
    }

    pub fn pixels(&self) -> ChunksExact<'_, u8> {
        self.pixels.get_frame().chunks_exact(4)
    }

    pub fn pixels_mut(&mut self) -> ChunksExactMut<'_, u8> {
        self.pixels.get_frame_mut().chunks_exact_mut(4)
    }

    pub fn draw_pixel(&mut self, x: f32, y: f32, color: Rgba) {
        let w = self.window_size.0 as f32;
        let h = self.window_size.1 as f32;

        let mut idx: usize = (4. * (h * x + y)) as usize;
        if idx >= (4. * h * w) as usize {
            idx = ((4. * h * w) - 4.) as usize;
        }

        let pixels = self.pixels.get_frame_mut();

        pixels[idx] = color[0];
        pixels[idx + 1] = color[0];
        pixels[idx + 2] = color[1];
        pixels[idx + 3] = color[2];
    }
}
