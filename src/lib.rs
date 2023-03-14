use std::error::Error;

use pixels::{wgpu::Color, Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

struct Size {
    width: i32,
    height: i32,
}

impl From<(i32, i32)> for Size {
    fn from(v: (i32, i32)) -> Self {
        Self {
            width: v.0,
            height: v.1,
        }
    }
}

pub trait Game: 'static {
    fn init() -> Self;
    fn is_running(&self) -> bool;
    fn stop(&mut self);
    fn draw(&mut self, frame: &mut [u8]);
    fn update(&mut self, inputs: &WinitInputHelper);
}

pub struct GameBuilder<'g, T: Game> {
    title: &'g str,
    size: Size,
    game: T,
}

impl<'g, T: Game> GameBuilder<'g, T> {
    pub fn init(title: &'g str, size: (i32, i32)) -> Self {
        Self {
            title,
            size: size.into(),
            game: T::init(),
        }
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();
        let window = {
            let size = PhysicalSize::new(self.size.width, self.size.height);
            WindowBuilder::new()
                .with_title(self.title)
                .with_inner_size(size)
                .with_max_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)?
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(
                self.size.width as u32,
                self.size.height as u32,
                surface_texture,
            )?
        };
        pixels.set_clear_color(Color::WHITE);

        event_loop.run(move |event, _, control_flow| {
            if !self.game.is_running() {
                *control_flow = ControlFlow::Exit;
                self.game.stop();
                return;
            }

            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
                self.game.draw(pixels.get_frame_mut());

                if let Err(_) = pixels.render() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Handle input
            if input.update(&event) {
                if input.close_requested() {
                    *control_flow = ControlFlow::Exit;
                    self.game.stop();
                    return;
                }

                self.game.update(&input);
            }

            window.request_redraw();
        });
    }
}
