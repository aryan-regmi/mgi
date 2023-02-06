use std::any::Any;

use graphics::Context;
use mgi::game::{Color, Entities, Entity, GameBuilder, RenderArgs, UpdateArgs};
use opengl_graphics::GlGraphics;

struct SpinningBox {
    size: f64,

    position: (f64, f64),

    velocity: (f64, f64),

    rotation: f64,

    color: Color,
}

impl Entity for SpinningBox {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

struct Game {
    width: f64,
    height: f64,
    background_color: Color,
}

impl Game {
    fn init(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            background_color: [0.0, 0.0, 0.0, 1.0],
        }
    }

    fn update(game_obj: &mut dyn Any, entities: &mut Entities, args: &UpdateArgs) {
        // Convert ctx to Self
        let this: &mut Self = game_obj.downcast_mut().unwrap();

        // Convert entity into SpinningBox
        let spinning: &mut SpinningBox = entities[0].as_any_mut().downcast_mut().unwrap();

        spinning.position = (this.width / 2.0, this.height / 2.0);
        spinning.rotation += 2.0 * args.dt;
    }

    fn render(
        ctx: Context,
        gl: &mut GlGraphics,
        game_obj: &mut dyn Any,
        entities: &mut Entities,
        args: &RenderArgs,
    ) {
        use graphics::*;

        // Convert ctx to Self
        let this: &mut Self = game_obj.downcast_mut().unwrap();

        // Update the width and height
        this.width = args.window_size[0];
        this.height = args.window_size[1];

        // Convert entity into SpinningBox
        let spinning: &mut SpinningBox = entities[0].as_any_mut().downcast_mut().unwrap();

        // Actual rendering
        let square = rectangle::square(0.0, 0.0, spinning.size);
        clear(this.background_color, gl);

        let transform = ctx
            .transform
            .trans(spinning.position.0, spinning.position.1)
            .rot_rad(spinning.rotation)
            .trans(-spinning.size / 2.0, -spinning.size / 2.0);

        rectangle(spinning.color, square, transform, gl)
    }
}

fn main() {
    let size = (640.0, 480.0);
    let game = &mut Game::init(size.0, size.1);
    let mut game_builder = GameBuilder::init("Test", size, game);

    let mut spin = SpinningBox {
        position: (size.0 / 2.0, size.1 / 2.0),
        velocity: (0.0, 0.0),
        rotation: 0.0,
        color: [1.0, 0.0, 0.0, 1.0],
        size: 50.0,
    };

    game_builder.set_update_fn(Game::update);
    game_builder.set_render_fn(Game::render);

    game_builder.add_entity(&mut spin);

    game_builder.run();
}
