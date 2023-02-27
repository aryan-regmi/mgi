#![allow(unused)]

use std::{collections::HashMap, path::Path};

use sdl2::{image::LoadTexture, pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::Drawable;

/// Stores a texture's path and name.
pub struct TileSet<'a>(HashMap<&'a str, &'a str>);
impl<'a> TileSet<'a> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_tile_type(&mut self, tile_type_name: &'a str, texture_path: &'a str) {
        self.0.insert(tile_type_name, texture_path);
    }
}

pub struct TextureAtlas;

pub struct Tile {
    // Row of tile map for tile
    row: u32,

    // Column of tile map for tile
    col: u32,

    // Texture of the tile
    texture_idx: u32,
}

pub struct TileMap<TextureType> {
    /// Number of rows in tilemap
    rows: u32,

    /// Number of columns in tilemap
    cols: u32,

    /// Size of a single tile
    tile_size: (u32, u32),

    /// All the tiles in the map
    tiles: Vec<Tile>,

    /// Type of the texture for the map
    texture: TextureType,
}

impl<'a> TileMap<TileSet<'a>> {
    pub fn from_tileset(
        rows: u32,
        cols: u32,
        tile_size: (u32, u32),
        tile_set: TileSet<'a>,
    ) -> Self {
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

impl<'a> Drawable for TileMap<TileSet<'a>> {
    fn setup(&mut self, canvas: &mut Canvas<Window>) {
        // Load all textures

        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) {
        todo!()
    }
}
