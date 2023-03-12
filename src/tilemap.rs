use std::cell::RefMut;

use raylib::{
    prelude::{Color, RaylibDraw, Rectangle, Vector2},
    texture::RaylibTexture2D,
};

use crate::{
    game_builder::{Drawable, MgiResult, ResourceManager},
    utils::Vec2,
};

struct Tile {
    /// Position in number of tiles
    position: Vec2,

    /// Index in the tileset that represents the underlying texture
    idx: usize,

    #[allow(unused)]
    /// The TileSet this tile belongs to.
    tileset: TileSetID,
}

pub enum TileSetID {
    NamedTileSet(String),
    NumberedTileSet(usize),
}

pub struct TileSet {
    #[allow(unused)]
    id: TileSetID,
    tile_texture_names: Vec<&'static str>,
}

// TODO: Add abitily to create TileSet from a TextureAtlas
impl TileSet {
    pub fn new(id: TileSetID) -> Self {
        Self {
            id,
            tile_texture_names: Vec::new(),
        }
    }

    pub fn add_tile_type(&mut self, texture: &'static str) {
        self.tile_texture_names.push(texture);
    }

    #[allow(unused)]
    /// Returns the index of the tileset where the texture of `name` is stored.
    fn get_idx_from_name(&self, name: &str) -> Option<usize> {
        for (idx, tile_texture_name) in self.tile_texture_names.iter().enumerate() {
            if *tile_texture_name == name {
                return Some(idx);
            }
        }

        None
    }
}

pub type TileIndexFunc = Box<dyn Fn(i32, i32) -> (TileSetID, usize)>;

pub struct TileMap {
    size: Vec2,
    tile_size: Vec2, // TODO: Add capability for varying tile sizes in the map
    tilesets: Vec<TileSet>,
    tile_idx_fn: TileIndexFunc,
    layer: isize,
}

impl TileMap {
    // TODO: Add capability for scrolling/infinite tilemap
    pub fn init(size: Vec2, tile_size: Vec2, tile_idx_fn: TileIndexFunc) -> Self {
        Self {
            size,
            tile_size,
            tilesets: Vec::new(),
            tile_idx_fn,
        }
    }

    pub fn add_tileset(mut self, tileset: TileSet) -> Self {
        self.tilesets.push(tileset);
        self
    }

    pub fn add_tilesets(mut self, mut tilesets: Vec<TileSet>) -> Self {
        self.tilesets.append(&mut tilesets);

        self
    }
}

impl Drawable for TileMap {
    fn render(
        &mut self,
        renderer: &crate::prelude::Renderer,
        resources: &ResourceManager,
    ) -> MgiResult<()> {
        let (mut rl, rt) = (renderer.rl(), renderer.rt());
        let (w, h) = (self.size.x, self.size.y);
        let mut d = rl.begin_drawing(rt);

        // Combine all tilesets into one
        let mut tile_textures: Vec<String> =
            Vec::with_capacity(self.tilesets.len() * self.tilesets[0].tile_texture_names.len());
        for tileset in &self.tilesets {
            for tile_texture in &tileset.tile_texture_names {
                tile_textures.push(tile_texture.to_string());
            }
        }

        for i in 0..w {
            for j in 0..h {
                let (tileset_id, idx) = (self.tile_idx_fn)(i, j);
                let tile = Tile {
                    position: (i, j).into(),
                    idx,
                    tileset: tileset_id,
                };

                // Get texture from corresponding tileset
                // TODO: Proper error handling
                let texture_manager = resources.texture_manager();
                let texture = texture_manager
                    .as_ref()
                    .unwrap()
                    .get_texture(&tile_textures[tile.idx])
                    .unwrap()
                    .raw_texture()
                    .unwrap();

                // Create source and destination rectangles
                let src = Rectangle::new(0., 0., texture.width() as f32, texture.height() as f32);
                let (x, y) = (
                    (tile.position.x * self.tile_size.x) as f32,
                    (tile.position.y * self.tile_size.y) as f32,
                );
                let dest = Rectangle::new(x, y, self.tile_size.x as f32, self.tile_size.y as f32);

                // Draw the texture
                d.draw_texture_pro(
                    texture,
                    src,
                    dest,
                    Vector2::new(0., 0.),
                    0.,
                    Color::new(255, 255, 255, 126), // TODO: Make this changeable
                );
            }
        }

        Ok(())
    }

    fn layer(&self) -> isize {
        self.layer
    }

    fn set_layer(&mut self, layer: isize) {
        self.layer = layer;
    }
}

pub struct TileMapRef<'t>(pub(crate) RefMut<'t, TileMap>);

impl<'t> Drawable for TileMapRef<'t> {
    fn render(
        &mut self,
        renderer: &crate::prelude::Renderer,
        resources: &ResourceManager,
    ) -> MgiResult<()> {
        self.0.render(renderer, resources)
    }

    fn layer(&self) -> isize {
        self.0.layer()
    }

    fn set_layer(&mut self, layer: isize) {
        self.0.set_layer(layer)
    }
}
