use glutin_window::GlutinWindow as Window;
use graphics::Context;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{Events, RenderEvent, UpdateEvent, WindowSettings};
use std::any::Any;

pub use piston::{RenderArgs, UpdateArgs};

pub type Color = [f32; 4];

pub type Entities<'a> = Vec<&'a mut dyn Entity>;

pub trait Entity {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct GameBuilder<'a> {
    title: &'a str,
    size: (f64, f64),

    // OpenGL drawing backend
    gl_version: OpenGL,
    gl_ctx: Option<GlGraphics>,

    entities: Vec<&'a mut dyn Entity>,

    // Render function
    render_fn: Option<
        fn(
            ctx: Context,
            gl: &mut GlGraphics,
            game_obj: &mut dyn Any,
            entities: &mut Entities,
            args: &RenderArgs,
        ),
    >,

    // Update function
    update_fn: Option<fn(game_obj: &mut dyn Any, entities: &mut Entities, args: &UpdateArgs)>,

    game_ctx: &'a mut dyn Any,
}

impl<'a> GameBuilder<'a> {
    pub fn init(title: &'a str, size: (f64, f64), game_ctx: &'a mut dyn Any) -> Self {
        let gl_version = OpenGL::V3_2;

        let entities = Vec::new();

        Self {
            title,
            size,
            gl_version,
            gl_ctx: None,
            entities,
            render_fn: None,
            update_fn: None,
            game_ctx,
        }
    }

    // TODO: implement entity ids? (or just use vec idx)
    pub fn add_entity(&mut self, entity: &'a mut dyn Entity) {
        self.entities.push(entity);
    }

    pub fn add_entities(&mut self, entities: &mut Entities<'a>) {
        self.entities.append(entities);
    }

    pub fn set_render_fn(
        &mut self,
        f: fn(
            ctx: Context,
            gl: &mut GlGraphics,
            game_obj: &mut dyn Any,
            entities: &mut Vec<&mut dyn Entity>,
            args: &RenderArgs,
        ),
    ) {
        self.render_fn = Some(f);
    }

    pub fn set_update_fn(
        &mut self,
        f: fn(game_obj: &mut dyn Any, entities: &mut Vec<&mut dyn Entity>, args: &UpdateArgs),
    ) {
        self.update_fn = Some(f);
    }

    pub fn run(&mut self) {
        // Create a Glutin window
        let mut window: Window = WindowSettings::new(self.title, self.size)
            .graphics_api(self.gl_version)
            .resizable(false) // TODO: Add as parameter
            .exit_on_esc(true) // TODO: Add as parameter
            .build()
            .unwrap();

        // Create OpenGL context
        self.gl_ctx = Some(GlGraphics::new(self.gl_version));

        // Event loop
        let mut events = Events::new(piston::EventSettings::new());
        while let Some(event) = events.next(&mut window) {
            if let Some(args) = event.render_args() {
                // TODO: Make sure render_fn is set first
                self.gl_ctx
                    .as_mut()
                    .unwrap()
                    .draw(args.viewport(), |c, gl| {
                        self.render_fn.unwrap()(c, gl, self.game_ctx, &mut self.entities, &args);
                    });
            }

            if let Some(args) = event.update_args() {
                // TODO: Make sure update_fn is set first
                self.update_fn.unwrap()(self.game_ctx, &mut self.entities, &args);
            }
        }
    }

    pub fn size(&self) -> (f64, f64) {
        self.size
    }

    pub fn entities(&mut self) -> &mut Vec<&'a mut dyn Entity> {
        &mut self.entities
    }
}
