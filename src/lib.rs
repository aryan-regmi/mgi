pub mod prelude {
    pub use crate::{BaseGame, Entity, GameBuilder, WindowConfig};
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

pub trait BaseGame {
    fn update(&mut self);
    fn render(&mut self, canvas: &mut Canvas<Window>);

    fn handle_events(&mut self, event: Event);
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

    game_obj: &'a mut dyn BaseGame,
}

impl<'a> GameBuilder<'a> {
    // TODO: Proper error handling!!
    pub fn init(
        title: &str,
        width: u32,
        height: u32,
        game_obj: &'a mut dyn BaseGame,
        window_config: WindowConfig,
    ) -> Self {
        // Initialize SDL
        let sdl_ctx = sdl2::init().unwrap();
        let video_sys = sdl_ctx.video().unwrap();
        // FIX: Propagate errors
        let window = window_config
            .build(&video_sys, title, width, height)
            .expect("Invalid window configuration");

        // let window = video_sys
        //     .window(title, width, height)
        //     .opengl()
        //     .build()
        //     .unwrap();

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

    pub fn run(&mut self) {
        let mut events = self.sdl_ctx.event_pump().unwrap();
        while self.game_obj.is_running() {
            self.canvas.clear();

            // Event handling
            for event in events.poll_iter() {
                self.game_obj.handle_events(event);
            }

            // Update
            self.game_obj.update();

            // Render
            self.game_obj.render(&mut self.canvas);

            // TODO: Replace 60 by fps
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BaseGame, Canvas, Color, Event, GameBuilder, Keycode, Window, WindowConfig};
    use std::error::Error;

    struct TestGame {
        running: bool,
    }

    impl TestGame {
        pub fn init() -> Self {
            Self { running: true }
        }
    }

    impl BaseGame for TestGame {
        fn update(&mut self) {}

        fn render(&mut self, canvas: &mut Canvas<Window>) {
            canvas.set_draw_color(Color::WHITE);
            canvas.clear();
            canvas.present();
        }

        fn is_running(&self) -> bool {
            self.running
        }

        fn handle_events(&mut self, event: sdl2::event::Event) {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.running = false,
                _ => (),
            }
        }
    }

    #[test]
    fn run_game() -> Result<(), Box<dyn Error>> {
        const WIDTH: u32 = 1280;
        const HEIGHT: u32 = 720;

        GameBuilder::init(
            "TEST",
            WIDTH,
            HEIGHT,
            &mut TestGame::init(),
            WindowConfig {
                position: None,
                fullscreen: false,
                borderless: false,
                resizeable: false,
                centered: true,
            },
        )
        .run();

        Ok(())
    }
}
