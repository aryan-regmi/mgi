use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

pub struct Renderer {
    pixels: Pixels,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);

        // TODO: Proper error handling
        let pixels = Pixels::new(size.width, size.height, surface_texture).unwrap();

        Self { pixels }
    }

    pub fn draw_point(&mut self, x: u32, y: u32, color: [u8; 4]) {
        let frame = self.pixels.get_frame_mut();

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let p = i as u32;

            if p == x * y {
                pixel.copy_from_slice(&color);
            }
        }
    }
}
