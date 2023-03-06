use std::error::Error;
use std::slice::{ChunksExact, ChunksExactMut};

use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use winit::window::Window;

use crate::colors::Rgba;
use crate::render_types::Rect;
use crate::utils::{screen_to_pixel, Position};

pub struct Renderer {
    window_size: (u32, u32),
    pub(crate) pixels: Pixels,
}

impl Renderer {
    pub fn new(window: &Window) -> Result<Self, Box<dyn Error>> {
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);

        let pixels = Pixels::new(size.width, size.height, surface_texture)?;

        Ok(Self {
            window_size: (size.width, size.height),
            pixels,
        })
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

    // Refrence: https://stackoverflow.com/questions/34440429/draw-a-line-in-a-bitmap-possibly-with-piston
    pub fn draw_line(&mut self, start: Position, end: Position, color: Rgba) {
        // Create local variables for moving start point
        let (x1, y1) = (start.x as i32, start.y as i32);
        let (x2, y2) = (end.x as i32, end.y as i32);
        let (mut x0, mut y0) = (x1, y1);

        // Get absolute x/y offset
        let dx = if x0 > x2 { x0 - x2 } else { x2 - x0 };
        let dy = if y0 > y2 { y0 - y2 } else { y2 - y0 };

        // Get slopes
        let sx = if x0 < x2 { 1 } else { -1 };
        let sy = if y0 < y2 { 1 } else { -1 };

        // Initialize error
        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut err2;

        loop {
            // Draw pixel
            self.draw_pixel(x0 as f32, y0 as f32, color);

            // Check end condition
            if x0 == x2 && y0 == y2 {
                break;
            };

            // Store old error
            err2 = 2 * err;

            // Adjust error and start position
            if err2 > -dx {
                err -= dy;
                x0 += sx;
            }
            if err2 < dy {
                err += dx;
                y0 += sy;
            }
        }
    }

    fn rotated(x: i32, y: i32, center: Position, theta: f32) -> Position {
        // Translate point to origin
        let temp_x = (x - center.x) as f32;
        let temp_y = (y - center.y) as f32;

        // Apply rotation
        let mut rx = temp_x * theta.cos() - temp_y * theta.sin();
        let mut ry = temp_x * theta.sin() + temp_y * theta.cos();

        // Translate back
        rx = rx + center.x as f32;
        ry = ry + center.y as f32;

        (ry, rx).into()
    }

    pub fn draw_rect(&mut self, rect: Rect, color: Rgba) {
        let rot = rect.rotation.as_radians();
        let (w, h) = (rect.size.width, rect.size.height);

        // Only do rotated calcs if angle is not zero
        if rot > 0. {
            let position: Position = (rect.position.x, rect.position.y).into();

            let center = (
                position.x as f32 + 0.5 * w as f32,
                position.y as f32 + 0.5 * h as f32,
            )
                .into();

            // Left edge
            let line1_start = Self::rotated(position.x, position.y, center, rot);
            let line1_end = Self::rotated(position.x, position.y + h, center, rot);
            self.draw_line(line1_start, line1_end, color);

            // Bottom edge
            let line2_start = line1_end;
            let line2_end = Self::rotated(position.x + w, position.y + h, center, rot);
            self.draw_line(line2_start, line2_end, color);

            // Right edge
            let line3_start = line2_end;
            let line3_end = Self::rotated(position.x + w, position.y, center, rot);
            self.draw_line(line3_start, line3_end, color);

            // Top edge
            let line4_start = line3_end;
            let line4_end = line1_start;
            self.draw_line(line4_start, line4_end, color);
        } else {
            let position: Position = (rect.position.y, rect.position.x).into();

            // Left edge
            let line1_start = (position.x, position.y).into();
            let line1_end = (position.x, position.y + h).into();
            self.draw_line(line1_start, line1_end, color);

            // Bottom edge
            let line2_start = line1_end;
            let line2_end = (position.x + w, position.y + h).into();
            self.draw_line(line2_start, line2_end, color);

            // Right edge
            let line3_start = line2_end;
            let line3_end = (position.x + w, position.y).into();
            self.draw_line(line3_start, line3_end, color);

            // Top edge
            let line4_start = line3_end;
            let line4_end = line1_start;
            self.draw_line(line4_start, line4_end, color);
        }
    }
}
