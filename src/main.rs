use std::any::Any;

use mgi::game::{Color, Entity, GameBuilder, RenderArgs, UpdateArgs};

struct SpinningBox {
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

struct Game {}

impl Game {
    fn init() -> Self {
        Self {}
    }

    fn update(ctx: &mut dyn Any, entities: &mut Vec<Box<dyn Entity>>, args: &UpdateArgs) {
        let spinning: &'static mut SpinningBox = GameBuilder::convert_entity_mut(entities, 0);
    }

    fn render(ctx: &mut dyn Any, entities: &mut Vec<Box<dyn Entity>>, args: &RenderArgs) {}
}

fn main() {
    let game = &mut Game::init();
    let mut game_builder = GameBuilder::init("Test", (640.0, 480.0), game);

    game_builder.set_background_color([0.0, 0.0, 0.0, 1.0]);

    let spin = SpinningBox {
        position: (game_builder.size().0 / 2.0, game_builder.size().1 / 2.0),
        velocity: (0.0, 0.0),
        rotation: 0.0,
        color: [1.0, 0.0, 0.0, 1.0],
    };

    game_builder.set_update_fn(Game::update);
    game_builder.set_render_fn(Game::render);

    game_builder.add_entity(Box::new(spin));

    game_builder.run();
}
