use raylib::prelude::Color;

pub trait Layer {
    /// The layer the object belongs to.
    fn layer(&self) -> usize {
        0
    }
    fn set_layer(&mut self, layer: usize);

    /// Determines whether to show the layer or not.
    fn visible(&self) -> bool {
        true
    }
    fn set_visible(&mut self, visible: bool);

    /// The color to tint the layer with.
    fn tint(&self) -> Color;
    fn set_tint(&mut self, tint: Color);
}

pub(crate) struct LayerInfo {
    pub(crate) layer_id: usize,
    pub(crate) visible: bool,
    pub(crate) tint: Color,
}

impl Default for LayerInfo {
    fn default() -> Self {
        Self {
            layer_id: 0,
            visible: true,
            tint: Color::WHITE,
        }
    }
}
