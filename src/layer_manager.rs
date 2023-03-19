use std::collections::HashMap;

pub trait Layerable {}

pub struct Layer {
    layer_name: String,
    layer_idx: usize,
    objects: HashMap<String, Box<dyn Layerable>>,
    alpha: f32,
}

pub struct LayerBuilder {
    layer_name: String,
    layer_idx: usize,
    objects: HashMap<String, Box<dyn Layerable>>,
    alpha: f32,
}

impl LayerBuilder {
    pub fn init(name: &str, idx: usize) -> Self {
        Self {
            layer_name: name.into(),
            layer_idx: idx,
            objects: HashMap::new(),
            alpha: 0.0,
        }
    }

    pub fn set_opacity(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    pub fn add_object<T: Layerable + 'static>(mut self, object_name: &str, object: T) -> Self {
        self.objects.insert(object_name.into(), Box::new(object));
        self
    }
}

pub struct LayerManager {
    layers: Vec<Layer>,
}

impl LayerManager {
    pub fn new() -> Self {
        Self { layers: vec![] }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }
}
