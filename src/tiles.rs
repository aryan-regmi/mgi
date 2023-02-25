#![allow(unused)]

use std::{collections::HashMap, path::Path};

use sdl2::{image::LoadTexture, pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::Drawable;

trait TileMapType {}

pub struct Texture {
    filepath: String,
}

impl Texture {
    pub fn new(path: &str) -> Self {
        Self {
            filepath: path.into(),
        }
    }
}

pub struct TileSet(HashMap<u32, (Texture, Option<String>)>);
impl TileMapType for TileSet {}
impl TileSet {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    // TODO: Make this take an enum and texture instead
    pub fn add_tile_type(&mut self, id: u32, texture: Texture, tile_type_name: Option<String>) {
        self.0.insert(id, (texture, tile_type_name));
    }
}

pub struct TextureAtlas;
impl TileMapType for TextureAtlas {}

pub struct Tile {
    // Row of tile map for tile
    row: u32,

    // Column of tile map for tile
    col: u32,

    // Texture of the tile
    texture_idx: u32,
}

pub struct TileMap<Texture> {
    // Number of rows in tilemap
    rows: u32,

    // Number of columns in tilemap
    cols: u32,

    // Size of a single tile
    tile_size: (u32, u32),

    // All the tiles in the map
    tiles: Vec<Tile>,

    // Type of the texture for the map
    texture: Texture,
}

impl<'a> TileMap<TileSet> {
    pub fn from_tileset(rows: u32, cols: u32, tile_size: (u32, u32), tile_set: TileSet) -> Self {
        Self {
            rows,
            cols,
            tile_size,
            tiles: Vec::with_capacity((rows * cols) as usize),
            texture: tile_set,
        }
    }
}

impl TileMap<TextureAtlas> {
    pub fn from_texture_atlas(
        rows: u32,
        cols: u32,
        tile_size: (u32, u32),
        texture_atlas: TextureAtlas,
    ) -> Self {
        Self {
            rows,
            cols,
            tile_size,
            tiles: Vec::with_capacity((rows * cols) as usize),
            texture: texture_atlas,
        }
    }
}

impl<T: TileMapType> Drawable for TileMap<T> {
    fn setup(&mut self, canvas: &mut Canvas<Window>) {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        for tile in &self.tiles {}
    }
}
