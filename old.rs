use std::any::Any;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    AdvancedWindow, Events, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, WindowSettings,
};

pub trait Entity {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug)]
struct SpinningRect {
    x: f64,
    y: f64,
    angle: f64,
    color: [f32; 4],
}

impl Entity for SpinningRect {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct TestGame<'a> {
    title: &'a str,

    // Game size
    width: f64,
    height: f64,

    // OpenGL drawing backend
    gl_version: OpenGL,
    gl: Option<GlGraphics>,

    // Game entities
    entities: Vec<Box<dyn Entity>>,
}

impl<'a> TestGame<'a> {
    fn init(title: &'a str, width: f64, height: f64) -> Self {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let mut entities: Vec<Box<dyn Entity>> = Vec::new();
        entities.push(Box::new(SpinningRect {
            x: width / 2.0,
            y: height / 2.0,
            angle: 0.0,
            color: RED,
        }));

        let gl_version = OpenGL::V2_1;

        Self {
            title,
            width,
            height,
            gl_version,
            gl: None,
            entities,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // Black background
        let background = [0.0, 0.0, 0.0, 1.0];

        // Update width and heights first!
        self.width = args.window_size[0];
        self.height = args.window_size[1];

        let sr = &self.entities[0]
            .as_any()
            .downcast_ref::<SpinningRect>()
            .unwrap();

        let square = rectangle::square(0.0, 0.0, 50.0);

        // Actual rendering
        self.gl.as_mut().unwrap().draw(args.viewport(), |c, gl| {
            // Clear the screen
            clear(background, gl);

            let transform = c
                .transform
                .trans(sr.x.into(), sr.y.into())
                .rot_rad(sr.angle)
                .trans(-25.0, -25.0);

            // The rotating box
            rectangle(sr.color, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let sr = &mut self.entities[0]
            .as_any_mut()
            .downcast_mut::<SpinningRect>()
            .unwrap();

        sr.x = self.width / 2.0;
        sr.y = self.height / 2.0;

        dbg!(self.width, self.height);

        sr.angle += 2.0 * args.dt;
    }

    fn run(&mut self) {
        // Greate a Glutin window.
        let mut window: Window = WindowSettings::new(self.title, (self.width, self.height))
            .graphics_api(self.gl_version)
            .resizable(false)
            .exit_on_esc(true) // TODO: Remove this later
            .build()
            .unwrap();

        self.gl = Some(GlGraphics::new(self.gl_version));

        // Event loop
        let mut events = Events::new(piston::EventSettings::new());
        while let Some(event) = events.next(&mut window) {
            if let Some(args) = event.render_args() {
                self.render(&args);
            }

            if let Some(args) = event.update_args() {
                self.update(&args);
            }
        }
    }
}

fn main() -> Result<(), String> {
    TestGame::init("Test", 720.0, 480.0).run();

    Ok(())
}
