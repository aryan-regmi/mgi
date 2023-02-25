mod helpers;

use crate::helpers::utils::TestGame;
use mgi::{Color, Drawable, Event, EventHandler, GameBuilder, WindowConfig};

struct Renderer;
impl Drawable for Renderer {
    fn setup(&mut self, canvas: &mut mgi::Canvas<mgi::Window>) {}

    fn update(&mut self) {}

    fn render(&mut self, canvas: &mut mgi::Canvas<mgi::Window>) {
        canvas.set_draw_color(Color::BLACK);
    }
}

struct Eventer;
impl EventHandler for Eventer {
    fn handle_events(&mut self, event: Event) {}
}

fn main() {
    GameBuilder::init(
        "Basic Window",
        400,
        400,
        &mut TestGame::new(&mut Renderer, &mut Eventer),
        WindowConfig {
            position: None,
            fullscreen: false,
            borderless: false,
            resizeable: false,
            centered: true,
        },
    )
    .run();
}
