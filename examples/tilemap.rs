use mgi::prelude::*;

struct MyGame {
    running: bool,
}

impl Game for MyGame {
    fn init() -> Self {
        Self { running: true }
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn update(&mut self, ctx: &mut mgi::prelude::Context) -> MgiResult<()> {
        if ctx.is_keydown(Keycode::Escape) || ctx.is_keydown(Keycode::Backspace) {
            self.running = false;
        }

        Ok(())
    }

    fn render(&mut self, ctx: &mut mgi::prelude::Context) -> MgiResult<()> {
        const TILEMAP: usize = 0;
        const PERSON: usize = TILEMAP + 1;

        ctx.draw_texture(
            "person",
            Some(Rectangle::new((400, 550).into(), 128, 128, Color::WHITE)),
            PERSON,
        )?;

        ctx.draw_tilemap(0, None, Some(0.6), TILEMAP);

        Ok(())
    }
}

fn main() -> MgiResult<()> {
    let mut texture_manager = TextureManager::new();
    texture_manager.add_texture("bg", "./examples/assets/bg.png");
    texture_manager.add_texture("person", "./examples/assets/person.png");
    texture_manager.add_texture("ground", "./examples/assets/tileset/ground.png");
    texture_manager.add_texture("water", "./examples/assets/tileset/water.png");

    let mut tileset = TileSet::new();
    tileset.add_tile_type("ground");
    tileset.add_tile_type("water");

    let mut tilemap = TileMap::new(100, 100, (32, 32));
    tilemap.add_tileset(tileset);
    tilemap.add_tile_placement_fn(Box::new(|_, y| {
        if y < 20 {
            return (0, 1);
        }

        (0, 0)
    }));

    GameBuilder::<MyGame>::init("TileMap", (800, 800))?
        .add_texture_manager(texture_manager)
        .add_tilemap(tilemap)
        .run()?;

    Ok(())
}
