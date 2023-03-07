use image::Pixel;

#[derive(Debug, Copy, Clone)]
pub struct Rgba {
    red: u8,
    green: u8,
    blue: u8,
    alpha: f32,
}

impl Rgba {
    pub fn new(red: u8, green: u8, blue: u8, alpha: f32) -> Result<Self, String> {
        if alpha > 1.0 || alpha < 0.0 {
            return Err("Alpha value must be between 0 and 1".into());
        }

        Ok(Self {
            red,
            green,
            blue,
            alpha,
        })
    }

    pub fn as_raw(&self) -> [u8; 4] {
        let alpha = (self.alpha * 255.) as u8;
        [self.red, self.green, self.blue, alpha]
    }
}

impl From<[u8; 4]> for Rgba {
    fn from(color: [u8; 4]) -> Self {
        let alpha = color[3] as f32 / 255.0;
        Self {
            red: color[0],
            green: color[1],
            blue: color[2],
            alpha,
        }
    }
}

impl From<(u32, u32, image::Rgba<u8>)> for Rgba {
    fn from(px: (u32, u32, image::Rgba<u8>)) -> Self {
        let rgba = px.2.channels();

        Self {
            red: rgba[0],
            green: rgba[1],
            blue: rgba[2],
            alpha: (rgba[3] / 255) as f32,
        }
    }
}

impl From<&image::Rgba<u8>> for Rgba {
    fn from(px: &image::Rgba<u8>) -> Self {
        let rgba = px.channels();

        Self {
            red: rgba[0],
            green: rgba[1],
            blue: rgba[2],
            alpha: (rgba[3] / 255) as f32,
        }
    }
}

pub trait Colorable {
    fn set_color(&mut self, color: Rgba);
}

impl Colorable for &mut [u8] {
    fn set_color(&mut self, color: Rgba) {
        self.copy_from_slice(&color.as_raw());
    }
}
