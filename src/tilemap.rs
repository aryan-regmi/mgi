use sdl2::pixels::Color;

use crate::prelude::{Rectangle, Rotation, Vec2};

pub struct Tile {
    pub(crate) rect: Rectangle,
    pub(crate) rotation: Rotation,
    pub(crate) texture_idx: (usize, usize),
}

pub struct TileSet {
    /// Vector of tile types (texture names)
    tile_types: Vec<String>,
}

impl TileSet {
    pub fn new() -> Self {
        Self {
            tile_types: Vec::new(),
        }
    }

    pub fn add_tile_type(&mut self, texture_name: &str) {
        self.tile_types.push(texture_name.into());
    }
}

pub type TilePlacementFn = Box<dyn Fn(i32, i32) -> (usize, usize)>;

// TODO: Make sure tile_placement_fn is set before rendering TileMap
// TODO: Add capability for differently sized tiles?
pub struct TileMap {
    pub(crate) id: usize,

    pub(crate) nrows: usize,
    pub(crate) ncols: usize,

    pub(crate) tile_size: (usize, usize),

    pub(crate) tilesets: Vec<TileSet>,

    pub(crate) tiles: Vec<Tile>,

    /// Function to choose tile texture given a position
    pub(crate) tile_placement_fn: Option<TilePlacementFn>,
}

// TODO: Implement layers for tilemaps correctly!
impl TileMap {
    pub fn new(nrows: usize, ncols: usize, tile_size: (usize, usize)) -> Self {
        Self {
            id: 0,
            ncols,
            nrows,
            tile_size,
            tilesets: Vec::new(),
            tiles: Vec::new(),
            tile_placement_fn: None,
        }
    }

    pub fn add_tileset(&mut self, tileset: TileSet) {
        self.tilesets.push(tileset);
    }

    pub fn add_tilesets(&mut self, mut tilesets: Vec<TileSet>) {
        self.tilesets.append(&mut tilesets);
    }

    pub fn add_tile_placement_fn(&mut self, f: TilePlacementFn) {
        self.tile_placement_fn = Some(f);
    }

    pub fn tiles(&self) -> &[Tile] {
        self.tiles.as_ref()
    }

    pub(crate) fn get_texture_name(&self, texture_idx: (usize, usize)) -> &str {
        &self.tilesets[texture_idx.0].tile_types[texture_idx.1]
    }

    // TODO: (DO PROPER ERROR HANDLING)
    /// Generates all the tiles using the provied `tile_placement_fn`
    pub(crate) fn generate(&mut self) {
        // NOTE: `tile_placement_fn` must be set first

        for x in 0..self.nrows {
            for y in 0..self.ncols {
                let texture_idx = (self.tile_placement_fn).as_ref().unwrap()(x as i32, y as i32);

                let tile_pos = (x * self.tile_size.0, y * self.tile_size.1);

                let tile = Tile {
                    rect: Rectangle::new(
                        Vec2::new(tile_pos.0 as i32, tile_pos.1 as i32),
                        self.tile_size.0 as u32,
                        self.tile_size.1 as u32,
                        Color::WHITE,
                    ),
                    rotation: Rotation::Degrees(0.),
                    texture_idx,
                };

                self.tiles.push(tile);
            }
        }
    }
}
