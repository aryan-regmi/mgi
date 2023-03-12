use raylib::prelude::Color;

pub trait Layer {
    /// The layer the object belongs to.
    fn layer(&self) -> usize {
        0
    }

    /// Determines whether to show the layer or not.
    fn visible(&self) -> bool;

    /// The color to tint the layer with.
    fn tint(&self) -> Color;
}
