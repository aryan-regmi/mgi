pub mod tiles;

pub mod prelude {
    pub use crate::{tiles, Entity, GameBuilder, WindowConfig};
    pub use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window};
}

pub use prelude::*;

pub use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window};
use sdl2::{
    video::{WindowBuildError, WindowBuilder},
    Sdl, VideoSubsystem,
};
use std::time::Duration;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

pub trait Drawable {
    fn setup(&mut self, canvas: &mut Canvas<Window>);
    fn update(&mut self);
    fn render(&mut self, canvas: &mut Canvas<Window>);
}

pub trait EventHandler {
    fn handle_events(&mut self, event: Event);
}

pub trait Game: Drawable + EventHandler {
    fn is_running(&self) -> bool;
}

pub trait Entity {
    fn update(&mut self);
    fn render(&mut self, canvas: &mut Canvas<Window>);

    fn id(&self) -> u32;
    fn position(&self) -> (f64, f64);
    fn velocity(&self) -> (f64, f64);

    /// Checks if two entities are the same
    fn is_same(&self, other: &dyn Entity) -> bool {
        self.id() == other.id()
    }
}

pub struct WindowConfig {
    pub position: Option<(i32, i32)>,
    pub fullscreen: bool,
    pub borderless: bool,
    pub resizeable: bool,
    pub centered: bool,
}

impl WindowConfig {
    fn build(
        self,
        video_sys: &VideoSubsystem,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Window, WindowBuildError> {
        let mut window_builder = &mut WindowBuilder::new(&video_sys, title, width, height);

        if self.fullscreen {
            window_builder = window_builder.fullscreen_desktop();
        }
        if self.borderless {
            window_builder = window_builder.borderless();
        }
        if self.resizeable {
            window_builder = window_builder.resizable();
        }
        if self.centered {
            window_builder = window_builder.position_centered();
        }

        if let Some(pos) = self.position {
            if (self.fullscreen == false) && (self.centered == false) {
                window_builder = window_builder.position(pos.0, pos.1);
            }
        };

        window_builder.build()
    }
}

pub struct GameBuilder<'a> {
    #[allow(dead_code)]
    size: (u32, u32),

    sdl_ctx: Sdl,

    canvas: Canvas<Window>,

    game_obj: &'a mut dyn Game,
}

impl<'a> GameBuilder<'a> {
    // TODO: Proper error handling!!
    pub fn init(
        title: &str,
        width: u32,
        height: u32,
        game_obj: &'a mut dyn Game,
        window_config: WindowConfig,
    ) -> Self {
        // Initialize SDL
        let sdl_ctx = sdl2::init().unwrap();
        let video_sys = sdl_ctx.video().unwrap();
        // FIX: Propagate errors
        let window = window_config
            .build(&video_sys, title, width, height)
            .expect("Invalid window configuration");

        Self {
            size: (width, height),

            sdl_ctx,

            canvas: window
                .into_canvas()
                .index(find_sdl_gl_driver().unwrap())
                .build()
                .unwrap(),

            game_obj,
        }
    }

    pub fn get_canvas(&self) -> &Canvas<Window> {
        &self.canvas
    }

    pub fn run(&mut self) {
        let mut events = self.sdl_ctx.event_pump().unwrap();

        // Pre-loop setup
        self.game_obj.setup(&mut self.canvas);

        // Main loop
        while self.game_obj.is_running() {
            // self.canvas.clear();

            // Event handling
            for event in events.poll_iter() {
                self.game_obj.handle_events(event);
            }

            // Update
            self.game_obj.update();

            // Render
            self.game_obj.render(&mut self.canvas);
            self.canvas.present();

            // TODO: Replace 60 by fps
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
