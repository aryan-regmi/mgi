use std::slice::{ChunksExact, ChunksExactMut};

use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use winit::window::Window;

use crate::colors::Rgba;
use crate::utils::screen_to_pixel;

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

    pub fn clear_color(&mut self, color: Rgba) {
        let color = color.as_raw();
        let c = Color {
            r: color[0] as f64,
            g: color[1] as f64,
            b: color[2] as f64,
            a: color[3] as f64,
        };
        self.pixels.set_clear_color(c);
    }

    pub fn draw_pixel(&mut self, x: f32, y: f32, color: Rgba) {
        let pixels = self.pixels.get_frame_mut();

        let w = self.window_size.0 as f32;
        let h = self.window_size.1 as f32;
        let idx = screen_to_pixel((w, h), x, y);

        let color = color.as_raw();
        pixels[idx] = color[0];
        pixels[idx + 1] = color[1];
        pixels[idx + 2] = color[2];
        pixels[idx + 3] = color[3];
    }

    pub fn draw_line(&mut self, start: (f32, f32), end: (f32, f32), color: Rgba) {
        let (x1, y1) = start;
        let (x2, y2) = end;

        let dx = if x1 < x2 { 1. } else { -1. };
        let dy = if y1 < y2 { 1. } else { -1. };

        let (mut x, mut y) = (x1, y1);
        while x != x2 && y != y2 {
            self.draw_pixel(x, y, color);
            x += dx;
            y += dy;
        }
    }

    pub fn draw_rect(&mut self, position: (f32, f32), size: (f32, f32), color: Rgba) {
        // Separate into 4 lines
        let line1_start = position;
        let line1_end = (position.0 + size.0, position.1);
    }
}
