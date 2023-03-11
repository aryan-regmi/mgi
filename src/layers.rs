use raylib::prelude::Rectangle;

pub struct Layer<'l, T> {
    pub(crate) objects: Vec<&'l T>,
    pub(crate) object_srcs: Vec<Option<Rectangle>>,
    pub(crate) object_dests: Vec<Rectangle>,
    pub(crate) object_rotations: Vec<f32>,
    pub(crate) visible: bool,
}

impl<'l, T> Layer<'l, T> {
    pub fn init() -> Self {
        Self {
            objects: Vec::new(),
            object_srcs: Vec::new(),
            object_dests: Vec::new(),
            object_rotations: Vec::new(),
            visible: true,
        }
    }

    /// The objects are rendered in the order that they are added to the layer.
    pub fn add_obj(
        mut self,
        obj: &'l T,
        src: Option<Rectangle>,
        dest: Rectangle,
        rotation: f32,
    ) -> Self {
        self.objects.push(obj);
        self.object_srcs.push(src);
        self.object_dests.push(dest);
        self.object_rotations.push(rotation);

        self
    }

    /// The vector of layer objects will be rendered in the order of the vector.
    pub fn add_objects(mut self, objects: Vec<(&'l T, Option<Rectangle>, Rectangle, f32)>) -> Self {
        for obj in objects {
            self.objects.push(obj.0);
            self.object_srcs.push(obj.1);
            self.object_dests.push(obj.2);
            self.object_rotations.push(obj.3);
        }

        self
    }
}

pub type TextureLayer<'l> = Layer<'l, &'l str>;
