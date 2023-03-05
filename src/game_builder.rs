use pixels::{Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::event::Event;
use winit::event_loop::ControlFlow;
use winit::window::Fullscreen;
use winit::{event_loop::EventLoop, window::WindowBuilder};
use winit_input_helper::WinitInputHelper;

pub trait Drawable {
    fn update(&mut self);
    fn render(&mut self, pixels: &mut Pixels);
}

pub trait EventHandler {
    fn handle_events(&mut self, event: &WinitInputHelper, cf: &mut ControlFlow);
}

pub trait Game: Drawable + EventHandler + 'static {
    fn setup() -> Self;
    fn is_running(&self) -> bool;
    fn stop(&mut self);
}

pub struct GameBuilder<T: Game> {
    title: String,
    size: Option<(u32, u32)>,
    resizeable: bool,
    fullscreen: bool,
    event_loop: EventLoop<()>,
    game_obj: T,
}

impl<T: Game> GameBuilder<T> {
    pub fn init(title: &str) -> Self {
        Self {
            title: title.into(),
            size: None,
            resizeable: false,
            fullscreen: false,
            event_loop: EventLoop::new(),
            game_obj: T::setup(),
        }
    }

    pub fn set_size(mut self, width: u32, height: u32) -> Self {
        self.size = Some((width, height));
        self
    }

    pub fn set_resizeable(mut self) -> Self {
        self.resizeable = true;
        self
    }

    pub fn set_fullscreen(mut self) -> Self {
        self.fullscreen = true;
        self
    }

    pub fn run(mut self) {
        // Setup the main window
        let size = self.size.expect("Size must be set first"); // TODO: Proper error handling
        let window = WindowBuilder::new()
            .with_title(self.title.to_owned())
            .with_inner_size(PhysicalSize::new(size.0, size.1))
            .with_resizable(self.resizeable)
            .build(&self.event_loop)
            .unwrap(); // TODO: Proper error handling
        if self.fullscreen {
            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
        }
        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(size.0, size.1, surface_texture).unwrap() // TODO: Proper error handling
        };

        // Game loop
        let mut input = WinitInputHelper::new();
        self.event_loop.run(move |event, _, control_flow| {
            // Handle input
            if input.update(&event) {
                // Stop running if window is quit
                if input.close_requested() {
                    self.game_obj.stop();
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                self.game_obj.handle_events(&input, control_flow);
                window.request_redraw();
            }

            // Update the game
            self.game_obj.update();

            // Render the game
            if let Event::RedrawRequested(_) = event {
                self.game_obj.render(&mut pixels);
                if let Err(_) = pixels.render() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
        });
    }
}
