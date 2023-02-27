mod helpers;

use helpers::utils::TestGame;
use mgi::{
    tiles::{TileMap, TileSet},
    Drawable, Event, EventHandler, GameBuilder, WindowConfig,
};

struct Renderer<'a> {
    tilemap: Option<TileMap<TileSet<'a>>>,
}

impl<'a> Renderer<'a> {
    fn new() -> Self {
        Self { tilemap: None }
    }
}

impl<'a> Drawable for Renderer<'a> {
    fn update(&mut self) {}

    fn render(&mut self, _canvas: &mut mgi::Canvas<mgi::Window>) {}

    fn setup(&mut self, canvas: &'a mut mgi::Canvas<mgi::Window>) {
        let mut tileset = TileSet::new(canvas);
        tileset.add_tile_type(
            "Water",
            "examples/assets/Sprout Lands - Sprites - Basic pack/Tilesets/Water.png",
        );
        tileset.add_tile_type(
            "Fences",
            "examples/assets/Sprout Lands - Sprites - Basic pack/Tilesets/Fences.png",
        );
        self.tilemap = Some(TileMap::from_tileset(10, 10, (32, 32), tileset));
    }
}

struct Eventer;
impl EventHandler for Eventer {
    fn handle_events(&mut self, _event: Event) {}
}

fn main() {
    // TODO: Add tilemap to game
    GameBuilder::init(
        "Tileset Tilemap",
        800,
        800,
        &mut TestGame::new(&mut Renderer::new(), &mut Eventer),
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
