use crate::{prelude::Context, renderer::Renderer, Color, Size};
use pixels::{Pixels, SurfaceTexture};
use std::{cell::RefCell, error::Error, rc::Rc};
use winit::{
    dpi::PhysicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

pub trait Game: 'static {
    fn init() -> Self;
    fn is_running(&self) -> bool;
    fn stop(&mut self);
    fn draw(&mut self, ctx: &Context);
    fn update(&mut self, ctx: &Context);
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
        let input = WinitInputHelper::new();
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
        pixels.set_clear_color(Color::WHITE.into());

        let mut ctx = Context {
            size: self.size,
            renderer: Renderer {
                layers: RefCell::new(Vec::new()),
            },
            pixels: Rc::new(RefCell::new(pixels)),
            inputs: Rc::new(RefCell::new(input)),
        };

        event_loop.run(move |event, _, control_flow| {
            if !self.game.is_running() {
                *control_flow = ControlFlow::Exit;
                self.game.stop();
                return;
            }

            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
                self.game.draw(&mut ctx);

                // TODO: Loop through all layers and draw them
                for layer in ctx.layers().iter_mut() {
                    for drawable in layer.iter_mut() {
                        drawable.draw(&ctx);
                    }
                }

                if let Err(_) = ctx.pixels.borrow().render() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Handle input
            if ctx.inputs.borrow_mut().update(&event) {
                if ctx.inputs.borrow().close_requested() {
                    *control_flow = ControlFlow::Exit;
                    self.game.stop();
                    return;
                }

                self.game.update(&mut ctx);
            }

            window.request_redraw();
        });
    }
}
