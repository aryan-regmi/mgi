use std::error::Error;

use mgi::{prelude::*, utils::Vec2};

struct MyGame {
    running: bool,
}

impl Drawable for MyGame {
    fn render(&mut self, renderer: &Renderer, resources: &ResourceManager) {
        resources.tilemap().unwrap().render(renderer, resources);
    }
}

impl Updateable for MyGame {
    fn update(&mut self) {}
}

impl Game for MyGame {
    fn setup() -> Self {
        Self { running: true }
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn stop(&mut self) {
        self.running = false;
    }

    fn handle_events(&mut self, rl: &raylib::RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE)
            || rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
        {
            self.stop();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut texture_manager = TextureManager::new();
    texture_manager.add_texture("water", "examples/assets/tileset/water.png");
    texture_manager.add_texture("ground", "examples/assets/tileset/ground.png");

    let mut tileset = TileSet::new(TileSetID::NumberedTileSet(0));
    tileset.add_tile_type("water");
    tileset.add_tile_type("ground");

    let tilemap = TileMap::init(
        Vec2::new(800 / 32, 800 / 32),
        Vec2::new(32, 32),
        Box::new(|_, y| {
            if y > 600 / 32 {
                (TileSetID::NumberedTileSet(0), 0)
            } else {
                (TileSetID::NumberedTileSet(0), 1)
            }
        }),
    )
    .add_tileset(tileset);

    GameBuilder::<MyGame>::init("Tilemap", (800, 800))
        .add_texture_manager(texture_manager)
        .add_tilemap(tilemap)
        .run()?;

    Ok(())
}
