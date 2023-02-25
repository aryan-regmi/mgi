mod helpers;

use helpers::utils::TestGame;
use mgi::{Drawable, Event, EventHandler, GameBuilder, WindowConfig};
use sdl2::image::LoadTexture;

struct Renderer;
impl Drawable for Renderer {
    fn update(&mut self) {}

    fn render(&mut self, _canvas: &mut mgi::Canvas<mgi::Window>) {}

    fn setup(&mut self, canvas: &mut mgi::Canvas<mgi::Window>) {
        let texture_creator = canvas.texture_creator();
        let bg_image = texture_creator
            .load_texture("examples/assets/bg.png")
            .expect("Unable to load texture");

        canvas
            .copy(&bg_image, None, None)
            .expect("Unable to copy texture");
    }
}

struct Eventer;
impl EventHandler for Eventer {
    fn handle_events(&mut self, _event: Event) {}
}

fn main() {
    GameBuilder::init(
        "Window With BG Image",
        800,
        800,
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
