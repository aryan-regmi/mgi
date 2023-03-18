```rust

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

        let mut tilemap = ctx
            .layer_manager()
            .get_layer("tilemap_layer")
            .get_drawable_mut("bg_tex");

        // set_tile gets x, y in screen coords and a new Tile to replace the tile at that posistion
        // with.
        //
        // It will have to convert the position to tile_coords and grab that tile and replace it
        //
        // Returns error if no tile is at that location
        tilemap.set_tile(
            30,
            40,
            Tile {
                tileset: "main",
                texture_name: "ground",
            },
        );

        Ok(())
    }

    fn render(&mut self, ctx: &mut mgi::prelude::Context) -> MgiResult<()> {
        // NOTE: All drawables will have a `set_opacity` function to set the opacity

        // ctx.draw_rect(x, y, width, height, color, alpha)
        // ctx.draw_line(x1, y1, x2, y2, color, alpha)

        ctx.draw_layer(ctx.layer_manager().get_layer("background"));

        Ok(())
    }
}

fn main() -> MgiResult<()> {
    let mut texture_manager = TextureManager::new();
    texture_manager.add_texture("bg", "./examples/assets/bg.png");
    texture_manager.add_texture("person", "./examples/assets/person.png");
    texture_manager.add_texture("ground", "./examples/assets/tileset/ground.png");
    texture_manager.add_texture("water", "./examples/assets/tileset/water.png");

    let mut tileset = TileSetBuilder::init("main")
        .add_tile_type("ground")
        .add_tile_type("water")
        .build();

    let mut tilemap = TileMapBuilder::init(100, 100, (32, 32))
        .add_tileset(tileset)
        .add_tile_placement_fn(Box::new(|_, y| {
            if y < 20 {
                return ("main", "ground");
            }

            ("main", "water")
        }))
        .build();

    let mut layer_manager = LayerManager::new();
    layer_manager.add_layer(Layer::init("bg_layer", 0).set_opacity(1.0).add_object(
        "bg_tex",
        Texture {
            name: "bg",
            position: (0, 0),
            rotation: 0.,
        },
    ));
    layer_manager.add_layer(
        Layer::init("tilemap_layer", 1)
            .set_opacity(0.5)
            .add_object("tilemap", tilemap),
    );

    GameBuilder::<MyGame>::init("TileMap", (800, 800))?
        .add_texture_manager(texture_manager)
        .add_layer_manager(layer_manager)
        .add_tilemap(tilemap)
        .run()?;

    Ok(())
}

```
