use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct Texture {
    pub color: Color,
}

pub struct Tile {
    x: i32,
    y: i32,
    texture: Texture,
}

impl Tile {
    pub fn init(x: i32, y: i32, texture: Texture) -> Self {
        Self { x, y, texture }
    }

    // TODO: Proper error handling
    fn render(&mut self, canvas: &mut Canvas<Window>, tile_width: u32, tile_height: u32) {
        let tile = Rect::new(self.x, self.y, tile_width, tile_height);

        // TODO: Specify tile color? (Texture struct)
        canvas.set_draw_color(self.texture.color);
        canvas.fill_rect(tile).expect("Error drawing the tile");
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn texture(&mut self) -> &mut Texture {
        &mut self.texture
    }
}

pub struct TileMap {
    width: u32,
    height: u32,
    tile_width: u32,
    tile_height: u32,
    tiles: Vec<Tile>,
}

impl TileMap {
    pub fn init(width: u32, height: u32, tile_width: u32, tile_height: u32) -> Self {
        let tiles = Vec::new();

        Self {
            width,
            height,
            tile_width,
            tile_height,
            tiles,
        }
    }

    pub fn insert_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }

    pub fn insert_tiles(&mut self, mut tiles: Vec<Tile>) {
        self.tiles.append(&mut tiles);
    }

    // TODO: Make sure canvas is set before this is called
    pub fn generate(&mut self, mut canvas: &mut Canvas<Window>) {
        for tile in &mut self.tiles {
            tile.render(&mut canvas, self.tile_width, self.tile_height);
        }
    }

    pub fn tiles(&mut self) -> &mut [Tile] {
        self.tiles.as_mut()
    }
}
