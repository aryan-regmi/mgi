use crate::{textures::Texture, utils::Vec2};

struct Tile {
    // Position in number of tiles
    position: Vec2,

    // Index in the tileset that represents the underlying texture
    idx: usize,
}

pub enum TileSetID {
    NamedTileSet(String),
    NumberedTileSet(usize),
}

pub struct TileSet {
    id: TileSetID,
    tiles: Vec<Texture>,
}

impl TileSet {
    pub fn new(id: TileSetID) -> Self {
        Self {
            id,
            tiles: Vec::new(),
        }
    }

    pub fn add_tile_type(&mut self, texture: Texture) {
        self.tiles.push(texture);
    }

    fn get_idx_from_name(&self, name: &str) -> Option<usize> {
        for (idx, tile) in self.tiles.iter().enumerate() {
            if tile.name == name {
                return Some(idx);
            }
        }

        None
    }
}

pub struct TileMap {
    tilesets: Vec<TileSet>,
}

impl TileMap {
    pub fn new() -> Self {
        Self {
            tilesets: Vec::new(),
        }
    }
}
