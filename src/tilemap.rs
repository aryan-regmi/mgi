use crate::{game_builder::Drawable, textures::Texture, utils::Vec2};

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

pub struct TileSet<'t> {
    id: TileSetID,
    tiles: Vec<&'t Texture>,
}

// TODO: Add abitily to create TileSet from a TextureAtlas
impl<'t> TileSet<'t> {
    pub fn new(id: TileSetID) -> Self {
        Self {
            id,
            tiles: Vec::new(),
        }
    }

    pub fn add_tile_type(&mut self, texture: &'t Texture) {
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

pub struct TileMap<'t> {
    size: Vec2,
    tile_size: Vec2, // TODO: Add capability for varying tile sizes in the map
    tilesets: Vec<TileSet<'t>>,
}

impl<'t> TileMap<'t> {
    // TODO: Add capability for scrolling/infinite tilemap
    pub fn init(size: Vec2, tile_size: Vec2) -> Self {
        Self {
            size,
            tile_size,
            tilesets: Vec::new(),
        }
    }

    pub fn add_tileset(mut self, tileset: TileSet<'t>) -> Self {
        self.tilesets.push(tileset);
        self
    }

    pub fn add_tilesets(mut self, mut tilesets: Vec<TileSet<'t>>) -> Self {
        self.tilesets.append(&mut tilesets);

        self
    }
}

impl<'t> Drawable for TileMap<'t> {
    fn render(
        &mut self,
        renderer: &crate::prelude::Renderer,
        texture_manager: &Option<crate::prelude::TextureManagerRef>,
    ) {
    }
}
