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

pub trait Colorable {
    fn set_color(&mut self, color: Rgba);
}

impl Colorable for &mut [u8] {
    fn set_color(&mut self, color: Rgba) {
        self.copy_from_slice(&color.as_raw());
    }
}
