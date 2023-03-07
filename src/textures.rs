use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

use image::{DynamicImage, ImageFormat};

pub struct TextureManager {
    textures: HashMap<String, DynamicImage>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn add_texture<P: AsRef<Path>>(
        &mut self,
        name: String,
        path: P,
    ) -> Result<(), Box<dyn Error>> {
        let rdr = BufReader::new(File::open(path)?);
        let texture = image::load(rdr, ImageFormat::Png)?;

        self.textures.insert(name, texture);

        Ok(())
    }

    pub fn add_textures<P: AsRef<Path>>(
        &mut self,
        paths: Vec<(String, P)>,
    ) -> Result<(), Box<dyn Error>> {
        for (name, path) in paths {
            self.add_texture(name, path)?;
        }

        Ok(())
    }

    pub fn get_texture(&self, name: &str) -> Option<&DynamicImage> {
        self.textures.get(name)
    }

    pub fn get_texture_mut(&mut self, name: &str) -> Option<&mut DynamicImage> {
        self.textures.get_mut(name)
    }
}
