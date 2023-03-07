use std::error::Error;
use std::slice::{ChunksExact, ChunksExactMut};

use image::imageops::FilterType;
use image::Pixel;
use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use winit::window::Window;

use crate::colors::Rgba;
use crate::prelude::pixel_to_screen;
use crate::render_types::Rect;
use crate::textures::TextureManager;
use crate::utils::{screen_to_pixel, Position};

pub struct Renderer {
    window_size: (u32, u32),
    pub(crate) pixels: Pixels,
    texture_manager: Option<TextureManager>,
}

impl Renderer {
    pub fn new(window: &Window) -> Result<Self, Box<dyn Error>> {
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);

        let pixels = Pixels::new(size.width, size.height, surface_texture)?;

        Ok(Self {
            window_size: (size.width, size.height),
            pixels,
            texture_manager: None,
        })
    }

    pub fn add_texture_manager(&mut self, texture_manager: TextureManager) {
        self.texture_manager = Some(texture_manager);
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

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: Rgba) {
        let pixels = self.pixels.get_frame_mut();

        let w = self.window_size.0;
        let h = self.window_size.1;

        // Don't wrap around the screen; ignore values bigger than the screen/window
        if x < 0 || y < 0 || x > w as i32 || y > h as i32 {
            return;
        }

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
            self.draw_pixel(x0, y0, color);

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

    // Reference: https://www.geeksforgeeks.org/bresenhams-circle-drawing-algorithm/
    pub fn draw_circle(&mut self, center: Position, radius: i32, color: Rgba) {
        // Function to put pixels at subsequence points
        let mut draw_sym_pixels = |center: Position, position: Position| {
            let (xc, yc) = (center.x, center.y);
            let (x, y) = (position.x, position.y);

            self.draw_pixel(xc + x, yc + y, color);
            self.draw_pixel(xc - x, yc + y, color);
            self.draw_pixel(xc + x, yc - y, color);
            self.draw_pixel(xc - x, yc - y, color);
            self.draw_pixel(xc + y, yc + x, color);
            self.draw_pixel(xc - y, yc + x, color);
            self.draw_pixel(xc + y, yc - x, color);
            self.draw_pixel(xc - y, yc - x, color);
        };

        let (mut x, mut y) = (0, radius);
        let mut d = 3 - 2 * radius;

        draw_sym_pixels(center, (x, y).into());
        while y >= x {
            x += 1;

            // Check for decision parameter and correspondingly update d, x, y
            if d > 0 {
                y -= 1;
                d = d + 4 * (x - y) + 10;
            } else {
                d = d + 4 * x + 6;
            }

            draw_sym_pixels(center, (x, y).into());
        }
    }

    pub fn draw_texture(
        &mut self,
        texture_name: &str,
        _src: Option<Rect>,
        dest: Rect,
    ) -> Result<(), Box<dyn Error>> {
        // TODO: Make sure texture manager is initalized first!
        if self.texture_manager.is_none() {
            return Err("Texture manager must be initalized before drawing a texture".into());
        }

        // TODO: Implement `src`

        // Grab the right texture
        let texture = self
            .texture_manager
            .as_mut()
            .unwrap()
            .get_texture_mut(texture_name)
            .ok_or(format!(
                "Texture `{}` doesn't exist in the TextureManager",
                texture_name
            ))?;

        // Clamp texture size by window size
        let (mut dest_width, mut dest_height) = dest.size.into();
        let (screen_width, screen_height) = (self.window_size.0 as i32, self.window_size.1 as i32);
        if dest_width > screen_width && dest_height > screen_height {
            dest_width = screen_width;
            dest_height = screen_height;
        } else if dest_width > screen_width {
            dest_width = screen_width;
        } else if dest_height > screen_height {
            dest_height = screen_height;
        }
        let texture = image::imageops::resize(
            texture,
            dest_width as u32,
            dest_height as u32,
            FilterType::Nearest,
        );

        //
        let mut texture_pixels = Vec::with_capacity(texture.pixels().len() * 4);
        for px in texture.pixels() {
            for val in px.channels() {
                texture_pixels.push(*val);
            }
        }

        let screen = self.pixels.get_frame_mut();

        let (width, height) = (dest_width * 4, dest_height);

        let mut s = 0;
        for y in 0..height {
            // Calculate pixel index
            let i = (dest.position.x * 4
                + dest.position.y * self.window_size.0 as i32 * 4
                + y * self.window_size.0 as i32 * 4) as usize;

            // TODO: Don't wrap around the screen; ignore values bigger than the screen/window
            let pos = pixel_to_screen(self.window_size, i);
            if pos.x < 0 || pos.y < 0 || pos.x > screen_width || pos.y > screen_height {
                return Ok(());
            }

            // Merge pixels from sprite into screen
            let zipped = screen[i..i + width as usize]
                .iter_mut()
                .zip(&texture_pixels[s..s + width as usize]);
            for (left, &right) in zipped {
                if right > 0 {
                    *left = right;
                }
            }

            s += width as usize;
        }

        Ok(())
    }
}
